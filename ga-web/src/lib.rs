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
use rand::distributions::Uniform;
use wasm_bindgen::{prelude::*, JsCast};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
) -> Option<String> {
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
                size: Vec2::splat(600.0),
                background: Some(Color::black()),
                ints_only: true,
                preserve_height: false,
            });

            Some(to_data_uri(svg.as_bytes(), "image/svg+xml"))
        }
        3 => {
            let mut png = Vec::new();

            let encoder = PngEncoder::new_with_quality(
                &mut png,
                CompressionType::Best,
                FilterType::default(),
            );

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

            Some(to_data_uri(&png, "image/png"))
        }
        _ => None,
    }
}

fn to_data_uri(data: &[u8], file_type: &str) -> String {
    format!("data:{};base64,{}", file_type, base64::encode(data))
}
