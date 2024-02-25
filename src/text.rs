use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    Font, Metrics,
};

use crate::{keycode_to_text::Cursor, rectangle::draw_rect_filled, Pos2};

pub fn draw_text(
    canvas: &mut [u8],
    x0: usize,
    y0: usize,
    canvas_width: usize,
    color: (u8, u8, u8, u8),
    layout: &Layout,
    fonts: &[Font],
) {
    for gl in layout.glyphs() {
        if gl.key.glyph_index == 0 || gl.key.glyph_index == 958 {
            // font.lookup_glyph_index('\n') == 0
            // font.lookup_glyph_index(' ') == 958
            continue;
        }

        let (metrics, rgb_bitmap) = get_rasterized_glyph(
            &fonts[0],
            gl.key.glyph_index,
            24,
            (
                color.0 as usize,
                color.1 as usize,
                color.2 as usize,
                color.3 as usize,
            ),
            (0, 0, 0, 255),
        );

        let start = (x0, y0);
        if gl.x < 1920. {
            for hh in 0..metrics.height {
                if canvas
                    .get(
                        // (start.1 + hh + gl.y as usize) * canvas_width
                        ((hh + gl.y as usize) - start.1) * canvas_width
                                // + start.0
                                + metrics.width
                                + gl.x as usize
                            - 1
                            - start.0,
                    )
                    .is_some()
                {
                    canvas[(hh + gl.y as usize) * (canvas_width * 4) + (gl.x as usize * 4)
                        ..(hh + gl.y as usize) * (canvas_width * 4)
                            + (metrics.width * 4)
                            + (gl.x as usize * 4)]
                        .copy_from_slice(
                            &rgb_bitmap[hh * (metrics.width * 4)
                                ..hh * (metrics.width * 4) + (metrics.width * 4)],
                        );
                }
            }
        }
    }
}

pub fn init_layout(size_px: f32, content: &str, x: f32, y: f32) -> (Layout, Vec<Font>) {
    let settings = fontdue::FontSettings {
        scale: size_px,
        ..fontdue::FontSettings::default()
    };
    let font = fontdue::Font::from_bytes(
        include_bytes!("../JetBrainsMono-Regular.ttf") as &[u8],
        settings,
    )
    .unwrap();
    let fonts = vec![font.clone()];

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        x,
        y,
        ..LayoutSettings::default()
    });
    layout.append(&[font.clone()], &TextStyle::new(content, size_px, 0));

    (layout, fonts)
}

pub fn get_rasterized_glyph(
    font: &Font,
    glyph_index: u16,
    size_px: usize,
    color: (usize, usize, usize, usize),
    bg_color: (usize, usize, usize, usize),
) -> (Metrics, Vec<u8>) {
    let (metrics, bitmap) = font.rasterize_indexed_subpixel(glyph_index, size_px as f32);

    let rgb_bitmap: Vec<u8> = bitmap
        .chunks(3)
        .flat_map(|chunk| {
            let bg_blue = chunk[2] as u32;
            let bg_green = chunk[1] as u32;
            let bg_red = chunk[0] as u32;

            convert_rgba((bg_red, bg_green, bg_blue, 255), bg_color, color)
        })
        .collect();

    (metrics, rgb_bitmap)
}

fn convert_rgba(
    current: (u32, u32, u32, u32),
    bg: (usize, usize, usize, usize),
    fg: (usize, usize, usize, usize),
) -> Vec<u8> {
    let new_r = ((current.0 as f64 / 255.0) * fg.0 as f64
        + (1.0 - current.0 as f64 / 255.0) * bg.0 as f64) as u8;

    let new_g = ((current.1 as f64 / 255.0) * fg.1 as f64
        + (1.0 - current.1 as f64 / 255.0) * bg.1 as f64) as u8;

    let new_b = ((current.2 as f64 / 255.0) * fg.2 as f64
        + (1.0 - current.2 as f64 / 255.0) * bg.2 as f64) as u8;

    let alpha = 255;

    vec![new_r, new_g, new_b, alpha]
}

pub fn draw_cursor(
    canvas: &mut [u8],
    canvas_width: usize,
    cursor: &mut Cursor,
    layout: &Layout,
    content: &str,
    start: Pos2,
    cursor_color: (u8, u8, u8, u8),
) {
    let mut skipped_lines_chars = 0;
    let mut skip_chars = 0;
    for (i, ch) in content.chars().enumerate() {
        if ch == '\n' {
            skipped_lines_chars += 1;
        }
        if cursor.global_lines_offset_from_beginning != 0
            && cursor.global_lines_offset_from_beginning == skipped_lines_chars
        {
            skip_chars = i + 1;
            break;
        }
    }
    let cursor_position = cursor
        .global_chars_offset_from_beginning
        .saturating_sub(skip_chars);
    let cursor_offset = update_cusror_position_text(layout, cursor_position, (start.0, start.1));
    cursor.px_coordinates = cursor_offset;

    draw_rect_filled(
        canvas,
        cursor.px_coordinates.0,
        cursor.px_coordinates.1,
        cursor.px_coordinates.0 + 2,
        cursor.px_coordinates.1 + 30,
        canvas_width,
        cursor_color,
    );
}

pub fn update_cusror_position_text(layout: &Layout, cursor_position: usize, start: Pos2) -> Pos2 {
    let gl = layout
        .glyphs()
        .get(cursor_position.saturating_sub(1))
        .copied();
    let cursor_offset = if let (Some(gl), true) = (gl, cursor_position != 0) {
        (
            std::cmp::max(
                gl.x as usize + (if gl.parent != ' ' { gl.width } else { 15 }),
                start.0,
            ),
            (gl.y as usize) - layout.glyphs().last().unwrap().y as usize + start.1,
        )
    } else {
        (start.0, start.1)
    };

    cursor_offset
}
