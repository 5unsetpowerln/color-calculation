use crate::color::{
    Color,
    operator::{OPERATORS, Operator},
};

mod greedy;

pub struct OperatorIndexImage {
    pub first_pixel: Color,
    pub second_pixel: Color,
    pub operator_index_list: Vec<usize>,
}

impl OperatorIndexImage {
    pub fn new(first_pixel: Color, second_pixel: Color, operator_index_list: Vec<usize>) -> Self {
        Self {
            first_pixel,
            second_pixel,
            operator_index_list,
        }
    }
}

type Encoder = fn(&[Color]) -> OperatorIndexImage;
type Decoder = fn(&OperatorIndexImage) -> Vec<Color>;

pub struct OperatorSelector {
    pub encode: Encoder,
    pub decode: Decoder,
}

impl OperatorSelector {
    pub const fn new(encoder: Encoder, decoder: Decoder) -> Self {
        Self {
            encode: encoder,
            decode: decoder,
        }
    }
}

pub const GREEDY: OperatorSelector = OperatorSelector::new(greedy::encode, greedy::decode);
