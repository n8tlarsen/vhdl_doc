use image::{Rgb, RgbImage};
use std::path::PathBuf;

pub fn make_symbol(doc_path: PathBuf) {
    let mut img = RgbImage::new(32, 32);
    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }
    let mut out_path = doc_path.into_os_string();
    out_path.push("/test.png");
    img.save(out_path).unwrap();
}
