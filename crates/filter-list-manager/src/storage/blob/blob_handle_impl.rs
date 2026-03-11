use crate::storage::blob::BlobHandle;
use rusqlite::{blob::Blob, Result};

/// Thin wrapper over [`Blob`] implementing [`BlobHandle`].
pub(crate) struct BlobHandleImpl<'conn> {
    inner: Blob<'conn>,
}

impl<'conn> BlobHandleImpl<'conn> {
    /// Creates a new blob handle wrapper.
    pub(in crate::storage) fn new(blob: Blob<'conn>) -> Self {
        Self { inner: blob }
    }
}

impl BlobHandle for BlobHandleImpl<'_> {
    fn read_at(&self, buf: &mut [u8], read_start: usize) -> Result<usize> {
        self.inner.read_at(buf, read_start)
    }
}
