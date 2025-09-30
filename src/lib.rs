#![feature(array_try_from_fn)]

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

use std::{
    collections::HashMap,
    io::{self, Read, Seek},
};

pub use repr::*;

mod parser;
mod repr;

/// Parse all entries from a WAD file.
///
/// # Arguments
/// - `reader` — Any type implementing [`Read`] + [`Seek`] + [`Send`] + `'static`.
///   Typically a file handle or in-memory buffer containing the WAD data.
/// - `remap_name_to_lower` — If `true`, forces all entries names to lowercase
///   (useful because WAD entries are case-insensitive in practice).
///
/// # Returns
/// A [`HashMap`] mapping lump names ([`CStr16`]) to their corresponding [`wad::Entry`].
///
/// # Errors
/// Returns an [`io::Error`] if the stream cannot be read or the WAD is invalid.
///
/// # Example
/// ```no_run
/// use std::fs::File;
///
/// let file = File::open("halflife.wad").expect("file open");
/// let entries = goldsrc_rs::wad_entries(file, true).expect("parsing wad header");
/// ```
pub fn wad_entries<R>(
    reader: R,
    remap_name_to_lower: bool,
) -> io::Result<HashMap<CStr16, wad::Entry>>
where
    R: Read + Seek + Send + 'static,
{
    parser::wad::entries(reader, remap_name_to_lower)
}

/// Parse a Half-Life BSP level file.
///
/// # Arguments
/// - `reader` — Any type implementing [`Read`] + [`Seek`].
///
/// # Returns
/// A fully parsed [`bsp::Level`] struct with geometry, entities, textures, and more.
///
/// # Errors
/// Returns an [`io::Error`] if the BSP is malformed or the stream cannot be read.
///
/// # Example
/// ```no_run
/// use std::fs::File;
///
/// let file = File::open("c1a0.bsp").expect("file opening");
/// let level = goldsrc_rs::bsp(file).expect("map parsing");
/// ```
pub fn bsp<R: Read + Seek>(reader: R) -> io::Result<bsp::Level> {
    parser::bsp::level(reader)
}

/// Parse a Quake-style `.pic` image (qpic).
///
/// # Arguments
/// - `reader` — Any type implementing [`Read`].
///
/// # Returns
/// A [`texture::Picture`] representing the decoded image.
///
/// # Errors
/// Returns an [`io::Error`] if the stream cannot be read or is not a valid qpic.
pub fn pic<R: Read>(reader: R) -> io::Result<texture::Picture> {
    parser::texture::qpic(reader)
}

/// Parse a Half-Life `miptex` texture lump.
///
/// # Arguments
/// - `reader` — Any type implementing [`Read`].
///
/// # Returns
/// A [`texture::MipTexture`] containing all mipmap levels and optional palette data.
///
/// # Errors
/// Returns an [`io::Error`] if parsing fails.
pub fn miptex<R: Read>(reader: R) -> io::Result<texture::MipTexture> {
    parser::texture::miptex(reader)
}

/// Parse a Half-Life font resource (e.g. used for HUD text).
///
/// # Arguments
/// - `reader` — Any type implementing [`Read`].
///
/// # Returns
/// A [`texture::Font`] object with character bitmaps and metadata.
///
/// # Errors
/// Returns an [`io::Error`] if parsing fails.
pub fn font<R: Read>(reader: R) -> io::Result<texture::Font> {
    parser::texture::font(reader)
}
