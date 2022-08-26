use smol_str::SmolStr;

const MIP_LEVEL: usize = 4;
const GLYPHS_COUNT: usize = 256;
const PALETTE_SIZE: usize = 256;
const COLORS: usize = 3; // rgb

pub type Palette = [u8; PALETTE_SIZE * COLORS]; // I hope it won't overflow the stack

pub struct ColourData<const N: usize> {
    pub indices: [Vec<u8>; N],
    pub palette: Palette,
}

pub struct MipTexture {
    pub name: SmolStr,
    pub width: u32,
    pub height: u32,
    pub data: Option<ColourData<MIP_LEVEL>>,
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
    pub chars_info: [CharInfo; GLYPHS_COUNT],
    pub data: ColourData<1>,
}
