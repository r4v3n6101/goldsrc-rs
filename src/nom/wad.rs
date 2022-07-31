use crate::nom::{cstr16, SliceExt};
use crate::repr::wad::Entry;

const MAGIC: &[u8] = b"WAD3";

#[inline]
fn entry<'a>(i: &'a [u8], file: &'a [u8]) -> nom::IResult<&'a [u8], Entry<'a>> {
    let (i, offset) = nom::number::complete::le_u32(i)?;
    let (i, size) = nom::number::complete::le_u32(i)?;
    let (i, full_size) = nom::number::complete::le_u32(i)?;
    let (i, ty) = nom::number::complete::le_u8(i)?;
    let (i, comp) = nom::number::complete::le_u8(i)?;
    let (i, _) = nom::number::complete::le_u16(i)?;
    let (i, name) = cstr16(i)?;

    Ok((
        i,
        Entry {
            full_size,
            ty,
            name,
            data: file.off_size(offset, size),
            compressed: comp != 0,
        },
    ))
}

pub fn entries(file: &[u8]) -> nom::IResult<&[u8], Vec<Entry>> {
    let (i, _) = nom::bytes::complete::tag(MAGIC)(file)?;
    let (i, count) = nom::number::complete::le_u32(i)?;
    let (_, offset) = nom::number::complete::le_u32(i)?;

    nom::multi::count(|i| entry(i, file), count as usize)(file.off(offset))
}

#[test]
fn parse_wad() {
    let data = std::fs::read("test.wad").expect("error reading file");
    let (_, entries) = entries(&data).expect("error parsing file");

    entries.iter().for_each(|e| println!("{}", e.name));
}
