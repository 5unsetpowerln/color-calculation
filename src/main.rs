use crate::color::Color;
use image::{Rgb, RgbImage};

mod bitstream;
mod chuffman;
mod code;
mod color;

fn main() {
    let img = image::open("sample.jpg").expect("Failed to open sample2.jpg");
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    let mut source_pixels: Vec<Color> = Vec::new();
    for pixel in rgb_img.pixels() {
        let [r, g, b] = pixel.0;
        source_pixels.push(Color { r, g, b });
    }

    if source_pixels.is_empty() {
        println!("Error: No pixels found in the image.");
        return;
    }

    let encoded = code::encode(&source_pixels);
    let decoded = code::decode(&encoded);

    let mut output = RgbImage::new(width, height);
    for (i, color) in decoded.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        output.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
    }

    output.save("output.png").unwrap();
    println!("saved: output.png");
}
