pub fn draw_rect_bordered(
    canvas: &mut [u8],
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    width: usize,
    color: (u8, u8, u8, u8),
) {
    // top
    draw_rect_filled(canvas, x0, y0, x1, y0 + 2, width, color);

    // right
    draw_rect_filled(canvas, x1.saturating_sub(2), y0, x1, y1, width, color);

    // bottom
    draw_rect_filled(canvas, x0, y1, x1, y1.saturating_sub(2), width, color);

    // left
    draw_rect_filled(canvas, x0, y0, x0 + 2, y1, width, color);
}

pub fn draw_rect_filled(
    canvas: &mut [u8],
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    width: usize,
    color: (u8, u8, u8, u8),
) {
    let red = color.0;
    let green = color.1;
    let blue = color.2;
    let alpha = color.3;

    let (x0, x1) = if x0 > x1 { (x1, x0) } else { (x0, x1) };
    let (y0, y1) = if y0 > y1 { (y1, y0) } else { (y0, y1) };

    for hh in y0..y1 {
        for ww in x0..x1 {
            if (hh >= y0 && hh <= y1) && (ww >= x0 && ww <= x1) {
                canvas[hh * (width * 4) + (ww * 4)] = red;
                canvas[hh * (width * 4) + (ww * 4) + 1] = green;
                canvas[hh * (width * 4) + (ww * 4) + 2] = blue;
                canvas[hh * (width * 4) + (ww * 4) + 3] = alpha;
            }
        }
    }
}
