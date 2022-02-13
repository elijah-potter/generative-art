use barium::{Color, Shape};

use crate::canvas::{OmniCanvas, RasterCanvas, VectorCanvas};

use super::Sketcher;

/// A sketcher that runs a function over each pixel of a [RasterCanvas].
/// Somewhat similar to a post-processing fragment shader.
pub struct RasterFragmentSketcher<F>
where
    F: Fn(usize, usize, usize, &[Color]) -> Color,
{
    kernel: F,
    canvas: RasterCanvas,
}

impl<F> RasterFragmentSketcher<F>
where
    F: Fn(usize, usize, usize, &[Color]) -> Color,
{
    pub fn new(canvas: RasterCanvas, kernel: F) -> Self {
        Self { canvas, kernel }
    }

    fn run_kernel<P: Fn(f32)>(&mut self, before_iter: P) {
        let width = self.canvas.width();

        for x in 0..self.canvas.width() {
            for y in 0..self.canvas.height() {
                let i = y * width + x;

                before_iter(i as f32 / width as f32);
                self.canvas
                    .set_pixel(x, y, (self.kernel)(x, y, i, self.canvas.get_raw()))
            }
        }
    }
}

impl<F, P> Sketcher<P> for RasterFragmentSketcher<F>
where
    P: Fn(f32),
    F: Fn(usize, usize, usize, &[Color]) -> Color,
{
    fn run(&mut self, before_iter: P) -> OmniCanvas {
        self.run_kernel(before_iter);

        self.canvas.clone().into()
    }

    fn run_and_dispose(mut self, before_iter: P) -> OmniCanvas {
        self.run_kernel(before_iter);

        self.canvas.into()
    }
}

/// A sketcher that runs a function over each element of a [VectorCanvas].
pub struct VectorFragmentSketcher<F>
where
    F: Fn(&mut Shape),
{
    kernel: F,
    canvas: VectorCanvas,
}

impl<F> VectorFragmentSketcher<F>
where
    F: Fn(&mut Shape),
{
    pub fn new(canvas: VectorCanvas, kernel: F) -> Self {
        Self { kernel, canvas }
    }

    fn run_kernel<P: Fn(f32)>(&mut self, before_iter: P) {
        let element_count = self.canvas.as_raw().len();

        for (i, element) in self.canvas.as_raw_mut().iter_mut().enumerate() {
            before_iter(i as f32 / element_count as f32);

            (self.kernel)(element)
        }
    }
}

impl<F, P> Sketcher<P> for VectorFragmentSketcher<F>
where
    P: Fn(f32),
    F: Fn(&mut Shape),
{
    fn run(&mut self, before_iter: P) -> OmniCanvas {
        self.run_kernel(before_iter);

        self.canvas.clone().into()
    }

    fn run_and_dispose(mut self, before_iter: P) -> OmniCanvas {
        self.run_kernel(before_iter);

        self.canvas.into()
    }
}
