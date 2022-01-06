use std::f32::consts::PI;

use denim::{Color, Mat2, UVec2, Vec2};

use crate::canvas::{OmniCanvas, RasterCanvas, VectorCanvas};

use super::Sketcher;

#[derive(Clone)]
pub struct HalftoneSketcherSettings {
    pub dot_density: f32,
    pub dot_scale: f32,
    pub dot_sides: usize,
    pub dot_color: Color,
}

pub struct HalftoneSketcher {
    settings: HalftoneSketcherSettings,
    input_image: RasterCanvas,
    canvas: VectorCanvas,
}

impl HalftoneSketcher {
    pub fn new(input_image: RasterCanvas, settings: HalftoneSketcherSettings) -> Self {
        Self {
            settings,
            input_image,
            canvas: VectorCanvas::new(),
        }
    }

    fn run<P: Fn(f32)>(&mut self, before_iter: P) {
        let half_size = Vec2::new(
            self.input_image.width() as f32,
            self.input_image.height() as f32,
        ) / 2.0;

        let major_dimension =
            usize::min(self.input_image.width(), self.input_image.height()) as f32;

        let scale_mat = Mat2::from_cols(
            Vec2::X * (2.0 / major_dimension),
            Vec2::Y * (-2.0 / major_dimension),
        );

        let rotate_mat = Mat2::from_angle(PI / 4.0);

        let dot_dim = Vec2::new(
            (self.input_image.width() as f32 / self.settings.dot_density),
            (self.input_image.height() as f32 / self.settings.dot_density),
        );

        let dot_count = (dot_dim.x * dot_dim.y) as f32;

        let mut i = -dot_dim;

        while i.x < dot_dim.x * 2.0{
            while i.y < dot_dim.y * 2.0 {
                before_iter(i.x * dot_dim.x * 3.0 * i.y * dot_dim.y * 3.0 / dot_count);

                let mut dot_pos = i * self.settings.dot_density
                    + self.settings.dot_density / 2.0;

                dot_pos -= half_size;
                dot_pos = rotate_mat.mul_vec2(dot_pos);
                dot_pos += half_size;

                let dot_size = self.settings.dot_density / major_dimension
                    * (1.0
                        - self
                            .input_image
                            .get_pixel(dot_pos.x as usize, dot_pos.y as usize)
                            .value())
                    / 2.0_f32.sqrt() * self.settings.dot_scale;

                self.canvas.draw_regular_polygon(
                    scale_mat.mul_vec2(dot_pos - half_size),
                    self.settings.dot_sides,
                    dot_size,
                    0.0,
                    None,
                    Some(self.settings.dot_color),
                );

                i.y += 1.0;
            }
            i.y = -dot_dim.y;
            i.x += 1.0;
        }
    }
}

impl<F> Sketcher<F> for HalftoneSketcher
where
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
