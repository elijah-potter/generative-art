mod canvas_renderer;

use canvas_renderer::CanvasRenderer;
use generative_art::{
    barium::{
        renderers::{SkiaRenderer, SvgRenderer},
        Color, LineEnd, Stroke, UVec2,
    },
    sketchers::{
        CelestialSketcher, CelestialSketcherSettings, HalftoneSketcher, HalftoneSketcherSettings,
        PreslavSketcher, PreslavSketcherSettings, Sketcher, WaveSketcher, WaveSketcherSettings,
    },
    RasterCanvas, VectorCanvas, VectorizerStyle,
};
use image::{codecs::png::PngEncoder, ImageFormat};
use js_sys::Uint8Array;
use rand::distributions::Uniform;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_panics")]
    console_error_panic_hook::set_once();
}

static mut LOADED_IMAGE: Option<RasterCanvas> = None;

#[wasm_bindgen]
pub fn load_image(image: &[u8], file_ext: &str) -> usize {
    let image_format = match file_ext {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "tiff" => ImageFormat::Tiff,
        "bmp" => ImageFormat::Bmp,
        _ => panic!("Unexpected file extension."),
    };

    let image = image::load_from_memory_with_format(image, image_format).unwrap();
    let image = RasterCanvas::from_rgba(&image.to_rgba8());

    let image_width = image.width();

    unsafe {
        LOADED_IMAGE = Some(image);
    }

    image_width
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

    render(
        canvas,
        Some(Color::black()),
        UVec2::splat(3000),
        render_type,
    )
}

#[wasm_bindgen]
pub fn waves(
    stroke_color: &str,
    background_color: &str,
    stroke_width: f32,
    skip_rows: usize,
    skip_columns: usize,
    frequency_multiplier: f32,
    amplitude_multiplier: f32,
    invert_brightness: bool,
    brightness_threshold: f32,
    box_blur_radius: usize,
    stroke_with_frequency: bool,
    render_type: u8,
) -> Option<Uint8Array> {
    let settings = WaveSketcherSettings {
        stroke: Stroke {
            color: Color::from_hex(&stroke_color).unwrap(),
            width: stroke_width,
            line_end: LineEnd::Round,
        },
        skip_rows,
        skip_columns,
        frequency_multiplier,
        amplitude_multiplier,
        invert_brightness,
        brightness_threshold,
        box_blur_radius,
        stroke_with_frequency,
    };

    let image = unsafe { LOADED_IMAGE.clone().unwrap() };

    let size = UVec2::new(
        (image.width() as f32 / image.height() as f32 * 3000.0) as u32,
        3000,
    );

    let sketcher = WaveSketcher::new(image, settings);

    let canvas = sketcher
        .run_and_dispose(|_| ())
        .into_vector_canvas(VectorizerStyle::Pixels);

    render(
        canvas,
        Some(Color::from_hex(background_color).unwrap()),
        size,
        render_type,
    )
}

#[wasm_bindgen]
pub fn preslav(
    stroke_jitter: f32,
    stroke_inversion_threshold: f32,
    alpha: f32,
    alpha_increase: f32,
    min_edge_count: usize,
    max_edge_count: usize,
    stroke_size: f32,
    stroke_reduction: f32,
    randomize_rotation: f32,
    shapes: usize,
    seed: u32,
    render_type: u8,
) -> Option<Uint8Array> {
    let settings = PreslavSketcherSettings {
        stroke_jitter,
        stroke_inversion_threshold,
        alpha,
        alpha_increase,
        edge_count: Uniform::new_inclusive(min_edge_count, max_edge_count),
        stroke_size,
        stroke_reduction,
        randomize_rotation,
        shapes,
    };

    let image = unsafe { LOADED_IMAGE.clone().unwrap() };

    let size = UVec2::new(
        (image.width() as f32 / image.height() as f32 * 3000.0) as u32,
        3000,
    );

    let sketcher = PreslavSketcher::new(image, settings, seed as u64);

    let canvas = sketcher
        .run_and_dispose(|_| ())
        .into_vector_canvas(VectorizerStyle::Pixels);

    render(canvas, Some(Color::white()), size, render_type)
}

#[wasm_bindgen]
pub fn halftone(
    dot_density: f32,
    dot_scale: f32,
    dot_sides: u32,
    dot_color: &str,
    render_type: u8,
) -> Option<Uint8Array> {
    let settings = HalftoneSketcherSettings {
        dot_density,
        dot_scale,
        dot_sides: dot_sides as usize,
        dot_color: Color::from_hex(&dot_color).unwrap(),
    };

    let image = unsafe { LOADED_IMAGE.clone().unwrap() };

    let size = UVec2::new(
        (image.width() as f32 / image.height() as f32 * 3000.0) as u32,
        3000,
    );

    let sketcher = HalftoneSketcher::new(image, settings);

    let canvas = sketcher
        .run_and_dispose(|_| ())
        .into_vector_canvas(VectorizerStyle::Pixels);

    render(canvas, Some(Color::white()), size, render_type)
}

fn render(
    canvas: VectorCanvas,
    background_color: Option<Color>,
    size: UVec2,
    render_type: u8,
) -> Option<Uint8Array> {
    match render_type {
        1 => {
            canvas.render::<CanvasRenderer>(CanvasRenderer::new("canvas".into(), background_color));
            None
        }
        2 => {
            let svg = canvas.render::<SvgRenderer>(SvgRenderer::new(
                size.as_vec2(),
                background_color,
                false,
                true,
                16,
            ));

            let output = Uint8Array::new(&JsValue::from_f64(svg.as_bytes().len() as f64));
            output.copy_from(svg.as_bytes());

            Some(output)
        }
        3 => {
            let mut png = Vec::new();

            let encoder = PngEncoder::new(&mut png);

            let image = canvas.render::<SkiaRenderer>(SkiaRenderer::new(
                size,
                background_color,
                true,
                true,
            ));

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
