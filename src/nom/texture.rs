use crate::{
    nom::{cstr16, SliceExt},
    repr::texture::{MipData, MipTexture},
};

const PALETTE_SIZE: usize = 256;

pub fn mip_texture(input: &[u8]) -> nom::IResult<&[u8], MipTexture> {
    let (i, name) = cstr16(input)?;

    let (i, width) = nom::number::complete::le_u32(i)?;
    let (i, height) = nom::number::complete::le_u32(i)?;

    let (i, mip0_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip2_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip4_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip8_offset) = nom::number::complete::le_u32(i)?;

    let data = if [mip0_offset, mip2_offset, mip4_offset, mip8_offset]
        .iter()
        .all(|&x| x != 0)
    {
        let mip0_size = width * height;
        let mip2_size = (width / 2) * (height / 2);
        let mip4_size = (width / 4) * (height / 4);
        let mip8_size = (width / 8) * (height / 8);

        Some(MipData {
            indices: [
                input.off(mip0_offset as usize, mip0_size as usize)?,
                input.off(mip2_offset as usize, mip2_size as usize)?,
                input.off(mip4_offset as usize, mip4_size as usize)?,
                input.off(mip8_offset as usize, mip8_size as usize)?,
            ],
            palette: input.off((mip8_offset + mip8_size + 2) as usize, PALETTE_SIZE * 3)?,
        })
    } else {
        None
    };

    Ok((
        i,
        MipTexture {
            name,
            width,
            height,
            data,
        },
    ))
}

#[test]
fn parse_empty_miptex() {
    let data = b"123456789012345\0\x05\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
    let (_, mip_texture) = mip_texture(data.as_slice()).expect("error parsing file");

    assert_eq!(mip_texture.name, "123456789012345");
    assert_eq!(mip_texture.width, 5);
    assert_eq!(mip_texture.height, 5);
    assert!(mip_texture.data.is_none());
}
