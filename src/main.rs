use crate::color::{Color, operator::OPERATORS};
use image::{Rgb, RgbImage};

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

    // アルゴリズム適応済みの画像
    let mut results: Vec<Color> = Vec::new();

    // 送信データ
    let mut transmitted_data: Vec<usize> = Vec::new();

    // 最初の2px分は送信するしかない
    if source_pixels.len() > 0 {
        results.push(source_pixels[0]);
    }
    if source_pixels.len() > 1 {
        results.push(source_pixels[1]);
    }

    for index in 2..source_pixels.len() {
        let target = source_pixels[index];

        let prev_1 = results[index - 1];
        let prev_2 = results[index - 2];

        let mut best_op_index = 0;
        let mut best_color = OPERATORS[0](prev_1, prev_2);
        let mut best_score = target.distance(&best_color);

        for op_idx in 1..OPERATORS.len() {
            let predicted = OPERATORS[op_idx](prev_1, prev_2);
            let score = target.distance(&predicted);

            if score < best_score {
                best_score = score;
                best_color = predicted;
                best_op_index = op_idx;
            }
        }

        transmitted_data.push(best_op_index);

        results.push(best_color);
    }

    let mut output = RgbImage::new(width, height);
    for (i, color) in results.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        output.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
    }

    output.save("output.png").unwrap();

    println!("saved: output.png");
    println!("Total operators transmitted: {}", transmitted_data.len());
}
