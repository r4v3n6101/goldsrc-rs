use std::ops::Range;

use zerocopy::{FromBytes, Immutable};

use crate::{
    common::{Lump, Table},
    error::{ParsingError, ParsingResult},
};

pub fn lump_ref<'a, T>(bytes: &'a [u8], lump: &Lump, label: &'static str) -> ParsingResult<&'a [T]>
where
    T: Immutable + FromBytes,
{
    if lump.size.get() == 0 {
        return Ok(&[]);
    }

    let data = bytes
        .get(to_validate_range(
            lump.offset.get(),
            lump.size.get(),
            label,
        )?)
        .ok_or(ParsingError::OutOfRange(label))?;

    <[T]>::ref_from_bytes(data).map_err(|_| ParsingError::Invalid(label))
}

pub fn table_ref<'a, T>(
    bytes: &'a [u8],
    table: &Table,
    label: &'static str,
) -> ParsingResult<&'a [T]>
where
    T: Immutable + FromBytes,
{
    if table.count.get() == 0 {
        return Ok(&[]);
    }

    let count =
        usize::try_from(table.count.get()).map_err(|_| ParsingError::NumberOverflow(label))?;
    let offset =
        usize::try_from(table.offset.get()).map_err(|_| ParsingError::NumberOverflow(label))?;
    let data = bytes.get(offset..).ok_or(ParsingError::OutOfRange(label))?;
    let (entries, _) =
        <[T]>::ref_from_prefix_with_elems(data, count).map_err(|_| ParsingError::Invalid(label))?;

    Ok(entries)
}

pub fn to_validate_range(
    offset: u32,
    size: u32,
    label: &'static str,
) -> ParsingResult<Range<usize>> {
    let offset = usize::try_from(offset).map_err(|_| ParsingError::NumberOverflow(label))?;
    let size = usize::try_from(size).map_err(|_| ParsingError::NumberOverflow(label))?;
    match offset.checked_add(size) {
        Some(res) => Ok(offset..res),
        None => Err(ParsingError::NumberOverflow(label)),
    }
}

pub fn mip_level_size(
    width: u32,
    height: u32,
    level: usize,
    kind: &'static str,
) -> ParsingResult<usize> {
    let w = width >> level;
    let h = height >> level;
    pixel_size(w, h, kind)
}

pub fn pixel_size(width: u32, height: u32, kind: &'static str) -> ParsingResult<usize> {
    let size = width
        .checked_mul(height)
        .ok_or(ParsingError::NumberOverflow(kind))?;
    usize::try_from(size).map_err(|_| ParsingError::NumberOverflow(kind))
}
