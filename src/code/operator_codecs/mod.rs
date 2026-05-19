use crate::bitstream::BitStream;

use super::operator_selectors::OperatorIndexImage;

mod chuffman;
mod chuffman_repeat_compact;
mod nothing;

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

pub const CANONICAL_HUFFMAN: OperatorCodec = OperatorCodec::new(chuffman::encode, chuffman::decode);
pub const CANONICAL_HUFFMAN_REPEAT_COMPACTION: OperatorCodec = OperatorCodec::new(
    chuffman_repeat_compact::encode,
    chuffman_repeat_compact::decode,
);
pub const NOTHING: OperatorCodec = OperatorCodec::new(nothing::encode, nothing::decode);
