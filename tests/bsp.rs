#[test]
fn parse_bsp() {
    for path in glob::glob("./assets/maps/*.bsp")
        .expect("error globing maps")
        .flatten()
    {
        #[cfg(feature = "nom")]
        let level = {
            let data = std::fs::read(&path).expect("error reading file");
            goldsrc_rs::nom::bsp::level(&data)
                .expect("error parsing file")
                .1
        };
        #[cfg(feature = "byteorder")]
        let level = {
            let mut reader =
                std::io::BufReader::new(std::fs::File::open(&path).expect("error opening file"));
            goldsrc_rs::byteorder::bsp::level(reader).expect("error parsing level")
        };

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
