use static_assertions::assert_eq_size;
use zerocopy::{
    FromBytes, Immutable,
    little_endian::{F32, I16, U32},
};
use zerocopy_derive::*;

use crate::{
    error::{ParsingError, ParsingResult},
    util,
};

/// 2D vector with 32-bit float components.
pub type Vec2f = [F32; 2];
/// 3D vector with 16-bit integer components.
pub type Vec3s = [I16; 3];
/// 3D vector with 32-bit float components.
pub type Vec3f = [F32; 3];

/// Axis-aligned bounding box.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct BBox<T> {
    /// Minimum coordinates.
    pub min: T,
    /// Maximum coordinates.
    pub max: T,
}

/// Lump entry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Lump {
    /// Offset from start of file.
    pub offset: U32,
    /// Size in bytes.
    pub size: U32,
}

/// Table entry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Table {
    /// Entry count.
    pub count: U32,
    /// Offset to entries.
    pub offset: U32,
}

pub fn lump_ref<'a, T>(bytes: &'a [u8], lump: &Lump, label: &'static str) -> ParsingResult<&'a [T]>
where
    T: Immutable + FromBytes,
{
    if lump.size.get() == 0 {
        return Ok(&[]);
    }

    let data = bytes
        .get(util::to_validate_range(
            lump.offset.get(),
            lump.size.get(),
            label,
        )?)
        .ok_or(ParsingError::OutOfRange(label))?;

    <[T]>::ref_from_bytes(data).map_err(|_| ParsingError::Invalid(label))
}

pub fn table_ref<'a, T>(
    bytes: &'a [u8],
    table: &Table,
    label: &'static str,
) -> ParsingResult<&'a [T]>
where
    T: Immutable + FromBytes,
{
    if table.count.get() == 0 {
        return Ok(&[]);
    }

    let count =
        usize::try_from(table.count.get()).map_err(|_| ParsingError::NumberOverflow(label))?;
    let offset =
        usize::try_from(table.offset.get()).map_err(|_| ParsingError::NumberOverflow(label))?;
    let data = bytes.get(offset..).ok_or(ParsingError::OutOfRange(label))?;
    let (entries, _) =
        <[T]>::ref_from_prefix_with_elems(data, count).map_err(|_| ParsingError::Invalid(label))?;

    Ok(entries)
}

pub fn cstring_bytes(bytes: &[u8]) -> &[u8] {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    &bytes[..end]
}

assert_eq_size!(Vec3s, [u8; 6]);
assert_eq_size!(Vec3f, [u8; 12]);
assert_eq_size!(BBox<Vec3s>, [Vec3s; 2]);
assert_eq_size!(BBox<Vec3f>, [Vec3f; 2]);
