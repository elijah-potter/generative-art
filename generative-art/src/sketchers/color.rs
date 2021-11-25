use std::{ops::{Add, Div, Mul, Rem, Sub}, process::Output};

use glam::Vec4;
use image::{Rgb, Rgba};

/// Color of an object.
/// Range is 0..1.
#[derive(Clone, Copy)]
pub struct Color {
    inner: Vec4,
}

impl Color {
    pub fn white() -> Self{
        Color::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self{
        Color::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn transparent() -> Self{
        Color::new(0.0, 0.0, 0.0, 0.0)
    }
    

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            inner: Vec4::new(r, g, b, a),
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.inner.x
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.inner.y
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.inner.z
    }

    #[inline]
    pub fn a(&self) -> f32 {
        self.inner.w
    }

    #[inline]
    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self.inner.x
    }

    #[inline]
    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self.inner.y
    }

    #[inline]
    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.inner.z
    }

    #[inline]
    pub fn a_mut(&mut self) -> &mut f32 {
        &mut self.inner.w
    }

    /// Get as a hex string. Alpha is optional.
    pub fn as_hex(&self, include_alpha: bool) -> String {
        if include_alpha {
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                (self.r() * 255.0) as u8,
                (self.g() * 255.0) as u8,
                (self.b() * 255.0) as u8,
                (self.a() * 255.0) as u8
            )
        } else {
            format!(
                "#{:02X}{:02X}{:02X}",
                (self.r() * 255.0) as u8,
                (self.g() * 255.0) as u8,
                (self.b() * 255.0) as u8
            )
        }
    }
}

impl From<Rgb<u8>> for Color {
    fn from(rgb: Rgb<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                1.0,
            ),
        }
    }
}

impl From<&Rgb<u8>> for Color {
    fn from(rgb: &Rgb<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                1.0,
            ),
        }
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgb: Rgba<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                rgb.0[3] as f32 / 255.0,
            ),
        }
    }
}

impl From<&Rgba<u8>> for Color {
    fn from(rgb: &Rgba<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                rgb.0[3] as f32 / 255.0,
            ),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            inner: self.inner * rhs,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {
            inner: self.inner * rhs,
        }
    }
}

impl Rem<f32> for Color{
    type Output = Color;

    fn rem(self, rhs: f32) -> Self::Output {
        Color{
            inner: self.inner % rhs
        }
    }
    
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner - rhs.inner,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner * rhs.inner,
        }
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner / rhs.inner,
        }
    }
}

impl Rem<Color> for Color{
    type Output = Color;

    fn rem(self, rhs: Color) -> Self::Output {
        Color{
            inner: self.inner % rhs.inner
        }
    }
    
}