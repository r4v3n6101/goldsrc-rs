use static_assertions::assert_eq_size;
use zerocopy::{
    FromBytes, Immutable,
    little_endian::{F32, I16, I32, U16, U32},
};
use zerocopy_derive::*;

use crate::{
    error::{ParsingError, ParsingResult},
    texture::{self, MipTexture},
    util,
};

/// BSP version (GoldSrc/Quake 1 format).
pub const BSP_VERSION: u32 = 30;
/// Number of lumps in a BSP header.
pub const BSP_LUMPS: usize = 15;

/// 3D vector with 16-bit integer components.
pub type Vec3s = [I16; 3];
/// 3D vector with 32-bit float components.
pub type Vec3f = [F32; 3];
/// Edge represented as two vertex indices.
pub type Edge = [U16; 2];

/// Complete level data loaded from a BSP file.
pub struct Level<'a> {
    /// List of entities in the level.
    pub entities: &'a [u8],
    /// Planes used for spatial partitioning.
    pub planes: &'a [Plane],
    /// Textures used in the level (miptex).
    pub textures: Vec<MipTexture<'a>>,
    /// Vertices positions in 3D space.
    pub vertices: &'a [Vec3f],
    /// RLE visibility data.
    pub visdata: &'a [u8],
    /// BSP nodes for the spatial partitioning tree.
    pub nodes: &'a [Node],
    /// Texture mapping info for faces.
    pub texture_infos: &'a [TextureInfo],
    /// Faces (polygons) of the level geometry.
    pub faces: &'a [Face],
    /// Lighting data (lightmaps).
    pub lighting: &'a [u8],
    /// Nodes used for collision detection.
    pub clip_nodes: &'a [ClipNode],
    /// Leaf nodes in the BSP tree.
    pub leaves: &'a [Leaf],
    /// Surface indices for each leaf.
    pub mark_surfaces: &'a [U16],
    /// Edges of the mesh.
    pub edges: &'a [Edge],
    /// Indices into the edges array for each face.
    pub surfedges: &'a [I32],
    /// Models in the level (usually for world or entities).
    pub models: &'a [Model],
}

/// BSP header (version + lump).
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct LevelHeader {
    /// BSP format version.
    pub version: U32,
    /// Lump entries.
    pub lumps: [LumpInfo; BSP_LUMPS],
}

/// Lump entry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct LumpInfo {
    /// Offset from start of file.
    pub offset: U32,
    /// Size in bytes.
    pub size: U32,
}

/// Plane used for BSP partitioning.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Plane {
    /// Normal vector of the plane.
    pub normal: Vec3f,
    /// Distance from the origin along the normal.
    pub distance: F32,
    /// Plane type / classification (specific to BSP format).
    pub ty: U32,
}

/// Node in the BSP tree.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Node {
    /// Index of the splitting plane.
    pub plane_id: U32,
    /// Indices of child nodes (-1 indicates a leaf).
    pub children: [I16; 2],
    /// Bounding box of the node.
    pub bounds: BBox<Vec3s>,
    /// First face associated with this node.
    pub first_face_id: U16,
    /// Number of faces in this node.
    pub faces_num: U16,
}

/// Texture mapping info for a face.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct TextureInfo {
    /// S vector for texture mapping.
    pub s: Vec3f,
    /// S vector shift/offset.
    pub s_shift: F32,
    /// T vector for texture mapping.
    pub t: Vec3f,
    /// T vector shift/offset.
    pub t_shift: F32,
    /// Index of the texture in the textures array.
    pub texture_id: U32,
    /// Flags for texture rendering.
    pub flags: U32,
}

/// Face (polygon) in the level geometry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Face {
    /// Plane this face lies on.
    pub plane_id: U16,
    /// Which side of the plane the face is on.
    pub plane_side: U16,
    /// First index into the `surfedges` array.
    pub first_surfedge_id: U32,
    /// Number of surfedges.
    pub surfedges_num: U16,
    /// Index into the texture infos array.
    pub texture_info_id: U16,
    /// Lighting styles for the face.
    pub lighting_styles: [u8; 4],
    /// Offset into the lightmap data.
    pub lightmap_offset: U32,
}

/// Collision node used for BSP-based physics.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct ClipNode {
    /// Plane index used for clipping.
    pub plane_id: U32,
    /// Indices of child nodes (-1 indicates a leaf).
    pub children: [I16; 2],
}

/// Leaf in the BSP tree.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Leaf {
    /// Contents type (solid, empty, water, etc.).
    pub contents: I32,
    /// Offset to visibility data (TODO: rename for clarity).
    pub vis_offset: I32,
    /// Bounding box of the leaf.
    pub bounds: BBox<Vec3s>,
    /// Index of the first mark surface in this leaf.
    pub first_mark_surface_id: U16,
    /// Number of mark surfaces.
    pub mark_surfaces_num: U16,
    /// Ambient lighting levels for the leaf.
    pub ambient_levels: [u8; 4],
}

/// Model (subsection of the level geometry).
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Model {
    /// Bounding box of the model.
    pub bounds: BBox<Vec3f>,
    /// Origin of the model.
    pub origin: Vec3f,
    /// Node indices associated with this model.
    pub nodes: [I32; 4],
    /// Number of visible leafs.
    pub vis_leafs: I32,
    /// First face index.
    pub first_face_id: U32,
    /// Number of faces in the model.
    pub faces_num: U32,
}

/// Axis-aligned bounding box.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct BBox<T> {
    /// Minimum coordinates.
    pub min: T,
    /// Maximum coordinates.
    pub max: T,
}

pub fn level(bytes: &[u8]) -> ParsingResult<Level<'_>> {
    let (header, _) =
        LevelHeader::ref_from_prefix(bytes).map_err(|_| ParsingError::OutOfRange("bsd header"))?;

    let version = header.version.get();
    if version != BSP_VERSION {
        return Err(ParsingError::WrongVersion {
            got: version,
            expected: BSP_VERSION,
        });
    }

    Ok(Level {
        entities: lump_ref(bytes, &header.lumps[0])?,
        planes: lump_ref(bytes, &header.lumps[1])?,
        textures: miptex_lump(lump_ref::<u8>(bytes, &header.lumps[2])?)?,
        vertices: lump_ref(bytes, &header.lumps[3])?,
        visdata: lump_ref(bytes, &header.lumps[4])?,
        nodes: lump_ref(bytes, &header.lumps[5])?,
        texture_infos: lump_ref(bytes, &header.lumps[6])?,
        faces: lump_ref(bytes, &header.lumps[7])?,
        lighting: lump_ref(bytes, &header.lumps[8])?,
        clip_nodes: lump_ref(bytes, &header.lumps[9])?,
        leaves: lump_ref(bytes, &header.lumps[10])?,
        mark_surfaces: lump_ref(bytes, &header.lumps[11])?,
        edges: lump_ref(bytes, &header.lumps[12])?,
        surfedges: lump_ref(bytes, &header.lumps[13])?,
        models: lump_ref(bytes, &header.lumps[14])?,
    })
}

fn lump_ref<'a, T>(bytes: &'a [u8], lump: &LumpInfo) -> ParsingResult<&'a [T]>
where
    T: Immutable + FromBytes,
{
    let data = bytes
        .get(util::to_validate_range(
            lump.offset.get(),
            lump.size.get(),
            "bsp lump",
        )?)
        .ok_or(ParsingError::OutOfRange("bsp lump"))?;
    <[T]>::ref_from_bytes(data).map_err(|_| ParsingError::Invalid("bsp lump"))
}

fn miptex_lump(bytes: &[u8]) -> ParsingResult<Vec<MipTexture<'_>>> {
    let offsets = miptex_offsets(bytes)?;
    let mut textures = Vec::with_capacity(offsets.len());

    for offset in offsets {
        let offset = usize::try_from(offset.get())
            .map_err(|_| ParsingError::NumberOverflow("bsp miptex offset"))?;
        let slice = bytes
            .get(offset..)
            .ok_or(ParsingError::OutOfRange("bsp miptex"))?;

        textures.push(texture::mip_texture(slice)?);
    }

    Ok(textures)
}

fn miptex_offsets(bytes: &[u8]) -> ParsingResult<&[U32]> {
    let (count, rest) =
        U32::ref_from_prefix(bytes).map_err(|_| ParsingError::OutOfRange("bsp miptex header"))?;
    let count = usize::try_from(count.get())
        .map_err(|_| ParsingError::NumberOverflow("bsp miptex offsets count"))?;
    let (offsets, _) = <[U32]>::ref_from_prefix_with_elems(rest, count)
        .map_err(|_| ParsingError::Invalid("bsp miptex offsets"))?;

    Ok(offsets)
}

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
assert_eq_size!(BBox<Vec3s>, [Vec3s; 2]);
assert_eq_size!(BBox<Vec3f>, [Vec3f; 2]);
