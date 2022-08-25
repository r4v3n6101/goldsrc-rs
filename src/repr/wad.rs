use crate::repr::texture::{Font, MipTexture, Picture};
use std::collections::HashMap;

pub type Archive<'file> = HashMap<&'file str, Content<'file>>;

#[non_exhaustive]
pub enum Content<'file> {
    Picture(Picture<'file>),
    MipTexture(MipTexture<'file>),
    Font(Font<'file>),

    Compressed { full_size: u32, data: &'file [u8] },
    Other(&'file [u8]),
}
