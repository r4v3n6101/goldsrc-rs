use smallstr::SmallString;

pub mod bsp;
pub mod map;
pub mod texture;
pub mod wad;

pub type CStr16 = SmallString<[u8; 16]>;
