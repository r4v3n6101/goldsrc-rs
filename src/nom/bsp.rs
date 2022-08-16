use nom::{
    combinator::{map, verify},
    multi::{length_count, many0},
    number::complete::{le_f32, le_i16, le_i32, le_u16, le_u32, le_u8},
    sequence::tuple,
};

use crate::{
    nom::{map::tmp_entities, texture::mip_texture, SliceExt},
    repr::{
        bsp::{
            BBoxFloat, BBoxShort, ClipNode, Face, Leaf, Level, Map, Model, Node, Plane,
            TextureInfo, Vec3f, Vec3s,
        },
        texture::MipTexture,
    },
};

const BSP_VERSION: u32 = 30;

fn vec3f(i: &[u8]) -> nom::IResult<&[u8], Vec3f> {
    let (i, (x, y, z)) = tuple((le_f32, le_f32, le_f32))(i)?;
    Ok((i, [x, y, z]))
}

fn vec3s(i: &[u8]) -> nom::IResult<&[u8], Vec3s> {
    let (i, (x, y, z)) = tuple((le_i16, le_i16, le_i16))(i)?;
    Ok((i, [x, y, z]))
}

fn bboxs(i: &[u8]) -> nom::IResult<&[u8], BBoxShort> {
    let (i, min) = vec3s(i)?;
    let (i, max) = vec3s(i)?;
    Ok((i, BBoxShort { min, max }))
}

fn bboxf(i: &[u8]) -> nom::IResult<&[u8], BBoxFloat> {
    let (i, min) = vec3f(i)?;
    let (i, max) = vec3f(i)?;
    Ok((i, BBoxFloat { min, max }))
}

fn lump<'a, F, T>(i: &'a [u8], file: &'a [u8], mut f: F) -> nom::IResult<&'a [u8], T>
where
    F: FnMut(&'a [u8]) -> nom::IResult<&[u8], T>,
{
    let (i, entities_offset) = le_u32(i)?;
    let (i, entities_len) = le_u32(i)?;
    let lump_data = file.off(entities_offset as usize, entities_len as usize)?;
    let (_, lump) = f(lump_data)?;
    Ok((i, lump))
}

fn plane(i: &[u8]) -> nom::IResult<&[u8], Plane> {
    let (i, (normal, distance, ty)) = tuple((vec3f, le_f32, le_u32))(i)?;
    Ok((
        i,
        Plane {
            normal,
            distance,
            ty,
        },
    ))
}

fn clip_node(i: &[u8]) -> nom::IResult<&[u8], ClipNode> {
    let (i, (plane, child1, child2)) = tuple((le_u32, le_i16, le_i16))(i)?;
    Ok((
        i,
        ClipNode {
            plane_id: plane as usize,
            children: [child1 as isize, child2 as isize],
        },
    ))
}

fn face(i: &[u8]) -> nom::IResult<&[u8], Face> {
    let (i, plane) = le_u16(i)?;
    let (i, plane_side) = le_u16(i)?;
    let (i, surf_id) = le_u32(i)?;
    let (i, surf_num) = le_u16(i)?;
    let (i, texinfo_id) = le_u16(i)?;
    let (i, light1) = le_u8(i)?;
    let (i, light2) = le_u8(i)?;
    let (i, light3) = le_u8(i)?;
    let (i, light4) = le_u8(i)?;
    let (i, lightmap) = le_u32(i)?;
    Ok((
        i,
        Face {
            plane_id: plane as usize,
            flipped: plane_side != 0,
            first_surfedge_id: surf_id as usize,
            surfedges_num: surf_num as usize,
            texture_info_id: texinfo_id as usize,
            lighting_styles: [light1, light2, light3, light4],
            lightmap_offset: lightmap as usize,
        },
    ))
}

fn leaf(i: &[u8]) -> nom::IResult<&[u8], Leaf> {
    let (i, contents) = le_i32(i)?;
    let (i, _) = le_i32(i)?;
    let (i, bounds) = bboxs(i)?;
    let (i, markface_id) = le_u16(i)?;
    let (i, markface_num) = le_u16(i)?;
    let (i, sound1) = le_u8(i)?;
    let (i, sound2) = le_u8(i)?;
    let (i, sound3) = le_u8(i)?;
    let (i, sound4) = le_u8(i)?;
    Ok((
        i,
        Leaf {
            contents,
            bounds,
            mark_surfaces_id: markface_id as usize,
            mark_surfaces_num: markface_num as usize,
            ambient_levels: [sound1, sound2, sound3, sound4],
        },
    ))
}

fn model(i: &[u8]) -> nom::IResult<&[u8], Model> {
    let (i, bounds) = bboxf(i)?;
    let (i, origin) = vec3f(i)?;

    let (i, _) = le_u32(i)?; // TODO : ???
    let (i, _) = le_u32(i)?;
    let (i, _) = le_u32(i)?;
    let (i, _) = le_u32(i)?;

    let (i, _) = le_u32(i)?; // TODO : ???

    let (i, face_id) = le_u32(i)?;
    let (i, face_num) = le_u32(i)?;
    Ok((
        i,
        Model {
            bounds,
            origin,
            faces_id: face_id as usize,
            faces_num: face_num as usize,
        },
    ))
}

fn node(i: &[u8]) -> nom::IResult<&[u8], Node> {
    let (i, plane) = le_u32(i)?;
    let (i, child1) = le_i16(i)?;
    let (i, child2) = le_i16(i)?;
    let (i, bounds) = bboxs(i)?;
    let (i, face_id) = le_u16(i)?;
    let (i, face_num) = le_u16(i)?;
    Ok((
        i,
        Node {
            plane_id: plane as usize,
            children: [child1 as isize, child2 as isize],
            first_face_id: face_id as usize,
            faces_num: face_num as usize,
            bounds,
        },
    ))
}

fn texinfo(i: &[u8]) -> nom::IResult<&[u8], TextureInfo> {
    let (i, s) = vec3f(i)?;
    let (i, s_shift) = le_f32(i)?;
    let (i, t) = vec3f(i)?;
    let (i, t_shift) = le_f32(i)?;
    let (i, miptex) = le_u32(i)?;
    let (i, flags) = le_u32(i)?;
    Ok((
        i,
        TextureInfo {
            s,
            s_shift,
            t,
            t_shift,
            texture_id: miptex as usize,
            flags,
        },
    ))
}

fn mip_textures(i: &[u8]) -> nom::IResult<&[u8], Vec<MipTexture>> {
    let (_, offsets) = length_count(le_u32, le_u32)(i)?;
    let mut miptexs = Vec::with_capacity(offsets.len());
    for off in offsets {
        let miptex_data = i.off_all(off as usize)?;
        let (_, miptex) = mip_texture(miptex_data)?;
        miptexs.push(miptex);
    }
    Ok((&[], miptexs))
}

pub fn level(file: &[u8]) -> nom::IResult<&[u8], Level> {
    let (i, _) = verify(le_u32, |x: &u32| *x == BSP_VERSION)(file)?;

    let (i, _) = lump(i, file, tmp_entities)?;
    let (i, planes) = lump(i, file, many0(plane))?;
    let (i, textures) = lump(i, file, mip_textures)?;
    let (i, vertices) = lump(i, file, many0(vec3f))?;
    let (i, _) = lump(i, file, |x| Ok((&[], x)))?; // TODO : vis
    let (i, nodes) = lump(i, file, many0(node))?;
    let (i, texture_infos) = lump(i, file, many0(texinfo))?;
    let (i, faces) = lump(i, file, many0(face))?;
    let (i, lighting) = lump(i, file, |x| Ok((&[], x)))?;
    let (i, clip_nodes) = lump(i, file, many0(clip_node))?;
    let (i, leaves) = lump(i, file, many0(leaf))?;
    let (i, mark_surfaces) = lump(i, file, many0(map(le_u16, |x| x as usize)))?;
    let (i, edges) = lump(
        i,
        file,
        many0(map(tuple((le_u16, le_u16)), |(a, b)| {
            (a as usize, b as usize)
        })),
    )?;
    let (i, surfedges) = lump(i, file, many0(map(le_i32, |x| x as isize)))?;
    let (_, models) = lump(i, file, many0(model))?;
    Ok((
        &[],
        Level {
            textures,
            lighting,
            map: Map {
                planes,
                vertices,
                nodes,
                texture_infos,
                faces,
                clip_nodes,
                leaves,
                mark_surfaces,
                edges,
                surfedges,
                models,
            },
        },
    ))
}

#[test]
fn parse_bsp() {
    let data = std::fs::read("test.bsp").expect("error reading file");
    let (_, level) = level(&data).expect("error parsing file");

    println!("Vertices: {}", level.map.vertices.len());
    println!("Textures: {}", level.textures.len());
    for miptex in &level.textures {
        println!("== {}", miptex.name);
    }
    println!("Faces: {}", level.map.faces.len());
    println!("Models: {}", level.map.models.len());
}
