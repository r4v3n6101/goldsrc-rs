use crate::CStr16;

/// Number of mip levels in a mipmapped texture.
pub const MIP_LEVELS: usize = 4;

/// RGB color as three u8 values (red, green, blue).
pub type Rgb = [u8; 3];

/// Index into a palette.
pub type Index = u8;

/// Palette of RGB colors.
pub type Palette = [Rgb];

/// Color data for textures, storing indices and palette.
#[derive(Debug, Clone)]
pub struct ColorData<const N: usize> {
    /// Indexed color data for each mip level (or single level for pictures).
    pub indices: [Vec<Index>; N],

    /// Palette mapping indices to RGB colors.
    pub palette: Box<Palette>,
}

/// Mipmapped texture from a WAD or BSP file.
#[derive(Debug, Clone)]
pub struct MipTexture {
    /// Texture name.
    pub name: CStr16,

    /// Width in pixels.
    pub width: u32,

    /// Height in pixels.
    pub height: u32,

    /// Indexed color data for each mip level, if available.
    pub data: Option<ColorData<MIP_LEVELS>>,
}

/// Simple picture (single-level texture).
#[derive(Debug, Clone)]
pub struct Picture {
    /// Width in pixels.
    pub width: u32,

    /// Height in pixels.
    pub height: u32,

    /// Color data.
    pub data: ColorData<1>,
}

/// Bitmap font.
#[derive(Debug, Clone)]
pub struct Font {
    /// Font width in pixels.
    pub width: u32,

    /// Font height in pixels.
    pub height: u32,

    /// Number of character rows in the font.
    pub row_count: u32,

    /// Height of each row in pixels (actually constant)
    pub row_height: u32,

    /// Information about individual characters.
    pub chars_info: Box<[CharInfo]>,

    /// Color data for the font.
    pub data: ColorData<1>,
}

/// Info about a single character in a bitmap font.
#[derive(Debug, Clone)]
pub struct CharInfo {
    /// Offset in the font data where this character starts.
    pub offset: u16,

    /// Width of the character in pixels.
    pub width: u16,
}
