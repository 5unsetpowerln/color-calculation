use crate::color::Color;
use image::{Rgb, RgbImage};

mod bitstream;
mod chuffman;
mod code;
mod color;

fn main() {
    let img = image::open("sample.jpg").expect("Failed to open sample.jpg");
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

    let encoded = code::encode(&source_pixels, width as usize, height as usize);

    let original_size = source_pixels.len() * 3;
    let compressed_size = encoded.len();
    let ratio = ((compressed_size as f64) / (original_size as f64)) * 100.0;
    println!("raw length: {}", source_pixels.len() * 3);
    println!("encoded length: {}", encoded.len());
    println!("compression ratio: {:.2}%", ratio);

    let (decoded, width, height) = code::decode(&encoded);
    let width = width as u32;
    let height = height as u32;

    let mut output = RgbImage::new(width, height);
    for (i, color) in decoded.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        output.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
    }

    output.save("output.png").unwrap();
    println!("saved: output.png");
}
