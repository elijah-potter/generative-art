mod canvas_renderer;

use canvas_renderer::{CanvasRenderer, CanvasRendererSettings};
use generative_art::{
    denim::{
        renderers::{SkiaRenderer, SkiaRendererSettings, SvgRenderer, SvgRendererSettings},
        Color, Renderer, Stroke, UVec2, Vec2,
    },
    sketchers::{CelestialSketcher, CelestialSketcherSettings, Sketcher},
    OmniCanvas, VectorCanvas, VectorizerStyle,
};
use image::{
    codecs::png::PngEncoder,
    png::{CompressionType, FilterType},
};
use js_sys::Uint8Array;
use rand::distributions::Uniform;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_panics")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn celestial(
    object_count: usize,
    render_count: usize,
    min_object_size: f32,
    max_object_size: f32,
    g: f32,
    steps: usize,
    step_length: f32,
    zoom: f32,
    seed: u32,
    render_type: u8,
) -> Option<Uint8Array> {
    let settings = CelestialSketcherSettings {
        object_count,
        render_count,
        object_position: Uniform::new(-1.0, 1.0),
        object_size: Uniform::new_inclusive(min_object_size, max_object_size),
        object_velocity: Uniform::new_inclusive(0.0, 0.0),
        g,
        foreground: Color::white(),
        steps,
        step_length,
        render_polygon: None,
    };

    let sketcher = CelestialSketcher::new(settings, seed as u64);

    let mut canvas = sketcher
        .run_and_dispose(|_| ())
        .into_vector_canvas(VectorizerStyle::Pixels);

    canvas.zoom_camera(zoom);

    render(canvas, render_type)
}

fn render(canvas: VectorCanvas, render_type: u8) -> Option<Uint8Array> {
    match render_type {
        1 => {
            canvas.render::<CanvasRenderer>(CanvasRendererSettings {
                id: "canvas".into(),
                background: Some(Color::black()),
            });
            None
        }
        2 => {
            let svg = canvas.render::<SvgRenderer>(SvgRendererSettings {
                size: Vec2::splat(3000.0),
                background: Some(Color::black()),
                ints_only: true,
                preserve_height: false,
            });

            let output = Uint8Array::new(&JsValue::from_f64(svg.as_bytes().len() as f64));
            output.copy_from(svg.as_bytes());

            Some(output)
        }
        3 => {
            let mut png = Vec::new();

            let encoder = PngEncoder::new(&mut png);

            let image = canvas.render::<SkiaRenderer>(SkiaRendererSettings {
                size: UVec2::splat(3000),
                background: Some(Color::black()),
                antialias: true,
                preserve_height: false,
            });

            encoder
                .encode(
                    image.as_raw(),
                    image.width(),
                    image.height(),
                    image::ColorType::Rgba8,
                )
                .unwrap();

            let output = Uint8Array::new(&JsValue::from_f64(png.len() as f64));
            output.copy_from(&png);

            Some(output)
        }
        _ => None,
    }
}
