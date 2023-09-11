use crate::point::Point;

pub fn draw_triangle_filled(
    canvas: &mut [u8],
    a: (usize, usize),
    b: (usize, usize),
    c: (usize, usize),
    width: usize,
    color: (u8, u8, u8, u8),
) {
    let mut vs = [a, b, c];
    vs.sort_by(|a, b| a.1.cmp(&b.1));

    let total_height = vs[2].1 - vs[0].1;
    for i in 0..total_height {
        let second_half = i > vs[1].1 - vs[0].1 || vs[1].1 == vs[0].1;

        let segment_height = if second_half {
            vs[2].1 - vs[1].1
        } else {
            vs[1].1 - vs[0].1 + 1
        };

        let alpha = i as f64 / total_height as f64;

        let beta = (i as f64
            - (if second_half {
                vs[1].1 as f64 - vs[0].1 as f64
            } else {
                0 as f64
            }))
            / segment_height as f64;

        // TODO(kakoc): refactor ugly
        let a = {
            let _two = Point {
                x: vs[2].0 as i32,
                y: vs[2].1 as i32,
            };
            let _zero = Point {
                x: vs[0].0 as i32,
                y: vs[0].1 as i32,
            };
            let s = _two - _zero;
            Into::<Point<f64>>::into(_zero)
                + ((Point::<f64> {
                    x: s.x.into(),
                    y: s.y.into(),
                }) * alpha)
        };

        let b = {
            if second_half {
                let _two = Point {
                    x: vs[2].0 as i32,
                    y: vs[2].1 as i32,
                };
                let _one = Point {
                    x: vs[1].0 as i32,
                    y: vs[1].1 as i32,
                };
                let s = _two - _one;
                Into::<Point<f64>>::into(_one)
                    + ((Point::<f64> {
                        x: s.x.into(),
                        y: s.y.into(),
                    }) * beta)
            } else {
                let _one = Point {
                    x: vs[1].0 as i32,
                    y: vs[1].1 as i32,
                };
                let _zero = Point {
                    x: vs[0].0 as i32,
                    y: vs[0].1 as i32,
                };
                let s = _one - _zero;
                Into::<Point<f64>>::into(_zero)
                    + ((Point::<f64> {
                        x: s.x.into(),
                        y: s.y.into(),
                    }) * beta)
            }
        };

        let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };

        for j in a.x as usize..=b.x as usize {
            let red = color.0;
            let green = color.1;
            let blue = color.2;
            let alpha = color.3;

            canvas[(vs[0].1 + i) * (width * 4) + (j * 4)] = red;
            canvas[((vs[0].1 + i) * (width * 4) + (j * 4)) + 1] = green;
            canvas[((vs[0].1 + i) * (width * 4) + (j * 4)) + 2] = blue;
            canvas[((vs[0].1 + i) * (width * 4) + (j * 4)) + 3] = alpha;
        }
    }
}
