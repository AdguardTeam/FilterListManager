use crate::storage::blob::BlobHandle;
use std::io::{Error as IOError, Read, Result as IOResult};

/// Sequential [`Read`] adapter over a [`BlobHandle`].
///
/// The adapter hides blob offset management and exposes the underlying SQLite
/// blob as a forward-only byte stream suitable for standard buffered I/O.
pub(crate) struct BlobReader<B: BlobHandle> {
    blob: B,
    offset: usize,
}

impl<B: BlobHandle> BlobReader<B> {
    /// Creates a new sequential reader over the provided blob handle.
    pub(crate) fn new(blob: B) -> Self {
        Self { blob, offset: 0 }
    }
}

impl<B: BlobHandle> Read for BlobReader<B> {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let bytes_read = self
            .blob
            .read_at(buf, self.offset)
            .map_err(IOError::other)?;

        self.offset += bytes_read;
        Ok(bytes_read)
    }
}
