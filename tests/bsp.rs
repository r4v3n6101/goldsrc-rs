use goldsrc_rs::bsp::level;

#[test]
fn parse_bsp() {
    for path in glob::glob("./assets/bsp/*.bsp")
        .expect("error globing bsp")
        .flatten()
    {
        println!("File: {:?}", path);
        let data = std::fs::read(&path).expect("error reading file");
        let level = level(&data).unwrap();

        println!("Planes: {}", level.planes.len());
        println!("Vertices: {}", level.vertices.len());
        println!("Visdata: {}", level.visdata.len());
        println!("Nodes: {}", level.nodes.len());
        println!("TextureInfos: {}", level.texture_infos.len());
        println!("Faces: {}", level.faces.len());
        println!("Lighting: {}", level.lighting.len());
        println!("ClipNodes: {}", level.clip_nodes.len());
        println!("Leaves: {}", level.leaves.len());
        println!("MarkSurfaces: {}", level.mark_surfaces.len());
        println!("Edges: {}", level.edges.len());
        println!("SurfEdges: {}", level.surfedges.len());
        println!("Models: {}", level.models.len());

        println!("MipTextures: {}", level.textures.len());
        for (idx, tex) in level.textures.iter().enumerate() {
            let name_end = tex
                .header
                .name
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(tex.header.name.len());
            let name = String::from_utf8_lossy(&tex.header.name[..name_end]);
            let (has_palette, palette_len) = match &tex.data {
                Some(data) => (!data.palette.is_empty(), data.palette.len()),
                None => (false, 0),
            };
            println!(
                "  {idx}: {name} {}x{} palette={} len={}",
                tex.header.width.get(),
                tex.header.height.get(),
                has_palette,
                palette_len
            );
        }
    }
}
