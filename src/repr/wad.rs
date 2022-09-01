use std::collections::HashMap;

use smol_str::SmolStr;

use super::texture::{Font, MipTexture, Picture};

pub type Archive = HashMap<SmolStr, Content>;

#[non_exhaustive]
pub enum Content {
    Picture(Picture),
    MipTexture(MipTexture),
    Font(Font),
    Other { ty: u8, data: Vec<u8> },
}
