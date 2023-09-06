use crate::blend::blend;

pub fn draw_circle(
    canvas: &mut Vec<u8>,
    x: usize,
    y: usize,
    r: usize,
    width: usize,
    color: (usize, usize, usize, usize),
) {
    let red = (color.0) as u8;
    let green = (color.1) as u8;
    let blue = (color.2) as u8;
    let alpha = (color.3) as u8;

    let x0 = x - r;
    let x1 = x + r;
    let y0 = y - r;
    let y1 = y + r;

    for hh in y0..y1 {
        for ww in x0..x1 {
            let dy = (y as isize - hh as isize).abs() as usize;
            let dx = (x as isize - ww as isize).abs() as usize;

            if dx * dx + dy * dy <= r * r {
                let (red, green, blue, alpha) = blend(
                    (
                        canvas[hh * (width * 4) + (ww * 4)],
                        canvas[hh * (width * 4) + (ww * 4) + 1],
                        canvas[hh * (width * 4) + (ww * 4) + 2],
                        canvas[hh * (width * 4) + (ww * 4) + 3],
                    ),
                    (red, green, blue, alpha),
                );
                canvas[hh * (width * 4) + (ww * 4)] = red;
                canvas[hh * (width * 4) + (ww * 4) + 1] = green;
                canvas[hh * (width * 4) + (ww * 4) + 2] = blue;
                canvas[hh * (width * 4) + (ww * 4) + 3] = alpha;
            }
        }
    }
}
