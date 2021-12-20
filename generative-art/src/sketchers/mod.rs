mod celestial;
mod fragment;
mod wave;

pub use celestial::{CelestialSketcher, CelestialSketcherSettings};
pub use fragment::{RasterFragmentSketcher, VectorFragmentSketcher};
pub use wave::{WaveSketcher, WaveSketcherSettings};

use crate::canvas::OmniCanvas;

pub trait Sketcher<F: Fn(f32)> {
    /// Runs the sketcher. Allows to run a function before each iteration of the sketcher, providing progress (ranges from 0.0 to 1.0).
    /// Behavior of progress will vary sketcher-to-sketcher.
    fn run(&mut self, before_iter: F) -> OmniCanvas;

    /// Runs the sketcher and consumes it. Allows to run a function before each iteration of the sketcher, providing progress (ranges from 0.0 to 1.0).
    /// Behavior of progress will vary sketcher-to-sketcher.
    fn run_and_dispose(self, before_iter: F) -> OmniCanvas;
}
