use goldsrc_rs::{
    common::cstring_bytes,
    mdl::{mdl, texture_data},
};

use std::path::Path;

mod common;

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
                    common::save_img(
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
