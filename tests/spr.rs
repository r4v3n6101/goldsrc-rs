use std::path::Path;

use goldsrc_rs::texture::{ColorData, SpriteFrame, sprite};

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
                    save_img(
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
                        save_img(
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
