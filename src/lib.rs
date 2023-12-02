#![feature(new_uninit)]
#![feature(array_try_from_fn)]

#[macro_use]
extern crate static_assertions;

use std::io::{self, Read, Seek};

pub use repr::*;

mod parser;
mod repr;

pub fn raw_wad<R: Read + Seek + 'static>(reader: R) -> io::Result<wad::RawArchive> {
    parser::wad::raw_archive(reader)
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
