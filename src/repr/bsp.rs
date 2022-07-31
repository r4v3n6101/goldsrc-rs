use bumpalo::Bump;
use either::Either;

use crate::repr::{map::Entity, texture::MipTexture};

pub type Vec3 = [f32; 3];
pub type BBoxShort = BBox<i16>;
pub type BBoxFloat = BBox<f32>;

#[derive(Copy, Clone)]
pub enum Content {
    Empty = -1,
    Solid = -2,
    Water = -3,
    Slime = -4,
    Lava = -5,
    Sky = -6,
    Origin = -7,
    Clip = -8,
    Current0 = -9,
    Current90 = -10,
    Current180 = -11,
    Current270 = -12,
    CurrentUp = -13,
    CurrentDown = -14,
    Translucent = -15,
}

pub struct BBox<T> {
    pub min: [T; 3],
    pub max: [T; 3],
}

pub struct TextureInfo<'a> {
    pub s_axis: Vec3,
    pub s_shift: f32,
    pub t_axis: Vec3,
    pub t_shift: f32,
    pub texture_name: &'a str,
    pub flags: u32,
}

pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
    pub ty: u32,
}

pub type ClipChild<'a> = Either<&'a ClipNode<'a>, Content>;
pub struct ClipNode<'a> {
    pub plane: &'a Plane,
    pub children: [ClipChild<'a>; 2],
}

pub struct Face<'a> {
    pub parallel_plane: &'a Plane,
    pub normal_reversed: bool,
    pub vertices: Vec<&'a Vec3, &'a Bump>,
    pub texture_info: &'a TextureInfo<'a>,
    pub lighting_styles: [u8; 4],
    pub lightmap_offset: &'a [u8], // not bounded by len
}

pub type NodeChild<'a> = Either<&'a Node<'a>, &'a Leaf<'a>>;
pub struct Node<'a> {
    pub plane: &'a Plane,
    pub children: [NodeChild<'a>; 2],
    pub bounds: BBoxShort,
    pub faces: &'a [Face<'a>],
}

pub struct Leaf<'a> {
    pub contents: Content,
    pub vis_offset: Option<usize>, // TODO : ???
    pub bounds: BBoxShort,
    pub faces: Vec<&'a Face<'a>, &'a Bump>,
    pub ambient_levels: [u8; 4],
}

pub struct Model<'a> {
    pub bounds: BBox<f32>,
    pub origin: Vec3,

    // TODO : unknown fields
    //_nodes: [i32; 4],
    //_vis_leafs: i32,
    pub faces: &'a [Face<'a>],
}

pub struct Map<'a> {
    pub arena: Bump,
    pub models: Vec<Model<'a>, &'a Bump>,
}

pub struct Level<'map, 'file> {
    // TODO : remove file lifetime and remove fields... or not?
    pub entities: Vec<Entity<'file>>,
    pub textures: Vec<MipTexture<'file>>,
    pub vis_data: &'file [u8], // TODO move into Map and allocate

    pub map: Map<'map>,
}
