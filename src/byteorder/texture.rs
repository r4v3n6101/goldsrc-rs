use crate::{
    byteorder::{
        chunk, chunk_with_offset, cstr16, IoRes, LittleEndian, Read, ReadBytesExt, Seek, SeekFrom,
    },
    repr::texture::{CharInfo, ColourData, Font, MipTexture, Palette, Picture, Rgb},
};
use std::{array, mem, slice};

fn palette<R: Read>(mut reader: R) -> IoRes<Box<Palette>> {
    const PALETTE_SIZE: usize = 256;

    let mut boxed_palette = Box::<Palette>::new_zeroed_slice(PALETTE_SIZE);
    let buf = unsafe {
        slice::from_raw_parts_mut(
            boxed_palette.as_mut_ptr() as *mut u8,
            PALETTE_SIZE * mem::size_of::<Rgb>(),
        )
    };
    reader.read_exact(buf)?;

    Ok(unsafe { boxed_palette.assume_init() })
}

pub fn qpic<R: Read>(mut reader: R) -> IoRes<Picture> {
    let width = reader.read_u32::<LittleEndian>()?;
    let height = reader.read_u32::<LittleEndian>()?;
    let indices = chunk(&mut reader, (width * height) as usize)?;
    let _ = reader.read_u16::<LittleEndian>()?; // palette size
    let palette = palette(&mut reader)?;

    Ok(Picture {
        width,
        height,
        data: ColourData {
            indices: [indices],
            palette,
        },
    })
}

pub fn miptex<R: Read + Seek>(mut reader: R) -> IoRes<MipTexture> {
    let begin = reader.stream_position()?;

    let name = cstr16(&mut reader)?;
    let width = reader.read_u32::<LittleEndian>()?;
    let height = reader.read_u32::<LittleEndian>()?;
    let offsets: [_; 4] = array::try_from_fn(|_| reader.read_u32::<LittleEndian>())?;
    let data = if offsets.iter().all(|&x| x != 0) {
        let pixels = (width * height) as usize;
        let indices = array::try_from_fn(|i| {
            chunk_with_offset(
                &mut reader,
                begin + offsets[i] as u64,
                pixels / (1 << (2 * i)),
            )
        })?;

        reader.seek(SeekFrom::Start(
            begin + 40 + ((pixels * 85) >> 6) as u64 + 2,
        ))?;
        let palette = palette(&mut reader)?;

        Some(ColourData { indices, palette })
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

fn char_info<R: Read>(mut reader: R) -> IoRes<CharInfo> {
    Ok(CharInfo {
        offset: reader.read_u16::<LittleEndian>()?,
        width: reader.read_u16::<LittleEndian>()?,
    })
}

pub fn font<R: Read + Seek>(mut reader: R, filesize: u32) -> IoRes<Font> {
    const GLYPHS_NUM: usize = 256;
    const QCHAR_WIDTH: u32 = 16;

    let width = reader.read_u32::<LittleEndian>()?;
    let height = reader.read_u32::<LittleEndian>()?;
    let row_count = reader.read_u32::<LittleEndian>()?;
    let row_height = reader.read_u32::<LittleEndian>()?;
    let chars_info: [_; GLYPHS_NUM] = array::try_from_fn(|_| char_info(&mut reader))?;

    let width = if filesize != 16 + 2 * 256 + (height * width * QCHAR_WIDTH) + 2 + 768 + 64 {
        256
    } else {
        width * QCHAR_WIDTH
    };

    let indices = chunk(&mut reader, (width * height) as usize)?;
    let _ = reader.read_u16::<LittleEndian>()?;
    let palette = palette(&mut reader)?;

    Ok(Font {
        width,
        height,
        row_count,
        row_height,
        chars_info,
        data: ColourData {
            indices: [indices],
            palette,
        },
    })
}
