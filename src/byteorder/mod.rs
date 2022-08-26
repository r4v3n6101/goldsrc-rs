use byteorder::{LittleEndian, ReadBytesExt};
use smol_str::SmolStr;
use std::{
    io::{Error as IoErr, ErrorKind as IoErrKind, Read, Result as IoRes, Seek, SeekFrom},
    str,
};

pub mod bsp;
pub mod texture;
pub mod wad;

fn chunk<R: Read>(mut reader: R, size: usize) -> IoRes<Vec<u8>> {
    let mut buf = vec![0u8; size];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

fn chunk_with_offset<R: Read + Seek>(mut reader: R, offset: u64, size: usize) -> IoRes<Vec<u8>> {
    reader.seek(SeekFrom::Start(offset))?;
    chunk(reader, size)
}

fn cstr16<R: Read>(mut reader: R) -> IoRes<SmolStr> {
    const NAME_LEN: usize = 16;

    let mut buf = [0u8; NAME_LEN];
    reader.read_exact(&mut buf)?;

    let nul_index = buf.iter().position(|&b| b == 0).unwrap_or(NAME_LEN);
    str::from_utf8(&buf[..nul_index])
        .map(SmolStr::new_inline)
        .map_err(|e| IoErr::new(IoErrKind::InvalidData, e))
}
