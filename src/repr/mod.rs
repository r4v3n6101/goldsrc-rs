use smallstr::SmallString;

pub mod bsp;
pub mod map;
pub mod texture;
pub mod wad;

/// Fixed-size C-style UTF-8 string (max 16 bytes).
///
/// Common in Half-Life file headers (e.g. texture names).
/// Backed by [`SmallString<[u8; 16]>`], so it's stack-allocated and
/// acts like a normal `&str`.
pub type CStr16 = SmallString<[u8; 16]>;
