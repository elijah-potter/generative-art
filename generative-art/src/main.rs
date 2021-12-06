use denim::{Color, Renderer, Vec2};
use rand::distributions::Uniform;
use sketchers::{CelestialSketcher, CelestialSketcherSettings, Sketcher};

mod canvas;
mod helpers;
mod sketchers;

fn main() {
    let settings = CelestialSketcherSettings {
        output_size: Vec2::splat(1000.0),
        object_count: 3,
        render_count: 3,
        object_position: Uniform::new(0.0, 1000.0),
        object_size: Uniform::new(0.0, 100.0),
        object_velocity: Uniform::new(0.0, 1.0),
        g: 1.0,
        foreground: Color::white(),
        steps: 1000,
        step_length: 0.01,
        render_dots: false,
    };

    let mut sketcher = CelestialSketcher::new(settings);

    let canvas = sketcher.run(|_| {});

    canvas.save("image.png", Vec2::splat(1000.0), None);
}
