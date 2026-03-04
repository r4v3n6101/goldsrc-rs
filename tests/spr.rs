use goldsrc_rs::texture::{ColorData, SpriteFrame, sprite};

use std::path::Path;

mod common;

#[test]
fn extract_spr() {
    let out_dir = Path::new("./output/");
    std::fs::create_dir_all(out_dir).expect("error creating output dir");

    for path in glob::glob("./valve/sprites/*.spr")
        .expect("error globing spr")
        .flatten()
    {
        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("sprite");
        let name = file_stem.replace(['/', '\\'], "_");

        println!("File: {:?}", path);
        let data = std::fs::read(&path).expect("error reading file");
        let spr = sprite(&data).unwrap();

        println!("Frames: {}", spr.frames.len());
        println!("Palette: {}", spr.palette.len());

        for (idx, frame) in spr.frames.iter().enumerate() {
            match frame {
                SpriteFrame::Single(single) => {
                    let width = single.header.width.get();
                    let height = single.header.height.get();
                    let data = ColorData {
                        indices: [single.indices],
                        palette: spr.palette,
                    };
                    common::save_img(
                        out_dir,
                        &format!("{name}_frame_{idx}"),
                        width,
                        height,
                        &data,
                    );
                }
                SpriteFrame::Group(group) => {
                    for (sub_idx, group_frame) in group.iter().enumerate() {
                        let single = &group_frame.subframe;
                        let width = single.header.width.get();
                        let height = single.header.height.get();
                        let data = ColorData {
                            indices: [single.indices],
                            palette: spr.palette,
                        };
                        common::save_img(
                            out_dir,
                            &format!("{name}_frame_{idx}_sub_{sub_idx}"),
                            width,
                            height,
                            &data,
                        );
                    }
                }
            }
        }
    }
}
