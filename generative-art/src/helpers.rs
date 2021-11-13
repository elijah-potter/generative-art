use std::f32::consts::PI;

use glam::Vec2;
use image::GenericImage;
use imageproc::pixelops::interpolate;
use imageproc::{
    drawing::{draw_antialiased_line_segment_mut, draw_line_segment_mut, Canvas},
    point::Point,
};
use rand::{Rng, RngCore};

use crate::convert::ToPoint;

/// Draws a hollow polygon
pub fn draw_hollow_polygon_mut<C>(canvas: &mut C, poly: &[Point<i32>], color: C::Pixel)
where
    C: Canvas,
    C::Pixel: 'static,
{
    for i in 0..(poly.len() - 1) {
        draw_line_segment_mut(
            canvas,
            (poly[i].x as f32, poly[i].y as f32),
            (poly[i + 1].x as f32, poly[i + 1].y as f32),
            color,
        )
    }
}

/// Generates points for a regular polygon
pub fn regular_polygon_points(
    position: Vec2,
    radius: f32,
    sides: u32,
    theta: f32,
) -> Vec<Point<i32>> {
    let mut points = Vec::with_capacity(sides as usize);

    for n in 0..sides {
        points.push(Point::new(
            (radius * (2.0 * PI * n as f32 / sides as f32 + theta).cos() + position.x) as i32,
            (radius * (2.0 * PI * n as f32 / sides as f32 + theta).sin() + position.y) as i32,
        ));
    }

    if points.first() == points.last() {
        return vec![
            Point::new(position.x as i32, position.y as i32),
            Point::new(position.x as i32 + 1, position.y as i32 + 1),
        ];
    }

    points
}

pub trait RngCoreExt {
    fn random_sign(&mut self) -> f32;
}

impl<R> RngCoreExt for R 
where R: RngCore {
    fn random_sign(&mut self) -> f32 {
        if self.gen::<bool>() {
            1.0
        } else {
            -1.0
        }
    }
}
