use std::{
    collections::HashMap,
    io::{self, Read, Seek, SeekFrom},
    sync::{Arc, Mutex},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::wad::{ContentType, RawArchive, RawEntry, Reader};

use super::cstr16;

pub fn raw_archive<R: Read + Seek + Send + Sync + 'static>(reader: R) -> io::Result<RawArchive> {
    const MAGIC: &[u8; 4] = b"WAD3";

    let reader: Arc<Mutex<dyn Reader>> = Arc::new(Mutex::new(reader));
    let mut reader_ref = reader.lock().unwrap();

    let mut header = [0u8; 4];
    reader_ref.read_exact(&mut header)?;
    if &header != MAGIC {
        return Err(io::Error::new(io::ErrorKind::Unsupported, "invalid magic"));
    }
    let size = reader_ref.read_u32::<LittleEndian>()?;
    let offset = reader_ref.read_u32::<LittleEndian>()?;

    reader_ref.seek(SeekFrom::Start(offset as u64))?;
    let mut entries = HashMap::with_capacity(size as usize);
    for _ in 0..size {
        let offset = reader_ref.read_u32::<LittleEndian>()?;
        let size = reader_ref.read_u32::<LittleEndian>()?;
        let full_size = reader_ref.read_u32::<LittleEndian>()?; // full_size, not used
        let ty = match reader_ref.read_u8()? {
            0x42 => ContentType::Picture,
            0x43 => ContentType::MipTexture,
            0x46 => ContentType::Font,
            unknown => ContentType::Other(unknown),
        };
        let compression = reader_ref.read_u8()?;
        let _ = reader_ref.read_u16::<LittleEndian>(); // dummy
        let name = cstr16(&mut *reader_ref)?;

        entries.insert(
            name,
            RawEntry {
                source: Arc::clone(&reader),
                offset,
                full_size,
                size,
                ty,
                compression,
            },
        );
    }

    Ok(RawArchive { entries })
}
