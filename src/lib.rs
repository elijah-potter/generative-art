mod sketch;
mod utils;

use image::{png::PngEncoder, ImageEncoder, ImageFormat};
use sketch::Sketcher;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/src/progress.js")]
extern "C" {
    fn progress(s: f32);
}

const ERROR_IMAGE: &str = "error.png";

/// Create from file
#[wasm_bindgen]
pub fn sketch_file(data: String, filename: String, iterations: usize) -> String {
    // Parse data url
    let file = from_data_url(&data);

    // Identify file format
    let format = match ImageFormat::from_path(filename) {
        Ok(v) => v,
        Err(err) => {
            alert(&err.to_string());
            return ERROR_IMAGE.to_owned();
        }
    };

    // Unpack image
    let canvas = match image::load_from_memory_with_format(&file, format) {
        Ok(v) => v,
        Err(err) => {
            alert(&err.to_string());
            return ERROR_IMAGE.to_owned();
        }
    };

    // Process image
    let canvas = canvas.into_rgba8();

    let mut sketcher = Sketcher::new_preslav(canvas.to_owned(), iterations);
    sketcher.max_edge_count = 6;

    for i in 0..iterations {
        progress(i as f32 / iterations as f32 * 100.0);

        sketcher.draw_iter(&canvas);
    }

    // Encode image into png
    let mut output = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut output,
        image::png::CompressionType::Best,
        image::png::FilterType::NoFilter,
    );

    let canvas = sketcher.get_canvas();
    if let Err(err) = encoder.write_image(
        canvas.as_flat_samples().samples,
        canvas.width(),
        canvas.height(),
        image::ColorType::Rgba8,
    ) {
        alert(&err.to_string());
        return ERROR_IMAGE.to_owned();
    };

    // Output to data url
    create_data_url(&output, "image/png")
}

fn create_data_url(data: &[u8], mediatype: &str) -> String {
    let mut output = String::new();

    output.push_str("data:");
    output.push_str(mediatype);
    output.push_str(";base64,");
    output.push_str(&base64::encode(data));

    output
}

/// Processes a png base64 data url.
fn from_data_url(url: &str) -> Vec<u8> {
    let mut comma_location = 0;
    for (i, c) in url.chars().enumerate() {
        if c == ',' {
            comma_location = i;
            break;
        }
    }

    base64::decode(&url[(comma_location + 1)..]).unwrap()
}
