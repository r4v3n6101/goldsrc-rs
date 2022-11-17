use std::{
    collections::HashMap,
    io::{self, Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
    wad::{Archive, Content},
    CStr16,
};

use super::{
    chunk, cstr16,
    texture::{font, miptex, qpic},
};

#[allow(dead_code)]
struct Entry {
    offset: u32,
    full_size: u32,
    size: u32,
    ty: u8,
    compression: u8,
    name: CStr16,
}

pub fn archive<R: Read + Seek>(mut reader: R) -> io::Result<Archive> {
    const MAGIC: &[u8; 4] = b"WAD3";

    let mut header = [0u8; 4];
    reader.read_exact(&mut header)?;
    if &header != MAGIC {
        return Err(io::Error::new(io::ErrorKind::Unsupported, "invalid magic"));
    }
    let size = reader.read_u32::<LittleEndian>()?;
    let offset = reader.read_u32::<LittleEndian>()?;

    reader.seek(SeekFrom::Start(offset as u64))?;
    let mut entries = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let offset = reader.read_u32::<LittleEndian>()?;
        let size = reader.read_u32::<LittleEndian>()?;
        let full_size = reader.read_u32::<LittleEndian>()?; // full_size, not used
        let ty = reader.read_u8()?;
        let compression = reader.read_u8()?;
        let _ = reader.read_u16::<LittleEndian>(); // dummy
        let name = cstr16(&mut reader)?;

        entries.push(Entry {
            offset,
            full_size,
            size,
            ty,
            compression,
            name,
        })
    }

    let mut archive = HashMap::with_capacity(entries.len());
    for entry in entries {
        reader.seek(SeekFrom::Start(entry.offset as u64))?;

        if entry.compression != 0 {
            unimplemented!("compression not supported by goldsrc")
        }

        let content = match entry.ty {
            0x42 => Content::Picture(qpic(&mut reader)?),
            0x43 => Content::MipTexture(miptex(&mut reader)?),
            0x46 => Content::Font(font(&mut reader)?),
            ty => Content::Other {
                ty,
                data: chunk(&mut reader, entry.size as usize)?,
            },
        };

        archive.insert(entry.name, content);
    }

    Ok(archive)
}
