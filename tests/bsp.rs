#[test]
fn parse_bsp() {
    for path in glob::glob("./assets/maps/*.bsp")
        .expect("error globing maps")
        .flatten()
    {
        let data = std::fs::read(&path).expect("error reading file");
        #[cfg(feature = "nom")]
        let level = goldsrc_rs::nom::bsp::level(&data)
            .expect("error parsing file")
            .1;
        #[cfg(feature = "byteorder")]
        let level = goldsrc_rs::byteorder::bsp::level(std::io::Cursor::new(data))
            .expect("error parsing level");

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
