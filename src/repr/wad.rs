use std::{
    fmt,
    io::{self, Read, Seek, SeekFrom},
    sync::{Arc, Mutex},
};

/// The kind of resource stored in a WAD entry.
///
/// May be changed in the future to have more variants
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    /// Raw picture (often palettes or small graphics).
    Picture,

    /// Mipmapped texture (used for BSP maps).
    MipTexture,

    /// Bitmap font resource.
    Font,

    /// Any other type
    Other(u8),
}

pub(crate) trait Reader: Read + Seek + Send {}
impl<T: Read + Seek + Send> Reader for T {}

struct SharedChunkReader {
    source: Arc<Mutex<dyn Reader>>,
    begin: usize,
    end: usize,
}

impl Read for SharedChunkReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if self.begin == self.end {
            return Ok(0);
        }

        let mut source = self.source.lock().unwrap();
        source.seek(SeekFrom::Start(self.begin as u64))?;
        let remaining = self.end - self.begin;
        let readlen = buf.len().min(remaining);
        source.read_exact(&mut buf[..readlen])?;
        self.begin += readlen;

        Ok(readlen)
    }
}

/// A single file entry inside a WAD archive.
#[derive(Clone)]
pub struct Entry {
    pub(crate) source: Arc<Mutex<dyn Reader>>,

    /// Byte offset from the start of the WAD file to the entry’s data.
    pub offset: u32,

    /// Uncompressed size of the resource in bytes.
    pub full_size: u32,

    /// Stored size of the resource in bytes (may differ if compressed).
    pub size: u32,

    /// Type of content (texture, palette, etc.).
    pub ty: ContentType,

    /// Compression method identifier (actually always 0).
    pub compression: u8,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("offset", &self.offset)
            .field("full_size", &self.full_size)
            .field("size", &self.size)
            .field("ty", &self.ty)
            .field("compression", &(self.compression != 0))
            .finish_non_exhaustive()
    }
}

impl Entry {
    #[must_use]
    pub fn reader(&self) -> impl Read {
        SharedChunkReader {
            source: Arc::clone(&self.source),
            begin: self.offset as usize,
            end: (self.offset + self.size) as usize,
        }
    }
}
