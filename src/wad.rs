use static_assertions::assert_eq_size;
use zerocopy::{
    FromBytes,
    little_endian::{U16, U32},
};
use zerocopy_derive::*;

use crate::{
    common::Table,
    error::{ParsingError, ParsingResult},
    util::{table_ref, to_validate_range},
};

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

pub fn wad(bytes: &[u8]) -> ParsingResult<Wad<'_>> {
    let (header, _) =
        WadHeader::ref_from_prefix(bytes).map_err(|_| ParsingError::OutOfRange("wad header"))?;

    if header.magic != WAD3_MAGIC {
        return Err(ParsingError::WrongFourCC {
            got: header.magic,
            expected: WAD3_MAGIC,
        });
    }

    let entries = table_ref(
        bytes,
        &Table {
            count: header.entries,
            offset: header.entry_offset,
        },
        "wad entries",
    )?;

    Ok(Wad { header, entries })
}

pub fn wad_entry<'a>(bytes: &'a [u8], entry: &WadEntry) -> ParsingResult<&'a [u8]> {
    bytes
        .get(to_validate_range(
            entry.offset.get(),
            entry.disk_size.get(),
            "wad entry",
        )?)
        .ok_or(ParsingError::OutOfRange("wad entry"))
}

assert_eq_size!(WadHeader, [u8; 12]);
assert_eq_size!(WadEntry, [u8; 32]);
