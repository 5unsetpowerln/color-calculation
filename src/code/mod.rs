use crate::bitstream::{BitStream, BitWriter};
use crate::code::operator_selectors::{OperatorIndexImage, OperatorSelector};
use crate::color::Color;

use self::operator_codecs::OperatorCodec;

mod operator_codecs;
mod operator_selectors;

const OPERATOR_CODEC: OperatorCodec = operator_codecs::CANONICAL_HUFFMAN;
const OPERATOR_SELECTOR: OperatorSelector = operator_selectors::GREEDY;

pub fn encode(source_pixels: &[Color], width: usize, height: usize) -> Vec<u8> {
    let operator_index_image = (OPERATOR_SELECTOR.encode)(source_pixels, width, height);
    let mut writer = BitWriter::new();

    // ヘッダー
    // width (4バイト)
    writer.write_msb(operator_index_image.width as u32, 32);
    // height (4バイト)
    writer.write_msb(operator_index_image.height as u32, 32);

    (OPERATOR_CODEC.encode)(&mut writer, &operator_index_image);

    writer.to_stream().to_bytes()
}

pub fn decode(source_bytes: &[u8]) -> (Vec<Color>, usize, usize) {
    let stream = BitStream::from_bytes(source_bytes);
    let mut reader = stream.reader();

    // ヘッダ
    let width = reader.read_msb(32).unwrap() as usize;
    let height = reader.read_msb(32).unwrap() as usize;

    let position_table = (OPERATOR_SELECTOR.position_table)(width, height);
    let operator_index_image = (OPERATOR_CODEC.decode)(&mut reader, position_table, width, height);

    let decoded = (OPERATOR_SELECTOR.decode)(&operator_index_image);
    (decoded, width, height)
}
