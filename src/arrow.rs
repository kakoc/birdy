use crate::line::draw_line;

pub fn draw_arrow(
    canvas: &mut [u8],
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    width: usize,
    color: (u8, u8, u8, u8),
) {
    if (x1 as isize - x0 as isize).abs() < 3 && (y1 as isize - y0 as isize).abs() < 3 {
        return;
    }

    let angle_in_deg = 30.0;
    let tip_length = 50.0;

    let (arrow_head_a, arrow_head_b) = calculate_arrow_head(
        (x0 as f64, y0 as f64),
        (x1 as f64, y1 as f64),
        angle_in_deg,
        tip_length,
    );

    if !arrow_head_a.0.is_nan() && !arrow_head_a.1.is_nan() {
        draw_line(
            canvas,
            arrow_head_a.0 as usize,
            arrow_head_a.1 as usize,
            x1,
            y1,
            width,
            color,
        );
    }

    if !arrow_head_b.0.is_nan() && !arrow_head_b.1.is_nan() {
        draw_line(
            canvas,
            arrow_head_b.0 as usize,
            arrow_head_b.1 as usize,
            x1,
            y1,
            width,
            color,
        );
    }

    draw_line(canvas, x0, y0, x1, y1, width, color);
}

pub fn calculate_arrow_head(
    start: (f64, f64),
    end: (f64, f64),
    angle_in_deg: f64,
    tip_length: f64,
) -> ((f64, f64), (f64, f64)) {
    let (x1, y1) = end;
    let (x2, y2) = start;
    let alpha = angle_in_deg.to_radians();
    let l1 = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    let l2 = tip_length;

    let a = (y2 - y1).powi(2) + (x2 - x1).powi(2);
    let b = -2.0 * l1 * l2 * alpha.cos() * (y2 - y1);
    let c = l1.powi(2) * l2.powi(2) * alpha.cos().powi(2) - l2.powi(2) * (x2 - x1).powi(2);

    let s2a = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let s2b = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let s1a = (l1 * l2 * alpha.cos() - s2a * (y2 - y1)) / (x2 - x1);
    let s1b = (l1 * l2 * alpha.cos() - s2b * (y2 - y1)) / (x2 - x1);

    let x3a = s1a + x1;
    let y3a = s2a + y1;
    let x3b = s1b + x1;
    let y3b = s2b + y1;

    ((x3a, y3a), (x3b, y3b))
}
