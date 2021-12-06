use std::{
    fs, io,
    path::{Path, PathBuf},
};

use denim::{
    rect_polygon_points, regular_polygon_points,
    renderers::{
        skia_renderer::{SkiaRenderer, SkiaRendererSettings},
        svg_renderer::{SvgRenderer, SvgRendererSettings},
    },
    CanvasElement, CanvasElementVariant, Color, UVec2, Vec2,
};
use image::RgbaImage;

/// Which algorithm to use when vectorizing a [RasterCanvas].
pub enum VectorizerStyle {
    /// Converts the [RasterCanvas] by creating [Squares](denim::canvas::CanvasElementVariant::Polygon) at each pixel's location, with the appropriate color.
    Pixels,
}

#[derive(Clone)]
pub enum OmniCanvas {
    VectorCanvas { inner: VectorCanvas },
    RasterCanvas { inner: RasterCanvas },
}

impl OmniCanvas {
    /// Use the OmniCanvas as a [VectorCanvas].
    ///
    /// If the OmniCanvas is a [RasterCanvas], it does conversion using [VectorizerStyle].
    ///
    /// Be warned: this operation is expensive if the [OmniCanvas] is [RasterCanvas](OmniCanvas::RasterCanvas).
    pub fn as_vector_canvas(&self, style: VectorizerStyle) -> VectorCanvas {
        self.clone().into_vector_canvas(style)
    }

    /// Use the OmniCanvas as a [RasterCanvas].
    ///
    /// If the OmniCanvas is a [VectorCanvas], it does conversion using [SkiaRenderer](denim::renderers::skia_renderer::SkiaRenderer).
    ///
    /// Be warned: this operation is expensive if the [OmniCanvas] is [RasterCanvas](OmniCanvas::RasterCanvas).
    pub fn as_raster_canvas(&self, resolution: UVec2) -> RasterCanvas {
        self.clone().into_raster_canvas(resolution)
    }

    /// Consume the [OmniCanvas] and return a [VectorCanvas].
    ///
    /// If the OmniCanvas is a [RasterCanvas], it does conversion using [SkiaRenderer](denim::renderers::skia_renderer::SkiaRenderer).
    ///
    /// Be warned: this operation is expensive if the [OmniCanvas] is [VectorCanvas](OmniCanvas::VectorCanvas).
    pub fn into_vector_canvas(self, style: VectorizerStyle) -> VectorCanvas {
        match self {
            OmniCanvas::VectorCanvas { inner } => inner,
            OmniCanvas::RasterCanvas { mut inner } => {
                let mut vector = VectorCanvas::default();
                match style {
                    VectorizerStyle::Pixels => {
                        let width = inner.width();
                        let height = inner.height();

                        for x in 0..width {
                            for y in 0..height {
                                vector.draw(CanvasElement {
                                    variant: CanvasElementVariant::Polygon {
                                        points: rect_polygon_points(
                                            Vec2::new(x as f32, y as f32),
                                            Vec2::new((x + 1) as f32, (y + 1) as f32),
                                        ),
                                        fill: Some(inner.get_pixel(x, y)),
                                        stroke: None,
                                    },
                                    ..Default::default()
                                })
                            }
                        }
                    }
                }

                vector
            }
        }
    }

    /// Consume the [OmniCanvas] and return a [RasterCanvas].
    ///
    /// If the OmniCanvas is a [VectorCanvas], it does conversion using [SkiaRenderer](denim::renderers::skia_renderer::SkiaRenderer).
    pub fn into_raster_canvas(self, resolution: UVec2) -> RasterCanvas {
        match self {
            OmniCanvas::VectorCanvas { inner } => RasterCanvas::from_rgba(
                &RgbaImage::from_raw(
                    resolution.x,
                    resolution.y,
                    inner
                        .render::<SkiaRenderer>(SkiaRendererSettings {
                            size: resolution,
                            background_color: None,
                        })
                        .take(),
                )
                .unwrap(),
            ),
            OmniCanvas::RasterCanvas { inner } => inner,
        }
    }

    /// Save the OmniCanvas to a file.
    /// 
    /// Can save to:
    /// * Png
    /// * Jpg
    /// * Tiff
    /// * Bmp
    /// * Svg
    pub fn save<T: Into<PathBuf>>(
        &self,
        path: T,
        size: Vec2,
        background_color: Option<Color>,
    ) -> io::Result<()> {
        let path = path.into();

        match path.extension().map(|v| v.to_str().unwrap()) {
            Some("bmp") | Some("png") | Some("jpg") | Some("tiff") => {
                if let Err(err) = self.as_raster_canvas(size.as_uvec2()).as_rgba().save(path) {
                    return Err(io::Error::new(io::ErrorKind::Other, err));
                }
            }
            Some("svg") => {
                fs::write(
                    path,
                    self.as_vector_canvas(VectorizerStyle::Pixels)
                        .render::<SvgRenderer>(SvgRendererSettings {
                            size,
                            background_color,
                        }),
                )?;
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "That is not a valid output type.",
                ))
            }
        }

        Ok(())
    }
}

impl From<VectorCanvas> for OmniCanvas {
    fn from(canvas: VectorCanvas) -> Self {
        Self::VectorCanvas { inner: canvas }
    }
}

impl From<RasterCanvas> for OmniCanvas {
    #[inline]
    fn from(canvas: RasterCanvas) -> Self {
        Self::RasterCanvas { inner: canvas }
    }
}

pub type VectorCanvas = denim::Canvas;

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

        for (_, _, pixel) in image.enumerate_pixels() {
            data.push(pixel.into());
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

    pub fn get_raw(&self) -> &[Color] {
        self.image.as_slice()
    }

    pub fn get_raw_mut(&mut self) -> &mut [Color] {
        self.image.as_mut_slice()
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
