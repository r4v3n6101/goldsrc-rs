use crate::repr::texture::MipTexture;
use std::collections::HashMap;

pub type Archive<'file> = HashMap<&'file str, Content<'file>>;

pub enum Content<'file> {
    // Font(..),
    MipTexture(MipTexture<'file>),
    Compressed { full_size: u32, data: &'file [u8] },
    Other(&'file [u8]),
}
