use super::{map::Entities, texture::MipTexture};

/// 3D vector with 16-bit integer components.
pub type Vec3s = [i16; 3];

/// 3D vector with 32-bit float components.
pub type Vec3f = [f32; 3];

/// Edge represented as two vertex indices.
pub type Edge = (u16, u16);

/// Axis-aligned bounding box with short integer components.
pub type BBoxShort = BBox<Vec3s>;

/// Axis-aligned bounding box with float components.
pub type BBoxFloat = BBox<Vec3f>;

/// Complete level data loaded from a BSP file.
#[derive(Debug, Clone)]
pub struct Level {
    /// List of entities in the level.
    pub entities: Entities,

    /// Planes used for spatial partitioning.
    pub planes: Vec<Plane>,

    /// Textures used in the level.
    pub textures: Vec<MipTexture>,

    /// Vertices positions in 3D space.
    pub vertices: Vec<Vec3f>,

    /// Raw visibility data (TODO: parsing details).
    pub visdata: Vec<u8>,

    /// BSP nodes for the spatial partitioning tree.
    pub nodes: Vec<Node>,

    /// Texture mapping info for faces.
    pub texture_infos: Vec<TextureInfo>,

    /// Faces (polygons) of the level geometry.
    pub faces: Vec<Face>,

    /// Lighting data (lightmaps).
    pub lighting: Vec<u8>,

    /// Nodes used for collision detection.
    pub clip_nodes: Vec<ClipNode>,

    /// Leaf nodes in the BSP tree.
    pub leaves: Vec<Leaf>,

    /// Surface indices for each leaf.
    pub mark_surfaces: Vec<u16>,

    /// Edges of the mesh.
    pub edges: Vec<Edge>,

    /// Indices into the edges array for each face.
    pub surfedges: Vec<i32>,

    /// Models in the level (usually for world or entities).
    pub models: Vec<Model>,
}

/// Plane used for BSP partitioning.
#[derive(Debug, Clone)]
pub struct Plane {
    /// Normal vector of the plane.
    pub normal: Vec3f,

    /// Distance from the origin along the normal.
    pub distance: f32,

    /// Plane type / classification (specific to BSP format).
    pub ty: u32,
}

/// Node in the BSP tree.
#[derive(Debug, Clone)]
pub struct Node {
    /// Index of the splitting plane.
    pub plane_id: u32,

    /// Indices of child nodes (-1 indicates a leaf).
    pub children: [i16; 2],

    /// Bounding box of the node.
    pub bounds: BBoxShort,

    /// First face associated with this node.
    pub first_face_id: u16,

    /// Number of faces in this node.
    pub faces_num: u16,
}

/// Texture mapping info for a face.
#[derive(Debug, Clone)]
pub struct TextureInfo {
    /// S vector for texture mapping.
    pub s: Vec3f,

    /// S vector shift/offset.
    pub s_shift: f32,

    /// T vector for texture mapping.
    pub t: Vec3f,

    /// T vector shift/offset.
    pub t_shift: f32,

    /// Index of the texture in the textures array.
    pub texture_id: u32,

    /// Flags for texture rendering.
    pub flags: u32,
}

/// Face (polygon) in the level geometry.
#[derive(Debug, Clone)]
pub struct Face {
    /// Plane this face lies on.
    pub plane_id: u16,

    /// Which side of the plane the face is on.
    pub plane_side: u16,

    /// First index into the `surfedges` array.
    pub first_surfedge_id: u32,

    /// Number of surfedges.
    pub surfedges_num: u16,

    /// Index into the texture infos array.
    pub texture_info_id: u16,

    /// Lighting styles for the face.
    pub lighting_styles: [u8; 4],

    /// Offset into the lightmap data.
    pub lightmap_offset: u32,
}

/// Collision node used for BSP-based physics.
#[derive(Debug, Clone)]
pub struct ClipNode {
    /// Plane index used for clipping.
    pub plane_id: u32,

    /// Indices of child nodes (-1 indicates a leaf).
    pub children: [i16; 2],
}

/// Leaf in the BSP tree.
#[derive(Debug, Clone)]
pub struct Leaf {
    /// Contents type (solid, empty, water, etc.).
    pub contents: i32,

    /// Offset to visibility data (TODO: rename for clarity).
    pub vis_offset: i32,

    /// Bounding box of the leaf.
    pub bounds: BBoxShort,

    /// Index of the first mark surface in this leaf.
    pub first_mark_surface_id: u16,

    /// Number of mark surfaces.
    pub mark_surfaces_num: u16,

    /// Ambient lighting levels for the leaf.
    pub ambient_levels: [u8; 4],
}

/// Model (subsection of the level geometry).
#[derive(Debug, Clone)]
pub struct Model {
    /// Bounding box of the model.
    pub bounds: BBoxFloat,

    /// Origin of the model.
    pub origin: Vec3f,

    /// Node indices associated with this model (TODO: rename).
    pub nodes: [i32; 4],

    /// Number of visible leafs.
    pub vis_leafs: i32,

    /// First face index.
    pub first_face_id: u32,

    /// Number of faces in the model.
    pub faces_num: u32,
}

/// Axis-aligned bounding box.
#[derive(Debug, Clone)]
pub struct BBox<T> {
    /// Minimum coordinates.
    pub min: T,

    /// Maximum coordinates.
    pub max: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_eq_size!(Plane, [u8; 20]);
    assert_eq_size!(Node, [u8; 24]);
    assert_eq_size!(TextureInfo, [u8; 40]);
    assert_eq_size!(Face, [u8; 20]);
    assert_eq_size!(ClipNode, [u8; 8]);
    assert_eq_size!(Leaf, [u8; 28]);
    assert_eq_size!(Model, [u8; 64]);
    assert_eq_size!(Vec3s, [u8; 6]);
    assert_eq_size!(Vec3f, [u8; 12]);
    assert_eq_size!(Edge, [u8; 4]);
    assert_eq_size!(BBoxShort, [Vec3s; 2]);
    assert_eq_size!(BBoxFloat, [Vec3f; 2]);
}
