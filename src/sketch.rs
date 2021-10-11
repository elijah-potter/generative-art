use image::{Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_line_segment_mut, draw_polygon_mut, Blend, Canvas},
    point::Point,
};
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::{convert::TryInto, f64::consts::PI};

pub struct Sketcher {
    pub stroke_ratio: f64,
    pub stroke_reduction: f64,
    pub stroke_jitter: f64,
    pub stroke_inversion_threshold: f64,
    pub alpha: f64,
    pub alpha_increase: f64,
    pub min_edge_count: u32,
    pub max_edge_count: u32,
    pub stroke_size: f64,
    pub initial_stroke_size: f64,
    canvas: Blend<RgbaImage>,
    rng: Option<StdRng>,
}

impl Default for Sketcher{
    fn default() -> Self {
        Self {
            stroke_ratio: Default::default(),
            stroke_reduction: Default::default(),
            stroke_jitter: Default::default(),
            stroke_inversion_threshold: Default::default(),
            alpha: Default::default(),
            alpha_increase: Default::default(),
            min_edge_count: Default::default(),
            max_edge_count: Default::default(),
            stroke_size: Default::default(),
            initial_stroke_size: Default::default(),
            canvas: Blend(RgbaImage::new(0, 0)),
            rng: Default::default(),
        }
    }
}

impl Sketcher {
    /// Create a new sketcher with default values based on Preslav Rachev's suggestions
    pub fn new_preslav(canvas: RgbaImage, expected_iterations: usize) -> Self {
        let initial_stroke_size = canvas.width() as f64 / 4.0;

        Self {
            stroke_ratio: 0.75,
            stroke_reduction: initial_stroke_size / 70.0 / expected_iterations as f64,
            stroke_jitter: 0.1 * canvas.width() as f64,
            stroke_inversion_threshold: 0.05,
            alpha: 70.0,
            alpha_increase: (256.0 - 70.0) / expected_iterations as f64,
            min_edge_count: 3,
            max_edge_count: 4,
            stroke_size: initial_stroke_size,
            initial_stroke_size,
            canvas: Blend(canvas),
            rng: Default::default(),
        }
    }

    /// Runs a single iteration
    pub fn draw_iter(&mut self, input: &RgbaImage) {
        if self.rng.is_none() {
            let seed = match input.chunks(32).next() {
                Some(chunk) => chunk.to_owned().try_into().unwrap(),
                None => [0; 32],
            };

            self.rng = Some(StdRng::from_seed(seed))
        }

        let rng = self.rng.as_mut().unwrap();

        let x = rng.gen_range(0.0..(self.canvas.width() as f64));
        let y = rng.gen_range(0.0..(self.canvas.height() as f64));

        let dx = x + rng.gen_range(-self.stroke_jitter..self.stroke_jitter);
        let dy = y + rng.gen_range(-self.stroke_jitter..self.stroke_jitter);

        let edge_count = rng.gen_range(self.min_edge_count..(self.max_edge_count + 1));

        let mut color = input.get_pixel(x as u32, y as u32).to_owned();
        color.0[3] = self.alpha as u8;

        let edge_color =
            if self.stroke_size <= self.stroke_inversion_threshold * self.initial_stroke_size {
                if color.0.iter().take(3).map(|v| *v as f64).sum::<f64>() / 3.0 < 128.0 {
                    Some(Rgba([255, 255, 255, (self.alpha * 2.0) as u8]))
                } else {
                    Some(Rgba([0, 0, 0, (self.alpha * 2.0) as u8]))
                }
            } else {
                None
            };

        let polygon_points =
            Self::regular_polygon_points(dx, dy, self.stroke_size, edge_count, rng.gen());
        draw_polygon_mut(&mut self.canvas, &polygon_points, color);

        if let Some(edge_color) = edge_color {
            Self::draw_hollow_polygon_mut(&mut self.canvas, &polygon_points, edge_color);
        }

        self.stroke_size -= self.stroke_reduction * self.stroke_size;
        self.alpha += self.alpha_increase;
    }

    fn regular_polygon_points(
        x: f64,
        y: f64,
        radius: f64,
        sides: u32,
        theta: f64,
    ) -> Vec<Point<i32>> {
        let mut points = Vec::with_capacity(sides as usize);

        for n in 0..sides {
            points.push(Point::new(
                (radius * (2.0 * PI * n as f64 / sides as f64 + theta).cos() + x) as i32,
                (radius * (2.0 * PI * n as f64 / sides as f64 + theta).sin() + y) as i32,
            ));
        }

        if points.first() == points.last() {
            return vec![
                Point::new(x as i32, y as i32),
                Point::new(x as i32 + 1, y as i32 + 1),
            ];
        }

        points
    }

    pub fn draw_hollow_polygon_mut<C>(canvas: &mut C, poly: &[Point<i32>], color: C::Pixel)
    where
        C: Canvas,
        C::Pixel: 'static,
    {
        for i in 0..(poly.len() - 1) {
            draw_line_segment_mut(
                canvas,
                (poly[i].x as f32, poly[i].y as f32),
                (poly[i + 1].x as f32, poly[i + 1].y as f32),
                color,
            )
        }

        draw_line_segment_mut(
            canvas,
            (poly[0].x as f32, poly[0].y as f32),
            (poly[poly.len() - 1].x as f32, poly[poly.len() - 1].y as f32),
            color,
        );
    }

    pub fn get_canvas(&self) -> &RgbaImage {
        &self.canvas.0
    }

}
