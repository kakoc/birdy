use fastblur::gaussian_blur;

pub fn draw_rect_blurred(
    canvas: &mut [u8],
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    width: usize,
) {
    let (x0, x1) = if x0 > x1 { (x1, x0) } else { (x0, x1) };
    let (y0, y1) = if y0 > y1 { (y1, y0) } else { (y0, y1) };

    let blur_width = x1 - x0;
    let blur_height = y1 - y0;
    let radius = 5;
    if blur_width < radius || blur_height < radius {
        return;
    }

    let mut bluring_area = vec![];
    for hh in y0..y1 {
        for ww in x0..x1 {
            if (hh >= y0 && hh <= y1) && (ww >= x0 && ww <= x1) {
                let arr: [u8; 3] = [
                    canvas[hh * (width * 4) + (ww * 4)],
                    canvas[hh * (width * 4) + (ww * 4) + 1],
                    canvas[hh * (width * 4) + (ww * 4) + 2],
                ];
                bluring_area.push(arr);
            }
        }
    }

    gaussian_blur(&mut bluring_area, blur_width, blur_height, radius as f32);

    for hh in 0..blur_height {
        for ww in 0..blur_width {
            let [r, g, b] = bluring_area[hh * blur_width + ww];

            canvas[(hh + y0) * (width * 4) + ((ww + x0) * 4)] = r;
            canvas[(hh + y0) * (width * 4) + ((ww + x0) * 4) + 1] = g;
            canvas[(hh + y0) * (width * 4) + ((ww + x0) * 4) + 2] = b;
            canvas[(hh + y0) * (width * 4) + ((ww + x0) * 4) + 3] = 255;
        }
    }
}
