use image::RgbaImage;

use super::Color;

#[derive(Clone)]
pub struct RasterCanvas {
    width: usize,
    height: usize,
    image: Vec<Color>,
}

impl RasterCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            image: vec![Color::transparent(); width * height],
        }
    }

    pub fn from_rgba(image: &RgbaImage) -> Self {
        let mut data = Vec::with_capacity(image.width() as usize * image.height() as usize);

        let flat = image.as_flat_samples();

        for i in 0..(flat.samples.len()) / 4 {
            let i = i * 3;
            data.push(Color::new(
                flat.samples[i] as f32 / 255.0,
                flat.samples[i + 1] as f32 / 255.0,
                flat.samples[i + 2] as f32 / 255.0,
                flat.samples[i + 3] as f32 / 255.0,
            ))
        }

        Self {
            width: image.width() as usize,
            height: image.height() as usize,
            image: data,
        }
    }

    pub fn as_rgba(&self) -> RgbaImage {
        let mut data = vec![0u8; self.width * self.height * 4];

        for (i, pixel) in self.image.iter().enumerate() {
            let i = i * 4;

            data[i] = (pixel.r() * 255.0) as u8;
            data[i + 1] = (pixel.g() * 255.0) as u8;
            data[i + 2] = (pixel.b() * 255.0) as u8;
            data[i + 3] = (pixel.a() * 255.0) as u8;
        }

        RgbaImage::from_raw(self.width as u32, self.height as u32, data).unwrap()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.image[y * self.width + x] = color;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Color {
        self.image[y * self.width + x]
    }

    pub fn get_raw(&self) -> &Vec<Color> {
        &self.image
    }

    pub fn get_raw_mut(&mut self) -> &mut Vec<Color> {
        &mut self.image
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }
}

impl From<RgbaImage> for RasterCanvas {
    fn from(image: RgbaImage) -> Self {
        RasterCanvas::from_rgba(&image)
    }
}
