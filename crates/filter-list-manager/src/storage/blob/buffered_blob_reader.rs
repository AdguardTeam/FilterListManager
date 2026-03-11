use crate::storage::blob::blob_reader::BlobReader;
use crate::storage::blob::{BlobHandle, BLOB_CHUNK_SIZE};
use std::io::BufReader;

/// Buffered blob reader used by streaming blob consumers.
///
/// The buffer capacity is aligned with [`BLOB_CHUNK_SIZE`].
pub(crate) type BufferedBlobReader<B> = BufReader<BlobReader<B>>;

/// Creates a buffered reader for sequential blob consumption.
pub(crate) fn create_buffered_reader<B: BlobHandle>(blob: B) -> BufferedBlobReader<B> {
    BufReader::with_capacity(BLOB_CHUNK_SIZE, BlobReader::new(blob))
}
