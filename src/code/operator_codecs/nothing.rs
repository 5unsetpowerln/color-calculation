use crate::{
    bitstream::{BitRead, BitReader, BitStream, BitWrite, BitWriter},
    chuffman,
    code::operator_selectors::OperatorIndexImage,
    color::{Color, operator::OPERATORS},
};

pub fn encode(writer: &mut BitWriter, operator_index_image: &OperatorIndexImage) {
    let max_operator_index = OPERATORS.len() - 1;
    let index_width = max_operator_index.ilog2() + 1;

    // ヘッダ
    // 演算子インデックスの最大bit長(8ビット)
    writer.write_msb(index_width, 8);

    // データ本体
    operator_index_image.first_pixel.bit_write_msb(writer);
    operator_index_image.second_pixel.bit_write_msb(writer);
    for operator_index in operator_index_image.operator_index_list.iter() {
        writer.write_msb(*operator_index as u32, index_width as usize);
    }
}

pub fn decode(
    reader: &mut BitReader,
    position_table: Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> OperatorIndexImage {
    //ヘッダ
    let index_width = reader.read_msb(8).unwrap() as usize;

    // データ本体
    let first_pixel = Color::bit_read_msb(reader);
    let second_pixel = Color::bit_read_msb(reader);
    let mut operator_index_list = Vec::new();
    for _ in 0..position_table.len() {
        operator_index_list.push(reader.read_msb(index_width).unwrap() as usize);
    }

    OperatorIndexImage::new(
        first_pixel,
        second_pixel,
        operator_index_list,
        position_table,
        width,
        height,
    )
}
