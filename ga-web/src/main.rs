use std::sync::Arc;

use crate::preslav::post_preslav;
use dashmap::DashMap;
use rocket::{
    http::{ContentType, Status},
    State,
};
mod preslav;

#[macro_use]
extern crate rocket;

#[get("/image/<hash>")]
fn get_image(
    hash: String,
    processed: &State<Arc<ProcessedImages>>,
) -> (Status, Result<(ContentType, Vec<u8>), String>) {
    let hash = match hex::decode(hash) {
        Ok(h) => h,
        Err(err) => return (Status::BadRequest, Err(err.to_string())),
    };

    if hash.len() != 32 {
        return (
            Status::BadRequest,
            Err(format!(
                "Hash of length {} bits is not valid. We expected 256 bits",
                hash.len() * 8
            )),
        );
    }

    let processed = (*processed).clone();

    let result = match processed.get(&hash[0..32]) {
        Some(r) => r.value().to_owned(),
        None => return (Status::BadRequest, Err("Value not found.".to_owned())),
    };

    (Status::Accepted, Ok((ContentType::PNG, result)))
}

#[get("/")]
fn get_index() -> (ContentType, &'static str) {
    (ContentType::HTML, include_str!("../www/index.html"))
}

#[get("/font")]
fn get_font() -> Vec<u8>{
    include_bytes!("../www/Louis George Cafe.ttf").to_vec()
}

#[get("/favicon.ico")]
fn get_favicon() -> (ContentType, Vec<u8>){
    (ContentType::Icon, include_bytes!("../www/favicon.ico").to_vec())
}

type ProcessedImages = DashMap<[u8; 32], Vec<u8>>;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(ProcessedImages::new()))
        .mount("/", routes![get_index, get_font, get_favicon, get_image, post_preslav])
}
