use crate::{
    bitstream::{BitRead, BitStream, BitWrite, BitWriter},
    chuffman,
    code::operator_selectors::OperatorIndexImage,
    color::{Color, operator::OPERATORS},
};

pub fn encode(operator_index_image: &OperatorIndexImage) -> BitStream {
    let mut writer = BitWriter::new();
    let max_operator_index = OPERATORS.len() - 1;
    let index_width = max_operator_index.ilog2() + 1;

    // ヘッダ

    // ピクセル数(64ビット)
    let pixel_count = operator_index_image.operator_index_list.len() + 2;
    writer.write_msb((pixel_count & 0xffffffff) as u32, 32);
    writer.write_msb((pixel_count >> 32) as u32, 32);

    // 演算子インデックスの最大bit長(8ビット)
    writer.write_msb(index_width, 8);

    // データ本体
    operator_index_image.first_pixel.bit_write_msb(&mut writer);
    operator_index_image.second_pixel.bit_write_msb(&mut writer);
    for operator_index in operator_index_image.operator_index_list.iter() {
        writer.write_msb(*operator_index as u32, index_width as usize);
    }

    writer.to_stream()
}

pub fn decode(stream: BitStream) -> OperatorIndexImage {
    let mut reader = stream.reader();

    // ヘッダ
    let pixel_count =
        reader.read_msb(32).unwrap() as usize | (reader.read_msb(32).unwrap() as usize) << 32;
    let index_width = reader.read_msb(8).unwrap() as usize;

    // データ本体
    let first_pixel = Color::bit_read_msb(&mut reader);
    let second_pixel = Color::bit_read_msb(&mut reader);
    let mut operator_index_list = Vec::new();
    for _ in 0..(pixel_count - 2) {
        operator_index_list.push(reader.read_msb(index_width).unwrap() as usize);
    }

    OperatorIndexImage::new(first_pixel, second_pixel, operator_index_list)
}
