use std::collections::HashMap;

use crate::CStr16;

use super::texture::{Font, MipTexture, Picture};

pub type Archive = HashMap<CStr16, Content>;

#[non_exhaustive]
pub enum Content {
    Picture(Picture),
    MipTexture(MipTexture),
    Font(Font),
    Other { ty: u8, data: Vec<u8> },
}
