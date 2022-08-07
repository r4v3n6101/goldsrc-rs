use nom::{
    bytes::complete::tag,
    multi::count,
    number::complete::{le_u16, le_u32, le_u8},
};

use crate::{
    nom::{cstr16, SliceExt},
    repr::wad::Archive,
    repr::wad::Entry,
};

const MAGIC: &[u8] = b"WAD3";

fn entry<'a>(i: &'a [u8], file: &'a [u8]) -> nom::IResult<&'a [u8], Entry<'a>> {
    let (i, offset) = le_u32(i)?;
    let (i, size) = le_u32(i)?;
    let (i, full_size) = le_u32(i)?;
    let (i, ty) = le_u8(i)?;
    let (i, comp) = le_u8(i)?;
    let (i, _) = le_u16(i)?;
    let (i, name) = cstr16(i)?;

    Ok((
        i,
        Entry {
            full_size,
            ty,
            name,
            compressed: comp != 0,
            data: file.off(offset as usize, size as usize)?,
        },
    ))
}

pub fn archive(file: &[u8]) -> nom::IResult<&[u8], Archive> {
    let (i, _) = tag(MAGIC)(file)?;
    let (i, size) = le_u32(i)?;
    let (_, offset) = le_u32(i)?;
    let entry_data = file.off_all(offset as usize)?;
    count(|i| entry(i, file), size as usize)(entry_data)
}

#[test]
fn parse_wad() {
    let data = std::fs::read("test.wad").expect("error reading file");
    let (_, entries) = archive(&data).expect("error parsing file");

    entries.iter().for_each(|e| println!("{}", e.name));
}
