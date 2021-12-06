use denim::{CanvasElement, Color, Stroke, Vec2};

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
        Self {
            stroke_jitter: settings.stroke_jitter,
            stroke_inversion_threshold: settings.stroke_inversion_threshold,
            alpha: settings.initial_alpha,
            alpha_increase: settings.alpha_increase,
            edge_count: settings.edge_count,
            initial_stroke_size: settings.initial_stroke_size,
            stroke_size: settings.initial_stroke_size,
            stroke_reduction: settings.stroke_reduction,
            shapes: settings.shapes,
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

        let x = rng.gen_range(0.0..(self.input_image.width() as f32));
        let y = rng.gen_range(0.0..(self.input_image.height() as f32));

        let d = Vec2::new(
            x + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
            y + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
        );

        let edge_count = (&self.edge_count).sample(&mut rng);
        let mut color: Color = self.input_image.get_pixel(x as usize, y as usize);
        *color.a_mut() = self.alpha;

        let edge_color =
            if self.stroke_size <= self.stroke_inversion_threshold * self.initial_stroke_size {
                if color.r() + color.g() + color.b() / 3.0 < 0.5 {
                    Some(Stroke {
                        color: Color::new(1.0, 1.0, 1.0, self.alpha * 2.0),
                        width: 1.0,
                    })
                } else {
                    Some(Stroke {
                        color: Color::new(0.0, 0.0, 0.0, self.alpha * 2.0),
                        width: 1.0,
                    })
                }
            } else {
                None
            };

        self.canvas.draw(CanvasElement {
            variant: denim::CanvasElementVariant::Polygon {
                points: denim::regular_polygon_points(d, edge_count, self.stroke_size, rng.gen()),
                fill: Some(color),
                stroke: edge_color,
            },
            ..Default::default()
        });

        self.stroke_size -= self.stroke_reduction * self.stroke_size;
        self.alpha += self.alpha_increase;
    }
}

impl<E, F> Sketcher<F> for PreslavSketcher<E>
where
    E: Distribution<usize> + Clone,
    F: Fn(f32),
{
    fn run(&mut self, before_iter: F) -> OmniCanvas {
        for i in 0..self.shapes {
            before_iter(i as f32 / self.shapes as f32);

            self.draw_shape();
        }

        self.canvas.clone().into()
    }
}