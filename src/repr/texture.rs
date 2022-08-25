const MIP_LEVEL: usize = 4;
const GLYPHS_COUNT: usize = 256;

pub struct ColourData<'a, const N: usize> {
    pub indices: [&'a [u8]; N],
    pub palette: &'a [u8],
}

pub struct MipTexture<'a> {
    pub name: &'a str,
    pub width: u32,
    pub height: u32,
    pub data: Option<ColourData<'a, MIP_LEVEL>>,
}

pub struct Picture<'a> {
    pub width: u32,
    pub height: u32,
    pub data: ColourData<'a, 1>,
}

pub struct CharInfo {
    pub offset: u16,
    pub width: u16,
}

pub struct Font<'a> {
    pub width: u32,
    pub height: u32,
    pub row_count: u32,
    pub row_height: u32,
    pub chars_info: [CharInfo; GLYPHS_COUNT],
    pub data: ColourData<'a, 1>,
}
