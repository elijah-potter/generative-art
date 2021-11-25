use glam::Vec2;

use super::{
    rastercanvas::RasterCanvas,
    vectorcanvas::{RegularPolygon, VectorCanvas},
    VectorSketcher,
};

#[derive(Clone)]
pub struct RasterConverterSketcherSettings {
    pub image: RasterCanvas,
}

pub struct RasterConverterSketcher {
    image: RasterCanvas,
}

impl RasterConverterSketcher {
    pub fn new(settings: RasterConverterSketcherSettings) -> Self {
        Self {
            image: settings.image.clone(),
        }
    }
}

impl<F> VectorSketcher<F> for RasterConverterSketcher
where
    F: Fn(f32),
{
    fn run(&mut self, before_iter: F) -> VectorCanvas {
        let mut output = VectorCanvas::default();

        let total_pixels = self.image.width() * self.image.height();
        let width = self.image.width();

        for (i, pixel) in self.image.get_raw_mut().iter().enumerate() {
            before_iter(i as f32 / total_pixels as f32);

            let y = (i / width) as f32;
            let x = (i % width) as f32;

            output.draw(Box::new(RegularPolygon {
                center: Vec2::new(x - 0.5, y - 0.5),
                sides: 4,
                rotation: 0.0,
                radius: 0.5,
                color: *pixel,
                outline_color: None,
            }))
        }

        output
    }
}
