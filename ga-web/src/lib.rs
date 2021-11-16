use generative_art::{
    image::{Pixel, Rgb},
    CelestialSketcher, Vec2,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// We have do create an adapter so it plays nicely with wasm_bindgen.
pub struct CelestialSketcherSettings {
    pub width: f32,
    pub height: f32,
    pub object_count: usize,
    pub min_object_size: f32,
    pub max_object_size: f32,
    pub min_object_velocity: f32,
    pub max_object_velocity: f32,
    pub g: f32,
    pub max_radius_from_center: Option<f32>,
    pub increase_mass_with_distance: bool,
    pub expected_steps: usize,
    pub dots: bool,
    pub render_count: usize,
    pub step_size: f32,
}

#[wasm_bindgen]
impl CelestialSketcherSettings {
    pub fn new() -> Self {
        Self {
            width: 1000.0,
            height: 1000.0,
            object_count: 3,
            min_object_size: 1.0,
            max_object_size: 100.0,
            min_object_velocity: 2.0,
            max_object_velocity: 1000.0,
            g: 1.0,
            max_radius_from_center: None,
            increase_mass_with_distance: false,
            dots: false,
            render_count: 3,
            step_size: 0.005,
            expected_steps: 10000,
        }
    }
}

impl CelestialSketcherSettings {
    pub fn as_native(&self) -> generative_art::CelestialSketcherSettings {
        generative_art::CelestialSketcherSettings {
            output_size: Vec2::new(self.width, self.height),
            object_size: self.min_object_size..self.max_object_size,
            object_count: self.object_count,
            object_velocity: self.min_object_velocity..self.max_object_velocity,
            g: self.g,
            max_radius_from_center: self.max_radius_from_center,
            increase_mass_with_distance: self.increase_mass_with_distance,
            expected_steps: self.expected_steps,
            foreground: Rgb::from_channels(255, 255, 255, 255),
        }
    }
}

#[wasm_bindgen]
pub fn celestial(settings: CelestialSketcherSettings, seed: u64) -> String {
    let mut sketcher = CelestialSketcher::new(&settings.as_native(), seed);

    for _i in 0..settings.expected_steps {
        sketcher.step(settings.step_size);
    }

    let mut file = Vec::new();
    generative_art::svg::write(
        &mut file,
        &sketcher.render(
            0..settings.expected_steps,
            0..settings.render_count,
            settings.dots,
        ),
    ).unwrap();

    to_data_uri(
        file.as_slice(),
        "image/svg+xml",
    )
}

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


fn to_data_uri(data: &[u8], file_type: &str) -> String {
    format!("data:{};base64,{}", file_type, base64::encode(data))
}
