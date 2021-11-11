use std::{f32::consts::PI, ops::Range};

use glam::{IVec2, Vec2};
use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_line_segment_mut, draw_polygon_mut};
use rand::{Rng, RngCore};

use crate::{
    convert::ToPoint,
    helpers::{regular_polygon_points, thick_line_points, RngCoreExt},
};

pub struct CelestialSketcher {
    objects: Vec<CelestialObject>,
    render_count: usize,
    g: f32,
    foreground: Rgba<u8>,
    canvas: RgbaImage,
}

impl CelestialSketcher {
    /// Creates a new sketcher with objects of a random size within a range.
    /// Allows to define how far the planets are instantiated from the center.
    /// If not defined, they will be instantiated randomly within the image.
    /// Allows to define how many of the simulated objects are rendered.
    /// If the render count is greater than the object count, it panics.
    pub fn new(
        canvas: RgbaImage,
        object_count: usize,
        object_size: Range<f32>,
        object_velocity: Range<f32>,
        foreground: Rgba<u8>,
        max_radius_from_center: Option<f32>,
        render_count: usize,
    ) -> Self {
        if render_count > object_count {
            panic!("render_count must be <= object_count.")
        }

        let mut rng = rand::thread_rng();

        let mut objects = Vec::with_capacity(object_count);

        let mut total_energy = Vec2::ZERO;

        while objects.len() < object_count {
            let (position, mass) = match max_radius_from_center {
                Some(dist) => {
                    let center =
                        Vec2::new(canvas.width() as f32 / 2.0, canvas.height() as f32 / 2.0);

                    let mut position = Vec2::new(
                        rng.gen_range(center.x - dist..center.y + dist),
                        rng.gen_range(center.y - dist..center.y + dist),
                    );

                    while center.distance(position) > dist {
                        position = Vec2::new(
                            rng.gen_range(center.x - dist..center.y + dist),
                            rng.gen_range(center.y - dist..center.y + dist),
                        );
                    }

                    (position, center.distance(position) / dist + object_size.start * (object_size.end - object_size.start))
                }
                None => (
                    Vec2::new(
                        rng.gen_range(0.0..canvas.width() as f32),
                        rng.gen_range(0.0..canvas.height() as f32),
                    ),
                    rng.gen_range(object_size.clone()),
                ),
            };

            let velocity = if objects.len() < object_count - 1 {
                let velocity = Vec2::new(
                    // TODO: Fix ranges??
                    rng.gen_range(object_velocity.clone()) * rng.random_sign(),
                    rng.gen_range(object_velocity.clone()) * rng.random_sign(),
                );

                total_energy += velocity * mass;

                velocity
            } else {
                -(total_energy / mass)
            };

            objects.push(CelestialObject {
                position,
                velocity,
                mass,
            });
        }

        Self {
            objects,
            render_count,
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
                        / object.position.distance(other_object.position)
                        * (other_object.position - object.position);
                }
            }
            object.velocity += force * delta_time;
            object.position += object.velocity * delta_time;

            let radius = (object.mass / PI).sqrt();

            if index < self.render_count {
                draw_filled_circle_mut(
                    &mut self.canvas,
                    (object.position.x as i32, object.position.y as i32),
                    radius as i32,
                    self.foreground,
                );
            }
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
