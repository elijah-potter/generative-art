use std::{f32::consts::PI, ops::Range};

use glam::{Vec2};
use image::{Rgb};
use rand::Rng;
use svg::node::element::Rectangle;
use svg::Node;
use svg::{node::element::Circle, Document};

use crate::helpers::{RgbExt, RngCoreExt};

pub struct CelestialSketcher {
    objects: Vec<CelestialObject>,
    render_count: usize,
    g: f32,
    foreground: Rgb<u8>,
    canvas: Document,
}

impl CelestialSketcher {
    /// Creates a new sketcher with objects of a random size within a range.
    /// Allows to define how far the planets are instantiated from the center.
    /// If not defined, they will be instantiated randomly within the image.
    /// Allows to define how many of the simulated objects are rendered.
    /// If the render count is greater than the object count, it panics.
    pub fn new(
        output_size: Vec2,
        object_count: usize,
        object_size: Range<f32>,
        object_velocity: Range<f32>,
        g: f32,
        foreground: Rgb<u8>,
        max_radius_from_center: Option<f32>,
        increase_mass_with_distance: bool,
        render_count: usize,
    ) -> Self {
        if render_count > object_count {
            panic!("render_count must be <= object_count.")
        }

        let mut rng = rand::thread_rng();

        let mut objects = Vec::with_capacity(object_count);

        let mut total_energy = Vec2::ZERO;

        while objects.len() < object_count {
            let mut radius = output_size.y;

            let position = match max_radius_from_center {
                Some(dist) => {
                    let center = Vec2::new(output_size.x / 2.0, output_size.y / 2.0);

                    let mut position = Vec2::new(
                        rng.gen_range(center.x - dist..center.x + dist),
                        rng.gen_range(center.y - dist..center.y + dist),
                    );

                    while center.distance(position) > dist {
                        position = Vec2::new(
                            rng.gen_range(center.x - dist..center.x + dist),
                            rng.gen_range(center.y - dist..center.y + dist),
                        );
                    }

                    radius = dist;

                    position
                }
                None => Vec2::new(
                    rng.gen_range(0.0..output_size.x),
                    rng.gen_range(0.0..output_size.y),
                ),
            };

            let mass = if increase_mass_with_distance {
                let center = Vec2::new(output_size.x / 2.0, output_size.y / 2.0);

                center.distance(position) / radius
                    + object_size.start * (object_size.end - object_size.start)
            } else {
                rng.gen_range(object_size.clone())
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

        let canvas = Document::new()
            .add(
                Rectangle::new()
                    .set("fill", "#000")
                    .set("width", output_size.x)
                    .set("height", output_size.y),
            )
            .set("width", output_size.x)
            .set("height", output_size.y);

        Self {
            objects,
            render_count,
            g,
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
                self.canvas.append(
                    Circle::new()
                        .set("cx", object.position.x)
                        .set("cy", object.position.y)
                        .set("r", radius)
                        .set("fill", self.foreground.as_hex()),
                );
            }
        }
    }

    pub fn get_canvas(&self) -> &Document {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut Document {
        &mut self.canvas
    }
}

#[derive(Clone, Debug)]
struct CelestialObject {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
}
