use glam::Vec2;
use image::RgbImage;
use rand::{prelude::Distribution, Rng};
#[cfg(feature = "small-rng")]
use rand::{rngs::SmallRng, SeedableRng};

use super::{
    canvas::{Color, RegularPolygon},
    VectorCanvas, VectorSketcher,
};

pub struct PreslavSketcherSettings<E>
where
    E: Distribution<usize> + Clone,
{
    /// The size of the final render.
    pub output_size: Vec2,
    /// How much each shape is offset from it's original sample position.
    pub stroke_jitter: f32,
    /// At point should the outline of the shapes invert.
    pub stroke_inversion_threshold: f32,
    /// The alpha of the first shapes drawn.
    /// They are rendered back to front, so these will be the background.
    /// This is in the space 0..1.
    pub initial_alpha: f32,
    /// How much the alpha of the shapes should increase each time one is drawn.
    pub alpha_increase: f32,
    /// The number of edges each shape can be drawn with.
    pub edge_count: E,
    /// The size of the first shapes drawn.
    /// They are rendered back to front, so these will be the background.
    pub initial_stroke_size: f32,
    /// How much smaller the stroke should become each time one is drawn.
    pub stroke_reduction: f32,
    /// The number of shapes to render.
    pub shapes: usize,
    /// The image to sample from.
    pub input_image: RgbImage,
}

/// Art generator based on Preslav's Book *Generative Art in Go*
pub struct PreslavSketcher<E>
where
    E: Distribution<usize> + Clone,
{
    stroke_jitter: f32,
    stroke_inversion_threshold: f32,
    alpha: f32,
    alpha_increase: f32,
    edge_count: E,
    initial_stroke_size: f32,
    stroke_size: f32,
    stroke_reduction: f32,
    shapes: usize,
    input_image: RgbImage,
    canvas: VectorCanvas,
    #[cfg(feature = "small-rng")]
    rng: SmallRng,
}

impl<E> PreslavSketcher<E>
where
    E: Distribution<usize> + Clone,
{
    pub fn new(
        settings: &PreslavSketcherSettings<E>,
        #[cfg(feature = "small-rng")] seed: u64,
    ) -> Self {
        Self {
            stroke_jitter: settings.stroke_jitter,
            stroke_inversion_threshold: settings.stroke_inversion_threshold,
            alpha: settings.initial_alpha,
            alpha_increase: settings.alpha_increase,
            edge_count: settings.edge_count.clone(),
            initial_stroke_size: settings.initial_stroke_size,
            stroke_size: settings.initial_stroke_size,
            stroke_reduction: settings.stroke_reduction,
            shapes: settings.shapes,
            input_image: settings.input_image.to_owned(),
            canvas: VectorCanvas::default(),
            #[cfg(feature = "small-rng")]
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    /// Runs the next step of the algorithm, thereby painting a new polygon.
    fn draw_shape(&mut self) {
        #[cfg(feature = "thread-rng")]
        let mut rng = rand::thread_rng();
        #[cfg(feature = "small-rng")]
        let mut rng = &mut self.rng;

        let x = rng.gen_range(0.0..(self.input_image.width() as f32));
        let y = rng.gen_range(0.0..(self.input_image.height() as f32));

        let d = Vec2::new(
            x + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
            y + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
        );

        let edge_count = (&self.edge_count).sample(&mut rng);
        let mut color: Color = self.input_image.get_pixel(x as u32, y as u32).into();
        color.a = self.alpha;

        let edge_color =
            if self.stroke_size <= self.stroke_inversion_threshold * self.initial_stroke_size {
                if color.r + color.g + color.b / 3.0 < 0.5 {
                    Some(Color::new(1.0, 1.0, 1.0, self.alpha * 2.0))
                } else {
                    Some(Color::new(0.0, 0.0, 0.0, self.alpha * 2.0))
                }
            } else {
                None
            };

        self.canvas.draw(Box::new(RegularPolygon {
            center: d,
            sides: edge_count,
            rotation: rng.gen(),
            radius: self.stroke_size,
            color,
            outline_color: edge_color,
        }));

        self.stroke_size -= self.stroke_reduction * self.stroke_size;
        self.alpha += self.alpha_increase;
    }
}

impl<E> VectorSketcher for PreslavSketcher<E>
where
    E: Distribution<usize> + Clone,
{
    fn run<F: Fn(f32)>(&mut self, before_iter: F) -> &VectorCanvas {
        for i in 0..self.shapes {
            before_iter(i as f32 / self.shapes as f32);

            self.draw_shape();
        }

        &self.canvas
    }
}
