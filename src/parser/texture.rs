use std::{
    array,
    io::{self, Cursor, Read},
    mem, slice,
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::texture::{CharInfo, ColorData, Font, MipTexture, Palette, Picture, Rgb, MIP_LEVELS};

use super::{chunk, chunk_with_offset, cstr16};

fn palette<R: Read>(mut reader: R) -> io::Result<Box<Palette>> {
    let colors_used = reader.read_u16::<LittleEndian>()?.min(256) as usize; // index is u8

    let mut boxed_palette = Box::<Palette>::new_zeroed_slice(colors_used);
    let buf = unsafe {
        slice::from_raw_parts_mut(
            boxed_palette.as_mut_ptr() as *mut u8,
            colors_used * mem::size_of::<Rgb>(),
        )
    };
    reader.read_exact(buf)?;

    Ok(unsafe { boxed_palette.assume_init() })
}

pub fn qpic<R: Read>(mut reader: R) -> io::Result<Picture> {
    let width = reader.read_u32::<LittleEndian>()?;
    let height = reader.read_u32::<LittleEndian>()?;
    let indices = [chunk(&mut reader, (width * height) as usize)?];
    let palette = palette(&mut reader)?;

    Ok(Picture {
        width,
        height,
        data: ColorData { indices, palette },
    })
}

pub fn miptex<R: Read>(mut reader: R) -> io::Result<MipTexture> {
    let name = cstr16(&mut reader)?;
    let width = reader.read_u32::<LittleEndian>()?;
    let height = reader.read_u32::<LittleEndian>()?;
    let offsets: [_; MIP_LEVELS] = array::try_from_fn(|_| reader.read_u32::<LittleEndian>())?;
    let data = if offsets.iter().all(|&x| x != 0) {
        let pixels = (width * height) as usize;

        // Skip something between header and first mip indices
        for _ in 0..(offsets[0].saturating_sub(40)) {
            reader.read_u8()?;
        }
        let data_len = (pixels * 4 / 3) as u32 + 2 + 256 * 3;
        let mut cursor = Cursor::new(vec![0; data_len as usize]);
        reader.read_exact(cursor.get_mut())?;

        let indices = array::try_from_fn(|i| {
            chunk_with_offset(
                &mut cursor,
                offsets[i].saturating_sub(40) as u64,
                pixels / (1 << (2 * i)),
            )
        })?;
        let palette = palette(&mut cursor)?;

        Some(ColorData { indices, palette })
    } else {
        None
    };

    Ok(MipTexture {
        name,
        width,
        height,
        data,
    })
}

fn char_info<R: Read>(mut reader: R) -> io::Result<CharInfo> {
    Ok(CharInfo {
        offset: reader.read_u16::<LittleEndian>()?,
        width: reader.read_u16::<LittleEndian>()?,
    })
}

pub fn font<R: Read>(mut reader: R) -> io::Result<Font> {
    const GLYPHS_NUM: usize = 256;

    let _ = reader.read_u32::<LittleEndian>()?;
    let width = 256; // constant?
    let height = reader.read_u32::<LittleEndian>()?;
    let row_count = reader.read_u32::<LittleEndian>()?;
    let row_height = reader.read_u32::<LittleEndian>()?;
    let chars_info = Box::new(array::try_from_fn::<_, GLYPHS_NUM, _>(|_| {
        char_info(&mut reader)
    })?);
    let indices = [chunk(&mut reader, (width * height) as usize)?];
    let palette = palette(&mut reader)?;

    Ok(Font {
        width,
        height,
        row_count,
        row_height,
        chars_info,
        data: ColorData { indices, palette },
    })
}
