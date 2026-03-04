use goldsrc_rs::{
    common::cstring_bytes,
    texture::{font, mip_texture, picture},
    wad::{wad, wad_entry},
};

use std::path::Path;

mod common;

#[test]
fn extract_wad() {
    let out_dir = Path::new("./output");
    std::fs::create_dir_all(out_dir).expect("error creating output dir");

    for path in glob::glob("./valve/*.wad")
        .expect("error globing wad")
        .flatten()
    {
        let data = std::fs::read(&path).expect("error reading file");
        let wad = wad(&data).unwrap();

        println!("File: {:?}", path);
        println!("Entries: {}", wad.entries.len());
        for (idx, entry) in wad.entries.iter().enumerate() {
            let raw_name = String::from_utf8_lossy(cstring_bytes(&entry.name));
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

            let bytes = wad_entry(&data, entry).unwrap();

            match entry.ty {
                0x46 => {
                    let font = font(bytes).unwrap();
                    common::save_img(
                        out_dir,
                        &format!("{name}_font"),
                        font.header.width.get(),
                        font.header.height.get(),
                        &font.data,
                    );
                }
                0x43 => {
                    let miptex = mip_texture(bytes).unwrap();
                    let Some(data) = miptex.data else {
                        eprintln!("  missing miptex data: {}", name);
                        continue;
                    };
                    common::save_img(
                        out_dir,
                        &format!("{name}_mip0"),
                        miptex.header.width.get(),
                        miptex.header.height.get(),
                        &data,
                    );
                }
                0x42 | 0x40 => {
                    if let Ok(pic) = picture(bytes) {
                        common::save_img(
                            out_dir,
                            &format!("{name}_pic"),
                            pic.header.width.get(),
                            pic.header.height.get(),
                            &pic.data,
                        );
                    } else if let Ok(miptex) = mip_texture(bytes) {
                        if let Some(data) = miptex.data {
                            common::save_img(
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
