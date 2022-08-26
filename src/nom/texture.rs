use crate::{
    nom::{cstr16, palette, SliceExt},
    repr::texture::{CharInfo, ColourData, Font, MipTexture, Picture},
};
use smol_str::SmolStr;

const PALETTE_SIZE: usize = 256;
const GLYPHS_NUM: usize = 256;
const QCHAR_WIDTH: u32 = 16;

pub fn mip_texture(input: &[u8]) -> nom::IResult<&[u8], MipTexture> {
    let (i, name) = cstr16(input)?;
    let name = SmolStr::new_inline(name);

    let (i, width) = nom::number::complete::le_u32(i)?;
    let (i, height) = nom::number::complete::le_u32(i)?;

    let (i, mip0_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip2_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip4_offset) = nom::number::complete::le_u32(i)?;
    let (_, mip8_offset) = nom::number::complete::le_u32(i)?;

    let data = if [mip0_offset, mip2_offset, mip4_offset, mip8_offset]
        .iter()
        .all(|&x| x != 0)
    {
        let mip0_size = width * height;
        let mip2_size = (width / 2) * (height / 2);
        let mip4_size = (width / 4) * (height / 4);
        let mip8_size = (width / 8) * (height / 8);

        Some(ColourData {
            indices: [
                input
                    .off(mip0_offset as usize, mip0_size as usize)?
                    .to_vec(),
                input
                    .off(mip2_offset as usize, mip2_size as usize)?
                    .to_vec(),
                input
                    .off(mip4_offset as usize, mip4_size as usize)?
                    .to_vec(),
                input
                    .off(mip8_offset as usize, mip8_size as usize)?
                    .to_vec(),
            ],
            palette: palette(input.off((mip8_offset + mip8_size + 2) as usize, PALETTE_SIZE * 3)?)?
                .1,
        })
    } else {
        None
    };

    Ok((
        &[],
        MipTexture {
            name,
            width,
            height,
            data,
        },
    ))
}

pub fn qpic(input: &[u8]) -> nom::IResult<&[u8], Picture> {
    let (i, width) = nom::number::complete::le_u32(input)?;
    let (i, height) = nom::number::complete::le_u32(i)?;

    let (i, indices) = nom::bytes::complete::take(width * height)(i)?;
    let (i, _) = nom::number::complete::le_u16(i)?; // palette size
    let (_, palette) = palette(i)?;

    Ok((
        &[],
        Picture {
            width,
            height,
            data: ColourData {
                indices: [indices.to_vec()],
                palette,
            },
        },
    ))
}

fn char_info(i: &[u8]) -> nom::IResult<&[u8], CharInfo> {
    let (i, offset) = nom::number::complete::le_u16(i)?;
    let (i, width) = nom::number::complete::le_u16(i)?;
    Ok((i, CharInfo { offset, width }))
}

pub fn font(input: &[u8]) -> nom::IResult<&[u8], Font> {
    let (i, width) = nom::number::complete::le_u32(input)?;
    let (i, height) = nom::number::complete::le_u32(i)?;

    let (i, row_count) = nom::number::complete::le_u32(i)?;
    let (i, row_height) = nom::number::complete::le_u32(i)?;

    let (i, chars_info) =
        nom::combinator::map_res(nom::multi::count(char_info, GLYPHS_NUM), TryFrom::try_from)(i)?;

    let needed = (height * width * QCHAR_WIDTH) + 2 + 768 + 64;
    let width = if i.len() != needed as usize {
        256
    } else {
        width * QCHAR_WIDTH
    };
    let (i, indices) = nom::bytes::complete::take(width * height)(i)?;
    let (i, _) = nom::number::complete::le_u16(i)?; // palette size
    let (_, palette) = palette(i)?;

    Ok((
        &[],
        Font {
            width,
            height,
            row_count,
            row_height,
            chars_info,
            data: ColourData {
                indices: [indices.to_vec()],
                palette,
            },
        },
    ))
}
