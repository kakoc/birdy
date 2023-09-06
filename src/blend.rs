pub fn blend(bg: (u8, u8, u8, u8), fg: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
    let a = fg.3 as f32 / bg.3 as f32;

    let result_red = (fg.0 as f32 * a) + (bg.0 as f32 * (1. - a));
    let result_green = (fg.1 as f32 * a) + (bg.1 as f32 * (1. - a));
    let result_blue = (fg.2 as f32 * a) + (bg.2 as f32 * (1. - a));
    let result_alpha = fg.3 as f32 + (bg.3 as f32 * (1. - fg.3 as f32 / bg.3 as f32));

    (
        result_red as u8,
        result_green as u8,
        result_blue as u8,
        result_alpha as u8,
    )
}
