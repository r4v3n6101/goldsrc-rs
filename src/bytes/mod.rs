pub use bytes::{Buf, Bytes};
use smol_str::SmolStr;
use std::{io, str};

pub mod texture;
pub mod wad;

fn eof<T>() -> io::Result<T> {
    return Err(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "not enough bytes",
    ));
}

fn invalid_magic<T>() -> io::Result<T> {
    return Err(io::Error::new(io::ErrorKind::Unsupported, "invalid magic"));
}

fn cstr16<B: Buf>(buf: &mut B) -> io::Result<SmolStr> {
    const NAME_LEN: usize = 16;

    let buf = buf.copy_to_bytes(NAME_LEN);
    let nul_index = buf.chunk().iter().position(|&b| b == 0).unwrap_or(NAME_LEN);

    str::from_utf8(&buf[..nul_index])
        .map(SmolStr::new_inline)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
