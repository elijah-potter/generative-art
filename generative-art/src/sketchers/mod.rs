mod celestial;
mod color;
mod fragment;
mod preslav;
pub mod rastercanvas;
mod rasterconverter;
pub mod vectorcanvas;

pub use celestial::{CelestialSketcher, CelestialSketcherSettings};
pub use color::Color;
pub use fragment::{FragmentSketcher, FragmentSketcherSettings};
use indicatif::ProgressBar;
pub use preslav::{PreslavSketcher, PreslavSketcherSettings};
pub use rasterconverter::{RasterConverterSketcher, RasterConverterSketcherSettings};

use rastercanvas::RasterCanvas;
use vectorcanvas::VectorCanvas;

/// A sketcher that works internally and outputs Vector graphics.
pub trait VectorSketcher<F: Fn(f32)> {
    /// Runs the sketcher. Allows to run function before each iteration of the sketcher, providing progress (ranges from 0.0 to 1.0).
    /// Behavior of progress will vary sketcher-to-sketcher.
    fn run(&mut self, before_iter: F) -> VectorCanvas;
}

/// A sketcher that works internally and outputs Raster graphics.
pub trait RasterSketcher<F: Fn(f32)> {
    /// Runs the sketcher. Allows to run function before each iteration of the sketcher, providing progress (ranges from 0.0 to 1.0).
    /// Behavior of progress will vary sketcher-to-sketcher.
    fn run(&mut self, before_iter: F) -> RasterCanvas;
}