use std::io;

use zerocopy::{
    FromBytes,
    little_endian::{U16, U32},
};
use zerocopy_derive::*;

/// Number of mip levels in a mipmapped texture.
pub const MIP_LEVELS: usize = 4;
/// Number of glyphs in a bitmap font.
pub const FONT_GLYPHS: usize = 256;

/// RGB color as three u8 values (red, green, blue).
pub type Rgb = [u8; 3];
/// Index into a palette.
pub type Index = u8;

/// Parsed mipmapped texture (header + indexed data + palette).
pub struct MipTexture<'a> {
    /// Miptex header.
    pub header: &'a MipTextureHeader,
    /// Indexed color data and palette.
    pub data: Option<ColorData<'a, MIP_LEVELS>>,
}

/// Parsed picture texture (header + indexed data + palette).
pub struct Picture<'a> {
    /// Picture header.
    pub header: &'a PictureHeader,
    /// Indexed color data and palette.
    pub data: ColorData<'a, 1>,
}

/// Parsed bitmap font (header + indexed data + palette).
pub struct Font<'a> {
    /// Font header.
    pub header: &'a FontHeader,
    /// Indexed color data and palette.
    pub data: ColorData<'a, 1>,
}

/// View of indexed color data and palette.
pub struct ColorData<'a, const N: usize> {
    /// Indexed color data for each mip level (or single level for pictures).
    pub indices: [&'a [Index]; N],
    /// Palette mapping indices to RGB colors.
    pub palette: &'a [Rgb],
}

/// Quake/GoldSrc miptex header.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct MipTextureHeader {
    /// Texture name (C string, not guaranteed UTF-8).
    pub name: [u8; 16],
    /// Width in pixels.
    pub width: U32,
    /// Height in pixels.
    pub height: U32,
    /// Offsets to each mip level, relative to start of this header.
    pub offsets: [U32; MIP_LEVELS],
}

/// Picture (single-level texture) header.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct PictureHeader {
    /// Width in pixels.
    pub width: U32,
    /// Height in pixels.
    pub height: U32,
}

/// Bitmap font header.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct FontHeader {
    /// Font width in pixels.
    pub width: U32,
    /// Font height in pixels.
    pub height: U32,
    /// Number of character rows in the font.
    pub row_count: U32,
    /// Height of each row in pixels.
    pub row_height: U32,
    /// Character info table.
    pub chars: [CharInfo; FONT_GLYPHS],
}

/// Info about a single character in a bitmap font.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct CharInfo {
    /// Offset in the font data where this character starts.
    pub offset: U16,
    /// Width of the character in pixels.
    pub width: U16,
}

pub fn mip_texture(bytes: &[u8]) -> io::Result<MipTexture<'_>> {
    let (header, _) = MipTextureHeader::ref_from_prefix(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "miptex header too short"))?;
    let width = header.width.get();
    let height = header.height.get();

    if header.offsets.iter().any(|offset| offset.get() == 0) {
        return Ok(MipTexture { header, data: None });
    }

    let mut indices: [&[Index]; MIP_LEVELS] = [&[], &[], &[], &[]];

    for (level, slot) in indices.iter_mut().enumerate() {
        let offset = usize::try_from(header.offsets[level].get())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "miptex offset overflow"))?;
        let size = mip_level_size(width, height, level)?;
        let data = bytes.get(offset..offset + size).ok_or_else(|| {
            io::Error::new(io::ErrorKind::UnexpectedEof, "miptex data out of range")
        })?;

        let (indices, _) = <[Index]>::ref_from_prefix_with_elems(data, size)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "miptex indices invalid"))?;
        *slot = indices;
    }

    let palette_offset = {
        let last_offset = usize::try_from(header.offsets[MIP_LEVELS - 1].get())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "miptex offset overflow"))?;
        let last_size = mip_level_size(width, height, MIP_LEVELS - 1)?;
        last_offset
            .checked_add(last_size)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "miptex size overflow"))?
    };

    let palette = {
        let data = bytes
            .get(palette_offset..)
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "palette out of range"))?;

        palette_ref(data)
    }?;

    Ok(MipTexture {
        header,
        data: Some(ColorData { indices, palette }),
    })
}

pub fn picture(bytes: &[u8]) -> io::Result<Picture<'_>> {
    let (header, bytes) = PictureHeader::ref_from_prefix(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "pic header too short"))?;
    let width = header.width.get();
    let height = header.height.get();
    let size = pixel_size(width, height, "picture")?;
    let (indices, bytes) = <[Index]>::ref_from_prefix_with_elems(bytes, size)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "pic indices invalid"))?;
    let palette = palette_ref(bytes)?;

    Ok(Picture {
        header,
        data: ColorData {
            indices: [indices],
            palette,
        },
    })
}

pub fn font(bytes: &[u8]) -> io::Result<Font<'_>> {
    let (header, bytes) = FontHeader::ref_from_prefix(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "font header too short"))?;

    let width = header.width.get();
    let height = header.height.get();
    let size = pixel_size(width, height, "font")?;
    let (indices, bytes) = <[Index]>::ref_from_prefix_with_elems(bytes, size)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "font indices invalid"))?;
    let palette = palette_ref(bytes)?;

    Ok(Font {
        header,
        data: ColorData {
            indices: [indices],
            palette,
        },
    })
}

fn palette_ref(bytes: &[u8]) -> io::Result<&'_ [Rgb]> {
    let (size, bytes) = U16::ref_from_prefix(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "palette out of range"))?;

    let count = usize::from(size.get()).min(Index::MAX.into());
    let (palette, _) = <[Rgb]>::ref_from_prefix_with_elems(bytes, count)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "palette invalid"))?;

    Ok(palette)
}

fn pixel_size(width: u32, height: u32, kind: &str) -> io::Result<usize> {
    let size = width.checked_mul(height).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, format!("{kind} size overflow"))
    })?;
    usize::try_from(size)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, format!("{kind} size overflow")))
}

fn mip_level_size(width: u32, height: u32, level: usize) -> io::Result<usize> {
    let w = width >> level;
    let h = height >> level;
    pixel_size(w, h, "miptex")
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_eq_size!(MipTextureHeader, [u8; 40]);
    assert_eq_size!(PictureHeader, [u8; 8]);
    assert_eq_size!(FontHeader, [u8; 1040]);
    assert_eq_size!(CharInfo, [u8; 4]);
    assert_eq_size!(Rgb, [u8; 3]);
}
