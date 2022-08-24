use crate::repr::texture::{MipTexture, Picture};
use std::collections::HashMap;

pub type Archive<'file> = HashMap<&'file str, Content<'file>>;

pub enum Content<'file> {
    // Font(..),
    Picture(Picture<'file>),
    MipTexture(MipTexture<'file>),
    Compressed { full_size: u32, data: &'file [u8] },
    Other(&'file [u8]),
}
