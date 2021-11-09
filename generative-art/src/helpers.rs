use std::f64::consts::PI;

use image::GenericImage;
use imageproc::{drawing::{Canvas, draw_antialiased_line_segment_mut, draw_line_segment_mut}, point::Point};
use imageproc::pixelops::interpolate;

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
    x: f64,
    y: f64,
    radius: f64,
    sides: u32,
    theta: f64,
) -> Vec<Point<i32>> {
    let mut points = Vec::with_capacity(sides as usize);

    for n in 0..sides {
        points.push(Point::new(
            (radius * (2.0 * PI * n as f64 / sides as f64 + theta).cos() + x) as i32,
            (radius * (2.0 * PI * n as f64 / sides as f64 + theta).sin() + y) as i32,
        ));
    }

    if points.first() == points.last() {
        return vec![
            Point::new(x as i32, y as i32),
            Point::new(x as i32 + 1, y as i32 + 1),
        ];
    }

    points
}