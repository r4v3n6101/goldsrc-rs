use std::path::Path;

use goldsrc_rs::{
    common::cstring_bytes,
    mdl::{mdl, texture_data},
    texture::ColorData,
};

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
fn parse_mdl() {
    let out_dir = Path::new("./output");
    std::fs::create_dir_all(out_dir).expect("error creating output dir");

    for path in glob::glob("./valve/models/**/*.mdl")
        .expect("error globing mdl")
        .flatten()
    {
        println!("File: {:?}", path);
        let data = std::fs::read(&path).expect("error reading file");
        let Ok(model) = mdl(&data) else {
            eprintln!("File can't be loaded: {path:?}");
            continue;
        };

        let name = String::from_utf8_lossy(cstring_bytes(&model.header.name));
        println!("Name: {}", name);
        println!("Bones: {}", model.bones.len());
        println!("BoneControllers: {}", model.bone_controllers.len());
        println!("Hitboxes: {}", model.hitboxes.len());
        println!("Sequences: {}", model.sequences.len());
        println!("SequenceGroups: {}", model.sequence_groups.len());
        println!("Textures: {}", model.textures.len());
        println!("Bodyparts: {}", model.bodyparts.len());
        println!("Attachments: {}", model.attachments.len());
        println!("Transitions: {}", model.transitions.len());

        println!(
            "Skins: families={} refs={}",
            model.skins.skin_families_num, model.skins.skin_refs_num
        );

        for (idx, tex) in model.textures.iter().enumerate() {
            let tex_name = String::from_utf8_lossy(cstring_bytes(&tex.name));
            let width = tex.width.get();
            let height = tex.height.get();
            let offset = tex.offset.get();
            if offset == 0 || width == 0 || height == 0 {
                println!("  {idx}: {tex_name} {}x{} offset=0", width, height);
                continue;
            }

            match texture_data(&data, tex) {
                Ok(data) => {
                    println!(
                        "  {idx}: {tex_name} {}x{} palette={}",
                        width,
                        height,
                        data.palette.len()
                    );
                    save_img(
                        out_dir,
                        &format!("{}_tex_{idx}", tex_name.replace(['/', '\\'], "_")),
                        width,
                        height,
                        &data,
                    );
                }
                Err(err) => {
                    eprintln!("  {idx}: {tex_name} error: {err}");
                }
            }
        }
    }
}
