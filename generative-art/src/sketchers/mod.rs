mod celestial;
mod fragment;
mod preslav;

pub use celestial::{CelestialSketcher, CelestialSketcherSettings};
pub use fragment::FragmentSketcher;
pub use preslav::{PreslavSketcher, PreslavSketcherSettings};

use denim::{
    regular_polygon_points,
    renderers::skia_renderer::{SkiaRenderer, SkiaRendererSettings, ToRgbaImage},
    CanvasElement, CanvasElementVariant, Color, UVec2, Vec2,
};

pub trait Sketcher<F: Fn(f32)> {
    /// Runs the sketcher. Allows to run function before each iteration of the sketcher, providing progress (ranges from 0.0 to 1.0).
    /// Behavior of progress will vary sketcher-to-sketcher.
    fn run(&mut self, before_iter: F) -> crate::canvas::OmniCanvas;
}
