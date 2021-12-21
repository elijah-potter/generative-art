mod canvas_renderer;

use canvas_renderer::{CanvasRenderer, CanvasRendererSettings};
use generative_art::{
    denim::{Color, Renderer, Stroke, Vec2},
    sketchers::{CelestialSketcherSettings, Sketcher, CelestialSketcher},
    OmniCanvas, VectorCanvas, VectorizerStyle,
};
use rand::distributions::Uniform;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
    zoom: f32
) {
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

    log("Created settings");

    let sketcher = CelestialSketcher::new(settings, 20);

    log("Created sketcher");

    let mut canvas = sketcher.run_and_dispose(|_| ())
    .into_vector_canvas(VectorizerStyle::Pixels);  
    
    canvas.zoom_camera(zoom);

    log("Ran sketcher");

    canvas.render::<CanvasRenderer>(CanvasRendererSettings {
        id: "canvas".into(),
        background: Some(Color::black()),
    });

    log("Rendered canvas")
}
