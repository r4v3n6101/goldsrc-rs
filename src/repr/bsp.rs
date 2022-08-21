use crate::{repr::map::Entities, repr::texture::MipTexture};

pub type Vec3s = [i16; 3];
pub type Vec3f = [f32; 3];
pub type Edge = (u16, u16);
pub type BBoxShort = BBox<Vec3s>;
pub type BBoxFloat = BBox<Vec3f>;

pub struct BBox<T> {
    pub min: T,
    pub max: T,
}

pub struct Plane {
    pub normal: Vec3f,
    pub distance: f32,
    pub ty: u32,
}

pub struct Node {
    pub plane_id: u32,
    pub children: [i16; 2],
    pub bounds: BBoxShort,
    pub first_face_id: u16,
    pub faces_num: u16,
}

pub struct TextureInfo {
    pub s: Vec3f,
    pub s_shift: f32,
    pub t: Vec3f,
    pub t_shift: f32,
    pub texture_id: u32,
    pub flags: u32,
}

pub struct Face {
    pub plane_id: u16,
    pub plane_side: u16,
    pub first_surfedge_id: u32,
    pub surfedges_num: u16,
    pub texture_info_id: u16,
    pub lighting_styles: [u8; 4],
    pub lightmap_offset: u32,
}

pub struct ClipNode {
    pub plane_id: u32,
    pub children: [i16; 2],
}

pub struct Leaf {
    pub contents: i32,
    //pub vis_offset: Option<u32>, // TODO : ???
    pub bounds: BBoxShort,
    pub first_mark_surface_id: u16,
    pub mark_surfaces_num: u16,
    pub ambient_levels: [u8; 4],
}

pub struct Model {
    pub bounds: BBoxFloat,
    pub origin: Vec3f,

    // TODO : unknown fields
    //_nodes: [i32; 4],
    //_vis_leafs: i32,
    pub first_face_id: u32,
    pub faces_num: u32,
}

pub struct Map {
    // TODO : pub entities: Entities,
    pub planes: Vec<Plane>,
    pub vertices: Vec<Vec3f>,
    pub nodes: Vec<Node>,
    pub texture_infos: Vec<TextureInfo>,
    pub faces: Vec<Face>,
    pub clip_nodes: Vec<ClipNode>,
    pub leaves: Vec<Leaf>,
    pub mark_surfaces: Vec<u16>,
    pub edges: Vec<Edge>,
    pub surfedges: Vec<i32>,
    pub models: Vec<Model>,
}

pub struct Level<'file> {
    pub textures: Vec<MipTexture<'file>>,
    pub lighting: &'file [u8],

    pub map: Map,
}
