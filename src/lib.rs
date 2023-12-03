#![feature(new_uninit)]
#![feature(array_try_from_fn)]

#[macro_use]
extern crate static_assertions;

use std::{
    collections::HashMap,
    io::{self, Read, Seek},
};

pub use repr::*;

mod parser;
mod repr;

pub fn wad_entries<R>(
    reader: R,
    remap_name_to_lower: bool,
) -> io::Result<HashMap<CStr16, wad::Entry>>
where
    R: Read + Seek + Send + Sync + 'static,
{
    parser::wad::entries(reader, remap_name_to_lower)
}

pub fn pic<R: Read>(reader: R) -> io::Result<texture::Picture> {
    parser::texture::qpic(reader)
}

pub fn miptex<R: Read>(reader: R) -> io::Result<texture::MipTexture> {
    parser::texture::miptex(reader)
}

pub fn font<R: Read>(reader: R) -> io::Result<texture::Font> {
    parser::texture::font(reader)
}

pub fn bsp<R: Read + Seek>(reader: R) -> io::Result<bsp::Level> {
    parser::bsp::level(reader)
}
