use crate::{
    bytes::{
        cstr16, eof, invalid_magic,
        texture::{font, miptex, qpic},
    },
    repr::wad::{Archive, Content},
};
use bytes::{Buf, Bytes};
use std::{collections::HashMap, io};

pub fn archive(buf: Bytes) -> io::Result<Archive> {
    const MAGIC: &[u8; 4] = b"WAD3";

    if buf.len() < 12 {
        return eof();
    }
    let mut header_buf = buf.slice(..12);
    let mut header = [0u8; 4];
    header_buf.copy_to_slice(&mut header);
    if &header != MAGIC {
        return invalid_magic();
    }
    let size = header_buf.get_u32_le() as usize;
    let offset = header_buf.get_u32_le() as usize;

    if buf.len() < offset + size * 32 {
        return eof();
    }
    let mut entries_buf = buf.slice(offset as usize..);

    let mut archive = HashMap::with_capacity(size as usize);
    for _ in 0..size {
        let offset = entries_buf.get_u32_le() as usize;
        let size = entries_buf.get_u32_le() as usize;
        let full_size = entries_buf.get_u32_le();
        let ty = entries_buf.get_u8();
        let compression = entries_buf.get_u8();
        let _ = entries_buf.get_u16(); // dummy
        let name = cstr16(&mut entries_buf)?;

        if compression != 0 {
            unimplemented!("compression not supported by goldsrc")
        }
        if buf.len() < offset + size {
            return eof();
        }
        let data_buf = buf.slice(offset..offset + size);

        let content = match ty {
            0x42 => Content::Picture(qpic(data_buf)?),
            0x43 => Content::MipTexture(miptex(data_buf)?),
            0x46 => Content::Font(font(data_buf)?),
            ty => Content::Other {
                ty,
                data: data_buf.to_vec(),
            },
        };

        archive.insert(name, content);
    }

    Ok(archive)
}
