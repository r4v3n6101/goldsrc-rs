use goldsrc_rs::{texture::ColourData, wad::Content};

fn save_img<const N: usize>(name: &str, width: u32, height: u32, data: &ColourData<N>) {
    let data = data.indices[0]
        .iter()
        .flat_map(|&i| {
            let rgb_i = i as usize;
            let [r, g, b] = data.palette[rgb_i];
            if r == 255 || g == 255 || b == 255 {
                [0u8; 4]
            } else {
                [r, g, b, 255]
            }
        })
        .collect();

    let imgbuf = image::RgbaImage::from_vec(width, height, data).unwrap();
    imgbuf
        .save(format!("./assets/output/{}.png", name))
        .unwrap();
}

#[test]
fn extract_wad() {
    for path in glob::glob("./assets/wad/*.wad")
        .expect("error globing wad")
        .flatten()
    {
        let data = std::fs::read(&path).expect("error reading file");
        let archive = goldsrc_rs::wad(std::io::Cursor::new(&data)).unwrap();

        for (name, content) in &archive {
            match content {
                Content::Font(font) => save_img(name, font.width, font.height, &font.data),
                Content::Picture(pic) => save_img(name, pic.width, pic.height, &pic.data),
                Content::MipTexture(miptex) => save_img(
                    name,
                    miptex.width,
                    miptex.height,
                    miptex.data.as_ref().unwrap(),
                ),
                _ => {
                    eprintln!("Unknown type: {}", name);
                }
            }
            println!("Saved: {}", name);
        }
    }
}
