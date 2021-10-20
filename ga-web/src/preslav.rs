use std::sync::Arc;

use rocket::{State, http::Status};
use generative_art::{PreslavSketcher, image::{self, ImageEncoder, ImageFormat, png::PngEncoder}};

use crate::ProcessedImages;

#[post("/preslav", data = "<file>")]
pub fn post_preslav(file: Vec<u8>, processed: &State<Arc<ProcessedImages>>) -> (Status, Result<Vec<u8>, String>) {
    let canvas = match image::load_from_memory(&file) {
        Ok(v) => v,
        Err(err) => {
            return (Status::BadRequest, Err("Could not process image.".to_owned()));
        }
    };

    // Process image
    let canvas = canvas.into_rgba8();

    let iterations = 5000;
    let mut sketcher = PreslavSketcher::new_preslav(canvas.to_owned(), iterations);
    sketcher.max_edge_count = 6;

    for _ in 0..iterations {
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
        return (Status::InternalServerError, Err(err.to_string()))
    };

    let hash = blake3::hash(&file);

    let processed = (*processed).clone();
    processed.insert(hash.as_bytes().to_owned(), output);

    (Status::Created, Ok(hash.as_bytes().to_vec()))
}