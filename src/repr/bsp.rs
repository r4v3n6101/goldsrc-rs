use super::{map::Entities, texture::MipTexture};

assert_eq_size!(Vec3s, [u8; 6]);
pub type Vec3s = [i16; 3];

assert_eq_size!(Vec3f, [u8; 12]);
pub type Vec3f = [f32; 3];

assert_eq_size!(Edge, [u8; 4]);
pub type Edge = (u16, u16);

assert_eq_size!(BBoxShort, [Vec3s; 2]);
pub type BBoxShort = BBox<Vec3s>;

assert_eq_size!(BBoxFloat, [Vec3f; 2]);
pub type BBoxFloat = BBox<Vec3f>;

pub struct BBox<T> {
    pub min: T,
    pub max: T,
}

assert_eq_size!(Plane, [u8; 20]);
pub struct Plane {
    pub normal: Vec3f,
    pub distance: f32,
    pub ty: u32,
}

assert_eq_size!(Node, [u8; 24]);
pub struct Node {
    pub plane_id: u32,
    pub children: [i16; 2],
    pub bounds: BBoxShort,
    pub first_face_id: u16,
    pub faces_num: u16,
}

assert_eq_size!(TextureInfo, [u8; 40]);
pub struct TextureInfo {
    pub s: Vec3f,
    pub s_shift: f32,
    pub t: Vec3f,
    pub t_shift: f32,
    pub texture_id: u32,
    pub flags: u32,
}

assert_eq_size!(Face, [u8; 20]);
pub struct Face {
    pub plane_id: u16,
    pub plane_side: u16,
    pub first_surfedge_id: u32,
    pub surfedges_num: u16,
    pub texture_info_id: u16,
    pub lighting_styles: [u8; 4],
    pub lightmap_offset: u32,
}

assert_eq_size!(ClipNode, [u8; 8]);
pub struct ClipNode {
    pub plane_id: u32,
    pub children: [i16; 2],
}

assert_eq_size!(Leaf, [u8; 28]);
pub struct Leaf {
    pub contents: i32,
    // TODO : rename probably
    pub vis_offset: i32,
    pub bounds: BBoxShort,
    pub first_mark_surface_id: u16,
    pub mark_surfaces_num: u16,
    pub ambient_levels: [u8; 4],
}

assert_eq_size!(Model, [u8; 64]);
pub struct Model {
    pub bounds: BBoxFloat,
    pub origin: Vec3f,

    // TODO : rename
    pub _nodes: [i32; 4],
    pub _vis_leafs: i32,

    pub first_face_id: u32,
    pub faces_num: u32,
}

pub struct Level {
    pub entities: Entities,
    pub planes: Vec<Plane>,
    pub textures: Vec<MipTexture>,
    pub vertices: Vec<Vec3f>,
    // TODO
    pub visdata: Vec<u8>,
    pub nodes: Vec<Node>,
    pub texture_infos: Vec<TextureInfo>,
    pub faces: Vec<Face>,
    pub lighting: Vec<u8>,
    pub clip_nodes: Vec<ClipNode>,
    pub leaves: Vec<Leaf>,
    pub mark_surfaces: Vec<u16>,
    pub edges: Vec<Edge>,
    pub surfedges: Vec<i32>,
    pub models: Vec<Model>,
}
