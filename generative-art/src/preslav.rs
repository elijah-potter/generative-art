use crate::helpers::{draw_hollow_polygon_mut, regular_polygon_points};
use glam::Vec2;
use image::{Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_line_segment_mut, draw_polygon_mut, Blend, Canvas},
    point::Point,
};
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::{convert::TryInto, f32::consts::PI};

/// Art generator based on Preslav's Book *Generative Art in Go*
pub struct PreslavSketcher {
    pub stroke_reduction: f32,
    pub stroke_jitter: f32,
    pub stroke_inversion_threshold: f32,
    pub initial_alpha: f32,
    pub alpha_increase: f32,
    pub min_edge_count: u32,
    pub max_edge_count: u32,
    pub stroke_size: f32,
    pub initial_stroke_size: f32,
    canvas: Blend<RgbaImage>,
}

impl Default for PreslavSketcher {
    fn default() -> Self {
        Self {
            stroke_reduction: Default::default(),
            stroke_jitter: Default::default(),
            stroke_inversion_threshold: Default::default(),
            initial_alpha: Default::default(),
            alpha_increase: Default::default(),
            min_edge_count: Default::default(),
            max_edge_count: Default::default(),
            stroke_size: Default::default(),
            initial_stroke_size: Default::default(),
            canvas: Blend(RgbaImage::new(0, 0)),
        }
    }
}

impl PreslavSketcher {
    /// Create a new sketcher with default values based on Preslav Rachev's suggestions
    pub fn new_preslav(canvas: RgbaImage, expected_iterations: usize) -> Self {
        let initial_stroke_size = canvas.width() as f32 / 4.0;

        Self {
            stroke_reduction: initial_stroke_size / 70.0 / expected_iterations as f32,
            stroke_jitter: 0.1 * canvas.width() as f32,
            stroke_inversion_threshold: 0.05,
            initial_alpha: 70.0,
            alpha_increase: (256.0 - 70.0) / expected_iterations as f32,
            min_edge_count: 3,
            max_edge_count: 4,
            stroke_size: initial_stroke_size,
            initial_stroke_size,
            canvas: Blend(canvas),
        }
    }

    /// Runs a single iteration
    pub fn draw_iter(&mut self, input: &RgbaImage) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0.0..(self.canvas.width() as f32));
        let y = rng.gen_range(0.0..(self.canvas.height() as f32));

        let d = Vec2::new(
            x + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
            y + rng.gen_range(-self.stroke_jitter..self.stroke_jitter),
        );

        let edge_count = rng.gen_range(self.min_edge_count..(self.max_edge_count + 1));

        let mut color = input.get_pixel(x as u32, y as u32).to_owned();
        color.0[3] = self.initial_alpha as u8;

        let edge_color =
            if self.stroke_size <= self.stroke_inversion_threshold * self.initial_stroke_size {
                if color.0.iter().take(3).map(|v| *v as f32).sum::<f32>() / 3.0 < 128.0 {
                    Some(Rgba([255, 255, 255, (self.initial_alpha * 2.0) as u8]))
                } else {
                    Some(Rgba([0, 0, 0, (self.initial_alpha * 2.0) as u8]))
                }
            } else {
                None
            };

        let polygon_points = regular_polygon_points(d, self.stroke_size, edge_count, rng.gen());
        draw_polygon_mut(&mut self.canvas, &polygon_points, color);

        if let Some(edge_color) = edge_color {
            draw_hollow_polygon_mut(&mut self.canvas, &polygon_points, edge_color);
        }

        self.stroke_size -= self.stroke_reduction * self.stroke_size;
        self.initial_alpha += self.alpha_increase;
    }

    pub fn get_canvas(&self) -> &RgbaImage {
        &self.canvas.0
    }
}
