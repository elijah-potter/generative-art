use std::{f32::consts::PI, ops::Range};

use glam::Vec2;
use image::Rgb;
use rand::Rng;
use svg::node::element::{Polyline};
use svg::Node;
use svg::{node::element::Circle, Document};

use crate::helpers::{RgbExt, RngCoreExt, clean_canvas};

pub struct CelestialSketcherSettings {
    pub output_size: Vec2,
    pub object_count: usize,
    pub object_size: Range<f32>,
    pub object_velocity: Range<f32>,
    pub g: f32,
    pub foreground: Rgb<u8>,
    pub max_radius_from_center: Option<f32>,
    pub increase_mass_with_distance: bool,
    pub expected_steps: usize,
}

pub struct CelestialSketcher {
    objects: Vec<CelestialObject>,
    g: f32,
    foreground: Rgb<u8>,
    canvas_size: Vec2,
}

impl CelestialSketcher {
    fn inside_view(&self, pos: Vec2, radius: f32) -> bool {
        pos.x > -radius
            && pos.y > -radius
            && pos.x < self.canvas_size.x + radius
            && pos.y < self.canvas_size.y + radius
    }

    /// Creates a new sketcher with objects of a random size within a range.
    /// Allows to define how far the planets are instantiated from the center.
    /// If not defined, they will be instantiated randomly within the image.
    /// Allows to define how many of the simulated objects are rendered.
    /// If the render count is greater than the object count, it panics.
    pub fn new(settings: CelestialSketcherSettings) -> Self {
        let mut rng = rand::thread_rng();

        let mut objects = Vec::with_capacity(settings.object_count);

        let mut total_energy = Vec2::ZERO;

        while objects.len() < settings.object_count {
            let mut radius = settings.output_size.y;

            let position = match settings.max_radius_from_center {
                Some(dist) => {
                    let center =
                        Vec2::new(settings.output_size.x / 2.0, settings.output_size.y / 2.0);

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
                    rng.gen_range(0.0..settings.output_size.x),
                    rng.gen_range(0.0..settings.output_size.y),
                ),
            };

            let mass = if settings.increase_mass_with_distance {
                let center = Vec2::new(settings.output_size.x / 2.0, settings.output_size.y / 2.0);

                center.distance(position) / radius
                    + settings.object_size.start
                        * (settings.object_size.end - settings.object_size.start)
            } else {
                rng.gen_range(settings.object_size.clone())
            };

            let velocity = if objects.len() < settings.object_count - 1 {
                let velocity = Vec2::new(
                    // TODO: Fix ranges??
                    rng.gen_range(settings.object_velocity.clone()) * rng.random_sign(),
                    rng.gen_range(settings.object_velocity.clone()) * rng.random_sign(),
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
                path: Vec::with_capacity(settings.expected_steps),
            });
        }

        Self {
            objects,
            g: settings.g,
            foreground: settings.foreground,
            canvas_size: settings.output_size,
        }
    }

    /// Renders the path of a given range of objects within a given a range of time.
    /// Allows to choose between rendering lines for path, or rendering dots at each time step.
    pub fn render(
        &mut self,
        steps: Range<usize>,
        render_objects: Range<usize>,
        dots: bool,
    ) -> Document {
        let mut canvas = clean_canvas(self.canvas_size);

        for index in render_objects {
            let object = &self.objects[index];
            let radius = (object.mass / PI).sqrt();
            let mut polyline = String::new();

            for s in steps.clone() {
                let object_pos = object.path[s];

                if dots {
                    // Only add dot if it will be visible on output.
                    if self.inside_view(object_pos, radius) {
                        canvas.append(
                            Circle::new()
                                .set("cx", object_pos.x)
                                .set("cy", object_pos.y)
                                .set("r", radius)
                                .set("fill", self.foreground.as_hex()),
                        );
                    }
                } else {
                    polyline.push_str(&format!("{},{} ", object_pos.x, object_pos.y))
                }
            }

            if !dots {
                canvas.append(
                    Polyline::new()
                        .set("fill", "none")
                        .set("points", polyline)
                        .set("stroke", self.foreground.as_hex())
                        .set("width", radius),
                );
            }
        }

        canvas
    }

    /// Steps the simulation forward.
    pub fn step(&mut self, delta_time: f32) {
        let previous_state: Vec<(Vec2, f32)> =
            self.objects.iter().map(|v| (v.position, v.mass)).collect();

        for (_index, object) in self.objects.iter_mut().enumerate() {
            let mut force = Vec2::ZERO;
            for other_object in &previous_state {
                if object.position != other_object.0 {
                    force += self.g * object.mass * other_object.1
                        / object.position.distance(other_object.0)
                        * (other_object.0 - object.position);
                }
            }
            object.path.push(object.position);
            object.velocity += force * delta_time;
            object.position += object.velocity * delta_time;
        }
    }
}

#[derive(Clone, Debug)]
struct CelestialObject {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    path: Vec<Vec2>,
}
