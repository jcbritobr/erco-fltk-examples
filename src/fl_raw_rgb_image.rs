use std::process;

use fltk::{image::JpegImage, prelude::ImageExt};

fn main() {
    let jpeg_image = JpegImage::load("thumbs/lotus-flower-pink.jpg").unwrap();
    let _size = jpeg_image.depth() as usize;
    println!("depth: {}", _size);
    let buf = jpeg_image.to_rgb_data();

    for index in (0..buf.len() / _size).step_by(_size) {
        let _r = buf[index];
        let _g = buf[index + 1];
        let _b = buf[index + 2];

        println!(
            "pixel index:{}, color-> r:{}, r:{}, b:{}",
            index, _r, _g, _b
        );
    }

    process::exit(0);
}
