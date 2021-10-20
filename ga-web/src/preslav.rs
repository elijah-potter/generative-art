use std::{sync::Arc, time::Duration};

use generative_art::{
    image::{self, png::PngEncoder, ImageEncoder},
    PreslavSketcher,
};
use rocket::{http::Status, State};

use crate::ProcessedImages;

#[post("/preslav", data = "<file>")]
pub fn post_preslav(
    file: Vec<u8>,
    processed: &State<Arc<ProcessedImages>>,
) -> (Status, Result<Vec<u8>, String>) {
    let canvas = match image::load_from_memory(&file) {
        Ok(v) => v,
        Err(_err) => {
            return (
                Status::BadRequest,
                Err("Could not process image.".to_owned()),
            );
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
        return (Status::InternalServerError, Err(err.to_string()));
    };

    let hash = blake3::hash(&file);

    
    let processed = (*processed).clone();
    processed.insert(hash.as_bytes().to_owned(), output);

    tokio::spawn(async move{
        tokio::time::sleep(Duration::from_secs(60 * 10)).await;
        processed.remove(hash.as_bytes()); // Might cause deadlock possibly
    });

    (Status::Created, Ok(hash.as_bytes().to_vec()))
}
