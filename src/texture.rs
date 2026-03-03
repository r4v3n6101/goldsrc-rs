use static_assertions::assert_eq_size;
use zerocopy::{
    FromBytes,
    little_endian::{F32, I32, U16, U32},
};
use zerocopy_derive::*;

use crate::{
    error::{ParsingError, ParsingResult},
    util::{mip_level_size, pixel_size},
};

/// Number of mip levels in a mipmapped texture.
pub const MIP_LEVELS: usize = 4;
/// Number of glyphs in a bitmap font.
pub const FONT_GLYPHS: usize = 256;

/// Sprite magic.
pub const SPRITE_MAGIC: [u8; 4] = *b"IDSP";
/// Sprite version.
pub const SPRITE_VERSION: u32 = 2;

/// Sprite camera alignment type values.
pub const SPRITE_TYPE_FWD_PARALLEL_UPRIGHT: u32 = 0;
pub const SPRITE_TYPE_FACING_UPRIGHT: u32 = 1;
pub const SPRITE_TYPE_FWD_PARALLEL: u32 = 2;
pub const SPRITE_TYPE_ORIENTED: u32 = 3;
pub const SPRITE_TYPE_FWD_PARALLEL_ORIENTED: u32 = 4;

/// Sprite draw type values.
pub const SPRITE_TEX_FORMAT_NORMAL: u32 = 0;
pub const SPRITE_TEX_FORMAT_ADDITIVE: u32 = 1;
pub const SPRITE_TEX_FORMAT_INDEX_ALPHA: u32 = 2;
pub const SPRITE_TEX_FORMAT_ALPH_TEST: u32 = 3;

/// Sprite sync type (animation timing) values.
pub const SPRITE_SYNC_TYPE_SYNC: u32 = 0;
pub const SPRITE_SYNC_TYPE_RAND: u32 = 1;

/// Frame type that precedes each frame entry in a sprite.
pub const SPRITE_FRAME_TYPE_SINGLE: u32 = 0;
pub const SPRITE_FRAME_TYPE_GROUP: u32 = 1;

/// RGB color as three u8 values (red, green, blue).
pub type Rgb = [u8; 3];
/// Index into a palette.
pub type PaletteIndex = u8;

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
    pub indices: [&'a [PaletteIndex]; N],
    /// Palette mapping indices to RGB colors.
    pub palette: &'a [Rgb],
}

/// Sprite loaded from a SPR file.
pub struct Sprite<'a> {
    /// Sprite header.
    pub header: &'a SpriteHeader,
    /// Shared palette (RGB).
    pub palette: &'a [Rgb],
    /// Sprite frames.
    pub frames: Vec<SpriteFrame<'a>>,
}

pub enum SpriteFrame<'a> {
    Single(SpriteFrameSingle<'a>),
    Group(Vec<SpriteFrameGroup<'a>>),
}

pub struct SpriteFrameGroup<'a> {
    pub interval: F32,
    pub subframe: SpriteFrameSingle<'a>,
}

pub struct SpriteFrameSingle<'a> {
    /// Sprite frame header.
    pub header: &'a SpriteFrameHeader,
    /// Indices pointing to the palette.
    pub indices: &'a [PaletteIndex],
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

/// Sprite header (Half-Life, version 2).
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct SpriteHeader {
    /// File magic ("IDSP").
    pub magic: [u8; 4],
    /// Sprite format version.
    pub version: U32,
    /// Sprite type.
    pub ty: U32,
    /// Texture format.
    pub tex_format: U32,
    /// Bounding radius (raw bits; usually float in tools).
    pub bounding_radius: F32,
    /// Bounds or size, depending on toolchain.
    pub bounds: [U32; 2],
    /// Number of frames (including groups).
    pub frames_num: U32,
    /// Beam length.
    pub beam_len: F32,
    /// Synchronisation type.
    pub sync_type: U32,
}

/// Single frame header.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct SpriteFrameHeader {
    /// Origin in sprite space.
    pub origin: [I32; 2],
    /// Frame width.
    pub width: U32,
    /// Frame height.
    pub height: U32,
}

pub fn mip_texture(bytes: &[u8]) -> ParsingResult<MipTexture<'_>> {
    let (header, _) = MipTextureHeader::ref_from_prefix(bytes)
        .map_err(|_| ParsingError::OutOfRange("miptex header"))?;
    let width = header.width.get();
    let height = header.height.get();

    if header.offsets.iter().any(|offset| offset.get() == 0) {
        return Ok(MipTexture { header, data: None });
    }

    let mut ptr = bytes;
    let mut indices = [Default::default(); MIP_LEVELS];
    for (level, slot) in indices.iter_mut().enumerate() {
        let offset = usize::try_from(header.offsets[level].get())
            .map_err(|_| ParsingError::NumberOverflow("miptex offset"))?;
        let size = mip_level_size(width, height, level, "miptex")?;
        let data = bytes
            .get(offset..)
            .ok_or(ParsingError::OutOfRange("miptex data"))?;

        let (indices, bytes) = <[PaletteIndex]>::ref_from_prefix_with_elems(data, size)
            .map_err(|_| ParsingError::Invalid("miptex palette indices"))?;
        *slot = indices;
        ptr = bytes;
    }

    let (palette, _) = palette(ptr)?;

    Ok(MipTexture {
        header,
        data: Some(ColorData { indices, palette }),
    })
}

pub fn picture(bytes: &[u8]) -> ParsingResult<Picture<'_>> {
    let (header, bytes) = PictureHeader::ref_from_prefix(bytes)
        .map_err(|_| ParsingError::OutOfRange("pic header"))?;
    let width = header.width.get();
    let height = header.height.get();
    let size = pixel_size(width, height, "picture")?;

    let (indices, bytes) = <[PaletteIndex]>::ref_from_prefix_with_elems(bytes, size)
        .map_err(|_| ParsingError::Invalid("pic indices"))?;
    let (palette, _) = palette(bytes)?;

    Ok(Picture {
        header,
        data: ColorData {
            indices: [indices],
            palette,
        },
    })
}

pub fn font(bytes: &[u8]) -> ParsingResult<Font<'_>> {
    let (header, bytes) =
        FontHeader::ref_from_prefix(bytes).map_err(|_| ParsingError::OutOfRange("font header"))?;

    let width = header.width.get();
    let height = header.height.get();
    let size = pixel_size(width, height, "font")?;

    let (indices, bytes) = <[PaletteIndex]>::ref_from_prefix_with_elems(bytes, size)
        .map_err(|_| ParsingError::Invalid("font indices"))?;
    let (palette, _) = palette(bytes)?;

    Ok(Font {
        header,
        data: ColorData {
            indices: [indices],
            palette,
        },
    })
}

pub fn sprite(bytes: &[u8]) -> ParsingResult<Sprite<'_>> {
    let (header, bytes) = SpriteHeader::ref_from_prefix(bytes)
        .map_err(|_| ParsingError::OutOfRange("sprite header"))?;

    if header.magic != SPRITE_MAGIC {
        return Err(ParsingError::WrongFourCC {
            got: header.magic,
            expected: SPRITE_MAGIC,
        });
    }

    let version = header.version.get();
    if version != SPRITE_VERSION {
        return Err(ParsingError::WrongVersion {
            got: version,
            expected: SPRITE_VERSION,
        });
    }

    let (palette, bytes) = palette(bytes)?;
    let frames = frames_ref(bytes, header)?;

    Ok(Sprite {
        header,
        palette,
        frames,
    })
}

pub fn palette(bytes: &[u8]) -> ParsingResult<(&'_ [Rgb], &'_ [u8])> {
    let (size, bytes) =
        U16::ref_from_prefix(bytes).map_err(|_| ParsingError::OutOfRange("palette header"))?;

    let count = usize::from(size.get()).min(256);
    let (palette, bytes) = <[Rgb]>::ref_from_prefix_with_elems(bytes, count)
        .map_err(|_| ParsingError::Invalid("palette"))?;

    Ok((palette, bytes))
}

fn frames_ref<'a>(bytes: &'a [u8], header: &SpriteHeader) -> ParsingResult<Vec<SpriteFrame<'a>>> {
    let count = usize::try_from(header.frames_num.get())
        .map_err(|_| ParsingError::NumberOverflow("sprite frame count"))?;

    let mut frames = Vec::with_capacity(count);
    let mut ptr = bytes;
    for _ in 0..count {
        let (frame, bytes) = frame_ref(ptr)?;
        frames.push(frame);
        ptr = bytes;
    }

    Ok(frames)
}

fn frame_ref(bytes: &[u8]) -> ParsingResult<(SpriteFrame<'_>, &'_ [u8])> {
    let (group, bytes) =
        U32::ref_from_prefix(bytes).map_err(|_| ParsingError::OutOfRange("sprite frame header"))?;

    match group.get() {
        SPRITE_FRAME_TYPE_SINGLE => {
            let (single, bytes) = frame_single_ref(bytes)?;
            Ok((SpriteFrame::Single(single), bytes))
        }
        SPRITE_FRAME_TYPE_GROUP => {
            let (count, bytes) = U32::ref_from_prefix(bytes)
                .map_err(|_| ParsingError::OutOfRange("sprite group header"))?;
            let count = usize::try_from(count.get())
                .map_err(|_| ParsingError::NumberOverflow("sprite group subframes count"))?;
            let (intervals, bytes) = <[F32]>::ref_from_prefix_with_elems(bytes, count)
                .map_err(|_| ParsingError::OutOfRange("sprite group intervals"))?;

            let mut subframes = Vec::with_capacity(count);
            let mut ptr = bytes;
            for interval in intervals.iter().copied() {
                let (subframe, bytes) = frame_single_ref(ptr)?;
                subframes.push(SpriteFrameGroup { interval, subframe });
                ptr = bytes;
            }

            Ok((SpriteFrame::Group(subframes), ptr))
        }
        _ => Err(ParsingError::Invalid("sprite group type")),
    }
}

fn frame_single_ref(bytes: &[u8]) -> ParsingResult<(SpriteFrameSingle<'_>, &'_ [u8])> {
    let (header, bytes) = SpriteFrameHeader::ref_from_prefix(bytes)
        .map_err(|_| ParsingError::OutOfRange("sprite single frame header"))?;
    let width = header.width.get();
    let height = header.height.get();
    let size = pixel_size(width, height, "sprite single frame")?;
    let (indices, bytes) = <[PaletteIndex]>::ref_from_prefix_with_elems(bytes, size)
        .map_err(|_| ParsingError::Invalid("sprite single frame indices"))?;

    Ok((SpriteFrameSingle { header, indices }, bytes))
}

assert_eq_size!(MipTextureHeader, [u8; 40]);
assert_eq_size!(PictureHeader, [u8; 8]);
assert_eq_size!(FontHeader, ([u8; 16], [u8; 4 * FONT_GLYPHS]));
assert_eq_size!(CharInfo, [u8; 4]);
assert_eq_size!(SpriteHeader, [u8; 40]);
assert_eq_size!(SpriteFrameHeader, [u8; 16]);
assert_eq_size!(Rgb, [u8; 3]);
