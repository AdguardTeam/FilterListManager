use crate::filters::parser::collectors::line_processor::{LineProcessor, ProcessedLine};
use crate::filters::parser::paths::try_to_resolve_include_path_from_parent_url;
use crate::storage::blob::filter_stream::FilterStream;
use crate::{Configuration, FLMError, FLMResult};
use std::io::Write;

/// Collects filter contents by streaming blob data through a [`FilterStream`].
pub(crate) struct StreamingFilterCollector<'c> {
    configuration: &'c Configuration,
}

impl<'c> StreamingFilterCollector<'c> {
    pub(crate) fn new(configuration: &'c Configuration) -> Self {
        Self { configuration }
    }

    /// Streams filter rules from the blob through the `filter_stream`,
    /// processing conditional directives and inlining includes.
    pub(crate) fn collect<W: Write>(
        &self,
        filter_stream: &mut FilterStream<'_, W>,
        root_url: &str,
    ) -> FLMResult<()> {
        let mut line_processor = LineProcessor::new(self.configuration);
        let mut line_buf = String::new();

        while let Some(has_newline) = filter_stream.next_line(&mut line_buf)? {
            match line_processor
                .process(&line_buf)
                .map_err(FLMError::from_display)?
            {
                ProcessedLine::Skip => {}
                ProcessedLine::Include(include_path) => {
                    let resolved_url =
                        try_to_resolve_include_path_from_parent_url(root_url, include_path)
                            .map_err(FLMError::from_display)?;

                    filter_stream.write_include(&resolved_url)?;
                }
                ProcessedLine::Rule => {
                    filter_stream.write_line(line_buf.as_bytes(), has_newline)?;
                }
            }
        }

        Ok(())
    }
}
