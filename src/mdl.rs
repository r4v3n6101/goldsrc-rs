use static_assertions::assert_eq_size;
use zerocopy::{
    FromBytes,
    little_endian::{F32, I32, U16, U32},
};
use zerocopy_derive::*;

use crate::{
    common::{BBox, Table, Vec3f, table_ref},
    error::{ParsingError, ParsingResult},
    texture::{ColorData, PaletteIndex, Rgb},
    util,
};

/// MDL magic (GoldSrc).
pub const MDL_MAGIC: [u8; 4] = *b"IDST";
/// MDL version (Half-Life 1).
pub const MDL_VERSION: u32 = 10;
/// Number of mip levels in a model texture.
pub const MIP_LEVELS: usize = 4;

/// Complete model data loaded from a MDL file.
pub struct SkeletalModel<'a> {
    /// MDL header.
    pub header: &'a MdlHeader,
    /// Bones.
    pub bones: &'a [Bone],
    /// Bone controllers.
    pub bone_controllers: &'a [BoneController],
    /// Hit boxes.
    pub hitboxes: &'a [HitBox],
    /// Sequence descriptions.
    pub sequences: &'a [SequenceDesc],
    /// Sequence groups.
    pub sequence_groups: &'a [SequenceGroup],
    /// Textures.
    pub textures: &'a [Texture],
    /// Skin references (family table).
    pub skins: SkinRefs<'a>,
    /// Body parts.
    pub bodyparts: &'a [BodyPart],
    /// Attachments.
    pub attachments: &'a [Attachment],
    /// Transition table.
    pub transitions: &'a [u8],
}

/// MDL header (studiohdr_t).
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct MdlHeader {
    /// File magic ("IDST").
    pub magic: [u8; 4],
    /// File version.
    pub version: U32,
    /// Model name (C string, not guaranteed UTF-8).
    pub name: [u8; 64],
    /// File length in bytes.
    pub length: U32,
    /// Eye position.
    pub eye_position: Vec3f,
    /// Bounding box.
    pub bbox: BBox<Vec3f>,
    /// Clipping box.
    pub clip: BBox<Vec3f>,
    /// Model flags.
    pub flags: I32,
    /// Bones table.
    pub bones: Table,
    /// Bone controllers table.
    pub bone_controllers: Table,
    /// Hit boxes table.
    pub hitboxes: Table,
    /// Sequences table.
    pub sequences: Table,
    /// Sequence groups table.
    pub sequence_groups: Table,
    /// Textures table.
    pub textures: Table,
    /// Texture data block offset.
    pub texture_data_offset: U32,
    /// Number of skin references.
    pub skin_refs_num: U32,
    /// Number of skin families.
    pub skin_families_num: U32,
    /// Skin reference table offset.
    pub skin_offset: U32,
    /// Body parts table.
    pub bodyparts: Table,
    /// Attachments table.
    pub attachments: Table,
    /// Sounds table (unused in GoldSrc).
    pub sounds: Table,
    /// Sound groups table (unused in GoldSrc).
    pub sound_groups: Table,
    /// Transitions table (square table).
    /// Indexing like transitions[from][to]
    pub transitions: Table,
}

/// Bone definition.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Bone {
    /// Bone name (C string, not guaranteed UTF-8).
    pub name: [u8; 32],
    /// Parent bone index.
    pub parent: I32,
    /// Bone flags.
    pub flags: I32,
    /// Bone controller indices.
    pub bone_controller: [I32; 6],
    /// Default values.
    pub value: [F32; 6],
    /// Scale values.
    pub scale: [F32; 6],
}

/// Bone controller definition.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct BoneController {
    /// Bone index.
    pub bone: I32,
    /// Controller type.
    pub ty: I32,
    /// Start value.
    pub start: F32,
    /// End value.
    pub end: F32,
    /// Rest value.
    pub rest: I32,
    /// Controller index.
    pub index: I32,
}

/// Hit box definition.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct HitBox {
    /// Bone index.
    pub bone: I32,
    /// Hit group.
    pub group: I32,
    /// Bounding box.
    pub bbox: BBox<Vec3f>,
}

/// Sequence description.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct SequenceDesc {
    /// Sequence label.
    pub label: [u8; 32],
    /// Frames per second.
    pub fps: F32,
    /// Sequence flags.
    pub flags: U32,
    /// Activity id.
    pub activity: I32,
    /// Activity weight.
    pub activity_weight: I32,
    /// Events table.
    pub events: Table,
    /// Number of frames.
    pub frames_num: I32,
    /// Pivots table.
    pub pivots: Table,
    /// Motion type.
    pub motion_type: I32,
    /// Motion bone.
    pub motion_bone: I32,
    /// Linear movement.
    pub linear_movement: Vec3f,
    /// Auto-move position index.
    pub automove_pos_index: I32,
    /// Auto-move angle index.
    pub automove_angle_index: I32,
    /// Bounding box.
    pub bbox: BBox<Vec3f>,
    /// Blend count.
    pub blends_num: I32,
    /// Animation index.
    pub anim_index: I32,
    /// Blend types.
    pub blend_type: [I32; 2],
    /// Blend start values.
    pub blend_start: [F32; 2],
    /// Blend end values.
    pub blend_end: [F32; 2],
    /// Blend parent.
    pub blend_parent: I32,
    /// Sequence group index.
    pub sequence_group: I32,
    /// Entry node.
    pub entry_node: I32,
    /// Exit node.
    pub exit_node: I32,
    /// Node flags.
    pub node_flags: I32,
    /// Next sequence.
    pub next_sequence: I32,
}

/// Sequence event.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct SequenceEvent {
    /// Frame number.
    pub frame: I32,
    /// Event id.
    pub event: I32,
    /// Event type.
    pub ty: I32,
    /// Event options (C string, not guaranteed UTF-8).
    pub options: [u8; 64],
}

/// Sequence pivot.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Pivot {
    /// Pivot origin.
    pub org: Vec3f,
    /// Start frame.
    pub start: I32,
    /// End frame.
    pub end: I32,
}

/// Sequence group.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct SequenceGroup {
    /// Group label.
    pub label: [u8; 32],
    /// Group name (file name).
    pub name: [u8; 64],
    /// Unused.
    pub unused1: I32,
    /// Unused.
    pub unused2: I32,
}

/// Texture entry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Texture {
    /// Texture name.
    pub name: [u8; 64],
    /// Texture flags.
    pub flags: U32,
    /// Width in pixels.
    pub width: U32,
    /// Height in pixels.
    pub height: U32,
    /// Offset to texture data.
    pub offset: U32,
}

/// Attachment point.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Attachment {
    /// Attachment name.
    pub name: [u8; 32],
    /// Attachment type.
    pub ty: I32,
    /// Bone index.
    pub bone: I32,
    /// Attachment origin.
    pub org: Vec3f,
    /// Attachment vectors.
    pub vectors: [Vec3f; 3],
}

/// Body part entry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct BodyPart {
    /// Body part name.
    pub name: [u8; 64],
    /// Model count.
    pub models_num: U32,
    /// Base index.
    pub base: I32,
    /// Model table offset.
    pub models_offset: U32,
}

/// Model entry inside a body part.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Model {
    /// Model name.
    pub name: [u8; 64],
    /// Model type.
    pub ty: I32,
    /// Bounding radius.
    pub bounding_radius: F32,
    /// Meshes table.
    pub meshes: Table,
    /// Vertex count.
    pub verts_num: U32,
    /// Vertex bone info offset.
    pub vert_info_offset: U32,
    /// Vertex data offset.
    pub verts_offset: U32,
    /// Normal count.
    pub norms_num: U32,
    /// Normal bone info offset.
    pub norm_info_offset: U32,
    /// Normal data offset.
    pub norms_offset: U32,
    /// Groups table.
    pub groups: Table,
}

/// Mesh entry.
#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct Mesh {
    // Triangles.
    pub tris: Table,
    /// Skin reference.
    pub skin_ref: I32,
    /// Normals table.
    pub norms: Table,
}

/// Skin reference table.
pub struct SkinRefs<'a> {
    /// Raw skin reference table (U16 values).
    pub refs: &'a [U16],
    /// Number of skin references per family.
    pub skin_refs_num: u32,
    /// Number of families.
    pub skin_families_num: u32,
}

pub fn mdl(bytes: &[u8]) -> ParsingResult<SkeletalModel<'_>> {
    let (header, _) =
        MdlHeader::ref_from_prefix(bytes).map_err(|_| ParsingError::OutOfRange("mdl header"))?;

    if header.magic != MDL_MAGIC {
        return Err(ParsingError::WrongFourCC {
            got: header.magic,
            expected: MDL_MAGIC,
        });
    }

    let version = header.version.get();
    if version != MDL_VERSION {
        return Err(ParsingError::WrongVersion {
            got: version,
            expected: MDL_VERSION,
        });
    }

    Ok(SkeletalModel {
        header,
        bones: table_ref(bytes, &header.bones, "mdl bones")?,
        bone_controllers: table_ref(bytes, &header.bone_controllers, "mdl bone controllers")?,
        hitboxes: table_ref(bytes, &header.hitboxes, "mdl hitboxes")?,
        sequences: table_ref(bytes, &header.sequences, "mdl sequences")?,
        sequence_groups: table_ref(bytes, &header.sequence_groups, "mdl sequence groups")?,
        textures: table_ref(bytes, &header.textures, "mdl textures")?,
        skins: skin_refs(bytes, header)?,
        bodyparts: table_ref(bytes, &header.bodyparts, "mdl bodyparts")?,
        attachments: table_ref(bytes, &header.attachments, "mdl attachments")?,
        transitions: transition_table(bytes, header)?,
    })
}

pub fn sequence_events<'a>(
    bytes: &'a [u8],
    sequence: &SequenceDesc,
) -> ParsingResult<&'a [SequenceEvent]> {
    table_ref(bytes, &sequence.events, "mdl sequence events")
}

pub fn sequence_pivots<'a>(bytes: &'a [u8], sequence: &SequenceDesc) -> ParsingResult<&'a [Pivot]> {
    table_ref(bytes, &sequence.pivots, "mdl sequence pivots")
}

pub fn bodypart_models<'a>(bytes: &'a [u8], bodypart: &BodyPart) -> ParsingResult<&'a [Model]> {
    table_ref(
        bytes,
        &Table {
            count: bodypart.models_num,
            offset: bodypart.models_offset,
        },
        "mdl bodypart models",
    )
}

pub fn model_meshes<'a>(bytes: &'a [u8], model: &Model) -> ParsingResult<&'a [Mesh]> {
    table_ref(bytes, &model.meshes, "mdl meshes")
}

pub fn model_vertices<'a>(bytes: &'a [u8], model: &Model) -> ParsingResult<&'a [Vec3f]> {
    table_ref(
        bytes,
        &Table {
            count: model.verts_num,
            offset: model.verts_offset,
        },
        "mdl vertices",
    )
}

pub fn model_normals<'a>(bytes: &'a [u8], model: &Model) -> ParsingResult<&'a [Vec3f]> {
    table_ref(
        bytes,
        &Table {
            count: model.norms_num,
            offset: model.norms_offset,
        },
        "mdl normals",
    )
}

pub fn model_vertex_bones<'a>(bytes: &'a [u8], model: &Model) -> ParsingResult<&'a [u8]> {
    table_ref(
        bytes,
        &Table {
            count: model.verts_num,
            offset: model.vert_info_offset,
        },
        "mdl vert bone info",
    )
}

pub fn model_normal_bones<'a>(bytes: &'a [u8], model: &Model) -> ParsingResult<&'a [u8]> {
    table_ref(
        bytes,
        &Table {
            count: model.norms_num,
            offset: model.norm_info_offset,
        },
        "mdl norm bone info",
    )
}

pub fn texture_data<'a>(bytes: &'a [u8], texture: &Texture) -> ParsingResult<ColorData<'a, 1>> {
    const PALETTE_SIZE: usize = 256;

    let offset = usize::try_from(texture.offset.get())
        .map_err(|_| ParsingError::NumberOverflow("mdl texture offset"))?;

    let bytes = bytes
        .get(offset..)
        .ok_or(ParsingError::OutOfRange("mdl texture"))?;

    let size = util::pixel_size(texture.width.get(), texture.height.get(), "mdl texture")?;
    let (indices, bytes) = <[PaletteIndex]>::ref_from_prefix_with_elems(bytes, size)
        .map_err(|_| ParsingError::Invalid("mdl texture indices"))?;

    // This goes without palette_size
    let (palette, _) = <[Rgb]>::ref_from_prefix_with_elems(bytes, PALETTE_SIZE)
        .map_err(|_| ParsingError::Invalid("palette"))?;

    Ok(ColorData {
        indices: [indices],
        palette,
    })
}

fn skin_refs<'a>(bytes: &'a [u8], header: &MdlHeader) -> ParsingResult<SkinRefs<'a>> {
    let skin_refs_num = header.skin_refs_num.get();
    let skin_families_num = header.skin_families_num.get();
    let total = skin_refs_num
        .checked_mul(skin_families_num)
        .ok_or(ParsingError::NumberOverflow("mdl skin refs"))?;

    let refs = table_ref(
        bytes,
        &Table {
            count: total.into(),
            offset: header.skin_offset,
        },
        "mdl skins",
    )?;

    Ok(SkinRefs {
        refs,
        skin_refs_num,
        skin_families_num,
    })
}

fn transition_table<'a>(bytes: &'a [u8], header: &MdlHeader) -> ParsingResult<&'a [u8]> {
    let size = header.transitions.count.get();
    let total = size
        .checked_mul(size)
        .ok_or(ParsingError::NumberOverflow("mdl transitions"))?;

    table_ref(
        bytes,
        &Table {
            count: total.into(),
            offset: header.transitions.offset,
        },
        "mdl transitions",
    )
}

assert_eq_size!(MdlHeader, [u8; 244]);
assert_eq_size!(Table, [u8; 8]);
assert_eq_size!(Bone, [u8; 112]);
assert_eq_size!(BoneController, [u8; 24]);
assert_eq_size!(HitBox, [u8; 32]);
assert_eq_size!(SequenceDesc, [u8; 176]);
assert_eq_size!(SequenceEvent, [u8; 76]);
assert_eq_size!(Pivot, [u8; 20]);
assert_eq_size!(SequenceGroup, [u8; 104]);
assert_eq_size!(Texture, [u8; 80]);
assert_eq_size!(Attachment, [u8; 88]);
assert_eq_size!(BodyPart, [u8; 76]);
assert_eq_size!(Model, [u8; 112]);
assert_eq_size!(Mesh, [u8; 20]);
