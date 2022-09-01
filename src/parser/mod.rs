use std::io;

use crate::{bsp::Level, wad::Archive};

#[cfg(feature = "byteorder")]
mod byteorder;
#[cfg(feature = "nom")]
mod nom;

#[cfg(feature = "nom")]
#[inline(always)]
fn nom_to_io<T, E>(res: Result<(&[u8], T), E>, msg: &'static str) -> io::Result<T> {
    res.map(|(_, a)| a)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, msg))
}

#[cfg(feature = "byteorder")]
pub fn wad<R: io::Read + io::Seek>(reader: R) -> io::Result<Archive> {
    byteorder::wad::archive(reader)
}

#[cfg(feature = "byteorder")]
pub fn bsp<R: io::Read + io::Seek>(reader: R) -> io::Result<Level> {
    byteorder::bsp::level(reader)
}

#[cfg(feature = "nom")]
pub fn wad_from_bytes(input: &[u8]) -> io::Result<Archive> {
    nom_to_io(nom::wad::archive(input), "error parsing wad")
}

#[cfg(feature = "nom")]
pub fn bsp_from_bytes(input: &[u8]) -> io::Result<Level> {
    nom_to_io(nom::bsp::level(input), "error parsing bsp")
}
