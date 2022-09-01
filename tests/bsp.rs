#[test]
fn parse_bsp() {
    for path in glob::glob("./assets/maps/*.bsp")
        .expect("error globing maps")
        .flatten()
    {
        let data = std::fs::read(&path).expect("error reading file");
        #[cfg(feature = "nom")]
        let level = goldsrc_rs::bsp_from_bytes(&data).unwrap();
        #[cfg(feature = "byteorder")]
        let level = goldsrc_rs::bsp(std::io::Cursor::new(data)).unwrap();

        println!("Vertices: {}", level.vertices.len());
        println!("Textures: {}", level.textures.len());
        for miptex in &level.textures {
            println!("== {}", miptex.name);
        }
        println!("Faces: {}", level.faces.len());
        println!("Models: {}", level.models.len());
        println!("Entities: {:#?}", level.entities);
    }
}
