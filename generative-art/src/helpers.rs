use glam::Vec2;
use image::Rgb;
use rand::{Rng, RngCore};

pub trait RngCoreExt {
    fn random_sign(&mut self) -> f32;
}

impl<R> RngCoreExt for R
where
    R: RngCore,
{
    fn random_sign(&mut self) -> f32 {
        if self.gen::<bool>() {
            1.0
        } else {
            -1.0
        }
    }
}
