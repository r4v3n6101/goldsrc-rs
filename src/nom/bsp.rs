use crate::{
    nom::{map::entities_bin, texture::mip_texture, SliceExt},
    repr::{
        bsp::{
            BBoxFloat, BBoxShort, ClipNode, Face, Leaf, Level, Model, Node, Plane, TextureInfo,
            Vec3f, Vec3s,
        },
        texture::MipTexture,
    },
};
use nom::{
    combinator::verify,
    multi::{length_count, many0},
    number::complete::{le_f32, le_i16, le_i32, le_u16, le_u32, le_u8},
    sequence::tuple,
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
            plane_id: plane,
            children: [child1, child2],
        },
    ))
}

fn face(i: &[u8]) -> nom::IResult<&[u8], Face> {
    let (i, plane_id) = le_u16(i)?;
    let (i, plane_side) = le_u16(i)?;
    let (i, first_surfedge_id) = le_u32(i)?;
    let (i, surfedges_num) = le_u16(i)?;
    let (i, texture_info_id) = le_u16(i)?;
    let (i, light1) = le_u8(i)?;
    let (i, light2) = le_u8(i)?;
    let (i, light3) = le_u8(i)?;
    let (i, light4) = le_u8(i)?;
    let (i, lightmap_offset) = le_u32(i)?;
    Ok((
        i,
        Face {
            plane_id,
            plane_side,
            first_surfedge_id,
            surfedges_num,
            texture_info_id,
            lighting_styles: [light1, light2, light3, light4],
            lightmap_offset,
        },
    ))
}

fn leaf(i: &[u8]) -> nom::IResult<&[u8], Leaf> {
    let (i, contents) = le_i32(i)?;
    let (i, vis_offset) = le_i32(i)?;
    let (i, bounds) = bboxs(i)?;
    let (i, mark_surfaces_id) = le_u16(i)?;
    let (i, mark_surfaces_num) = le_u16(i)?;
    let (i, sound1) = le_u8(i)?;
    let (i, sound2) = le_u8(i)?;
    let (i, sound3) = le_u8(i)?;
    let (i, sound4) = le_u8(i)?;
    Ok((
        i,
        Leaf {
            contents,
            vis_offset,
            bounds,
            first_mark_surface_id: mark_surfaces_id,
            mark_surfaces_num,
            ambient_levels: [sound1, sound2, sound3, sound4],
        },
    ))
}

fn model(i: &[u8]) -> nom::IResult<&[u8], Model> {
    let (i, bounds) = bboxf(i)?;
    let (i, origin) = vec3f(i)?;

    let (i, node0) = le_i32(i)?; // TODO : ???
    let (i, node1) = le_i32(i)?;
    let (i, node2) = le_i32(i)?;
    let (i, node3) = le_i32(i)?;

    let (i, vis_leafs) = le_i32(i)?; // TODO : ???

    let (i, faces_id) = le_u32(i)?;
    let (i, faces_num) = le_u32(i)?;
    Ok((
        i,
        Model {
            bounds,
            origin,
            _nodes: [node0, node1, node2, node3],
            _vis_leafs: vis_leafs,
            first_face_id: faces_id,
            faces_num,
        },
    ))
}

fn node(i: &[u8]) -> nom::IResult<&[u8], Node> {
    let (i, plane_id) = le_u32(i)?;
    let (i, child1) = le_i16(i)?;
    let (i, child2) = le_i16(i)?;
    let (i, bounds) = bboxs(i)?;
    let (i, first_face_id) = le_u16(i)?;
    let (i, faces_num) = le_u16(i)?;
    Ok((
        i,
        Node {
            plane_id,
            children: [child1, child2],
            first_face_id,
            faces_num,
            bounds,
        },
    ))
}

fn texinfo(i: &[u8]) -> nom::IResult<&[u8], TextureInfo> {
    let (i, s) = vec3f(i)?;
    let (i, s_shift) = le_f32(i)?;
    let (i, t) = vec3f(i)?;
    let (i, t_shift) = le_f32(i)?;
    let (i, texture_id) = le_u32(i)?;
    let (i, flags) = le_u32(i)?;
    Ok((
        i,
        TextureInfo {
            s,
            s_shift,
            t,
            t_shift,
            texture_id,
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

    let (i, entities) = lump(i, file, entities_bin)?;
    let (i, planes) = lump(i, file, many0(plane))?;
    let (i, textures) = lump(i, file, mip_textures)?;
    let (i, vertices) = lump(i, file, many0(vec3f))?;
    let (i, visdata) = lump(i, file, |x| Ok((&[], x.to_vec())))?; // TODO : vis
    let (i, nodes) = lump(i, file, many0(node))?;
    let (i, texture_infos) = lump(i, file, many0(texinfo))?;
    let (i, faces) = lump(i, file, many0(face))?;
    let (i, lighting) = lump(i, file, |x| Ok((&[], x.to_vec())))?;
    let (i, clip_nodes) = lump(i, file, many0(clip_node))?;
    let (i, leaves) = lump(i, file, many0(leaf))?;
    let (i, mark_surfaces) = lump(i, file, many0(le_u16))?;
    let (i, edges) = lump(i, file, many0(tuple((le_u16, le_u16))))?;
    let (i, surfedges) = lump(i, file, many0(le_i32))?;
    let (_, models) = lump(i, file, many0(model))?;
    Ok((
        &[],
        Level {
            textures,
            lighting,
            visdata,
            entities,
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
    ))
}
