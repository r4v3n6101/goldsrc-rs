use crate::repr::texture::{Font, MipTexture, Picture};
use smol_str::SmolStr;
use std::collections::HashMap;

pub type Archive = HashMap<SmolStr, Content>;

#[non_exhaustive]
pub enum Content {
    Picture(Picture),
    MipTexture(MipTexture),
    Font(Font),
    Other { ty: u8, data: Vec<u8> },
}
