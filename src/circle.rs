#[allow(dead_code)]
pub fn draw_circle_filled(
    canvas: &mut [u8],
    x: usize,
    y: usize,
    r: usize,
    width: usize,
    color: (u8, u8, u8, u8),
) {
    let red = color.0;
    let green = color.1;
    let blue = color.2;
    let alpha = color.3;

    let x0 = x - r;
    let x1 = x + r;
    let y0 = y - r;
    let y1 = y + r;

    for hh in y0..y1 {
        for ww in x0..x1 {
            let dy = (y as isize - hh as isize).unsigned_abs();
            let dx = (x as isize - ww as isize).unsigned_abs();

            if dx * dx + dy * dy <= r * r {
                canvas[hh * (width * 4) + (ww * 4)] = red;
                canvas[hh * (width * 4) + (ww * 4) + 1] = green;
                canvas[hh * (width * 4) + (ww * 4) + 2] = blue;
                canvas[hh * (width * 4) + (ww * 4) + 3] = alpha;
            }
        }
    }
}
