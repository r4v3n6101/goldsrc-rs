use crate::byteorder::texture::miptex;
use crate::repr::texture::MipTexture;
use crate::{
    byteorder::{IoErr, IoErrKind, IoRes, LittleEndian, Read, ReadBytesExt, Seek, SeekFrom},
    repr::{
        bsp::{
            BBoxFloat, BBoxShort, ClipNode, Edge, Face, Leaf, Level, Model, Node, Plane,
            TextureInfo, Vec3f, Vec3s,
        },
        map::Entities,
    },
};
use std::{array, mem::size_of};

struct Lump {
    offset: u32,
    size: u32,
}

fn lump<R: Read>(reader: &mut R) -> IoRes<Lump> {
    Ok(Lump {
        offset: reader.read_u32::<LittleEndian>()?,
        size: reader.read_u32::<LittleEndian>()?,
    })
}

fn lump_content<R: Read + Seek, T>(
    reader: &mut R,
    lump: &Lump,
    f: fn(&mut R) -> IoRes<T>,
) -> IoRes<Vec<T>> {
    reader.seek(SeekFrom::Start(lump.offset as u64))?;

    let size = lump.size as usize / size_of::<T>();
    let mut data = Vec::with_capacity(size);
    for _ in 0..size {
        data.push(f(reader)?);
    }

    Ok(data)
}

fn vec3f<R: Read>(reader: &mut R) -> IoRes<Vec3f> {
    array::try_from_fn(|_| reader.read_f32::<LittleEndian>())
}

fn vec3s<R: Read>(reader: &mut R) -> IoRes<Vec3s> {
    array::try_from_fn(|_| reader.read_i16::<LittleEndian>())
}

fn edge<R: Read>(reader: &mut R) -> IoRes<Edge> {
    Ok((
        reader.read_u16::<LittleEndian>()?,
        reader.read_u16::<LittleEndian>()?,
    ))
}

fn bboxs<R: Read>(reader: &mut R) -> IoRes<BBoxShort> {
    Ok(BBoxShort {
        min: vec3s(reader)?,
        max: vec3s(reader)?,
    })
}

fn bboxf<R: Read>(reader: &mut R) -> IoRes<BBoxFloat> {
    Ok(BBoxFloat {
        min: vec3f(reader)?,
        max: vec3f(reader)?,
    })
}

fn plane<R: Read>(reader: &mut R) -> IoRes<Plane> {
    Ok(Plane {
        normal: vec3f(reader)?,
        distance: reader.read_f32::<LittleEndian>()?,
        ty: reader.read_u32::<LittleEndian>()?,
    })
}

fn clip_node<R: Read>(reader: &mut R) -> IoRes<ClipNode> {
    Ok(ClipNode {
        plane_id: reader.read_u32::<LittleEndian>()?,
        children: [
            reader.read_i16::<LittleEndian>()?,
            reader.read_i16::<LittleEndian>()?,
        ],
    })
}

fn face<R: Read>(reader: &mut R) -> IoRes<Face> {
    Ok(Face {
        plane_id: reader.read_u16::<LittleEndian>()?,
        plane_side: reader.read_u16::<LittleEndian>()?,
        first_surfedge_id: reader.read_u32::<LittleEndian>()?,
        surfedges_num: reader.read_u16::<LittleEndian>()?,
        texture_info_id: reader.read_u16::<LittleEndian>()?,
        lighting_styles: [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ],
        lightmap_offset: reader.read_u32::<LittleEndian>()?,
    })
}

fn leaf<R: Read>(reader: &mut R) -> IoRes<Leaf> {
    Ok(Leaf {
        contents: reader.read_i32::<LittleEndian>()?,
        vis_offset: reader.read_i32::<LittleEndian>()?,
        bounds: bboxs(reader)?,
        first_mark_surface_id: reader.read_u16::<LittleEndian>()?,
        mark_surfaces_num: reader.read_u16::<LittleEndian>()?,
        ambient_levels: [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ],
    })
}

fn model<R: Read>(reader: &mut R) -> IoRes<Model> {
    Ok(Model {
        bounds: bboxf(reader)?,
        origin: vec3f(reader)?,
        _nodes: [
            reader.read_i32::<LittleEndian>()?,
            reader.read_i32::<LittleEndian>()?,
            reader.read_i32::<LittleEndian>()?,
            reader.read_i32::<LittleEndian>()?,
        ],
        _vis_leafs: reader.read_i32::<LittleEndian>()?,
        first_face_id: reader.read_u32::<LittleEndian>()?,
        faces_num: reader.read_u32::<LittleEndian>()?,
    })
}

fn node<R: Read>(reader: &mut R) -> IoRes<Node> {
    Ok(Node {
        plane_id: reader.read_u32::<LittleEndian>()?,
        children: [
            reader.read_i16::<LittleEndian>()?,
            reader.read_i16::<LittleEndian>()?,
        ],
        bounds: bboxs(reader)?,
        first_face_id: reader.read_u16::<LittleEndian>()?,
        faces_num: reader.read_u16::<LittleEndian>()?,
    })
}

fn texinfo<R: Read>(reader: &mut R) -> IoRes<TextureInfo> {
    Ok(TextureInfo {
        s: vec3f(reader)?,
        s_shift: reader.read_f32::<LittleEndian>()?,
        t: vec3f(reader)?,
        t_shift: reader.read_f32::<LittleEndian>()?,
        texture_id: reader.read_u32::<LittleEndian>()?,
        flags: reader.read_u32::<LittleEndian>()?,
    })
}

fn textures<R: Read + Seek>(mut reader: R, lump: &Lump) -> IoRes<Vec<MipTexture>> {
    let begin = lump.offset as u64;
    reader.seek(SeekFrom::Start(begin))?;

    let num = reader.read_u32::<LittleEndian>()?;
    let mut offsets = Vec::with_capacity(num as usize);
    for _ in 0..num {
        offsets.push(reader.read_u32::<LittleEndian>()?);
    }

    let mut miptexs = Vec::with_capacity(offsets.len());
    for offset in offsets {
        reader.seek(SeekFrom::Start(begin + offset as u64))?;
        miptexs.push(miptex(&mut reader)?);
    }

    Ok(miptexs)
}

pub fn level<R: Read + Seek>(mut reader: R) -> IoRes<Level> {
    const BSP_VERSION: u32 = 30;
    const LUMPS_NUM: usize = 15;

    let version = reader.read_u32::<LittleEndian>()?;
    if version != BSP_VERSION {
        return Err(IoErr::new(IoErrKind::Unsupported, "invalid bsp version"));
    }
    let lumps: [Lump; LUMPS_NUM] = array::try_from_fn(|_| lump(&mut reader))?;

    Ok(Level {
        entities: Entities::new(), // TODO
        planes: lump_content(&mut reader, &lumps[1], plane)?,
        textures: textures(&mut reader, &lumps[2])?,
        vertices: lump_content(&mut reader, &lumps[3], vec3f)?,
        visdata: lump_content(&mut reader, &lumps[4], ReadBytesExt::read_u8)?,
        nodes: lump_content(&mut reader, &lumps[5], node)?,
        texture_infos: lump_content(&mut reader, &lumps[6], texinfo)?,
        faces: lump_content(&mut reader, &lumps[7], face)?,
        lighting: lump_content(&mut reader, &lumps[8], ReadBytesExt::read_u8)?,
        clip_nodes: lump_content(&mut reader, &lumps[9], clip_node)?,
        leaves: lump_content(&mut reader, &lumps[10], leaf)?,
        mark_surfaces: lump_content(
            &mut reader,
            &lumps[11],
            ReadBytesExt::read_u16::<LittleEndian>,
        )?,
        edges: lump_content(&mut reader, &lumps[12], edge)?,
        surfedges: lump_content(
            &mut reader,
            &lumps[13],
            ReadBytesExt::read_i32::<LittleEndian>,
        )?,
        models: lump_content(&mut reader, &lumps[14], model)?,
    })
}