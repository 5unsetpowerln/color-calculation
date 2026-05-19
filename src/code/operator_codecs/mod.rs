use crate::bitstream::{BitReader, BitStream, BitWriter};

use super::operator_selectors::OperatorIndexImage;

mod chuffman;
mod chuffman_repeat_compact;
mod nothing;

type Encoder = fn(writer: &mut BitWriter, operator_index_image: &OperatorIndexImage);
type Decoder = fn(
    reader: &mut BitReader,
    position_table: Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> OperatorIndexImage;

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
