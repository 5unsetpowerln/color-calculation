use crate::{
    bitstream::{BitReader, BitWrite, BitWriter},
    color::{Color, operator::OPERATORS},
};
use image::{Rgb, RgbImage};

mod bitstream;
mod canonical_huffman;
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
    let mut index_list: Vec<usize> = Vec::new();

    // 実際に送信するストリーム
    let mut bit_writer = BitWriter::new();

    // 最初の2px分は送信するしかない
    if source_pixels.len() > 0 {
        results.push(source_pixels[0]);
    }
    if source_pixels.len() > 1 {
        results.push(source_pixels[1]);
    }

    // 頻度表
    let mut freq_table = vec![0; OPERATORS.len()];

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

        index_list.push(best_op_index);

        freq_table[best_op_index] += 1;

        results.push(best_color);
    }

    println!("huffman encoding");
    let (encoded, length_table, index_width, length_width, index_count) =
        canonical_huffman::encode(&index_list, &freq_table, OPERATORS.len());
    println!("done");

    // ヘッダー
    // indexの最大bit数 (4バイト)
    // lengthの最大bit数 (4バイト)
    // lengthテーブルの長さ (4バイト)
    bit_writer.write_msb(index_width, 32);
    bit_writer.write_msb(length_width, 32);
    bit_writer.write_msb(index_count as u32, 32);
    // index:length (indexの最大bit数 + lengthの最大bit数) のテーブル
    for (index, length) in length_table.iter().enumerate() {
        if *length == 0 {
            continue;
        }

        bit_writer.write_msb(index as u32, index_width as usize);
        bit_writer.write_msb(*length as u32, length_width as usize);
    }

    // データ本体
    // 最初の2ピクセル
    source_pixels[0].bit_write_msb(&mut bit_writer);
    source_pixels[1].bit_write_msb(&mut bit_writer);
    for (code, length) in encoded {
        bit_writer.write_msb(code, length);
    }

    // 実際に送信されるデータ
    let bit_stream = bit_writer.to_stream();

    // 受信側の処理
    let mut bit_reader = bit_stream.reader();

    // ヘッダ
    let recv_index_width = bit_reader.read_msb(32);
    let recv_length_width = bit_reader.read_msb(32);
    let recv_index_count = bit_reader.read_msb(32) as usize;
    assert_eq!(index_width, recv_index_width);
    assert_eq!(length_width, recv_length_width);
    assert_eq!(index_count, recv_index_count);
    println!("index_width: {}", recv_index_width);
    println!("length_width: {}", recv_length_width);
    println!("index_count: {}", recv_index_count);
    let mut recv_length_table = vec![0; recv_index_count as usize];
    for _ in 0..recv_index_count {
        let index = bit_reader.read_msb(index_width as usize);
        let length = bit_reader.read_msb(length_width as usize);
        recv_length_table[index as usize] = length as usize;
    }
    assert_eq!(length_table, recv_length_table);
    println!("length_table: {:?}", length_table);

    // let mut pairs: Vec<(usize, usize, usize)> = freq_table
    //     .iter()
    //     .enumerate()
    //     .map(|(op_index, freq)| (op_index, *freq, length_table[op_index]))
    //     .collect();
    // pairs.sort_by(|a, b| b.1.cmp(&a.1));
    // let mut total_bits = 0usize;
    // let mut total_count = 0usize;
    // for (rank, (op_index, freq, length)) in pairs.iter().enumerate() {
    //     println!("{}, {}, {}, {}", rank, op_index, freq, length);
    //     total_bits += freq * length;
    //     total_count += freq;
    // }
    // let avg_bits = total_bits as f64 / total_count as f64;
    // println!(
    //     "average code length: {:.4} bits/symbol  ({} bits / {} symbols)",
    //     avg_bits, total_bits, total_count
    // );

    canonical_huffman::decode(&mut bit_reader, &recv_length_table);

    // 2:
    //  00
    //  01
    //  10
    // 3:
    //  110
    // 4:
    //  1110

    // 01 == 00 + 1
    // 10 == 01 + 1

    // freq_table.sort();
    //
    // freq_table.reverse();
    // for (index, freq) in freq_table.iter().enumerate() {
    //     let length = length_table[index];

    //     println!("{}, {}, {}", index, freq, length);
    // }

    // let mut output = RgbImage::new(width, height);
    // for (i, color) in results.iter().enumerate() {
    //     let x = (i as u32) % width;
    //     let y = (i as u32) / width;
    //     output.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
    // }

    // output.save("output.png").unwrap();

    // println!("saved: output.png");
    // println!("Total operators transmitted: {}", index_list.len());
}
