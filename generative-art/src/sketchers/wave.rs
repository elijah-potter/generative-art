use std::f32::consts::PI;

use denim::{Stroke, Vec2};
use crate::canvas::{OmniCanvas, RasterCanvas, VectorCanvas};

use super::Sketcher;

#[derive(Clone)]
pub struct WaveSketcherSettings {
    /// What stroke to create the lines with
    pub stroke: Stroke,
    /// Only draw every __ rows of pixels
    pub skip_rows: usize,
    /// Adjust the frequency of the lines.
    pub frequency_multiplier: f32,
    /// Adjust the amplitude of the lines.
    pub amplitude_multiplier: f32,
    /// Invert color so that dark areas have higher frequency
    pub invert_brightness: bool,
    /// Set a brightness threshold under which, no lines are drawn. This is effected by [invert_brightness](Self::invert_brightness)
    pub brightness_threshold: f32,
    /// Radius of a box blur applied to the input image. For no box blur, set this to zero.
    pub box_blur_radius: usize,
    /// Modulate stroke with frequency
    pub stroke_with_frequency: bool,
}

pub struct WaveSketcher {
    settings: WaveSketcherSettings,
    input_image: RasterCanvas,
    canvas: VectorCanvas,
}

impl WaveSketcher {
    pub fn new(input_image: RasterCanvas, settings: WaveSketcherSettings) -> Self {
        Self {
            settings,
            input_image,
            canvas: VectorCanvas::new(),
        }
    }

    fn run<P: Fn(f32)>(&mut self, before_iter: P) {
        // Scale a pixel to fit within canvas bounds of -1..1.
        let aspect_ratio = self.input_image.width() as f32 / self.input_image.height() as f32;

        let pixel_scale = Vec2::new(
            2.0 / self.input_image.width() as f32,
            2.0 / self.input_image.height() as f32,
        );

        // Create a line for each row of pixels, skipping the necessary number.
        let mut row = 0;
        while row < self.input_image.height() {
            before_iter(row as f32 / self.input_image.height() as f32);

            // TODO: Add option to randomize phase
            let mut a = 0.0;

            let mut line_points = Vec::new();

            for x in 0..self.input_image.width() {
                let mut blurred_brightness = 0.0;

                if self.settings.box_blur_radius > 0 {
                    for x in x - self.settings.box_blur_radius..x + self.settings.box_blur_radius {
                        for y in
                            row - self.settings.box_blur_radius..row + self.settings.box_blur_radius
                        {
                            let pixel = self.input_image.get_pixel(x, y);

                            blurred_brightness +=
                                (pixel.r() + pixel.g() + pixel.b()) * pixel.a() / 3.0
                        }
                    }

                    let box_diameter = self.settings.box_blur_radius as f32 * 2.0;

                    blurred_brightness /= box_diameter * box_diameter;
                } else {
                    let pixel = self.input_image.get_pixel(x, row);

                    blurred_brightness = (pixel.r() + pixel.g() + pixel.b()) * pixel.a() / 3.0;
                }

                if blurred_brightness > self.settings.brightness_threshold {
                    let delta_a = blurred_brightness * self.settings.frequency_multiplier;

                    a += delta_a;
                    a %= 2.0 * PI;

                    let y = a.sin() * self.settings.amplitude_multiplier;

                    // TODO: Clean this up.
                    let p = (Vec2::new(x as f32 * aspect_ratio, y + row as f32) * pixel_scale
                        - Vec2::new(aspect_ratio, 1.0))
                        * Vec2::new(1.0, -1.0);

                    line_points.push(p);

                    if self.settings.stroke_with_frequency && line_points.len() >= 2 {
                        self.canvas.draw_shape(
                            line_points,
                            Some(Stroke {
                                color: self.settings.stroke.color,
                                width: self.settings.stroke.width * delta_a * 10.0,
                                line_end: self.settings.stroke.line_end
                            }),
                            None,
                        );
                        line_points = vec![p];
                    }
                } else if line_points.len() >= 2 {
                    self.canvas
                        .draw_shape(line_points, Some(self.settings.stroke), None);
                    line_points = Vec::new();
                } else if !line_points.is_empty() {
                    line_points = Vec::new();
                }
            }

            if line_points.len() >= 2 {
                self.canvas
                    .draw_shape(line_points, Some(self.settings.stroke), None);
            }

            row += 1 + self.settings.skip_rows;
        }
    }
}

impl<P> Sketcher<P> for WaveSketcher
where
    P: Fn(f32),
{
    fn run(&mut self, before_iter: P) -> OmniCanvas {
        // TODO: Add progress bar
        self.run(before_iter);

        self.canvas.clone().into()
    }

    fn run_and_dispose(mut self, before_iter: P) -> OmniCanvas {
        self.run(before_iter);

        self.canvas.into()
    }
}
