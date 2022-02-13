use std::f32::consts::PI;

use rand::prelude::Distribution;
#[cfg(feature = "small-rng")]
use rand::{rngs::SmallRng, SeedableRng};

use crate::canvas::{OmniCanvas, VectorCanvas};
use crate::sketchers::Sketcher;
use barium::{Color, LineEnd, Stroke, Vec2};

#[derive(Clone)]
pub struct CelestialSketcherSettings<P, S, V>
where
    P: Distribution<f32>,
    S: Distribution<f32>,
    V: Distribution<f32>,
{
    /// The number of objects to simulate.
    pub object_count: usize,
    /// The number of objects to include in the render.
    pub render_count: usize,
    /// The distribution of initial object positions.
    pub object_position: P,
    /// The distribution of object masses.
    pub object_size: S,
    /// The distribution of initial object speeds.
    pub object_velocity: V,
    /// The universal gravitational constant.
    pub g: f32,
    /// The color to draw objects.
    pub foreground: Color,
    /// The number of steps to simulate.
    pub steps: usize,
    /// How much time should pass between steps.
    pub step_length: f32,
    /// Whether to render object paths as a series of regular polygons instead of as a path.
    pub render_polygon: Option<usize>,
}

pub struct CelestialSketcher {
    objects: Vec<CelestialObject>,
    render_count: usize,
    g: f32,
    foreground: Color,
    steps: usize,
    step_length: f32,
    render_polygon: Option<usize>,
    canvas: VectorCanvas,
}

impl CelestialSketcher {
    /// Creates a new sketcher with objects of a random size within a range.
    /// Allows to define how far the planets are instantiated from the center.
    /// If not defined, they will be instantiated randomly within the image.
    /// Allows to define how many of the simulated objects are rendered.
    /// If the render count is greater than the object count, it panics.
    pub fn new<P: Distribution<f32>, S: Distribution<f32>, V: Distribution<f32>>(
        settings: CelestialSketcherSettings<P, S, V>,
        #[cfg(feature = "small-rng")] seed: u64,
    ) -> Self {
        #[cfg(feature = "thread-rng")]
        let mut rng = rand::thread_rng();
        #[cfg(feature = "small-rng")]
        let mut rng = SmallRng::seed_from_u64(seed);

        let mut objects = Vec::with_capacity(settings.object_count);
        let _total_energy = Vec2::ZERO;

        while objects.len() < settings.object_count {
            let position = Vec2::new(
                settings.object_position.sample(&mut rng),
                settings.object_position.sample(&mut rng),
            );

            let mass = settings.object_size.sample(&mut rng);

            let velocity = Vec2::new(
                settings.object_velocity.sample(&mut rng),
                settings.object_velocity.sample(&mut rng),
            );

            objects.push(CelestialObject {
                position,
                velocity,
                mass,
                path: Vec::with_capacity(settings.steps),
            });
        }

        Self {
            objects,
            render_count: settings.render_count,
            g: settings.g,
            foreground: settings.foreground,
            steps: settings.steps,
            step_length: settings.step_length,
            render_polygon: settings.render_polygon,
            canvas: VectorCanvas::default(),
        }
    }

    /// Computes the next step of the simulation..
    fn step(&mut self) {
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
            object.velocity += force * self.step_length;
            object.position += object.velocity * self.step_length;
        }
    }

    /// Renders the path of a given number of objects.
    fn render(&mut self) {
        self.canvas = VectorCanvas::default();

        if let Some(sides) = self.render_polygon {
            for i in 0..self.objects[0].path.len() {
                for object in &self.objects {
                    let position = object.path[i];
                    let radius = (object.mass / PI).sqrt() / 5000.0;

                    self.canvas.draw_regular_polygon(
                        position,
                        sides,
                        radius,
                        0.0,
                        None,
                        Some(self.foreground),
                    )
                }
            }
        } else {
            for index in 0..self.render_count {
                let object = &self.objects[index];
                let radius = (object.mass / PI).sqrt() / 5000.0;

                self.canvas.draw_shape(
                    object.path.clone(),
                    Some(Stroke {
                        color: self.foreground,
                        width: radius * 2.0,
                        line_end: LineEnd::Round,
                    }),
                    None,
                );
            }
        }
    }

    /// Simulates and renders the system.
    fn run<P: Fn(f32)>(&mut self, before_iter: P) {
        for i in 0..self.steps {
            before_iter(i as f32 / self.steps as f32);

            self.step();
        }

        self.render();
    }
}

impl<F> Sketcher<F> for CelestialSketcher
where
    F: Fn(f32),
{
    fn run(&mut self, before_iter: F) -> OmniCanvas {
        self.run(before_iter);

        self.canvas.clone().into()
    }

    fn run_and_dispose(mut self, before_iter: F) -> OmniCanvas {
        self.run(before_iter);

        self.canvas.into()
    }
}

#[derive(Clone, Debug)]
struct CelestialObject {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    path: Vec<Vec2>,
}
