use crate::color::Color;

pub type Operator = fn(base: Color, input: Color) -> Color;

/// 演算子の種類（全100パターン）
pub const OPERATORS: [Operator; 100] = [
    // 1-10: 基本加減算・そのまま
    add_rgb,
    sub_rgb,
    reverse_sub_rgb,
    base_add_only_r,
    input_add_only_r,
    pass_base,
    pass_input,
    max_rgb,
    min_rgb,
    average_rgb,
    // 11-20: 平均・線形予測・飽和・反転
    avg_base_heavy,
    avg_input_heavy,
    linear_extrapolate,
    sat_add_rgb,
    sat_sub_rgb,
    abs_diff_rgb,
    invert_base,
    invert_input,
    invert_average,
    xor_rgb,
    // 21-30: ビット演算・チャンネル置換(Base)
    and_rgb,
    or_rgb,
    nand_rgb,
    nor_rgb,
    xnor_rgb,
    swap_gb_base,
    swap_rb_base,
    swap_rg_base,
    shift_rgb_base,
    shift_bgr_base,
    // 31-40: チャンネル置換(Input)・クロスミックス
    swap_gb_input,
    swap_rb_input,
    swap_rg_input,
    shift_rgb_input,
    shift_bgr_input,
    mix_r_base,
    mix_g_base,
    mix_b_base,
    mix_rg_base,
    mix_gb_base,
    // 41-50: クロスミックス・ブレンドモード
    mix_rb_base,
    multiply_blend,
    screen_blend,
    overlay_approx,
    difference_blend,
    half_base_add_input,
    half_input_add_base,
    bit_shift_left,
    bit_shift_right,
    base_luma_to_input,
    // 51-60: 高度な飽和・平方・細分化された重み付き平均
    sat_reverse_sub,
    add_half_half,
    sub_half_half,
    sat_mult_2x,
    sq_diff,
    weight_7_1,
    weight_1_7,
    weight_3_5,
    weight_5_3,
    weight_15_1,
    // 61-70: 高度なブレンドモード・クロスチャンネル演算
    color_dodge_approx,
    color_burn_approx,
    hard_light_approx,
    linear_burn_approx,
    linear_light_approx,
    cross_add,
    cross_sub,
    cross_avg,
    cross_max,
    cross_min,
    // 71-80: クロスビット演算・モジュロ演算
    xor_cross,
    and_cross,
    or_cross,
    shl_cross,
    shr_cross,
    mod_add,
    mod_sub,
    mod_div,
    posterize_4_b,
    posterize_4_i,
    // 81-90: ポスタリゼーション・2値化・グレースケール変換
    posterize_8_b,
    posterize_8_i,
    thresh_128_b,
    thresh_128_i,
    thresh_avg,
    gray_avg_b,
    gray_avg_i,
    gray_bt709_b,
    gray_bt709_i,
    input_luma_to_base,
    // 91-100: 増幅・減衰・極端なコントラストとローテーション
    amplify_diff,
    dampen_diff,
    not_xor_half,
    rot_left_b,
    rot_right_b,
    extreme_contrast_b,
    extreme_contrast_i,
    dist_rg_add,
    dist_gb_add,
    dist_br_add,
];

// ==========================================
// 1-10
// ==========================================
fn add_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(i.r),
        g: b.g.wrapping_add(i.g),
        b: b.b.wrapping_add(i.b),
    }
}
fn sub_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_sub(i.r),
        g: b.g.wrapping_sub(i.g),
        b: b.b.wrapping_sub(i.b),
    }
}
fn reverse_sub_rgb(b: Color, i: Color) -> Color {
    Color {
        r: i.r.wrapping_sub(b.r),
        g: i.g.wrapping_sub(b.g),
        b: i.b.wrapping_sub(b.b),
    }
}
fn base_add_only_r(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(i.r),
        g: b.g,
        b: b.b,
    }
}
fn input_add_only_r(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(i.r),
        g: i.g,
        b: i.b,
    }
}
fn pass_base(b: Color, _i: Color) -> Color {
    b
}
fn pass_input(_b: Color, i: Color) -> Color {
    i
}
fn max_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r.max(i.r),
        g: b.g.max(i.g),
        b: b.b.max(i.b),
    }
}
fn min_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r.min(i.r),
        g: b.g.min(i.g),
        b: b.b.min(i.b),
    }
}
fn average_rgb(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 + i.r as u16) / 2) as u8,
        g: ((b.g as u16 + i.g as u16) / 2) as u8,
        b: ((b.b as u16 + i.b as u16) / 2) as u8,
    }
}

// ==========================================
// 11-20
// ==========================================
fn avg_base_heavy(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 * 3 + i.r as u16) / 4) as u8,
        g: ((b.g as u16 * 3 + i.g as u16) / 4) as u8,
        b: ((b.b as u16 * 3 + i.b as u16) / 4) as u8,
    }
}
fn avg_input_heavy(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 + i.r as u16 * 3) / 4) as u8,
        g: ((b.g as u16 + i.g as u16 * 3) / 4) as u8,
        b: ((b.b as u16 + i.b as u16 * 3) / 4) as u8,
    }
}
fn linear_extrapolate(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(b.r.wrapping_sub(i.r)),
        g: b.g.wrapping_add(b.g.wrapping_sub(i.g)),
        b: b.b.wrapping_add(b.b.wrapping_sub(i.b)),
    }
}
fn sat_add_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r.saturating_add(i.r),
        g: b.g.saturating_add(i.g),
        b: b.b.saturating_add(i.b),
    }
}
fn sat_sub_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r.saturating_sub(i.r),
        g: b.g.saturating_sub(i.g),
        b: b.b.saturating_sub(i.b),
    }
}
fn abs_diff_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r.abs_diff(i.r),
        g: b.g.abs_diff(i.g),
        b: b.b.abs_diff(i.b),
    }
}
fn invert_base(b: Color, _i: Color) -> Color {
    Color {
        r: !b.r,
        g: !b.g,
        b: !b.b,
    }
}
fn invert_input(_b: Color, i: Color) -> Color {
    Color {
        r: !i.r,
        g: !i.g,
        b: !i.b,
    }
}
fn invert_average(b: Color, i: Color) -> Color {
    Color {
        r: !((b.r as u16 + i.r as u16) / 2) as u8,
        g: !((b.g as u16 + i.g as u16) / 2) as u8,
        b: !((b.b as u16 + i.b as u16) / 2) as u8,
    }
}
fn xor_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r ^ i.r,
        g: b.g ^ i.g,
        b: b.b ^ i.b,
    }
}

// ==========================================
// 21-30
// ==========================================
fn and_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r & i.r,
        g: b.g & i.g,
        b: b.b & i.b,
    }
}
fn or_rgb(b: Color, i: Color) -> Color {
    Color {
        r: b.r | i.r,
        g: b.g | i.g,
        b: b.b | i.b,
    }
}
fn nand_rgb(b: Color, i: Color) -> Color {
    Color {
        r: !(b.r & i.r),
        g: !(b.g & i.g),
        b: !(b.b & i.b),
    }
}
fn nor_rgb(b: Color, i: Color) -> Color {
    Color {
        r: !(b.r | i.r),
        g: !(b.g | i.g),
        b: !(b.b | i.b),
    }
}
fn xnor_rgb(b: Color, i: Color) -> Color {
    Color {
        r: !(b.r ^ i.r),
        g: !(b.g ^ i.g),
        b: !(b.b ^ i.b),
    }
}
fn swap_gb_base(b: Color, _i: Color) -> Color {
    Color {
        r: b.r,
        g: b.b,
        b: b.g,
    }
}
fn swap_rb_base(b: Color, _i: Color) -> Color {
    Color {
        r: b.b,
        g: b.g,
        b: b.r,
    }
}
fn swap_rg_base(b: Color, _i: Color) -> Color {
    Color {
        r: b.g,
        g: b.r,
        b: b.b,
    }
}
fn shift_rgb_base(b: Color, _i: Color) -> Color {
    Color {
        r: b.b,
        g: b.r,
        b: b.g,
    }
}
fn shift_bgr_base(b: Color, _i: Color) -> Color {
    Color {
        r: b.g,
        g: b.b,
        b: b.r,
    }
}

// ==========================================
// 31-40
// ==========================================
fn swap_gb_input(_b: Color, i: Color) -> Color {
    Color {
        r: i.r,
        g: i.b,
        b: i.g,
    }
}
fn swap_rb_input(_b: Color, i: Color) -> Color {
    Color {
        r: i.b,
        g: i.g,
        b: i.r,
    }
}
fn swap_rg_input(_b: Color, i: Color) -> Color {
    Color {
        r: i.g,
        g: i.r,
        b: i.b,
    }
}
fn shift_rgb_input(_b: Color, i: Color) -> Color {
    Color {
        r: i.b,
        g: i.r,
        b: i.g,
    }
}
fn shift_bgr_input(_b: Color, i: Color) -> Color {
    Color {
        r: i.g,
        g: i.b,
        b: i.r,
    }
}
fn mix_r_base(b: Color, i: Color) -> Color {
    Color {
        r: b.r,
        g: i.g,
        b: i.b,
    }
}
fn mix_g_base(b: Color, i: Color) -> Color {
    Color {
        r: i.r,
        g: b.g,
        b: i.b,
    }
}
fn mix_b_base(b: Color, i: Color) -> Color {
    Color {
        r: i.r,
        g: i.g,
        b: b.b,
    }
}
fn mix_rg_base(b: Color, i: Color) -> Color {
    Color {
        r: b.r,
        g: b.g,
        b: i.b,
    }
}
fn mix_gb_base(b: Color, i: Color) -> Color {
    Color {
        r: i.r,
        g: b.g,
        b: b.b,
    }
}

// ==========================================
// 41-50
// ==========================================
fn mix_rb_base(b: Color, i: Color) -> Color {
    Color {
        r: b.r,
        g: i.g,
        b: b.b,
    }
}
fn multiply_blend(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 * i.r as u16) / 255) as u8,
        g: ((b.g as u16 * i.g as u16) / 255) as u8,
        b: ((b.b as u16 * i.b as u16) / 255) as u8,
    }
}
fn screen_blend(b: Color, i: Color) -> Color {
    Color {
        r: 255 - (((255 - b.r as u16) * (255 - i.r as u16)) / 255) as u8,
        g: 255 - (((255 - b.g as u16) * (255 - i.g as u16)) / 255) as u8,
        b: 255 - (((255 - b.b as u16) * (255 - i.b as u16)) / 255) as u8,
    }
}
fn overlay_approx(b: Color, i: Color) -> Color {
    let calc = |bs: u8, in_c: u8| -> u8 {
        if bs < 128 {
            ((2 * bs as u16 * in_c as u16) / 255) as u8
        } else {
            255 - ((2 * (255 - bs as u16) * (255 - in_c as u16)) / 255) as u8
        }
    };
    Color {
        r: calc(b.r, i.r),
        g: calc(b.g, i.g),
        b: calc(b.b, i.b),
    }
}
fn difference_blend(b: Color, i: Color) -> Color {
    Color {
        r: b.r.abs_diff(i.r),
        g: b.g.abs_diff(i.g),
        b: b.b.abs_diff(i.b),
    }
}
fn half_base_add_input(b: Color, i: Color) -> Color {
    Color {
        r: (b.r >> 1).wrapping_add(i.r),
        g: (b.g >> 1).wrapping_add(i.g),
        b: (b.b >> 1).wrapping_add(i.b),
    }
}
fn half_input_add_base(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(i.r >> 1),
        g: b.g.wrapping_add(i.g >> 1),
        b: b.b.wrapping_add(i.b >> 1),
    }
}
fn bit_shift_left(b: Color, _i: Color) -> Color {
    Color {
        r: b.r << 1,
        g: b.g << 1,
        b: b.b << 1,
    }
}
fn bit_shift_right(b: Color, _i: Color) -> Color {
    Color {
        r: b.r >> 1,
        g: b.g >> 1,
        b: b.b >> 1,
    }
}
fn base_luma_to_input(b: Color, i: Color) -> Color {
    let luma = ((b.r as u16 + b.g as u16 + b.b as u16) / 3) as u8;
    Color {
        r: luma.saturating_add(i.r).min(255),
        g: luma.saturating_add(i.g).min(255),
        b: luma.saturating_add(i.b).min(255),
    }
}

// ==========================================
// 51-60
// ==========================================
fn sat_reverse_sub(b: Color, i: Color) -> Color {
    Color {
        r: i.r.saturating_sub(b.r),
        g: i.g.saturating_sub(b.g),
        b: i.b.saturating_sub(b.b),
    }
}
fn add_half_half(b: Color, i: Color) -> Color {
    Color {
        r: (b.r >> 1) + (i.r >> 1),
        g: (b.g >> 1) + (i.g >> 1),
        b: (b.b >> 1) + (i.b >> 1),
    }
}
fn sub_half_half(b: Color, i: Color) -> Color {
    Color {
        r: (b.r >> 1).wrapping_sub(i.r >> 1),
        g: (b.g >> 1).wrapping_sub(i.g >> 1),
        b: (b.b >> 1).wrapping_sub(i.b >> 1),
    }
}
fn sat_mult_2x(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u32 * i.r as u32 * 2) / 255).min(255) as u8,
        g: ((b.g as u32 * i.g as u32 * 2) / 255).min(255) as u8,
        b: ((b.b as u32 * i.b as u32 * 2) / 255).min(255) as u8,
    }
}
fn sq_diff(b: Color, i: Color) -> Color {
    let calc = |v1: u8, v2: u8| -> u8 {
        let d = v1.abs_diff(v2) as u16;
        ((d * d) >> 4).min(255) as u8
    };
    Color {
        r: calc(b.r, i.r),
        g: calc(b.g, i.g),
        b: calc(b.b, i.b),
    }
}
fn weight_7_1(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 * 7 + i.r as u16) / 8) as u8,
        g: ((b.g as u16 * 7 + i.g as u16) / 8) as u8,
        b: ((b.b as u16 * 7 + i.b as u16) / 8) as u8,
    }
}
fn weight_1_7(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 + i.r as u16 * 7) / 8) as u8,
        g: ((b.g as u16 + i.g as u16 * 7) / 8) as u8,
        b: ((b.b as u16 + i.b as u16 * 7) / 8) as u8,
    }
}
fn weight_3_5(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 * 3 + i.r as u16 * 5) / 8) as u8,
        g: ((b.g as u16 * 3 + i.g as u16 * 5) / 8) as u8,
        b: ((b.b as u16 * 3 + i.b as u16 * 5) / 8) as u8,
    }
}
fn weight_5_3(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 * 5 + i.r as u16 * 3) / 8) as u8,
        g: ((b.g as u16 * 5 + i.g as u16 * 3) / 8) as u8,
        b: ((b.b as u16 * 5 + i.b as u16 * 3) / 8) as u8,
    }
}
fn weight_15_1(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 * 15 + i.r as u16) / 16) as u8,
        g: ((b.g as u16 * 15 + i.g as u16) / 16) as u8,
        b: ((b.b as u16 * 15 + i.b as u16) / 16) as u8,
    }
}

// ==========================================
// 61-70
// ==========================================
fn color_dodge_approx(b: Color, i: Color) -> Color {
    let calc = |bs: u8, in_c: u8| -> u8 {
        if in_c == 255 {
            255
        } else {
            ((bs as u16 * 255) / (255 - in_c as u16)).min(255) as u8
        }
    };
    Color {
        r: calc(b.r, i.r),
        g: calc(b.g, i.g),
        b: calc(b.b, i.b),
    }
}
fn color_burn_approx(b: Color, i: Color) -> Color {
    let calc = |bs: u8, in_c: u8| -> u8 {
        if in_c == 0 {
            0
        } else {
            255 - ((255 - bs as u16) * 255 / in_c as u16).min(255) as u8
        }
    };
    Color {
        r: calc(b.r, i.r),
        g: calc(b.g, i.g),
        b: calc(b.b, i.b),
    }
}
fn hard_light_approx(b: Color, i: Color) -> Color {
    let calc = |bs: u8, in_c: u8| -> u8 {
        if in_c < 128 {
            ((2 * bs as u16 * in_c as u16) / 255) as u8
        } else {
            255 - ((2 * (255 - bs as u16) * (255 - in_c as u16)) / 255) as u8
        }
    };
    Color {
        r: calc(b.r, i.r),
        g: calc(b.g, i.g),
        b: calc(b.b, i.b),
    }
}
fn linear_burn_approx(b: Color, i: Color) -> Color {
    Color {
        r: b.r.saturating_add(i.r).saturating_sub(255),
        g: b.g.saturating_add(i.g).saturating_sub(255),
        b: b.b.saturating_add(i.b).saturating_sub(255),
    }
}
fn linear_light_approx(b: Color, i: Color) -> Color {
    let calc = |bs: u8, in_c: u8| -> u8 {
        (bs as u16)
            .saturating_add(in_c as u16)
            .saturating_sub(128)
            .min(255) as u8
    };
    Color {
        r: calc(b.r, i.r),
        g: calc(b.g, i.g),
        b: calc(b.b, i.b),
    }
}
fn cross_add(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(i.g),
        g: b.g.wrapping_add(i.b),
        b: b.b.wrapping_add(i.r),
    }
}
fn cross_sub(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_sub(i.g),
        g: b.g.wrapping_sub(i.b),
        b: b.b.wrapping_sub(i.r),
    }
}
fn cross_avg(b: Color, i: Color) -> Color {
    Color {
        r: ((b.r as u16 + i.g as u16) / 2) as u8,
        g: ((b.g as u16 + i.b as u16) / 2) as u8,
        b: ((b.b as u16 + i.r as u16) / 2) as u8,
    }
}
fn cross_max(b: Color, i: Color) -> Color {
    Color {
        r: b.r.max(i.g),
        g: b.g.max(i.b),
        b: b.b.max(i.r),
    }
}
fn cross_min(b: Color, i: Color) -> Color {
    Color {
        r: b.r.min(i.g),
        g: b.g.min(i.b),
        b: b.b.min(i.r),
    }
}

// ==========================================
// 71-80
// ==========================================
fn xor_cross(b: Color, i: Color) -> Color {
    Color {
        r: b.r ^ i.g,
        g: b.g ^ i.b,
        b: b.b ^ i.r,
    }
}
fn and_cross(b: Color, i: Color) -> Color {
    Color {
        r: b.r & i.g,
        g: b.g & i.b,
        b: b.b & i.r,
    }
}
fn or_cross(b: Color, i: Color) -> Color {
    Color {
        r: b.r | i.g,
        g: b.g | i.b,
        b: b.b | i.r,
    }
}
fn shl_cross(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_shl((i.g % 8) as u32),
        g: b.g.wrapping_shl((i.b % 8) as u32),
        b: b.b.wrapping_shl((i.r % 8) as u32),
    }
}
fn shr_cross(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_shr((i.g % 8) as u32),
        g: b.g.wrapping_shr((i.b % 8) as u32),
        b: b.b.wrapping_shr((i.r % 8) as u32),
    }
}
fn mod_add(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(i.r % 64),
        g: b.g.wrapping_add(i.g % 64),
        b: b.b.wrapping_add(i.b % 64),
    }
}
fn mod_sub(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_sub(i.r % 64),
        g: b.g.wrapping_sub(i.g % 64),
        b: b.b.wrapping_sub(i.b % 64),
    }
}
fn mod_div(b: Color, i: Color) -> Color {
    Color {
        r: b.r / (i.r / 16 + 1),
        g: b.g / (i.g / 16 + 1),
        b: b.b / (i.b / 16 + 1),
    }
}
fn posterize_4_b(b: Color, _i: Color) -> Color {
    Color {
        r: b.r & 0xC0,
        g: b.g & 0xC0,
        b: b.b & 0xC0,
    }
}
fn posterize_4_i(_b: Color, i: Color) -> Color {
    Color {
        r: i.r & 0xC0,
        g: i.g & 0xC0,
        b: i.b & 0xC0,
    }
}

// ==========================================
// 81-90
// ==========================================
fn posterize_8_b(b: Color, _i: Color) -> Color {
    Color {
        r: b.r & 0xE0,
        g: b.g & 0xE0,
        b: b.b & 0xE0,
    }
}
fn posterize_8_i(_b: Color, i: Color) -> Color {
    Color {
        r: i.r & 0xE0,
        g: i.g & 0xE0,
        b: i.b & 0xE0,
    }
}
fn thresh_128_b(b: Color, _i: Color) -> Color {
    Color {
        r: if b.r > 128 { 255 } else { 0 },
        g: if b.g > 128 { 255 } else { 0 },
        b: if b.b > 128 { 255 } else { 0 },
    }
}
fn thresh_128_i(_b: Color, i: Color) -> Color {
    Color {
        r: if i.r > 128 { 255 } else { 0 },
        g: if i.g > 128 { 255 } else { 0 },
        b: if i.b > 128 { 255 } else { 0 },
    }
}
fn thresh_avg(b: Color, i: Color) -> Color {
    Color {
        r: if (b.r as u16 + i.r as u16) > 255 {
            255
        } else {
            0
        },
        g: if (b.g as u16 + i.g as u16) > 255 {
            255
        } else {
            0
        },
        b: if (b.b as u16 + i.b as u16) > 255 {
            255
        } else {
            0
        },
    }
}
fn gray_avg_b(b: Color, _i: Color) -> Color {
    let gr = ((b.r as u16 + b.g as u16 + b.b as u16) / 3) as u8;
    Color {
        r: gr,
        g: gr,
        b: gr,
    }
}
fn gray_avg_i(_b: Color, i: Color) -> Color {
    let gr = ((i.r as u16 + i.g as u16 + i.b as u16) / 3) as u8;
    Color {
        r: gr,
        g: gr,
        b: gr,
    }
}
fn gray_bt709_b(b: Color, _i: Color) -> Color {
    let gr = ((b.r as u32 * 21 + b.g as u32 * 72 + b.b as u32 * 7) / 100) as u8;
    Color {
        r: gr,
        g: gr,
        b: gr,
    }
}
fn gray_bt709_i(_b: Color, i: Color) -> Color {
    let gr = ((i.r as u32 * 21 + i.g as u32 * 72 + i.b as u32 * 7) / 100) as u8;
    Color {
        r: gr,
        g: gr,
        b: gr,
    }
}
fn input_luma_to_base(b: Color, i: Color) -> Color {
    let luma = ((i.r as u16 + i.g as u16 + i.b as u16) / 3) as u8;
    Color {
        r: luma.saturating_add(b.r).min(255),
        g: luma.saturating_add(b.g).min(255),
        b: luma.saturating_add(b.b).min(255),
    }
}

// ==========================================
// 91-100
// ==========================================
fn amplify_diff(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add((b.r.wrapping_sub(i.r)).wrapping_mul(2)),
        g: b.g.wrapping_add((b.g.wrapping_sub(i.g)).wrapping_mul(2)),
        b: b.b.wrapping_add((b.b.wrapping_sub(i.b)).wrapping_mul(2)),
    }
}
fn dampen_diff(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(i.r.wrapping_sub(b.r) / 4),
        g: b.g.wrapping_add(i.g.wrapping_sub(b.g) / 4),
        b: b.b.wrapping_add(i.b.wrapping_sub(b.b) / 4),
    }
}
fn not_xor_half(b: Color, i: Color) -> Color {
    Color {
        r: !(b.r ^ (i.r >> 1)),
        g: !(b.g ^ (i.g >> 1)),
        b: !(b.b ^ (i.b >> 1)),
    }
}
fn rot_left_b(b: Color, _i: Color) -> Color {
    Color {
        r: b.r.rotate_left(1),
        g: b.g.rotate_left(1),
        b: b.b.rotate_left(1),
    }
}
fn rot_right_b(b: Color, _i: Color) -> Color {
    Color {
        r: b.r.rotate_right(1),
        g: b.g.rotate_right(1),
        b: b.b.rotate_right(1),
    }
}
fn extreme_contrast_b(b: Color, _i: Color) -> Color {
    let calc = |v: u8| -> u8 {
        if v < 128 {
            v.saturating_sub(64)
        } else {
            v.saturating_add(64)
        }
    };
    Color {
        r: calc(b.r),
        g: calc(b.g),
        b: calc(b.b),
    }
}
fn extreme_contrast_i(_b: Color, i: Color) -> Color {
    let calc = |v: u8| -> u8 {
        if v < 128 {
            v.saturating_sub(64)
        } else {
            v.saturating_add(64)
        }
    };
    Color {
        r: calc(i.r),
        g: calc(i.g),
        b: calc(i.b),
    }
}
fn dist_rg_add(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(b.g.abs_diff(i.g)),
        g: b.g.wrapping_add(b.b.abs_diff(i.b)),
        b: b.b.wrapping_add(b.r.abs_diff(i.r)),
    }
}
fn dist_gb_add(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(b.b.abs_diff(i.b)),
        g: b.g.wrapping_add(b.r.abs_diff(i.r)),
        b: b.b.wrapping_add(b.g.abs_diff(i.g)),
    }
}
fn dist_br_add(b: Color, i: Color) -> Color {
    Color {
        r: b.r.wrapping_add(b.r.abs_diff(i.r)),
        g: b.g.wrapping_add(b.g.abs_diff(i.g)),
        b: b.b.wrapping_add(b.b.abs_diff(i.b)),
    }
}
