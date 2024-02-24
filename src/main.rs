#![forbid(unsafe_code)]

use std::{
    path::{Path, PathBuf},
    str::FromStr,
    thread::sleep,
    time::{Duration, SystemTime},
};

#[cfg(target_os = "linux")]
use arboard::{Clipboard, ImageData};
use arrow::{draw_arrow_bordered, draw_arrow_filled};
use blur::draw_rect_blurred;
use chrono::{DateTime, Utc};
use clap::Parser;
use error_iter::ErrorIter as _;
use image::ColorType;
use keycode_to_text::{handle_key_press, Cursor};
use line::draw_line;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rectangle::{draw_rect_bordered, draw_rect_filled};
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use text::{draw_cursor, draw_text};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{CursorIcon, Fullscreen, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

const BORDER_WIDTH: usize = 2;

mod arrow;
mod blend;
mod blur;
mod circle;
mod keycode_to_text;
mod line;
mod point;
mod rectangle;
mod text;
mod triangle;

#[derive(Clone, Copy, Debug)]
struct BorderColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for BorderColor {
    fn default() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }
    }
}

impl From<BorderColor> for (u8, u8, u8, u8) {
    fn from(value: BorderColor) -> Self {
        (value.r, value.g, value.b, value.a)
    }
}

impl FromStr for BorderColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Result<Vec<u8>, _> = s.split(',').map(|s| s.parse()).collect();

        match split {
            Ok(v) => {
                if v.len() != 4 {
                    Err("Incorrect number of u8 values.".to_string())
                } else {
                    Ok(Self {
                        r: v[0],
                        g: v[1],
                        b: v[2],
                        a: v[3],
                    })
                }
            }
            Err(e) => Err(format!("Bad value: {e}")),
        }
    }
}

///Hotkeys while running (see lower for cli args):
///
///  Enter - take a screenshot of selected area, save to a clipboard and exit
///
///  f - take a screenshot where selected area is focused, save to a clipboard and exit
///
///  a - draw an arrow
///
///  z - draw a filled arrow
///
///  l - draw a line
///
///  r - draw a rectangular border
///
///  p - draw a filled rectangle
///
///  b - draw a blurred rectangle
///
///  t - draw a text
///
///  Tab - toggle latest drawn shape between filled/not filled states
///
///  Esc - exit
#[derive(Parser)]
struct BirdyArgs {
    #[arg(short, long)]
    border_color: Option<BorderColor>,
    #[arg(short, long)]
    screen: Option<usize>,
    /// save directory
    #[arg(short, long)]
    dir: Option<PathBuf>,
    /// save to clipboard instead of path
    #[arg(short, long)]
    clipboard: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let BirdyArgs {
        border_color,
        screen,
        dir,
        clipboard,
    } = BirdyArgs::parse();

    let clipboard = match (clipboard, &dir) {
        (true, Some(_)) => {
            println!("Provided save dir not used when saving to clipboard.");
            true
        }
        (false, None) => {
            println!("No save options provided, defaulting to clipboard save.");
            true
        }
        (c, _) => c,
    };

    let screens = Screen::all()?;
    let original_screenshot = if let Some(screen) = screens.get(screen.unwrap_or(0)) {
        let image = screen.capture()?;
        image.to_vec()
    } else {
        panic!("can't find an available screen for a screenshot");
    };

    env_logger::init();
    let mut event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Hello Pixels")
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_maximized(true)
        .build(&event_loop)?;

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    let mut screenshot = Screenshot::new(
        original_screenshot,
        window.inner_size().width as usize,
        window.inner_size().height as usize,
        border_color.unwrap_or_default(),
        dir,
        clipboard,
    );

    let ret_code = event_loop.run_return(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            screenshot.draw(pixels.frame_mut());

            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, .. },
                ..
            } => {
                if let ElementState::Pressed = state {
                    screenshot.on_mouse_pressed();
                } else {
                    screenshot.on_mouse_released();
                    window.set_cursor_icon(CursorIcon::Default);
                }

                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                screenshot.on_mouse_move(position);

                let cursor = match screenshot.what_resize_opt() {
                    BoundaryResize::Top => CursorIcon::NResize,
                    BoundaryResize::TopLeft => CursorIcon::NwResize,
                    BoundaryResize::TopRight => CursorIcon::NeResize,
                    BoundaryResize::Right => CursorIcon::EResize,
                    BoundaryResize::Bottom => CursorIcon::SResize,
                    BoundaryResize::BottomLeft => CursorIcon::SwResize,
                    BoundaryResize::BottomRight => CursorIcon::SeResize,
                    BoundaryResize::Left => CursorIcon::WResize,
                    _ => CursorIcon::Default,
                };
                window.set_cursor_icon(cursor);
            }

            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            event @ KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Some(DrawMode::Text) = &screenshot.draw_mode {
                    screenshot.handle_input_text_keypress(event);
                } else {
                    if let Some(VirtualKeyCode::Return) = virtual_keycode {
                        screenshot.save_image(screenshot.get_cropped_image());
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    if let Some(VirtualKeyCode::F) = virtual_keycode {
                        screenshot.save_image(screenshot.get_full_image());
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if let Some(VirtualKeyCode::A) = virtual_keycode {
                        screenshot.draw_mode = Some(DrawMode::Arrow);
                    }
                    if let Some(VirtualKeyCode::Z) = virtual_keycode {
                        screenshot.draw_mode = Some(DrawMode::ArrowFilled);
                    }
                    if let Some(VirtualKeyCode::L) = virtual_keycode {
                        screenshot.draw_mode = Some(DrawMode::Line);
                    }
                    if let Some(VirtualKeyCode::R) = virtual_keycode {
                        screenshot.draw_mode = Some(DrawMode::RectBorder);
                    }
                    if let Some(VirtualKeyCode::P) = virtual_keycode {
                        screenshot.draw_mode = Some(DrawMode::RectFilled);
                    }
                    if let Some(VirtualKeyCode::B) = virtual_keycode {
                        screenshot.draw_mode = Some(DrawMode::RectBlurred);
                    }
                    if let Some(VirtualKeyCode::T) = virtual_keycode {
                        screenshot.draw_mode = Some(DrawMode::Text);
                    }
                    if let Some(VirtualKeyCode::Tab) = virtual_keycode {
                        screenshot.toggle_filling_latest();
                    }
                }

                if screenshot.draw_mode.is_some() {
                    window.set_cursor_icon(CursorIcon::Crosshair);
                }

                window.request_redraw();
            }

            _ => {}
        }

        // Handle input events
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                if let Err(err) = pixels.resize_buffer(size.width, size.height) {
                    log_error("pixels.resize_buffer", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                };
                screenshot.resize_viewport(size.width as usize, size.height as usize);
            }

            window.request_redraw();
        }
    });

    if ret_code != 0 {
        return Err(format!(
            "Non-zero return code in event loop ({ret_code}), display server exit?"
        )
        .into());
    }

    #[cfg(target_os = "linux")]
    {
        if clipboard {
            drop(event_loop); // closes overlay but generates "queue destroyed while proxies still attached",
                              // not ideal?
            println!("Hanging around for a minute so that clipboard contents persist.");

            sleep(Duration::from_secs(60));
        }
    }

    Ok(())
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

pub type Pos2 = (usize, usize);

struct Screenshot {
    original_screenshot: Vec<u8>,
    modified_screenshot: Vec<u8>,
    p0: Pos2,
    p1: Pos2,
    width: usize,
    height: usize,
    save_dir: Option<PathBuf>,
    use_clipboard: bool,

    boundary_resize_on_press: BoundaryResize,
    draw_mode: Option<DrawMode>,
    drawing_item: Option<DrawnItem>,
    drawn_items: Vec<DrawnItem>,
    border_color: BorderColor,

    mouse_coordinates: Option<PhysicalPosition<f64>>,
}

impl Screenshot {
    fn new(
        screenshot: Vec<u8>,
        width: usize,
        height: usize,
        border_color: BorderColor,
        save_dir: Option<PathBuf>,
        use_clipboard: bool,
    ) -> Self {
        Self {
            original_screenshot: screenshot.clone(),
            modified_screenshot: screenshot,
            save_dir,
            use_clipboard,

            boundary_resize_on_press: BoundaryResize::None,
            draw_mode: None,
            drawing_item: None,
            drawn_items: vec![],
            border_color,

            p0: (0, 0),
            p1: (width, height),
            width,
            height,
            mouse_coordinates: None,
        }
    }

    pub fn resize_viewport(&mut self, width: usize, height: usize) {
        *self = Self::new(
            self.original_screenshot.clone(),
            width,
            height,
            self.border_color,
            self.save_dir.clone(),
            self.use_clipboard,
        );
    }

    fn get_full_image(&self) -> Image {
        Image {
            width: self.width,
            height: self.height,
            bytes: self.modified_screenshot.clone(),
        }
    }

    fn get_cropped_image(&self) -> Image {
        let ymin = self.p0.1 + 1 + (BORDER_WIDTH / 2);
        let ymax = self.p1.1 - 1 - (BORDER_WIDTH / 2);
        let xmin = self.p0.0 + 1 + (BORDER_WIDTH / 2);
        let xmax = self.p1.0 - 1 - (BORDER_WIDTH / 2);
        let height = ymax - ymin;
        let width = xmax - xmin;
        let mut bytes = Vec::with_capacity(height * width * 4);
        for y in ymin..ymax {
            for x in xmin..xmax {
                bytes.push(self.modified_screenshot[y * (self.width * 4) + (x * 4)]);
                bytes.push(self.modified_screenshot[y * (self.width * 4) + (x * 4) + 1]);
                bytes.push(self.modified_screenshot[y * (self.width * 4) + (x * 4) + 2]);
                bytes.push(self.modified_screenshot[y * (self.width * 4) + (x * 4) + 3]);
            }
        }

        Image {
            width,
            height,
            bytes,
        }
    }

    pub fn save_image(&self, image: Image) {
        if self.use_clipboard {
            let mut ctx = Clipboard::new().unwrap();

            let img_data = ImageData {
                width: image.width,
                height: image.height,
                bytes: image.bytes.clone().into(),
            };
            ctx.set_image(img_data).unwrap();
        } else {
            let now: DateTime<Utc> = SystemTime::now().into();
            let now_str = now.format("%Y-%m-%d_%H:%M:%S").to_string();
            let name = format!("birdy_{now_str}.png");

            let fpath = self
                .save_dir
                .as_ref()
                .unwrap()
                .join(Path::new(name.as_str()));

            image::save_buffer(
                fpath,
                image.bytes.as_slice(),
                image.width as u32,
                image.height as u32,
                ColorType::Rgba8,
            )
            .expect("Failed to save image.");
        }
    }

    fn draw(&mut self, pixels: &mut [u8]) {
        self.modified_screenshot = self.original_screenshot.clone();
        self.draw_boundaries();
        self.darken_not_selected_area();

        for draw_item in self.drawn_items.clone() {
            self.draw_draw_item(&draw_item);
        }

        if let Some(drawing_item) = &self.drawing_item {
            self.draw_draw_item(&drawing_item.clone());
        }

        if pixels.len() == self.modified_screenshot.len() {
            pixels.copy_from_slice(&self.modified_screenshot);
        }
    }

    fn draw_draw_item(&mut self, draw_item: &DrawnItem) {
        match &draw_item {
            DrawnItem::Arrow((x0, y0), (x1, y1)) => {
                draw_arrow_bordered(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    *x1,
                    *y1,
                    self.width,
                    self.border_color.into(),
                );
            }
            DrawnItem::ArrowFilled((x0, y0), (x1, y1)) => {
                draw_arrow_filled(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    *x1,
                    *y1,
                    self.width,
                    self.border_color.into(),
                );
            }
            DrawnItem::Line((x0, y0), (x1, y1)) => {
                draw_line(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    *x1,
                    *y1,
                    self.width,
                    self.border_color.into(),
                );
            }
            DrawnItem::RectBorder((x0, y0), (x1, y1)) => {
                draw_rect_bordered(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    *x1,
                    *y1,
                    self.width,
                    self.border_color.into(),
                );
            }
            DrawnItem::RectBlurred((x0, y0), (x1, y1)) => {
                draw_rect_blurred(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    *x1,
                    *y1,
                    self.width,
                );
            }
            DrawnItem::Text((mut cursor, ref content, (x0, y0))) => {
                let layout = draw_text(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    self.width,
                    self.border_color.into(),
                    content,
                );
                if let (Some(first), Some(last)) = (layout.glyphs().first(), layout.glyphs().last())
                {
                    draw_rect_filled(
                        &mut self.modified_screenshot,
                        (first.x as usize).saturating_sub(5),
                        (first.y as usize).saturating_sub(5),
                        (last.x as usize + last.width) + 5,
                        (last.y as usize + last.height) + 5,
                        self.width,
                        (0, 0, 0, 255),
                    );
                }
                let layout = draw_text(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    self.width,
                    self.border_color.into(),
                    content,
                );
                if self.drawing_item.as_ref() == Some(draw_item) {
                    draw_cursor(
                        &mut self.modified_screenshot,
                        self.width,
                        &mut cursor,
                        &layout,
                        content,
                        (*x0, *y0),
                        self.border_color.into(),
                    );
                }
            }
            DrawnItem::RectFilled((x0, y0), (x1, y1)) => {
                draw_rect_filled(
                    &mut self.modified_screenshot,
                    *x0,
                    *y0,
                    *x1,
                    *y1,
                    self.width,
                    self.border_color.into(),
                );
            }
        }
    }

    fn draw_boundaries(&mut self) {
        draw_rect_filled(
            &mut self.modified_screenshot,
            self.p0.0,
            self.p0.1,
            self.p1.0,
            self.p0.1 + BORDER_WIDTH,
            self.width,
            self.border_color.into(),
        );
        draw_rect_filled(
            &mut self.modified_screenshot,
            self.p1.0 - BORDER_WIDTH,
            self.p0.1,
            self.p1.0,
            self.p1.1,
            self.width,
            self.border_color.into(),
        );
        draw_rect_filled(
            &mut self.modified_screenshot,
            self.p0.0,
            self.p1.1 - BORDER_WIDTH,
            self.p1.0,
            self.p1.1,
            self.width,
            self.border_color.into(),
        );
        draw_rect_filled(
            &mut self.modified_screenshot,
            self.p0.0,
            self.p0.1,
            self.p0.0 + BORDER_WIDTH,
            self.p1.1,
            self.width,
            self.border_color.into(),
        );
    }

    fn darken_not_selected_area(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if x < self.p0.0 || x > self.p1.0 || y < self.p0.1 || y > self.p1.1 {
                    self.modified_screenshot[y * (self.width * 4) + (x * 4) + 3] = 100;
                }
            }
        }
    }

    pub fn toggle_filling_latest(&mut self) {
        if let Some(item) = self.drawn_items.pop() {
            let filled_item = self.toggle_item_filling(&item);
            self.drawn_items.push(filled_item);
        }
    }

    pub fn toggle_item_filling(&mut self, draw_item: &DrawnItem) -> DrawnItem {
        match draw_item {
            DrawnItem::Arrow(p0, p1) => DrawnItem::ArrowFilled(*p0, *p1),
            DrawnItem::ArrowFilled(p0, p1) => DrawnItem::Arrow(*p0, *p1),
            DrawnItem::RectBorder(p0, p1) => DrawnItem::RectFilled(*p0, *p1),
            DrawnItem::RectFilled(p0, p1) | DrawnItem::RectBlurred(p0, p1) => {
                DrawnItem::RectBorder(*p0, *p1)
            }
            DrawnItem::Line(..) | DrawnItem::Text(..) => draw_item.clone(),
        }
    }

    pub fn handle_input_text_keypress(&mut self, event: KeyboardInput) {
        match event.virtual_keycode {
            Some(VirtualKeyCode::Escape | VirtualKeyCode::Return) => {
                if let Some(DrawnItem::Text((cursor, ref mut content, p0))) = &mut self.drawing_item
                {
                    self.drawn_items
                        .push(DrawnItem::Text((*cursor, content.clone(), *p0)));
                    self.drawing_item = None;
                }

                self.draw_mode = None;
            }
            _ => {
                if let Some(DrawnItem::Text((ref mut cursor, ref mut content, _))) =
                    &mut self.drawing_item
                {
                    handle_key_press(content, event, cursor);
                }
            }
        };
    }

    pub fn on_mouse_move(&mut self, coordinates: PhysicalPosition<f64>) {
        self.mouse_coordinates = Some(coordinates);
        let PhysicalPosition { x, y } = coordinates;

        match self.boundary_resize_on_press {
            BoundaryResize::None => match (&self.draw_mode, &mut self.drawing_item) {
                (Some(DrawMode::Arrow), Some(DrawnItem::Arrow(_, p1))) => {
                    *p1 = (x as usize, y as usize);
                }
                (Some(DrawMode::ArrowFilled), Some(DrawnItem::ArrowFilled(_, p1))) => {
                    *p1 = (x as usize, y as usize);
                }
                (Some(DrawMode::Line), Some(DrawnItem::Line(_, p1))) => {
                    *p1 = (x as usize, y as usize);
                }
                (Some(DrawMode::RectBorder), Some(DrawnItem::RectBorder(_, p1))) => {
                    *p1 = (x as usize, y as usize);
                }
                (Some(DrawMode::RectFilled), Some(DrawnItem::RectFilled(_, p1))) => {
                    *p1 = (x as usize, y as usize);
                }
                (Some(DrawMode::RectBlurred), Some(DrawnItem::RectBlurred(_, p1))) => {
                    *p1 = (x as usize, y as usize);
                }
                _ => {}
            },
            BoundaryResize::Top => {
                self.p0.1 = y as usize;
            }
            BoundaryResize::TopLeft => {
                self.p0.1 = y as usize;
                self.p0.0 = x as usize;
            }
            BoundaryResize::TopRight => {
                self.p0.1 = y as usize;
                self.p1.0 = x as usize;
            }
            BoundaryResize::Right => {
                self.p1.0 = x as usize;
            }
            BoundaryResize::Bottom => {
                self.p1.1 = y as usize;
            }
            BoundaryResize::BottomLeft => {
                self.p1.1 = y as usize;
                self.p0.0 = x as usize;
            }
            BoundaryResize::BottomRight => {
                self.p1.1 = y as usize;
                self.p1.0 = x as usize;
            }
            BoundaryResize::Left => {
                self.p0.0 = x as usize;
            }
        }
    }

    pub fn what_resize_opt(&self) -> BoundaryResize {
        if let Some(PhysicalPosition { x, y }) = self.mouse_coordinates {
            let x = x as usize;
            let y = y as usize;

            // top left resize
            if x > self.p0.0 && x < self.p0.0 + 20 && y > self.p0.1 && y < self.p0.1 + 20 {
                BoundaryResize::TopLeft
            // top right resize
            } else if x < self.p1.0 && x > self.p1.0 - 20 && y > self.p0.1 && y < self.p0.1 + 20 {
                BoundaryResize::TopRight
            }
            // top resize
            else if x > self.p0.0
                && x < self.p1.0
                && y >= self.p0.1.saturating_sub(10)
                && y <= self.p0.1 + 10
            {
                BoundaryResize::Top
            // right resize
            } else if y > self.p0.1
                && y < self.p1.1
                && x >= self.p1.0.saturating_sub(10)
                && x <= self.p1.0 + 10
            {
                BoundaryResize::Right
            }
            // bottom left resize
            else if x > self.p0.0
                && x < self.p0.0 + 20
                && y > self.p1.1.saturating_sub(20)
                && y < self.p1.1
            {
                BoundaryResize::BottomLeft
            // bottom right resize
            } else if x < self.p1.0
                && x > self.p1.0 - 20
                && y > self.p1.1.saturating_sub(20)
                && y < self.p1.1
            {
                BoundaryResize::BottomRight
            }
            // bottom resize
            else if x > self.p0.0
                && x < self.p1.0
                && y >= self.p1.1.saturating_sub(10)
                && y <= self.p1.1 + 10
            {
                BoundaryResize::Bottom
            }
            // left resize
            else if y > self.p0.1
                && y < self.p1.1
                && x >= self.p0.0.saturating_sub(10)
                && x <= self.p0.0 + 10
            {
                BoundaryResize::Left
            } else {
                BoundaryResize::None
            }
        } else {
            BoundaryResize::None
        }
    }

    pub fn on_mouse_pressed(&mut self) {
        if let Some(PhysicalPosition { x, y }) = self.mouse_coordinates {
            let x = x as usize;
            let y = y as usize;

            self.boundary_resize_on_press = self.what_resize_opt();
            if let BoundaryResize::None = self.boundary_resize_on_press {
                match self.draw_mode {
                    Some(DrawMode::Arrow) => {
                        self.drawing_item = Some(DrawnItem::Arrow((x, y), (x, y)));
                    }
                    Some(DrawMode::ArrowFilled) => {
                        self.drawing_item = Some(DrawnItem::ArrowFilled((x, y), (x, y)));
                    }
                    Some(DrawMode::Line) => {
                        self.drawing_item = Some(DrawnItem::Line((x, y), (x, y)));
                    }
                    Some(DrawMode::RectBorder) => {
                        self.drawing_item = Some(DrawnItem::RectBorder((x, y), (x, y)));
                    }
                    Some(DrawMode::RectBlurred) => {
                        self.drawing_item = Some(DrawnItem::RectBlurred((x, y), (x, y)));
                    }
                    Some(DrawMode::Text) => {
                        self.drawing_item = Some(DrawnItem::Text((
                            Default::default(),
                            "".to_string(),
                            (x, y),
                        )));
                    }
                    Some(DrawMode::RectFilled) => {
                        self.drawing_item = Some(DrawnItem::RectFilled((x, y), (x, y)));
                    }
                    None => {}
                };
            }
        }
    }

    pub fn on_mouse_released(&mut self) {
        self.boundary_resize_on_press = BoundaryResize::None;

        if let (Some(item), Some(PhysicalPosition { x, y })) =
            (&self.drawing_item, self.mouse_coordinates)
        {
            let (x, y) = (x as usize, y as usize);
            match (&self.draw_mode, item) {
                (Some(DrawMode::Arrow), DrawnItem::Arrow(p0, _)) => {
                    self.drawn_items.push(DrawnItem::Arrow(*p0, (x, y)));
                    self.drawing_item = None;
                    self.draw_mode = None;
                }
                (Some(DrawMode::ArrowFilled), DrawnItem::ArrowFilled(p0, _)) => {
                    self.drawn_items.push(DrawnItem::ArrowFilled(*p0, (x, y)));
                    self.drawing_item = None;
                    self.draw_mode = None;
                }
                (Some(DrawMode::Line), DrawnItem::Line(p0, _)) => {
                    self.drawn_items.push(DrawnItem::Line(*p0, (x, y)));
                    self.drawing_item = None;
                    self.draw_mode = None;
                }
                (Some(DrawMode::RectBorder), DrawnItem::RectBorder(p0, _)) => {
                    self.drawn_items.push(DrawnItem::RectBorder(*p0, (x, y)));
                    self.drawing_item = None;
                    self.draw_mode = None;
                }
                (Some(DrawMode::RectFilled), DrawnItem::RectFilled(p0, _)) => {
                    self.drawn_items.push(DrawnItem::RectFilled(*p0, (x, y)));
                    self.drawing_item = None;
                    self.draw_mode = None;
                }
                (Some(DrawMode::RectBlurred), DrawnItem::RectBlurred(p0, _)) => {
                    self.drawn_items.push(DrawnItem::RectBlurred(*p0, (x, y)));
                    self.drawing_item = None;
                    self.draw_mode = None;
                }
                (Some(DrawMode::Text), DrawnItem::Text(..)) => {}
                _ => {
                    self.draw_mode = None;
                }
            }
        }
    }
}

enum DrawMode {
    Arrow,
    ArrowFilled,
    Line,
    RectBorder,
    RectFilled,
    RectBlurred,
    Text,
}

#[derive(Clone, PartialEq, Eq)]
enum DrawnItem {
    Arrow(Pos2, Pos2),
    ArrowFilled(Pos2, Pos2),
    Line(Pos2, Pos2),
    RectBorder(Pos2, Pos2),
    RectFilled(Pos2, Pos2),
    RectBlurred(Pos2, Pos2),
    Text((Cursor, String, Pos2)),
}

#[derive(PartialEq)]
enum BoundaryResize {
    None,
    Top,
    TopLeft,
    TopRight,
    Right,
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
}
