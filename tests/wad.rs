use goldsrc_rs::{texture::ColourData, wad::ContentType};

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
        let entries = goldsrc_rs::wad_entries(std::io::Cursor::new(data), true).unwrap();

        for (name, entry) in entries {
            let reader = entry.reader();
            match entry.ty {
                ContentType::Font => {
                    let font = goldsrc_rs::font(reader).unwrap();
                    save_img(&name, font.width, font.height, &font.data);
                }
                ContentType::Picture => {
                    let pic = goldsrc_rs::pic(reader).unwrap();
                    save_img(&name, pic.width, pic.height, &pic.data);
                }
                ContentType::MipTexture => {
                    let miptex = goldsrc_rs::miptex(reader).unwrap();
                    save_img(
                        &name,
                        miptex.width,
                        miptex.height,
                        miptex.data.as_ref().unwrap(),
                    );
                }
                _ => {
                    eprintln!("Unknown type: {}", name);
                }
            }
            println!("Saved: {}", name);
        }
    }
}
