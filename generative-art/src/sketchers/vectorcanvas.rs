use std::{
    f32::consts::PI,
    fmt::{self, Write},
};

use glam::Vec2;
use image::{RgbaImage};

use super::Color;

#[derive(Default, Clone)]
pub struct VectorCanvas {
    elements: Vec<Box<dyn VectorElement<String>>>,
}

impl VectorCanvas {
    /// Add an element to the canvas
    pub fn draw(&mut self, element: Box<dyn VectorElement<String>>) {
        self.elements.push(element);
    }

    /// Renders the canvas to SVG.
    pub fn render_svg(&self, size: Vec2, background: Option<Color>) -> String {
        let mut output = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">",
            size.x, size.y
        );

        if let Some(background) = background {
            let _ = write!(
                output,
                "<rect fill=\"{}\" width=\"{}\" height=\"{}\"/>",
                background.as_hex(false),
                size.x,
                size.y
            );
        }

        for element in &self.elements {
            let _ = element.write_svg(&mut output);
        }

        let _ = write!(output, "</svg>");

        output
    }

    /// Renders the canvas to a raster image.
    /// This works by first rendering the canvas to SVG, then, using resvg, rendering it to raster.
    pub fn render_rgb(&self, size: Vec2, zoom: f32, background: Option<Color>) -> RgbaImage {
        let svg = self.render_svg(size, background);

        let options = usvg::Options::default();
        let tree = usvg::Tree::from_str(&svg, &options.to_ref()).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(size.x as u32, size.y as u32).unwrap();
        resvg::render(&tree, usvg::FitTo::Zoom(zoom), pixmap.as_mut()).unwrap();

        RgbaImage::from_raw(size.x as u32, size.y as u32, pixmap.data().to_owned()).unwrap()
    }
}

pub trait VectorElement<W: Write>: VectorElementClone<W> {
    /// Converts the type to an SVG String.
    fn write_svg(&self, w: &mut W) -> fmt::Result;
}

/// Blanket trait that allows all VectorElements to be cloneable.
pub trait VectorElementClone<W> {
    fn clone_box(&self) -> Box<dyn VectorElement<W>>;
}

impl<T, W> VectorElementClone<W> for T
where
    W: Write,
    T: 'static + VectorElement<W> + Clone,
{
    fn clone_box(&self) -> Box<dyn VectorElement<W>> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl<W> Clone for Box<dyn VectorElement<W>> {
    fn clone(&self) -> Box<dyn VectorElement<W>> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Line {
    pub points: Vec<Vec2>,
    pub radius: f32,
    pub color: Color,
}

impl<W: Write> VectorElement<W> for Line {
    fn write_svg(&self, w: &mut W) -> fmt::Result {
        write!(w, "<polyline points=\"")?;

        for point in &self.points {
            write!(w, "{},{} ", point.x, point.y)?;
        }

        write!(
            w,
            "\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\" stroke-linecap=\"round\" ",
            self.color.as_hex(false),
            self.radius * 2.0
        )?;

        if self.color.a() < 1.0 {
            write!(w, "stroke-opacity=\"{}\" ", self.color.a())?;
        }

        write!(w, "/>")
    }
}

#[derive(Clone)]
pub struct Polygon {
    pub points: Vec<Vec2>,
    pub color: Color,
    pub outline_color: Option<Color>,
}

impl<W: Write> VectorElement<W> for Polygon {
    fn write_svg(&self, w: &mut W) -> fmt::Result {
        write!(w, "<polygon points=\"")?;

        for point in &self.points {
            write!(w, "{},{} ", point.x, point.y)?;
        }

        write!(w, "\" fill=\"{}\" ", self.color.as_hex(false))?;

        if self.color.a() < 1.0 {
            write!(w, "fill-opacity=\"{}\" ", self.color.a())?;
        }

        if let Some(outline_color) = self.outline_color {
            write!(w, "stroke=\"{}\" ", outline_color.as_hex(false))?;

            if outline_color.a() < 1.0 {
                write!(w, "stroke-opacity=\"{}\" ", self.color.a())?;
            }
        }

        write!(w, "/>")
    }
}

#[derive(Clone)]
pub struct RegularPolygon {
    pub center: Vec2,
    pub sides: usize,
    pub rotation: f32,
    pub radius: f32,
    pub color: Color,
    pub outline_color: Option<Color>,
}

impl<W: Write> VectorElement<W> for RegularPolygon {
    fn write_svg(&self, w: &mut W) -> fmt::Result {
        self.as_polygon().write_svg(w)
    }
}

impl RegularPolygon {
    pub fn as_polygon(&self) -> Polygon {
        let mut points = Vec::new();

        for n in 0..self.sides {
            points.push(Vec2::new(
                self.radius * (2.0 * PI * n as f32 / self.sides as f32 + self.rotation).cos()
                    + self.center.x,
                self.radius * (2.0 * PI * n as f32 / self.sides as f32 + self.rotation).sin()
                    + self.center.y,
            ))
        }

        Polygon {
            points,
            color: self.color,
            outline_color: self.outline_color,
        }
    }
}

#[derive(Clone)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
    pub color: Color,
}

impl<W: Write> VectorElement<W> for Circle {
    fn write_svg(&self, w: &mut W) -> fmt::Result {
        write!(
            w,
            "<circle fill=\"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\"/>",
            self.color.as_hex(false),
            self.center.x,
            self.center.y,
            self.radius
        )
    }
}
