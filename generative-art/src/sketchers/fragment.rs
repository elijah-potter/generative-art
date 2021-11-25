use glam::{IVec2, UVec2};

use super::{rastercanvas::RasterCanvas, Color, RasterSketcher};

#[derive(Clone)]
pub struct FragmentSketcherSettings<F>
where
    F: Fn(usize, usize, &mut Color) + Clone,
{
    pub kernel: F,
    pub input_image: Option<RasterCanvas>,
    pub width: usize,
    pub height: usize,
}

pub struct FragmentSketcher<F>
where
    F: Fn(usize, usize, &mut Color) + Clone,
{
    kernel: F,
    canvas: RasterCanvas,
}

impl<F> FragmentSketcher<F>
where
    F: Fn(usize, usize, &mut Color) + Clone,
{
    pub fn new(settings: FragmentSketcherSettings<F>) -> Self {
        Self {
            kernel: settings.kernel.clone(),
            canvas: settings
                .input_image
                .clone()
                .unwrap_or_else(|| RasterCanvas::new(settings.width, settings.height)),
        }
    }
}

impl<F, P> RasterSketcher<P> for FragmentSketcher<F>
where
    F: Fn(usize, usize, &mut Color) + Clone,
    P: Fn(f32)
{
    fn run(&mut self, before_iter: P) -> RasterCanvas {
        let total_pixels = (self.canvas.width() * self.canvas.height()) as f32;
        let width = self.canvas.width();

        for (i, pixel) in self.canvas.get_raw_mut().iter_mut().enumerate() {
            before_iter(i as f32 / total_pixels);

            let y = i / width;
            let x = i % width;

            (self.kernel)(x, y, pixel);
        }

        self.canvas.clone()
    }
}
