use barium::{Color, LineEnd, Mat2, Stroke, Vec2};

use rand::{prelude::Distribution, Rng};
#[cfg(feature = "small-rng")]
use rand::{rngs::SmallRng, SeedableRng};

use super::Sketcher;
use crate::canvas::{OmniCanvas, RasterCanvas, VectorCanvas};

#[derive(Clone)]
pub struct PreslavSketcherSettings<E>
where
    E: Distribution<usize> + Clone,
{
    /// How much each shape is offset from it's original sample position.
    pub stroke_jitter: f32,
    /// At point should the outline of the shapes invert.
    pub stroke_inversion_threshold: f32,
    /// The alpha of the first shapes drawn.
    /// They are rendered back to front, so these will be the background.
    /// This is in the space 0..1.
    pub alpha: f32,
    /// How much the alpha of the shapes should increase each time one is drawn.
    pub alpha_increase: f32,
    /// The number of edges each shape can be drawn with.
    pub edge_count: E,
    /// The size of the first shapes drawn.
    /// They are rendered back to front, so these will be the background.
    pub stroke_size: f32,
    /// How much smaller the stroke should become each time one is drawn.
    pub stroke_reduction: f32,
    /// Randomize rotation multiplier
    pub randomize_rotation: f32,
    /// The number of shapes to render.
    pub shapes: usize,
}

/// Art generator based on Preslav's Book *Generative Art in Go*
pub struct PreslavSketcher<E>
where
    E: Distribution<usize> + Clone,
{
    settings: PreslavSketcherSettings<E>,
    half_size: Vec2,
    scale_mat: Mat2,
    input_image: RasterCanvas,
    canvas: VectorCanvas,
    #[cfg(feature = "small-rng")]
    rng: SmallRng,
}

impl<E> PreslavSketcher<E>
where
    E: Distribution<usize> + Clone,
{
    pub fn new(
        input_image: RasterCanvas,
        settings: PreslavSketcherSettings<E>,
        #[cfg(feature = "small-rng")] seed: u64,
    ) -> Self {
        let major_dimension = usize::min(input_image.width(), input_image.height()) as f32;

        Self {
            settings,
            half_size: Vec2::new(input_image.width() as f32, input_image.height() as f32) / 2.0,
            scale_mat: Mat2::from_cols(
                Vec2::X * (2.0 / major_dimension),
                Vec2::Y * (-2.0 / major_dimension),
            ),
            input_image,
            canvas: VectorCanvas::default(),
            #[cfg(feature = "small-rng")]
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    /// Consumes the sketcher and returns the input image.
    pub fn take_input(self) -> RasterCanvas {
        self.input_image
    }

    /// Runs the next step of the algorithm, thereby painting a new polygon.
    fn draw_shape(&mut self) {
        #[cfg(feature = "thread-rng")]
        let mut rng = rand::thread_rng();
        #[cfg(feature = "small-rng")]
        let mut rng = &mut self.rng;

        let p = Vec2::new(
            rng.gen_range(0.0..(self.input_image.width() as f32)),
            rng.gen_range(0.0..(self.input_image.height() as f32)),
        );

        let jitter = Vec2::new(
            rng.gen_range(-self.settings.stroke_jitter..self.settings.stroke_jitter),
            rng.gen_range(-self.settings.stroke_jitter..self.settings.stroke_jitter),
        );

        // TODO: Clean this up.
        // Transform from input image pixel coordinates to canvas coordinates, then apply jitter.
        let d = self.scale_mat.mul_vec2(p - self.half_size) + jitter;

        let edge_count = (&self.settings.edge_count).sample(&mut rng);
        let mut color: Color = self.input_image.get_pixel(p.x as usize, p.y as usize);
        *color.a_mut() = self.settings.alpha;

        let edge_color = if self.settings.stroke_size <= self.settings.stroke_inversion_threshold {
            if color.r() + color.g() + color.b() / 3.0 < 0.5 {
                Some(Stroke {
                    color: Color::new(1.0, 1.0, 1.0, self.settings.alpha * 2.0),
                    width: 0.001,
                    line_end: LineEnd::Round,
                })
            } else {
                Some(Stroke {
                    color: Color::new(0.0, 0.0, 0.0, self.settings.alpha * 2.0),
                    width: 0.001,
                    line_end: LineEnd::Round,
                })
            }
        } else {
            None
        };

        self.canvas.draw_regular_polygon(
            d,
            edge_count,
            self.settings.stroke_size,
            rng.gen::<f32>() * self.settings.randomize_rotation,
            edge_color,
            Some(color),
        );

        self.settings.stroke_size -= self.settings.stroke_reduction * self.settings.stroke_size;
        self.settings.alpha += self.settings.alpha_increase / self.settings.alpha;
        self.settings.alpha = self.settings.alpha.clamp(0.0, 1.0);
    }

    /// Runs entire sketcher
    fn run<P: Fn(f32)>(&mut self, before_iter: P) {
        for i in 0..self.settings.shapes {
            before_iter(i as f32 / self.settings.shapes as f32);

            self.draw_shape();
        }
        }
}

impl<E, F> Sketcher<F> for PreslavSketcher<E>
where
    E: Distribution<usize> + Clone,
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
