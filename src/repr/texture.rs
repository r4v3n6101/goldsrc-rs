use std::array;

use smol_str::SmolStr;

pub type Rgb = [u8; 3];
pub type Palette = [Rgb];

pub struct ColourData<const N: usize> {
    pub indices: [Vec<u8>; N],
    pub palette: Box<Palette>,
}

impl<const N: usize> Default for ColourData<N> {
    fn default() -> Self {
        Self {
            indices: array::from_fn(|_| Vec::new()),
            palette: Box::default(),
        }
    }
}

pub struct MipTexture {
    pub name: SmolStr,
    pub width: u32,
    pub height: u32,
    pub data: Option<ColourData<4>>,
}

pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: ColourData<1>,
}

pub struct CharInfo {
    pub offset: u16,
    pub width: u16,
}

pub struct Font {
    pub width: u32,
    pub height: u32,
    pub row_count: u32,
    pub row_height: u32,
    pub chars_info: Box<[CharInfo]>,
    pub data: ColourData<1>,
}
