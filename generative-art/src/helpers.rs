use std::f32::consts::PI;

use glam::Vec2;
use image::Rgba;
use rand::{Rng, RngCore};

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

pub trait RgbaExt {
    fn as_hex(&self, include_alpha: bool) -> String;
}

impl RgbaExt for Rgba<u8> {
    fn as_hex(&self, include_alpha: bool) -> String {
        if include_alpha {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2], self.0[3])
        }
    else {
            format!("#{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
        }
    }
}
