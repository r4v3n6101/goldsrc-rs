use crate::nom::{cstr16, SliceExt};
use crate::repr::texture::{MipData, MipTexture};

const PALETTE_SIZE: u32 = 256;

pub fn miptexture(input: &[u8]) -> nom::IResult<&[u8], MipTexture> {
    let (i, name) = cstr16(input)?;

    let (i, width) = nom::number::complete::le_u32(i)?;
    let (i, height) = nom::number::complete::le_u32(i)?;

    let (i, mip0_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip2_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip4_offset) = nom::number::complete::le_u32(i)?;
    let (i, mip8_offset) = nom::number::complete::le_u32(i)?;

    let data = if [mip0_offset, mip2_offset, mip4_offset, mip8_offset] != [0; 4] {
        let mip0_size = width * height;
        let mip2_size = (width / 2) * (height / 2);
        let mip4_size = (width / 4) * (height / 4);
        let mip8_size = (width / 8) * (height / 8);

        Some(MipData {
            mip0: input.off_size(mip0_offset, mip0_size),
            mip2: input.off_size(mip2_offset, mip2_size),
            mip4: input.off_size(mip4_offset, mip4_size),
            mip8: input.off_size(mip8_offset, mip8_size),
            palette: input.off_size(mip8_offset + mip8_size + 2, PALETTE_SIZE * 3),
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
    let (_, miptexture) = miptexture(data.as_slice()).expect("error parsing file");

    assert_eq!(miptexture.name, "123456789012345");
    assert_eq!(miptexture.width, 5);
    assert_eq!(miptexture.height, 5);
    assert!(miptexture.data.is_none());
}
