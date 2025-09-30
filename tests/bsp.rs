#[test]
fn parse_bsp() {
    for path in glob::glob("./assets/maps/*.bsp")
        .expect("error globing maps")
        .flatten()
    {
        println!("File: {:?}", path);
        let data = std::fs::read(&path).expect("error reading file");
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
