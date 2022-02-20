use std::f32::consts::PI;

use crate::canvas::{OmniCanvas, RasterCanvas, VectorCanvas};
use barium::{Stroke, Vec2, Mat2};

use super::Sketcher;

#[derive(Clone)]
pub struct WaveSketcherSettings {
    /// What stroke to create the lines with.
    pub stroke: Stroke,
    /// Only draw every __ rows of pixels.
    pub skip_rows: usize,
    /// Only draw every __ columns of pixels.
    pub skip_columns: usize,
    /// Adjust the frequency of the lines.
    pub frequency_multiplier: f32,
    /// Adjust the amplitude of the lines.
    pub amplitude_multiplier: f32,
    /// Invert color so that dark areas have higher frequency.
    pub invert_brightness: bool,
    /// Set a brightness threshold under which, no lines are drawn. This is effected by [invert_brightness](Self::invert_brightness).
    pub brightness_threshold: f32,
    /// Radius of a box blur applied to the input image. For no box blur, set this to zero.
    pub box_blur_radius: usize,
    /// Modulate stroke with frequency.
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
            canvas: VectorCanvas::default(),
        }
    }

    fn run<P: Fn(f32)>(&mut self, before_iter: P) {
        let major_dimension = usize::min(self.input_image.width(), self.input_image.height()) as f32;

        let half_size = Vec2::new(
            self.input_image.width() as f32,
            self.input_image.height() as f32,
        ) / 2.0;

        let scale_mat = Mat2::from_cols(
            Vec2::X * (2.0 / major_dimension as f32),
            Vec2::Y * (-2.0 / major_dimension as f32),
        );

        // Create a line for each row of pixels, skipping the necessary number.
        let mut row = 0;
        while row < self.input_image.height() {
            before_iter(row as f32 / self.input_image.height() as f32);

            // TODO: Add option to randomize phase
            let mut a = 0.0;
            let mut line_points = Vec::new();

            let mut column = 0;
            while column < self.input_image.width() {
                let mut blurred_brightness = 0.0;

                if self.settings.box_blur_radius == 0 {
                    let pixel = self.input_image.get_pixel(column, row);

                    blurred_brightness = (pixel.r() + pixel.g() + pixel.b()) * pixel.a() / 3.0;
                } else {
                    for x in column - self.settings.box_blur_radius
                        ..column + self.settings.box_blur_radius
                    {
                        for y in
                            row - self.settings.box_blur_radius..row + self.settings.box_blur_radius
                        {
                            let pixel = self.input_image.get_pixel(x, y);

                            blurred_brightness +=
                                (pixel.r() + pixel.g() + pixel.b()) * pixel.a() / 3.0
                        }
                    }

                    let k = self.settings.box_blur_radius as f32 * 2.0;

                    blurred_brightness /= k * k;
                }

                if blurred_brightness >= self.settings.brightness_threshold {
                    let delta_a = blurred_brightness * self.settings.frequency_multiplier;

                    a += delta_a * (self.settings.skip_columns as f32 + 1.0);
                    a %= 2.0 * PI;

                    let y = a.sin() * self.settings.amplitude_multiplier;

                    let p = scale_mat.mul_vec2(Vec2::new(column as f32, y + row as f32) - half_size);

                    line_points.push(p);

                    if self.settings.stroke_with_frequency && line_points.len() >= 2 {
                        self.canvas.draw_shape(
                            line_points,
                            Some(Stroke {
                                color: self.settings.stroke.color,
                                width: self.settings.stroke.width * delta_a * 10.0,
                                line_end: self.settings.stroke.line_end,
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

                column += 1 + self.settings.skip_columns;
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
        self.run(before_iter);

        self.canvas.clone().into()
    }

    fn run_and_dispose(mut self, before_iter: P) -> OmniCanvas {
        self.run(before_iter);

        self.canvas.into()
    }
}
