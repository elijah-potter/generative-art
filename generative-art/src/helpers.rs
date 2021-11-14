use std::f32::consts::PI;

use glam::Vec2;
use image::{Rgb};
use rand::{Rng, RngCore};
use svg::{Document, node::element::Rectangle};

/// Generates points for a regular polygon in svg format. Example: "215,200 100,100 400,100"
pub fn regular_polygon_points(position: Vec2, radius: f32, sides: u32, theta: f32) -> String {
    let mut output = String::new();

    for n in 0..sides {
        output.push_str(&format!(
            "{},{} ",
            (radius * (2.0 * PI * n as f32 / sides as f32 + theta).cos() + position.x),
            (radius * (2.0 * PI * n as f32 / sides as f32 + theta).sin() + position.y),
        ));
    }

    output
}

pub trait RngCoreExt {
    fn random_sign(&mut self) -> f32;
}

impl<R> RngCoreExt for R
where
    R: RngCore,
{
    fn random_sign(&mut self) -> f32 {
        if self.gen::<bool>() {
            1.0
        } else {
            -1.0
        }
    }
}

pub trait RgbExt {
    fn as_hex(&self) -> String;
}

impl RgbExt for Rgb<u8> {
    fn as_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
    }
}

pub fn clean_canvas(size: Vec2) -> Document {
    Document::new()
        .add(
            Rectangle::new()
                .set("fill", "#000")
                .set("width", size.x)
                .set("height", size.y),
        )
        .set("width", size.x)
        .set("height", size.y)
}