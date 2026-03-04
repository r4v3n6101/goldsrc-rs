use std::path::Path;

use goldsrc_rs::texture::ColorData;

pub fn save_img<const N: usize>(
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
