use std::{
    fs::File,
    io::Write,
    num::NonZeroU32,
    time::{Duration, Instant},
};

use forma::{cpu, prelude::*};
use winit::{
    dpi::PhysicalSize,
    event::VirtualKeyCode,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

// use cpu;

// use crate::{App, Keyboard, Runner};

fn statistics(durations: &mut Vec<f64>) -> (f64, f64, f64) {
    let min = durations
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap();
    let max = durations
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap();
    let count = durations.len() as f64;
    (durations.drain(..).sum::<f64>() / count, min, max)
}

fn measure<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();

    f();

    start.elapsed()
}

#[derive(Debug)]
pub struct CpuRunner {
    composition: Composition,
    renderer: cpu::Renderer,
    pub buffer: Vec<u8>,
    layer_cache: BufferLayerCache,
    window: Window,
    layout: LinearLayout,
}

impl CpuRunner {
    pub fn new(event_loop: &EventLoop<()>, width: u32, height: u32) -> Self {
        let composition = Composition::new();
        let mut renderer = cpu::Renderer::new();
        let layer_cache = renderer.create_buffer_layer_cache().unwrap();

        let window = WindowBuilder::new()
            .with_title("demo | compose: ???ms, render: ???ms")
            .with_inner_size(PhysicalSize::new(width, height))
            .build(event_loop)
            .unwrap();

        let layout = LinearLayout::new(width as usize, width as usize * 4, height as usize);

        Self {
            composition,
            renderer,
            layer_cache,
            window,
            buffer: vec![0; (width * 4 * height) as usize],
            layout,
        }
    }
}

impl CpuRunner {
    fn resize(&mut self, width: u32, height: u32) {
        self.buffer.resize((width * 4 * height) as usize, 0);
        self.layout = LinearLayout::new(width as usize, width as usize * 4, height as usize);

        // self.config.width = width;
        // self.config.height = height;
        // self.surface.configure(&self.device, &self.config);
    }

    pub fn render(
        &mut self,
        // app: &mut dyn App,
        // buffer: Vec<u8>,
        // , elapsed: Duration, keyboard: &Keyboard
    ) {
        // if self.compose_durations.len() == 50 {
        //     let (compose_avg, compose_min, compose_max) = statistics(&mut self.compose_durations);
        //     let (render_avg, render_min, render_max) = statistics(&mut self.render_durations);

        //     self.window.set_title(&format!(
        //         "demo | compose: {:.2}ms ({:.2}/{:.2}), render: {:.2}ms ({:.2}/{:.2})",
        //         compose_avg, compose_min, compose_max, render_avg, render_min, render_max,
        //     ));
        // }

        // let compose_duration = measure(|| {
        // app.compose(&mut self.composition, elapsed, keyboard);
        // });
        let color = Color {
            r: 255.,
            g: 255.,
            b: 0.,
            a: 1.,
        };

        self.composition
            .get_mut_or_insert_default(Order::new(0 as u32).unwrap())
            .clear()
            .insert(&circle(
                100.0, 100.0,
                // rng.gen_range(0.0..App::width(self) as f32),
                // rng.gen_range(0.0..App::height(self) as f32),
                100.,
                // rng.gen_range(radius_range.clone()),
            ))
            // .insert(&line(
            //     (100.0, 100.0),
            //     (
            //         500.0,
            //         500.0, // App::width(self) as f32 - 10.,
            //               // App::height(self) as f32 - 10.,
            //     ),
            // ))
            // .set_props(Default::default())
            .set_props(Props {
                fill_rule: FillRule::NonZero,
                func: Func::Draw(Style {
                    fill: Fill::Solid(color),
                    ..Default::default()
                }),
            });

        // let render_duration = measure(|| {
        self.renderer.render(
            &mut self.composition,
            &mut BufferBuilder::new(&mut self.buffer, &mut self.layout)
                .layer_cache(self.layer_cache.clone())
                .build(),
            BGR1,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            None,
        );
        // });

        // self.compose_durations
        //     .push(compose_duration.as_secs_f64() * 1000.0);
        // self.render_durations
        //     .push(render_duration.as_secs_f64() * 1000.0);

        // if keyboard.is_key_down(VirtualKeyCode::S) {
        //     let mut bytes = Vec::with_capacity(self.layout.width() * self.layout.height() * 3);
        //     for pixel in self.buffer.chunks(4) {
        //         if let &[b, g, r, _] = pixel {
        //             bytes.push(r);
        //             bytes.push(g);
        //             bytes.push(b);
        //         }
        //     }
        //     let new_path = "capture.ppm";
        //     let mut output = File::options()
        //         .write(true)
        //         .create(true)
        //         .open(new_path)
        //         .unwrap();
        //     output
        //         .write_all(
        //             format!(
        //                 "P6\n{} {}\n255\n",
        //                 self.layout.width(),
        //                 self.layout.height()
        //             )
        //             .as_bytes(),
        //         )
        //         .unwrap();
        //     output.write_all(&bytes).unwrap();
        // }
    }
}

fn line(p0: (f32, f32), p1: (f32, f32)) -> Path {
    PathBuilder::new()
        .move_to(Point::new(p0.0, p0.1))
        .line_to(Point::new(p0.0 + 2., p0.1))
        .line_to(Point::new(p0.0 + 2., p0.1 + 2.))
        .line_to(Point::new(p0.0, p0.1 + 2.))
        .build()
}

fn circle(x: f32, y: f32, radius: f32) -> Path {
    let weight = 2.0f32.sqrt() / 2.0;

    let mut builder = PathBuilder::new();

    builder.move_to(Point::new(x + radius, y));
    builder.rat_quad_to(
        Point::new(x + radius, y - radius),
        Point::new(x, y - radius),
        weight,
    );
    builder.rat_quad_to(
        Point::new(x - radius, y - radius),
        Point::new(x - radius, y),
        weight,
    );
    builder.rat_quad_to(
        Point::new(x - radius, y + radius),
        Point::new(x, y + radius),
        weight,
    );
    builder.rat_quad_to(
        Point::new(x + radius, y + radius),
        Point::new(x + radius, y),
        weight,
    );

    builder.build()
}
