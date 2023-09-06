pub fn draw_line(
    canvas: &mut Vec<u8>,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    width: usize,
    color: (u8, u8, u8, u8),
) {
    let red = (color.0) as u8;
    let green = (color.1) as u8;
    let blue = (color.2) as u8;
    let alpha = (color.3) as u8;

    let mut steep = false;

    let mut x0 = x0;
    let mut x1 = x1;

    let mut y0 = y0;
    let mut y1 = y1;

    if (x0 as isize - x1 as isize).abs() < (y0 as isize - y1 as isize).abs() {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }

    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    for x in x0..x1 {
        let t = (x as f64 - x0 as f64) / (x1 as f64 - x0 as f64);
        let y = (y0 as f64 * (1. - t) + y1 as f64 * t) - 1.;

        if steep {
            canvas[x * (width * 4) + (y as usize * 4)] = red;
            canvas[x * (width * 4) + (y as usize * 4) + 1] = green;
            canvas[x * (width * 4) + (y as usize * 4) + 2] = blue;
            canvas[x * (width * 4) + (y as usize * 4) + 3] = alpha;
        } else {
            canvas[y as usize * (width * 4) + (x * 4)] = red;
            canvas[y as usize * (width * 4) + (x * 4) + 1] = green;
            canvas[y as usize * (width * 4) + (x * 4) + 2] = blue;
            canvas[y as usize * (width * 4) + (x * 4) + 3] = alpha;
        }
    }
}
