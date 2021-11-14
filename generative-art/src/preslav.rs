use crate::helpers::{clean_canvas, regular_polygon_points, RgbExt};
use glam::Vec2;
use image::{Rgb, RgbImage};
use rand::Rng;
use svg::{node::element::Polygon, Document, Node};

pub struct PreslavSketcherSettings {
    pub output_size: Vec2,
    pub expected_iterations: usize,
    pub stroke_reduction: f32,
    pub stroke_jitter: f32,
    pub stroke_inversion_threshold: f32,
    pub initial_alpha: f32,
    pub alpha_increase: f32,
    pub min_edge_count: u32,
    pub max_edge_count: u32,
    pub initial_stroke_size: f32,
}

/// Art generator based on Preslav's Book *Generative Art in Go*
pub struct PreslavSketcher {
    stroke_reduction: f32,
    stroke_jitter: f32,
    stroke_inversion_threshold: f32,
    initial_alpha: f32,
    alpha_increase: f32,
    min_edge_count: u32,
    max_edge_count: u32,
    stroke_size: f32,
    initial_stroke_size: f32,
    canvas: Document,
}

impl PreslavSketcher {
    pub fn new(settings: &PreslavSketcherSettings) -> Self {
        Self {
            stroke_reduction: settings.stroke_reduction,
            stroke_jitter: settings.stroke_jitter,
            stroke_inversion_threshold: settings.stroke_inversion_threshold,
            initial_alpha: settings.initial_alpha,
            alpha_increase: settings.alpha_increase,
            min_edge_count: settings.min_edge_count,
            max_edge_count: settings.max_edge_count,
            stroke_size: settings.initial_stroke_size,
            initial_stroke_size: settings.initial_stroke_size,
            canvas: clean_canvas(settings.output_size),
        }
    }

    pub fn step(&mut self, input: &RgbImage) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0.0..(input.width() as f32));
        let y = rng.gen_range(0.0..(input.height() as f32));

        let d = Vec2::new(
            x + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
            y + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
        );

        let edge_count = rng.gen_range(self.min_edge_count..(self.max_edge_count + 1));
        let color = input.get_pixel(x as u32, y as u32).to_owned();

        let edge_color =
            if self.stroke_size <= self.stroke_inversion_threshold * self.initial_stroke_size {
                if color.0.iter().take(3).map(|v| *v as f32).sum::<f32>() / 3.0 < 128.0 {
                    Some((Rgb([255, 255, 255]), (self.initial_alpha * 2.0)))
                } else {
                    Some((Rgb([0, 0, 0]), (self.initial_alpha * 2.0)))
                }
            } else {
                None
            };

        let polygon_points = regular_polygon_points(d, self.stroke_size, edge_count, rng.gen());
        let mut polygon = Polygon::new()
            .set("points", polygon_points)
            .set("fill", color.as_hex())
            .set("fill-opacity", self.initial_alpha);

        if let Some(edge_color) = edge_color {
            polygon.assign("stroke", edge_color.0.as_hex());
            polygon.assign("stroke-opacity", edge_color.1);
        }

        self.canvas.append(polygon);

        self.stroke_size -= self.stroke_reduction * self.stroke_size;
        self.initial_alpha += self.alpha_increase;
    }

    pub fn render(&self) -> Document {
        self.canvas.clone()
    }
}
