use denim::Color;

use crate::canvas::{OmniCanvas, RasterCanvas};

use super::Sketcher;

/// A sketcher that runs a function over each pixel of a [RasterCanvas].
/// Somewhat similar to a Post-Processing fragment shader.
pub struct FragmentSketcher<F>
where
    F: Fn(usize, usize, usize, &[Color]) -> Color,
{
    kernel: F,
    canvas: RasterCanvas,
}

impl<F> FragmentSketcher<F>
where
    F: Fn(usize, usize, usize, &[Color]) -> Color,
{
    pub fn new(canvas: RasterCanvas, kernel: F) -> Self {
        Self { canvas, kernel }
    }
}

impl<F, P> Sketcher<P> for FragmentSketcher<F>
where
    P: Fn(f32),
    F: Fn(usize, usize, usize, &[Color]) -> Color,
{
    fn run(&mut self, before_iter: P) -> OmniCanvas {
        let width = self.canvas.width();

        for x in 0..self.canvas.width() {
            for y in 0..self.canvas.height() {
                let i = y * width + x;

                before_iter(i as f32 / width as f32);
                self.canvas
                    .set_pixel(x, y, (self.kernel)(x, y, i, self.canvas.get_raw()))
            }
        }

        self.canvas.clone().into()
    }
}
