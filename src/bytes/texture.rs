use crate::{
    bytes::{cstr16, eof},
    repr::texture::{CharInfo, ColourData, Font, MipTexture, Palette, Picture, Rgb},
};
use bytes::{Buf, Bytes};
use std::{array, io, mem};

const PALETTE_SIZE: usize = 256 * 3;

fn palette<B: Buf>(buf: &B) -> Box<Palette> {
    let mut boxed_palette =
        Box::<Palette>::new_zeroed_slice(buf.remaining() / mem::size_of::<Rgb>());
    unsafe {
        (boxed_palette.as_mut_ptr() as *mut u8)
            .copy_from_nonoverlapping(buf.chunk().as_ptr(), buf.remaining());
        boxed_palette.assume_init()
    }
}

pub fn qpic(mut buf: Bytes) -> io::Result<Picture> {
    if buf.len() < 8 {
        return eof();
    }
    let width = buf.get_u32_le();
    let height = buf.get_u32_le();
    let pixels = (width * height) as usize;

    if buf.len() < pixels + 2 + PALETTE_SIZE {
        return eof();
    }
    let indices = [buf.split_to(pixels).to_vec()];
    let _ = buf.get_u16_le(); // palette size
    let palette = palette(&buf.split_to(PALETTE_SIZE));

    Ok(Picture {
        width,
        height,
        data: ColourData { indices, palette },
    })
}

pub fn miptex(buf: Bytes) -> io::Result<MipTexture> {
    if buf.len() < 40 {
        return eof();
    }
    let mut header_buf = buf.slice(..40);
    let name = cstr16(&mut header_buf)?;
    let width = header_buf.get_u32_le();
    let height = header_buf.get_u32_le();
    let offsets: [_; 4] = array::from_fn(|_| header_buf.get_u32_le());

    let data = if offsets.iter().all(|&x| x != 0) {
        let pixels = (width * height) as usize;
        let indices = array::try_from_fn(|i| {
            let offset = offsets[i] as usize;
            let size = pixels / (1 << (2 * i));
            if buf.len() < offset + size {
                return eof();
            }
            Ok(buf.slice(offset..offset + size).to_vec())
        })?;

        let palette_offset = 40 + ((pixels * 85) >> 6) + 2;
        if buf.len() < palette_offset + PALETTE_SIZE {
            return eof();
        }
        let palette = palette(&buf.slice(palette_offset..palette_offset + PALETTE_SIZE));

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

pub fn font(mut buf: Bytes) -> io::Result<Font> {
    const GLYPHS_NUM: usize = 256;
    const QCHAR_WIDTH: u32 = 16;

    if buf.len() < 16 + 4 * GLYPHS_NUM {
        return eof();
    }
    let width = buf.get_u32_le();
    let height = buf.get_u32_le();
    let row_count = buf.get_u32_le();
    let row_height = buf.get_u32_le();
    let chars_info: [_; GLYPHS_NUM] = array::from_fn(|_| CharInfo {
        offset: buf.get_u16_le(),
        width: buf.get_u16_le(),
    });

    let width = if buf.len() != (height * width * QCHAR_WIDTH) as usize + 2 + PALETTE_SIZE + 64 {
        256
    } else {
        width * QCHAR_WIDTH
    };
    let pixels = (width * height) as usize;

    if buf.len() < pixels + 2 + PALETTE_SIZE {
        return eof();
    }
    let indices = [buf.split_to(pixels).to_vec()];
    let _ = buf.get_u16_le();
    let palette = palette(&buf.split_to(PALETTE_SIZE));

    Ok(Font {
        width,
        height,
        row_count,
        row_height,
        chars_info,
        data: ColourData { indices, palette },
    })
}
