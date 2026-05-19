use crate::color::Color;

mod greedy;

pub struct OperatorIndexImage {
    pub first_pixel: Color,
    pub second_pixel: Color,
    pub operator_index_list: Vec<usize>,
    pub position_table: Vec<(usize, usize)>,
    pub width: usize,
    pub height: usize,
}

impl OperatorIndexImage {
    pub fn new(
        first_pixel: Color,
        second_pixel: Color,
        operator_index_list: Vec<usize>,
        position_table: Vec<(usize, usize)>,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            first_pixel,
            second_pixel,
            operator_index_list,
            position_table,
            width,
            height,
        }
    }
}

type Encoder = fn(&[Color], width: usize, height: usize) -> OperatorIndexImage;
type Decoder = fn(&OperatorIndexImage) -> Vec<Color>;
type PositionTableGenerator = fn(width: usize, height: usize) -> Vec<(usize, usize)>;

pub struct OperatorSelector {
    pub encode: Encoder,
    pub decode: Decoder,
    pub position_table: PositionTableGenerator,
}

impl OperatorSelector {
    pub const fn new(
        encoder: Encoder,
        decoder: Decoder,
        position_table_generator: PositionTableGenerator,
    ) -> Self {
        Self {
            encode: encoder,
            decode: decoder,
            position_table: position_table_generator,
        }
    }
}

pub const GREEDY: OperatorSelector = OperatorSelector::new(
    greedy::encode,
    greedy::decode,
    greedy::position_table_generator,
);
