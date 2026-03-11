use crate::storage::blob::{
    create_buffered_reader, read_next_line, write_to_stream, BlobHandleImpl, BufferedBlobReader,
};
use crate::storage::repositories::filter_includes_repository::FilterIncludesRepository;
use crate::{FLMError, FLMResult};
use std::collections::{HashMap, HashSet};
use std::io::Write;

/// Stateful blob reader + writer for streaming filter rules.
/// Reads lines from a blob, writes rules/includes to a [`Write`] stream.
/// Handles chunked blob reading, line accumulation across chunk boundaries,
/// disabled rules filtering, and include body streaming.
pub(crate) struct FilterStream<'a, W: Write> {
    /// Input BLOB
    input_stream: BufferedBlobReader<BlobHandleImpl<'a>>,
    /// Output write stream
    output_stream: &'a mut W,
    /// Container for single line
    line_accumulator: Vec<u8>,
    /// Disabled rules set for skipping lines in output stream
    disabled_rules_set: &'a HashSet<Vec<u8>>,
    /// Include resolution
    includes_url_to_row_id: &'a HashMap<String, i64>,
    /// Storage connection handle
    conn: &'a rusqlite::Connection,
}

impl<'a, W: Write> FilterStream<'a, W> {
    pub(crate) fn new(
        blob: BlobHandleImpl<'a>,
        stream: &'a mut W,
        disabled_rules_set: &'a HashSet<Vec<u8>>,
        includes_url_to_row_id: &'a HashMap<String, i64>,
        conn: &'a rusqlite::Connection,
    ) -> Self {
        Self {
            input_stream: create_buffered_reader(blob),
            line_accumulator: Vec::new(),
            output_stream: stream,
            disabled_rules_set,
            includes_url_to_row_id,
            conn,
        }
    }

    /// Reads the next line from the blob into `line_buf`.
    ///
    /// Returns:
    /// - `Ok(Some(true))` — line was read and was `\n`-terminated
    /// - `Ok(Some(false))` — line was read but is the final remainder (no trailing `\n`)
    /// - `Ok(None)` — EOF, no more lines
    pub(crate) fn next_line(&mut self, line_buf: &mut String) -> FLMResult<Option<bool>> {
        let Some(has_newline) = read_next_line(&mut self.input_stream, &mut self.line_accumulator)?
        else {
            line_buf.clear();
            return Ok(None);
        };

        let line = std::str::from_utf8(&self.line_accumulator).map_err(FLMError::from_display)?;
        line_buf.clear();
        line_buf.push_str(line);

        Ok(Some(has_newline))
    }

    /// Writes a rule line to the output stream, skipping disabled rules.
    /// If `has_newline` is true, a trailing `\n` is appended.
    pub(crate) fn write_line(&mut self, line: &[u8], has_newline: bool) -> FLMResult<()> {
        if self.disabled_rules_set.contains(line) {
            // Skip
            return Ok(());
        }

        self.output_stream
            .write_all(line)
            .map_err(FLMError::from_io)?;

        if has_newline {
            self.output_stream
                .write_all(b"\n")
                .map_err(FLMError::from_io)?;
        }

        Ok(())
    }

    /// Streams the body of an include (identified by resolved URL) to the output stream.
    /// Include body lines are subject to disabled rules filtering.
    pub(crate) fn write_include(&mut self, resolved_url: &str) -> FLMResult<()> {
        let row_id = self
            .includes_url_to_row_id
            .get(resolved_url)
            .copied()
            .ok_or_else(|| {
                FLMError::from_display(format!(
                    "Could not find include for resolved URL: {}",
                    resolved_url
                ))
            })?;

        let include_blob = FilterIncludesRepository::new()
            .get_include_blob_handle(self.conn, row_id)
            .map_err(FLMError::from_database)?;

        let ended_with_newline =
            write_to_stream(self.output_stream, include_blob, self.disabled_rules_set)?;

        if !ended_with_newline {
            self.output_stream
                .write_all(b"\n")
                .map_err(FLMError::from_io)?;
        }

        Ok(())
    }
}
