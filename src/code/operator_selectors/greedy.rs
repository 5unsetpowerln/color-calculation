use crate::color::{
    Color,
    operator::{OPERATORS, Operator},
};

use super::OperatorIndexImage;

pub fn encode(source_pixels: &[Color]) -> OperatorIndexImage {
    let mut virt_results = Vec::new();
    let mut index_list = Vec::new();

    // とりあえず
    assert!(source_pixels.len() >= 2);

    virt_results.push(source_pixels[0]);
    virt_results.push(source_pixels[1]);

    for index in 2..source_pixels.len() {
        let target = source_pixels[index];

        let prev_0 = virt_results[index - 2];
        let prev_1 = virt_results[index - 1];

        let mut best_op_index = 0;
        let mut best_color = OPERATORS[0](prev_0, prev_1);
        let mut best_score = target.distance(&best_color);

        for op_idx in 1..OPERATORS.len() {
            let predicted = OPERATORS[op_idx](prev_0, prev_1);
            let score = target.distance(&predicted);

            if score < best_score {
                best_score = score;
                best_color = predicted;
                best_op_index = op_idx;
            }
        }

        index_list.push(best_op_index);
        virt_results.push(best_color);
    }

    OperatorIndexImage::new(source_pixels[0], source_pixels[1], index_list)
}

pub fn decode(operator_index_image: &OperatorIndexImage) -> Vec<Color> {
    let mut colors = vec![
        operator_index_image.first_pixel,
        operator_index_image.second_pixel,
    ];

    for (index, operator_index) in operator_index_image.operator_index_list.iter().enumerate() {
        let color = (OPERATORS[*operator_index])(colors[index], colors[index + 1]);
        colors.push(color);
    }

    colors
}
