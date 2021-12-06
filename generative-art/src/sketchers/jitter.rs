use std::f32::consts::PI;

use rand::prelude::Distribution;
#[cfg(feature = "small-rng")]
use rand::{rngs::SmallRng, SeedableRng};

use crate::canvas::VectorCanvas;
use crate::sketchers::VectorSketcher;
use denim::{CanvasElement, CanvasElementVariant, Color, Stroke, Vec2};

#[derive(Clone)]
pub struct CelestialSketcherSettings<P, S, V>
where
    P: Distribution<f32>,
    S: Distribution<f32>,
    V: Distribution<f32>,
{
    /// The size of the rendered output in universe space.
    pub output_size: Vec2,
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
    /// Whether to render object paths as dots instead of as a path.
    pub render_dots: bool,
}

pub struct JitterSketcher {
    canvas: VectorCanvas
}

impl CelestialSketcher {
    /// Creates a new sketcher with objects of a random size within a range.
    /// Allows to define how far the planets are instantiated from the center.
    /// If not defined, they will be instantiated randomly within the image.
    /// Allows to define how many of the simulated objects are rendered.
    /// If the render count is greater than the object count, it panics.
    pub fn new<P: Distribution<f32>, S: Distribution<f32>, V: Distribution<f32>>(
        
    ) -> Self {
    }
}

impl<F> VectorSketcher<F> for CelestialSketcher
where
    F: Fn(f32),
{
    fn run(&mut self, before_iter: F) -> VectorCanvas {
        for i in 0..self.steps {
            before_iter(i as f32 / self.steps as f32);

            self.step();
        }

        self.render();

        self.canvas.clone()
    }
}
