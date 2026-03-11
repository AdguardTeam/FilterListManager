/// SQLite blob handle wrapper implementation.
pub(crate) mod blob_handle_impl;
/// Buffered sequential blob reading helpers.
pub(crate) mod blob_reader;
/// Buffered wrapper for sequential blob readers.
pub(crate) mod buffered_blob_reader;
pub(crate) mod filter_stream;

use crate::{FLMError, FLMResult};
use rusqlite::Result;
use std::collections::HashSet;
use std::io::{BufRead, Write};

pub(crate) use blob_handle_impl::BlobHandleImpl;
pub(crate) use buffered_blob_reader::{create_buffered_reader, BufferedBlobReader};

#[cfg(test)]
pub const BLOB_CHUNK_SIZE: usize = 12; // 12 bytes for "hello world\n"
#[cfg(not(test))]
pub const BLOB_CHUNK_SIZE: usize = 1024 * 1024; // 1 mb

/// Blob handle wrapper
pub(crate) trait BlobHandle {
    /// Inherits behaviour of [`Blob::read_at`]
    fn read_at(&self, buf: &mut [u8], read_start: usize) -> Result<usize>;
}

/// Streams a blob into a [`Write`] sink line by line.
///
/// Disabled lines are skipped using exact byte matching against
/// `disabled_rules_set`.
///
/// Returns `true` when the last emitted line ended with `\n`, otherwise `false`.
pub(crate) fn write_to_stream<W, B>(
    stream: &mut W,
    blob: B,
    disabled_rules_set: &HashSet<Vec<u8>>,
) -> FLMResult<bool>
where
    W: Write,
    B: BlobHandle,
{
    let mut reader = create_buffered_reader(blob);
    let mut line_buf = Vec::new();
    let mut ended_with_newline = false;

    while let Some(has_newline) = read_next_line(&mut reader, &mut line_buf)? {
        if !disabled_rules_set.contains(&line_buf) {
            stream.write_all(&line_buf).map_err(FLMError::from_io)?;

            if has_newline {
                stream.write_all(b"\n").map_err(FLMError::from_io)?;
            }

            ended_with_newline = has_newline;
        }
    }

    Ok(ended_with_newline)
}

/// Reads the next line from a buffered blob reader.
///
/// The resulting bytes are written into `line_buf` without the trailing `\n`.
///
/// Returns:
/// - `Ok(Some(true))` if a line was read and ended with `\n`
/// - `Ok(Some(false))` if the final unterminated line was read at EOF
/// - `Ok(None)` if no more data is available
pub(crate) fn read_next_line<R: BufRead>(
    reader: &mut R,
    line_buf: &mut Vec<u8>,
) -> FLMResult<Option<bool>> {
    line_buf.clear();

    let bytes_read = reader
        .read_until(b'\n', line_buf)
        .map_err(FLMError::from_io)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    let has_newline = line_buf.ends_with(b"\n");

    if has_newline {
        line_buf.pop();
    }

    Ok(Some(has_newline))
}

#[cfg(test)]
mod tests {
    use super::write_to_stream;
    use super::BlobHandle;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::io::Cursor;
    use std::io::Read;
    use std::io::Seek;

    struct TestBlobHandle {
        storage: RefCell<Cursor<Vec<u8>>>,
    }

    impl TestBlobHandle {
        fn new(buf: Vec<u8>) -> Self {
            Self {
                storage: RefCell::new(Cursor::new(buf)),
            }
        }
    }

    impl BlobHandle for TestBlobHandle {
        fn read_at(&self, buf: &mut [u8], read_start: usize) -> rusqlite::Result<usize> {
            let mut handle = self.storage.borrow_mut();
            handle.set_position(read_start as u64);
            let bytes_read = handle.read(buf).unwrap();

            Ok(bytes_read)
        }
    }

    /// For testing purposes
    fn write_to_stream_test_internal(data: &str, disabled_rules_set: HashSet<Vec<u8>>) -> String {
        let blob = TestBlobHandle::new(data.as_bytes().to_vec());
        let mut fake_file = Cursor::new(Vec::new());

        write_to_stream(&mut fake_file, blob, &disabled_rules_set).unwrap();

        let mut test_string = String::new();

        fake_file.rewind().unwrap();
        fake_file.read_to_string(&mut test_string).unwrap();

        test_string
    }

    #[test]
    fn test_write_to_stream() {
        let data = "\nHello world!\n Hello world, again and again\n00";
        let test_string = write_to_stream_test_internal(data, HashSet::new());

        assert_eq!(test_string.get((test_string.len() - 3)..), Some("\n00"));
        assert!(test_string.starts_with("\nHello world!\n H"));
    }

    #[test]
    fn test_write_to_stream_empty_string() {
        let test_string = write_to_stream_test_internal("", HashSet::new());
        assert!(test_string.is_empty())
    }

    #[test]
    fn test_write_to_stream_with_disabled_rules() {
        let text = "Hallo world!\n This line won't be removed; \nfewf\n\n\n123456890";
        let disabled_rules = HashSet::from([b"Hallo world!".to_vec(), b"fewf".to_vec()]);

        let test_string = write_to_stream_test_internal(text, disabled_rules);
        assert_eq!(
            test_string.as_str(),
            " This line won't be removed; \n\n\n123456890"
        )
    }

    #[test]
    fn test_write_to_stream_all_disabled_rules() {
        let text = "Hello world!\n Hello world, again and again\n00";
        let disabled_rules = HashSet::from([
            b"Hello world!".to_vec(),
            b" Hello world, again and again".to_vec(),
            b"00".to_vec(),
        ]);

        let test_string = write_to_stream_test_internal(text, disabled_rules);
        assert!(test_string.is_empty())
    }
}
