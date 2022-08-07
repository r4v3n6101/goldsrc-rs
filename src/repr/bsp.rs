use crate::{repr::map::Entities, repr::texture::MipTexture};

pub type Vec3s = [i16; 3];
pub type Vec3f = [f32; 3];
pub type Edge = (usize, usize);
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
    pub plane_id: usize,
    pub children: [isize; 2],
    pub bounds: BBoxShort,
    pub first_face_id: usize,
    pub faces_num: usize,
}

pub struct TextureInfo {
    pub s: Vec3f,
    pub s_shift: f32,
    pub t: Vec3f,
    pub t_shift: f32,
    pub texture_id: usize,
    pub flags: u32,
}

pub struct Face {
    pub plane_id: usize,
    pub flipped: bool,
    pub first_surfedge_id: usize,
    pub surfedges_num: usize,
    pub texture_info_id: usize,
    pub lighting_styles: [u8; 4],
    pub lightmap_offset: usize,
}

pub struct ClipNode {
    pub plane_id: usize,
    pub children: [isize; 2],
}

pub struct Leaf {
    pub contents: i32,
    //pub vis_offset: Option<usize>, // TODO : ???
    pub bounds: BBoxShort,
    pub mark_surfaces_id: usize,
    pub mark_surfaces_num: usize,
    pub ambient_levels: [u8; 4],
}

pub struct Model {
    pub bounds: BBoxFloat,
    pub origin: Vec3f,

    // TODO : unknown fields
    //_nodes: [i32; 4],
    //_vis_leafs: i32,
    pub faces_id: usize,
    pub faces_num: usize,
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
    pub mark_surfaces: Vec<usize>,
    pub edges: Vec<Edge>,
    pub surfedges: Vec<isize>,
    pub models: Vec<Model>,
}

pub struct Level<'file> {
    pub textures: Vec<MipTexture<'file>>,
    pub lighting: &'file [u8],

    pub map: Map,
}
