use crate::line::draw_line;

pub fn draw_rect_borders(
    canvas: &mut Vec<u8>,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    width: usize,
    color: (u8, u8, u8, u8),
) {
    // top
    draw_line(canvas, x0, y0, x1, y0, width, color);

    // right
    draw_line(canvas, x1, y0, x1, y1, width, color);

    // bottom
    draw_line(canvas, x0, y1, x1, y1, width, color);

    // left
    draw_line(canvas, x0, y0, x0, y1, width, color);
}
