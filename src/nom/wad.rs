use crate::{
    nom::{
        cstr16,
        texture::{font, mip_texture, qpic},
        SliceExt,
    },
    repr::{wad::Archive, wad::Content},
};
use nom::{
    bytes::complete::tag,
    multi::count,
    number::complete::{le_u16, le_u32, le_u8},
};

const MAGIC: &[u8] = b"WAD3";

fn entry<'a>(i: &'a [u8], file: &'a [u8]) -> nom::IResult<&'a [u8], (&'a str, Content<'a>)> {
    let (i, offset) = le_u32(i)?;
    let (i, size) = le_u32(i)?;
    let (i, full_size) = le_u32(i)?;
    let (i, ty) = le_u8(i)?;
    let (i, comp) = le_u8(i)?;
    let (i, _) = le_u16(i)?;
    let (i, name) = cstr16(i)?;
    let data = file.off(offset as usize, size as usize)?;

    let content = match ty {
        0x42 => Content::Picture(qpic(data)?.1),
        0x43 => Content::MipTexture(mip_texture(data)?.1),
        0x46 => Content::Font(font(data)?.1),
        _ if comp != 0 => Content::Compressed { full_size, data },
        _ => Content::Other(data),
    };

    Ok((i, (name, content)))
}

pub fn archive(file: &[u8]) -> nom::IResult<&[u8], Archive> {
    let (i, _) = tag(MAGIC)(file)?;
    let (i, size) = le_u32(i)?;
    let (_, offset) = le_u32(i)?;
    let entry_data = file.off_all(offset as usize)?;
    Ok((
        &[],
        count(|i| entry(i, file), size as usize)(entry_data)?
            .1
            .into_iter()
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    fn save_img<'a, const N: usize>(
        name: &str,
        width: u32,
        height: u32,
        data: &'a crate::repr::texture::ColourData<'a, N>,
    ) {
        let data = data.indices[0]
            .into_iter()
            .flat_map(|&i| {
                let rgb_i = i as usize;
                let r = data.palette[3 * rgb_i];
                let g = data.palette[3 * rgb_i + 1];
                let b = data.palette[3 * rgb_i + 2];
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
        use crate::repr::wad::Content;

        for path in glob::glob("./assets/wad/*.wad")
            .expect("error globing wad")
            .flatten()
        {
            let data = std::fs::read(path).expect("error reading file");
            let (_, archive) = super::archive(&data).expect("error parsing file");

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
}
