use std::{f32::consts::PI, ops::Range};

use glam::Vec2;
use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_line_segment_mut, draw_polygon_mut};
use rand::{Rng, RngCore};

use crate::{convert::ToPoint, helpers::{regular_polygon_points, thick_line_points}};

pub struct CelestialSketcher {
    objects: Vec<CelestialObject>,
    g: f32,
    foreground: Rgba<u8>,
    canvas: RgbaImage,
}

impl CelestialSketcher {
    /// Creates a new sketcher with objects of a random size within a range.
    pub fn new(
        canvas: RgbaImage,
        object_count: usize,
        object_size: Range<f32>,
        foreground: Rgba<u8>,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let objects = (0..object_count)
            .map(|_| CelestialObject {
                position: Vec2::new(
                    rng.gen_range(0.0..canvas.width() as f32),
                    rng.gen_range(0.0..canvas.height() as f32),
                ),
                velocity: Vec2::ZERO,
                mass: rng.gen_range(object_size.clone()),
            })
            .collect();

        Self {
            objects,
            g: 0.005,
            foreground,
            canvas,
        }
    }

    /// Steps the simulation forward and draws a frame.
    pub fn draw_iter(&mut self, delta_time: f32) {
        let previous_state = self.objects.clone();

        for (index, object) in self.objects.iter_mut().enumerate() {
            let mut force = Vec2::ZERO;
            for other_object in &previous_state {
                if object.position != other_object.position {
                    force += self.g * object.mass * other_object.mass
                        / object.position.distance(other_object.position) * (other_object.position - object.position);
                }
            }
            object.velocity += force * delta_time;
            object.position += object.velocity * delta_time;

            let radius = (object.mass / PI).sqrt();

            draw_filled_circle_mut(&mut self.canvas, (object.position.x as i32, object.position.y as i32), radius as i32, self.foreground);
        }
    }

    pub fn get_canvas(&self) -> &RgbaImage {
        &self.canvas
    }
}

#[derive(Clone, Debug)]
struct CelestialObject {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
}
