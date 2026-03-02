use std::io;

use zerocopy::{
    FromBytes,
    little_endian::{U16, U32},
};
use zerocopy_derive::*;

use crate::util;

/// WAD3 magic (Half-Life).
pub const WAD3_MAGIC: [u8; 4] = *b"WAD3";

/// Complete WAD loaded from a WAD file.
pub struct Wad<'a> {
    /// WAD header.
    pub header: &'a WadHeader,
    /// WAD entries.
    pub entries: &'a [WadEntry],
}

/// WAD3 header.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct WadHeader {
    /// File magic ("WAD3").
    pub magic: [u8; 4],
    /// Number of entries.
    pub entries: U32,
    /// Offset to the table.
    pub entry_offset: U32,
}

/// WAD entry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct WadEntry {
    /// Offset to entry data.
    pub offset: U32,
    /// Size of entry data on disk (may be compressed).
    pub disk_size: U32,
    /// Uncompressed size of entry data.
    pub size: U32,
    /// Entry type (e.g. 0x43 for texture).
    pub ty: u8,
    /// Compression type (0 = none).
    pub compression: u8,
    /// Padding.
    pub pad: U16,
    /// entry name (C string, not guaranteed UTF-8).
    pub name: [u8; 16],
}

impl WadEntry {
    /// Entry name as bytes without trailing NULs.
    pub fn name(&self) -> &[u8] {
        let end = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        &self.name[..end]
    }
}

pub fn wad(bytes: &[u8]) -> io::Result<Wad<'_>> {
    let (header, _) = WadHeader::ref_from_prefix(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "wad header too short"))?;

    if header.magic != WAD3_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "invalid wad magic",
        ));
    }

    let entries = entry_ref(bytes, header)?;

    Ok(Wad { header, entries })
}

pub fn entry_bytes<'a>(bytes: &'a [u8], entry: &WadEntry) -> io::Result<&'a [u8]> {
    let (offset, size) = util::validate_range(entry.offset.get(), entry.disk_size.get())?;
    let data = bytes
        .get(offset..offset + size)
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "wad entry out of range"))?;

    Ok(data)
}

fn entry_ref<'a>(bytes: &'a [u8], header: &WadHeader) -> io::Result<&'a [WadEntry]> {
    let count = usize::try_from(header.entries.get())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "wad entry count overflow"))?;
    let offset = usize::try_from(header.entry_offset.get())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "wad entry offset overflow"))?;
    let data = bytes
        .get(offset..)
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "wad entry out of range"))?;
    let (entries, _) = <[WadEntry]>::ref_from_prefix_with_elems(data, count)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "wad entry invalid"))?;

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_eq_size!(WadHeader, [u8; 12]);
    assert_eq_size!(WadEntry, [u8; 32]);
}
