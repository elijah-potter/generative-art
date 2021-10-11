mod sketch;

use image::{ImageEncoder, ImageFormat, png::PngEncoder};
use sketch::Sketcher;
use wasm_bindgen::prelude::wasm_bindgen;

/// Create from file
#[wasm_bindgen]
pub extern "C" fn sketch_file(file: &[u8], filename: &str, iterations: usize) -> Option<Vec<u8>> {
    let format = match ImageFormat::from_path(filename) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let canvas = match image::load_from_memory_with_format(file, format) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let canvas = canvas.into_rgba8();

    let mut sketcher = Sketcher::new_preslav(canvas.to_owned(), iterations);
    sketcher.max_edge_count = 6;

    for _ in 0..iterations {
        sketcher.draw_iter(&canvas);
    }

    let mut output = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut output,
        image::png::CompressionType::Best,
        image::png::FilterType::NoFilter,
    );

    let canvas = sketcher.get_canvas();
    if encoder.write_image(canvas.as_flat_samples().samples, canvas.width(), canvas.height(), image::ColorType::Rgba8).is_err(){
        return None;
    };

    Some(output)
}
