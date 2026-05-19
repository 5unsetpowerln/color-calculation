use crate::bitstream::BitStream;
use crate::code::operator_selectors::{OperatorIndexImage, OperatorSelector};
use crate::color::Color;

use self::operator_codecs::OperatorCodec;

mod operator_codecs;
mod operator_selectors;

const OPERATOR_CODEC: OperatorCodec = operator_codecs::CANONICAL_HUFFMAN;
const OPERATOR_SELECTOR: OperatorSelector = operator_selectors::GREEDY;

pub fn encode(source_pixels: &[Color]) -> Vec<u8> {
    let operator_index_image = (OPERATOR_SELECTOR.encode)(source_pixels);
    let stream = (OPERATOR_CODEC.encode)(&operator_index_image);
    stream.to_bytes()
}

pub fn decode(source_bytes: &[u8]) -> Vec<Color> {
    let stream = BitStream::from_bytes(source_bytes);
    let operator_index_image = (OPERATOR_CODEC.decode)(stream);

    (OPERATOR_SELECTOR.decode)(&operator_index_image)
}
