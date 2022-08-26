use crate::{
    byteorder::{
        chunk, chunk_with_offset, cstr16, IoRes, LittleEndian, Read, ReadBytesExt, Seek, SeekFrom,
    },
    repr::{
        texture::CharInfo,
        texture::{ColourData, Font, MipTexture, Palette, Picture},
    },
};
use std::array;

fn palette<R: Read>(mut reader: R) -> IoRes<Palette> {
    const PALETTE_SIZE: usize = 256;

    let mut buf = [0u8; PALETTE_SIZE * 3]; // TODO : no default?
    reader.read_exact(&mut buf)?;
    Ok(buf)
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

        reader.seek(SeekFrom::Start(begin + ((pixels * 85) >> 6) as u64 + 2))?;
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
