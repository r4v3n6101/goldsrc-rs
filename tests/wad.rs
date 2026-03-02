use std::path::Path;

use goldsrc_rs::texture::ColorData;

fn save_img<const N: usize>(
    out_dir: &Path,
    name: &str,
    width: u32,
    height: u32,
    data: &ColorData<'_, N>,
) {
    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
    for &i in data.indices[0].iter() {
        let [r, g, b] = data
            .palette
            .get(i as usize)
            .copied()
            .unwrap_or([0u8, 0u8, 0u8]);
        if r == 255 || g == 255 || b == 255 {
            rgba.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]);
        } else {
            rgba.extend_from_slice(&[r, g, b, 255u8]);
        }
    }

    let imgbuf = image::RgbaImage::from_vec(width, height, rgba).unwrap();
    imgbuf.save(out_dir.join(format!("{name}.png"))).unwrap();
}

#[test]
fn extract_wad() {
    let out_dir = Path::new("./assets/output");
    std::fs::create_dir_all(out_dir).expect("error creating output dir");

    for path in glob::glob("./assets/wad/*.wad")
        .expect("error globing wad")
        .flatten()
    {
        let data = std::fs::read(&path).expect("error reading file");
        let wad = goldsrc_rs::wad::wad(&data).unwrap();

        println!("File: {:?}", path);
        println!("Entries: {}", wad.entries.len());
        for (idx, entry) in wad.entries.iter().enumerate() {
            let raw_name = String::from_utf8_lossy(entry.name());
            let name = if raw_name.is_empty() {
                format!("unnamed_{idx}")
            } else {
                raw_name.replace(['/', '\\'], "_")
            };

            println!(
                "  {} ty=0x{:02x} comp={} disk={} size={}",
                name,
                entry.ty,
                entry.compression,
                entry.disk_size.get(),
                entry.size.get()
            );

            if entry.compression != 0 {
                eprintln!("  skipping compressed lump: {}", name);
                continue;
            }

            let bytes = goldsrc_rs::wad::entry_bytes(&data, entry).unwrap();

            match entry.ty {
                0x46 => {
                    let font = goldsrc_rs::texture::font(bytes).unwrap();
                    save_img(
                        out_dir,
                        &format!("{name}_font"),
                        font.header.width.get(),
                        font.header.height.get(),
                        &font.data,
                    );
                }
                0x43 => {
                    let miptex = goldsrc_rs::texture::mip_texture(bytes).unwrap();
                    let Some(data) = miptex.data else {
                        eprintln!("  missing miptex data: {}", name);
                        continue;
                    };
                    save_img(
                        out_dir,
                        &format!("{name}_mip0"),
                        miptex.header.width.get(),
                        miptex.header.height.get(),
                        &data,
                    );
                }
                0x42 | 0x40 => {
                    if let Ok(pic) = goldsrc_rs::texture::picture(bytes) {
                        save_img(
                            out_dir,
                            &format!("{name}_pic"),
                            pic.header.width.get(),
                            pic.header.height.get(),
                            &pic.data,
                        );
                    } else if let Ok(miptex) = goldsrc_rs::texture::mip_texture(bytes) {
                        if let Some(data) = miptex.data {
                            save_img(
                                out_dir,
                                &format!("{name}_mip0"),
                                miptex.header.width.get(),
                                miptex.header.height.get(),
                                &data,
                            );
                        } else {
                            eprintln!("  missing miptex data: {}", name);
                        }
                    } else {
                        eprintln!("  unsupported lump format: {}", name);
                    }
                }
                _ => {
                    eprintln!("  unknown lump type: {}", name);
                }
            }
        }
    }
}
