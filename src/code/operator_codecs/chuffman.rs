use crate::{
    bitstream::{BitRead, BitStream, BitWrite, BitWriter},
    chuffman,
    code::operator_selectors::OperatorIndexImage,
    color::{
        Color,
        operator::{OPERATORS, Operator},
    },
};

pub fn encode(operator_index_image: &OperatorIndexImage) -> BitStream {
    let encode_result =
        chuffman::default::encode(&operator_index_image.operator_index_list, OPERATORS.len());
    let encoded = encode_result.encoded;
    let operator_index_length_table = encode_result.operator_index_length_table;

    // ヘッダで使う幅やエントリ数を operator_index_length_table から計算
    let mut max_operator_index = 0;
    let mut max_operator_index_length = 0;
    let mut index_count = 0;
    for (operator_index, &length) in operator_index_length_table.iter().enumerate() {
        if length > 0 {
            max_operator_index = max_operator_index.max(operator_index);
            max_operator_index_length = max_operator_index_length.max(length);
            index_count += 1;
        }
    }
    let index_width = (max_operator_index as u32).ilog2() + 1;
    let length_width = (max_operator_index_length as u32).ilog2() + 1;

    let mut writer = BitWriter::new();
    // ヘッダー
    // ピクセル数 (8バイト)
    let pixel_count = operator_index_image.operator_index_list.len() + 2;
    writer.write_msb((pixel_count & 0xffffffff) as u32, 32);
    writer.write_msb((pixel_count >> 32) as u32, 32);
    // indexの最大bit数 (4バイト)
    // lengthの最大bit数 (4バイト)
    // lengthテーブルの長さ (4バイト)
    writer.write_msb(index_width, 32);
    writer.write_msb(length_width, 32);
    writer.write_msb(index_count as u32, 32);
    // index:length (indexの最大bit数 + lengthの最大bit数) のテーブル
    for (index, length) in operator_index_length_table.iter().enumerate() {
        if *length == 0 {
            continue;
        }

        writer.write_msb(index as u32, index_width as usize);
        writer.write_msb(*length as u32, length_width as usize);
    }

    // データ本体
    // 最初の2ピクセル
    operator_index_image.first_pixel.bit_write_msb(&mut writer);
    operator_index_image.second_pixel.bit_write_msb(&mut writer);
    for (code, length) in encoded {
        writer.write_msb(code, length);
    }

    writer.to_stream()
}

pub fn decode(stream: BitStream) -> OperatorIndexImage {
    let mut reader = stream.reader();

    // ヘッダ
    let pixel_count =
        reader.read_msb(32).unwrap() as usize | (reader.read_msb(32).unwrap() as usize) << 32;
    let index_width = reader.read_msb(32).unwrap();
    let length_width = reader.read_msb(32).unwrap();
    let index_count = reader.read_msb(32).unwrap() as usize;
    let mut operator_index_length_table = vec![0; OPERATORS.len()];
    for _ in 0..index_count {
        let index = reader.read_msb(index_width as usize).unwrap();
        let length = reader.read_msb(length_width as usize).unwrap();
        operator_index_length_table[index as usize] = length as usize;
    }

    // データ本体
    // 最初の2ピクセル
    let first_color = Color::bit_read_msb(&mut reader);
    let second_color = Color::bit_read_msb(&mut reader);
    let decoded =
        chuffman::default::decode(&mut reader, &operator_index_length_table, pixel_count - 2);

    OperatorIndexImage::new(first_color, second_color, decoded)
}
