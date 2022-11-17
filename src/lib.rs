#![feature(new_uninit)]
#![feature(array_try_from_fn)]

#[macro_use]
extern crate static_assertions;

use std::io::{self, Read, Seek};

pub use repr::*;

mod parser;
mod repr;

pub fn wad<R: Read + Seek>(reader: R) -> io::Result<wad::Archive> {
    parser::wad::archive(reader)
}

pub fn bsp<R: Read + Seek>(reader: R) -> io::Result<bsp::Level> {
    parser::bsp::level(reader)
}
