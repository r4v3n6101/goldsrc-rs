use static_assertions::assert_eq_size;
use zerocopy::little_endian::{F32, I16, U32};
use zerocopy_derive::*;

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
/// Lump array of data at `offset` and with `size` in bytes.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Lump {
    /// Offset from start of file.
    pub offset: U32,
    /// Size in bytes.
    pub size: U32,
}

/// Table entry.
/// Table is array of data at `offset` with length equals to `count` (i.e. `count` is in elements).
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Table {
    /// Entry count.
    pub count: U32,
    /// Offset to entries.
    pub offset: U32,
}

pub fn cstring_bytes(bytes: &[u8]) -> &[u8] {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    &bytes[..end]
}

assert_eq_size!(Vec3s, [u8; 6]);
assert_eq_size!(Vec3f, [u8; 12]);
assert_eq_size!(BBox<Vec3s>, [Vec3s; 2]);
assert_eq_size!(BBox<Vec3f>, [Vec3f; 2]);
assert_eq_size!(Lump, [u8; 8]);
assert_eq_size!(Table, [u8; 8]);
