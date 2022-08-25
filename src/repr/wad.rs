use crate::repr::texture::{Font, MipTexture, Picture};
use std::collections::HashMap;

pub type Archive = HashMap<String, Content>;

#[non_exhaustive]
pub enum Content {
    Picture(Picture),
    MipTexture(MipTexture),
    Font(Font),
    Other { ty: u8, data: Vec<u8> },
}
