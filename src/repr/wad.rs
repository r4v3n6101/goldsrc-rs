use crate::repr::texture::{Font, MipTexture, Picture};
use std::collections::HashMap;

pub type Archive<'file> = HashMap<&'file str, Content<'file>>;

#[non_exhaustive]
pub enum Content<'file> {
    Picture(Picture<'file>),
    MipTexture(MipTexture<'file>),
    Font(Font<'file>),
    Other { ty: u8, data: &'file [u8] },
}
