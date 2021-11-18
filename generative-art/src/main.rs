
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;



use glam::Vec2;
use image::Pixel;
use sketchers::CelestialSketcher;
use sketchers::CelestialSketcherSettings;
use sketchers::Color;
use sketchers::PreslavSketcher;
use sketchers::PreslavSketcherSettings;
use sketchers::VectorCanvas;
use sketchers::VectorSketcher;


use image::Rgba;
use image::RgbaImage;

mod helpers;
mod sketchers;

use indicatif::ProgressBar;

use rand::distributions::Uniform;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Art")]
enum Opt {
    Preslav {
        #[structopt(parse(from_os_str))]
        /// Name of file to use as input
        input: PathBuf,
        #[structopt(short, long, parse(from_os_str), default_value = "generated.svg")]
        /// Name of file to save generated art to.
        /// If not specified, the result will be writted to "generated.svg".
        output: PathBuf,
        #[structopt(long)]
        /// Defines the initial size of shapes.
        /// By default, this is automatically determined by the image width.
        initial_stroke_size: Option<f32>,
        #[structopt(long)]
        /// Defines how much smaller each shape gets per iteration.
        /// If not specified, it will be decided based on the number of expected steps.
        stroke_reduction: Option<f32>,
        #[structopt(long)]
        /// This specifies how large the random offset each shape should be from it's sample location.
        /// By default, this is automatically determined by the width of the image.
        stroke_jitter: Option<f32>,
        #[structopt(long, default_value = "0.05")]
        /// The maximum threshold at which the program will begin drawing edges on the shapes.
        stroke_inversion_threshold: f32,
        #[structopt(long, default_value = "0.274")]
        /// The alpha the program will start at (background shapes).
        /// This is in the space 0.0..1.0.
        initial_alpha: f32,
        #[structopt(long)]
        /// How much the alpha of each shape should increase by each iteration.
        /// By default, this is automatically determined by the number of expected steps.
        alpha_increase: Option<f32>,
        #[structopt(long, default_value = "3")]
        /// The minimum number of sides a shape can have.
        /// This must be more than 3.
        min_edge_count: usize,
        #[structopt(long, default_value = "4")]
        /// The maximum number of sides a shape can have.
        /// This must be greater or equal to min-edge-count.
        max_edge_count: usize,
        #[structopt(short, long, default_value = "5000")]
        /// The number of steps to run.
        steps: usize,
    },
    Celestial {
        #[structopt(short, long, default_value = "celestial.svg")]
        /// The file to render to.
        output: PathBuf,
        #[structopt(short, long, default_value = "1000")]
        /// The width of the output image.
        width: u32,
        #[structopt(short, long, default_value = "1000")]
        /// The hight of the output image.
        height: u32,
        #[structopt(long, default_value = "3")]
        /// Defines the number of celestial bodies in the simulation.
        object_count: usize,
        #[structopt(long, default_value = "1.0")]
        /// Defines the minimum mass that can be assigned to a object.
        min_mass: f32,
        #[structopt(long, default_value = "100.0")]
        /// Defines the maximum mass that can be assigned to a object.
        max_mass: f32,
        #[structopt(long, default_value = "2.0")]
        /// Defines the minimum initial velocity that can be assigned to a object.
        minimum_initial_velocity: f32,
        #[structopt(long, default_value = "1000.0")]
        /// Defines the maximum initial velocity that can be assigned to a object.
        maximum_initial_velocity: f32,
        #[structopt(short, default_value = "1.0")]
        /// Defines G.
        g: f32,
        #[structopt(long)]
        /// The maximum number of pixels from the center of the image that an object can be instantiated.
        /// If omitted, they can be instantiated anywhere on the screen.
        maximum_radius_from_center: Option<f32>,
        #[structopt(long)]
        /// Whether to increase the initial mass of the bodies the further they are instantiated from the center of the image.
        increase_mass_with_distance: bool,
        #[structopt(short, long)]
        /// Whether to render dots instead of paths.
        dots: bool,
        #[structopt(long)]
        /// How many of the objects in the simulation will be rendered.
        /// By default, this is equal to object-count.
        /// This must be less than or equal to object-count.
        render_count: Option<usize>,
        #[structopt(short, long, default_value = "10000")]
        /// How many steps the simulation is run.
        steps: usize,
        #[structopt(long, default_value = "0.005")]
        /// How much time passes between steps.
        step_size: f32,
    },
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Preslav {
            input,
            output,
            initial_stroke_size,
            stroke_reduction,
            stroke_jitter,
            stroke_inversion_threshold,
            initial_alpha,
            alpha_increase,
            min_edge_count,
            max_edge_count,
            steps,
        } => {
            // Verify that inputs are valid.
            if min_edge_count > max_edge_count {
                println!("The min-edge-count cannot be greater than the max-edge-count");
                return Ok(());
            }
            if min_edge_count < 3 || max_edge_count < 3 {
                println!("The min-edge-count and max-edge-count cannot be less than 3.");
                return Ok(());
            }

            let in_image = image::open(input)?;
            let in_image = in_image.into_rgb8();

            let dimensions = Vec2::new(in_image.width() as f32, in_image.height() as f32);

            let settings = PreslavSketcherSettings {
                output_size: dimensions,
                stroke_jitter: stroke_jitter.unwrap_or_else(|| 0.1 * dimensions.x),
                stroke_inversion_threshold,
                initial_alpha,
                alpha_increase: alpha_increase.unwrap_or_else(|| (1.0 - 0.274) / steps as f32),
                edge_count: Uniform::new(min_edge_count, max_edge_count),
                initial_stroke_size: initial_stroke_size.unwrap_or_else(|| dimensions.x / 4.0),
                stroke_reduction: stroke_reduction
                    .unwrap_or_else(|| dimensions.x / 4.0 / 70.0 / steps as f32),
                shapes: steps,
                input_image: in_image,
            };

            #[cfg(feature = "thread-rng")]
            let mut sketcher = PreslavSketcher::new(&settings);

            #[cfg(feature = "small-rng")]
            let mut sketcher = PreslavSketcher::new(
                &settings,
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );

            #[cfg(not(feature = "wasm"))]
            let bar = ProgressBar::new(100);

            save(
                sketcher.run(|progress| {
                    #[cfg(not(feature = "wasm"))]
                    bar.set_length((progress * 100.0) as u64);
                }),
                output.as_path(),
                Vec2::new(
                    settings.input_image.width() as f32,
                    settings.input_image.height() as f32,
                ),
            )?;
        }
        Opt::Celestial {
            output,
            width,
            height,
            object_count,
            min_mass,
            max_mass,
            minimum_initial_velocity,
            maximum_initial_velocity,
            g,
            maximum_radius_from_center: _max_radius_from_center,
            increase_mass_with_distance: _,
            dots,
            render_count,
            steps,
            step_size,
        } => {
            // Verify that inputs are valid.
            if min_mass > max_mass {
                println!("The minimum-mass cannot be greater than the maximum-mass.");
                return Ok(());
            }
            if minimum_initial_velocity > maximum_initial_velocity {
                println!("The minimum-initial-velocity cannot be greater than the maximum-initial-velocity.");
                return Ok(());
            }
            if let Some(render_count) = render_count {
                if render_count > object_count {
                    println!("The render-count cannot be greater than the object-count.");
                    return Ok(());
                }
            }

            let mut initial_image = RgbaImage::new(width, height);
            for (_x, _y, pixel) in initial_image.enumerate_pixels_mut() {
                *pixel = Rgba::from_channels(0, 0, 0, 255);
            }

            let settings = CelestialSketcherSettings {
                output_size: Vec2::new(width as f32, height as f32),
                object_count,
                object_size: Uniform::new(min_mass, max_mass),
                object_velocity: Uniform::new(minimum_initial_velocity, maximum_initial_velocity),
                g,
                render_count: render_count.unwrap_or(object_count),
                object_position: Uniform::new(0.0, width as f32),
                foreground: Color::WHITE,
                steps,
                step_length: step_size,
                render_dots: dots,
            };

            let mut sketcher = CelestialSketcher::new(&settings);

            #[cfg(not(feature = "wasm"))]
            let bar = ProgressBar::new(100);

            sketcher.run(|progress| {
                #[cfg(not(feature = "wasm"))]
                bar.set_length((progress * 100.0) as u64);
            });

            save(
                sketcher.run(|progress| {
                    #[cfg(not(feature = "wasm"))]
                    bar.set_length((progress * 100.0) as u64);
                }),
                output.as_path(),
                settings.output_size
            )?;
        }
    }

    Ok(())
}

fn save(canvas: &VectorCanvas, path: &Path, size: Vec2) -> anyhow::Result<()> {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap() {
            "svg" => {
                let svg = canvas.render_svg(size, Some(Color::new(0.0, 0.0, 0.0, 1.0)));

                let mut file = File::create(path)?;
                file.write_all(svg.as_bytes())?;
            }
            "bmp" | "jpg" | "jpeg" | "png" | "tiff" => {
                let image = canvas.render_rgb(size, 1.0, Some(Color::BLACK));

                image.save(path)?;
            }
            _ => println!("Couldn't save with that extension."),
        }
    } else {
        println!("Couldn't save with that extension.")
    }

    Ok(())
}
