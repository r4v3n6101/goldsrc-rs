use crate::{
    nom::{
        cstr16,
        texture::{font, mip_texture, qpic},
        SliceExt,
    },
    repr::{wad::Archive, wad::Content},
};
use nom::{
    bytes::complete::tag,
    multi::count,
    number::complete::{le_u16, le_u32, le_u8},
};
use smol_str::SmolStr;

const MAGIC: &[u8] = b"WAD3";

fn entry<'a>(i: &'a [u8], file: &'a [u8]) -> nom::IResult<&'a [u8], (SmolStr, Content)> {
    let (i, offset) = le_u32(i)?;
    let (i, size) = le_u32(i)?;
    let (i, _) = le_u32(i)?; // full_size, not used
    let (i, ty) = le_u8(i)?;
    let (i, comp) = le_u8(i)?;
    let (i, _) = le_u16(i)?;
    let (i, name) = cstr16(i)?;
    let data = file.off(offset as usize, size as usize)?;

    if comp != 0 {
        unimplemented!("compression not supported by goldsrc")
    }

    let content = match ty {
        0x42 => Content::Picture(qpic(data)?.1),
        0x43 => Content::MipTexture(mip_texture(data)?.1),
        0x46 => Content::Font(font(data)?.1),
        ty => Content::Other {
            ty,
            data: data.to_vec(),
        },
    };

    Ok((i, (SmolStr::new_inline(name), content)))
}

pub fn archive(file: &[u8]) -> nom::IResult<&[u8], Archive> {
    let (i, _) = tag(MAGIC)(file)?;
    let (i, size) = le_u32(i)?;
    let (_, offset) = le_u32(i)?;
    let entry_data = file.off_all(offset as usize)?;
    Ok((
        &[],
        count(|i| entry(i, file), size as usize)(entry_data)?
            .1
            .into_iter()
            .collect(),
    ))
}
