use crate::{FLMError, FLMResult};
use nom::bytes::complete::{tag, take_until};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;
use rusqlite::{blob::Blob, Result};
use std::collections::HashSet;
use std::io::Write;

#[cfg(test)]
pub const BLOB_CHUNK_SIZE: usize = 12; // 12 bytes for "hello world\n"
#[cfg(not(test))]
pub const BLOB_CHUNK_SIZE: usize = 1024 * 1024; // 1 mb

/// Blob handle wrapper
pub(crate) trait BlobHandle {
    /// Inherits behaviour of [`Blob::read_at`]
    fn read_at(&self, buf: &mut [u8], read_start: usize) -> Result<usize>;
}

/// Blob handler wrapper with [`BlobHandle`] trait implementation
pub(crate) struct BlobHandleImpl<'conn> {
    inner: Blob<'conn>,
}

impl<'conn> BlobHandleImpl<'conn> {
    pub(in crate::storage) fn new(blob: Blob<'conn>) -> Self {
        Self { inner: blob }
    }
}

impl BlobHandle for BlobHandleImpl<'_> {
    fn read_at(&self, buf: &mut [u8], read_start: usize) -> Result<usize> {
        self.inner.read_at(buf, read_start)
    }
}

/// Divides the text into several lines and the rest of text as a *remainder*
fn parse_blob_chunk(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>, nom::error::Error<&[u8]>> {
    many0(terminated(take_until("\n"), tag(b"\n")))(input)
}

/// [`BlobHandle`] blob chunking into [`Write`] stream.
pub(crate) fn write_to_stream<W, B>(
    stream: &mut W,
    blob: B,
    disabled_rules_set: HashSet<Vec<u8>>,
) -> FLMResult<()>
where
    W: Write,
    B: BlobHandle,
{
    let mut offset = 0;
    let mut buffer: Vec<u8> = vec![0; BLOB_CHUNK_SIZE];
    let mut accumulator = Vec::new();

    loop {
        // Read the buffer
        let bytes_read = blob.read_at(&mut buffer, offset)?;
        if bytes_read == 0 {
            if !accumulator.is_empty() && !disabled_rules_set.contains(&accumulator) {
                stream.write(&accumulator).map_err(FLMError::from_io)?;
            }

            break;
        }

        accumulator.extend_from_slice(&buffer[0..bytes_read]);

        let current_accum_len = accumulator.len();
        let (remainder, matched_chunks) =
            parse_blob_chunk(&accumulator).map_err(FLMError::from_display)?;

        if !matched_chunks.is_empty() {
            // Write found chunks
            for chunk in matched_chunks {
                if !disabled_rules_set.contains(chunk) {
                    stream.write(chunk).map_err(FLMError::from_io)?;
                    stream.write(b"\n").map_err(FLMError::from_io)?;
                }
            }

            // Move remainder from end to start and shrinks length without extra allocations
            let remainder_len = remainder.len();
            accumulator.copy_within((current_accum_len - remainder_len).., 0);
            accumulator.truncate(remainder_len);
        }

        offset += bytes_read;
    }

    Ok(())
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

        write_to_stream(&mut fake_file, blob, disabled_rules_set).unwrap();

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
