mod celestial;
mod preslav;

pub use celestial::{CelestialSketcher, CelestialSketcherSettings};
pub use preslav::{PreslavSketcher, PreslavSketcherSettings};

/// A sketcher that works internally and outputs Vector graphics.
pub trait VectorSketcher<F: Fn(f32)> {
    /// Runs the sketcher. Allows to run function before each iteration of the sketcher, providing progress (ranges from 0.0 to 1.0).
    /// Behavior of progress will vary sketcher-to-sketcher.
    fn run(&mut self, before_iter: F) -> crate::canvas::VectorCanvas;
}