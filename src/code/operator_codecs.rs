use crate::{
    bitstream::{BitRead, BitStream, BitWrite, BitWriter},
    chuffman,
    code::operator_selectors::OperatorIndexImage,
    color::{Color, operator::OPERATORS},
};

type Encoder = fn(operator_index_image: &OperatorIndexImage) -> BitStream;
type Decoder = fn(stream: BitStream) -> OperatorIndexImage;

pub struct OperatorCodec {
    pub encode: Encoder,
    pub decode: Decoder,
}

impl OperatorCodec {
    const fn new(encoder: Encoder, decoder: Decoder) -> Self {
        Self {
            encode: encoder,
            decode: decoder,
        }
    }
}

pub const CANONICAL_HUFFMAN: OperatorCodec =
    OperatorCodec::new(canonical_huffman_encoder, canonical_huffman_decoder);

fn canonical_huffman_encoder(operator_index_image: &OperatorIndexImage) -> BitStream {
    let mut operator_index_freq_table = vec![0; OPERATORS.len()];

    for operator_index in operator_index_image.operator_index_list.iter() {
        operator_index_freq_table[*operator_index] += 1;
    }

    let (encoded, length_table, index_width, length_width, index_count) = chuffman::encode(
        &operator_index_image.operator_index_list,
        &operator_index_freq_table,
        OPERATORS.len(),
    );

    let mut writer = BitWriter::new();
    // ヘッダー
    // ピクセル数 (8バイト)
    let pixel_count = encoded.len() + 2;
    writer.write_msb((pixel_count & 0xffffffff) as u32, 32);
    writer.write_msb((pixel_count >> 32) as u32, 32);
    // indexの最大bit数 (4バイト)
    // lengthの最大bit数 (4バイト)
    // lengthテーブルの長さ (4バイト)
    writer.write_msb(index_width, 32);
    writer.write_msb(length_width, 32);
    writer.write_msb(index_count as u32, 32);
    // index:length (indexの最大bit数 + lengthの最大bit数) のテーブル
    for (index, length) in length_table.iter().enumerate() {
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

fn canonical_huffman_decoder<'a>(stream: BitStream) -> OperatorIndexImage {
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
    let decoded = chuffman::decode(&mut reader, &operator_index_length_table, pixel_count);

    OperatorIndexImage::new(first_color, second_color, decoded)
}
