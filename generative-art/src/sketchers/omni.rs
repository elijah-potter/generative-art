use std::{path::Path, io};

use denim::{
    regular_polygon_points,
    renderers::{skia_renderer::{SkiaRenderer, SkiaRendererSettings, ToRgbaImage}, svg_renderer::{SvgRenderer, SvgRendererSettings}},
    CanvasElement, CanvasElementVariant, Color, UVec2, Vec2,
};

use crate::canvas::VectorCanvas;

use super::{RasterSketcher, VectorSketcher};

/// A sketcher wraps either a VectorSketcher or a RasterSketcher and allows to use each interchangably.
///
/// ## Notes about conversions.
/// If the resulting VectorCanvas or RasterCanvas needs to be converted to the opposite type, that processing is not included in `progress` for `before_iter`.
///
/// To convert a RasterCanvas to a VectorCanvas, each pixel is simply converted to a [Square](denim::CanvasElementVariant::Polygon) with the corresponding [Color](denim::Color) and position.
/// To convert a VectorCanvas to a RasterCanvas, it is simply rendered using a
pub enum OmniSketcher<F> {
    VectorSketcher {
        inner: Box<dyn VectorSketcher<F>>,
        /// If it needs to be rendered to a [RasterCanvas](crate::canvas::RasterCanvas), what resolution to be done at.
        resolution: UVec2,
        /// If it needs to be rendered to a [RasterCanvas](crate::canvas::RasterCanvas), what the background should be filled with.
        background_color: Option<Color>,
    },
    RasterSketcher {
        inner: Box<dyn RasterSketcher<F>>,
    },
}

impl<F> OmniSketcher<F>
where
    F: Fn(f32),
{
    pub fn from_vector(
        vector_sketcher: Box<dyn VectorSketcher<F>>,
        resolution: UVec2,
        background_color: Option<Color>,
    ) -> Self {
        Self::VectorSketcher {
            inner: vector_sketcher,
            resolution,
            background_color,
        }
    }

    pub fn from_raster(raster_sketcher: Box<dyn RasterSketcher<F>>) -> Self {
        Self::RasterSketcher {
            inner: raster_sketcher,
        }
    }
}

impl<F> VectorSketcher<F> for OmniSketcher<F>
where
    F: Fn(f32),
{
    fn run(&mut self, before_iter: F) -> crate::canvas::VectorCanvas {
        match self {
            OmniSketcher::VectorSketcher {
                inner,
                resolution,
                background_color,
            } => inner.run(before_iter),
            OmniSketcher::RasterSketcher { inner } => {
                // Do the conversion to a VectorCanvas
                
            }
        }
    }
}

impl<F> RasterSketcher<F> for OmniSketcher<F>
where
    F: Fn(f32),
{
    fn run(&mut self, before_iter: F) -> crate::canvas::RasterCanvas {
        match self {
            OmniSketcher::VectorSketcher {
                inner,
                resolution,
                background_color,
            } => {
                // Do the conversion to a raster image.
                let vector = inner.run(before_iter);

                vector
                    .render::<SkiaRenderer>(SkiaRendererSettings {
                        size: resolution.to_owned(),
                        background_color: background_color.to_owned(),
                    })
                    .to_rgba_image()
                    .into()
            }
            OmniSketcher::RasterSketcher { inner } => inner.run(before_iter),
        }
    }
}
