use std::ops::Range;

use crate::error::{ParsingError, ParsingResult};

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
