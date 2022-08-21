use crate::repr::texture::MipTexture;
use std::collections::HashMap;

pub type Archive<'file> = HashMap<&'file str, Content<'file>>;

pub enum Content<'file> {
    // Compressed(Content<'file>),
    // Font(..),
    MipTexture(MipTexture<'file>),
    Other(&'file [u8]),
}
