use crate::CStr16;

pub const MIP_LEVELS: usize = 4;

pub type Rgb = [u8; 3];
pub type Index = u8;
pub type Palette = [Rgb];

#[derive(Debug, Clone)]
pub struct ColorData<const N: usize> {
    pub indices: [Vec<Index>; N],
    pub palette: Box<Palette>,
}

#[derive(Debug, Clone)]
pub struct MipTexture {
    pub name: CStr16,
    pub width: u32,
    pub height: u32,
    pub data: Option<ColorData<MIP_LEVELS>>,
}

#[derive(Debug, Clone)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: ColorData<1>,
}

#[derive(Debug, Clone)]
pub struct CharInfo {
    pub offset: u16,
    pub width: u16,
}

#[derive(Debug, Clone)]
pub struct Font {
    pub width: u32,
    pub height: u32,
    pub row_count: u32,
    pub row_height: u32,
    pub chars_info: Box<[CharInfo]>,
    pub data: ColorData<1>,
}
