use std::path::PathBuf;

use image::Pixel;

use celestial::CelestialSketcher;
use image::Rgba;
use image::RgbaImage;

mod celestial;
mod convert;
mod helpers;
mod preslav;

use indicatif::ProgressBar;
use preslav::PreslavSketcher;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Art")]
enum Opt {
    Preslav {
        #[structopt(parse(from_os_str))]
        /// Name of file to use as input
        input: PathBuf,
        #[structopt(short, long, parse(from_os_str), default_value = "generated.png")]
        /// Name of file to save generated art to.
        /// If not specified, the result will be writted to "generated.png".
        output: PathBuf,
        #[structopt(long)]
        /// Defines the initial size of shapes.
        /// By default, this is automatically determined by the image width.
        initial_stroke_size: Option<f32>,
        #[structopt(long)]
        /// Defines how much smaller each shape gets per iteration.
        /// If not specified, it will be decided based on the number of expected iterations.
        stroke_reduction: Option<f32>,
        #[structopt(long)]
        /// This specifies how large the random offset each shape should be from it's sample location.
        /// By default, this is automatically determined by the width of the image.
        stroke_jitter: Option<f32>,
        #[structopt(long, default_value = "0.05")]
        /// The maximum threshold at which the program will begin drawing edges on the shapes.
        stroke_inversion_threshold: f32,
        #[structopt(long, default_value = "70.0")]
        /// The alpha the program will start at (background shapes).
        initial_alpha: f32,
        #[structopt(long)]
        /// How much the alpha of each shape should increase by each iteration.
        /// By default, this is automatically determined by the number of expected iterations.
        alpha_increase: Option<f32>,
        #[structopt(long, default_value = "3")]
        /// The minimum number of sides a shape can have.
        /// This must be more than 3.
        min_edge_count: u32,
        #[structopt(long, default_value = "4")]
        /// The maximum number of sides a shape can have.
        /// This must be greater or equal to min-edge-count.
        max_edge_count: u32,
        #[structopt(short, long, default_value = "5000")]
        /// The number of iterations to run.
        iterations: usize,
    },
    Celestial {
        #[structopt(short, long, default_value = "celestial.png")]
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
        /// Defines the minimum mass that can be assigned to a body.
        minimum_mass: f32,
        #[structopt(long, default_value = "100.0")]
        /// Defines the maximum mass that can be assigned to a body.
        maximum_mass: f32,
        #[structopt(long, default_value = "2.0")]
        /// Defines the minimum initial velocity that can be assigned to a body.
        minimum_initial_velocity: f32,
        #[structopt(long, default_value = "1000.0")]
        /// Defines the maximum initial velocity that can be assigned to a body.
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
        #[structopt(long)]
        /// How many of the objects in the simulation will be rendered.
        /// By default, this is equal to object-count.
        /// This must be less than or equal to object-count.
        render_count: Option<usize>,
        #[structopt(short, long, default_value = "100000")]
        /// How many steps the simulation is run.
        steps: usize,
        #[structopt(long, default_value = "0.0005")]
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
            iterations,
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
            let in_image = in_image.into_rgba8();

            let bar = ProgressBar::new(iterations as u64);

            let mut sketcher = PreslavSketcher::new_preslav(in_image.to_owned(), iterations);
            if let Some(initial_stroke_size) = initial_stroke_size {
                sketcher.initial_stroke_size = initial_stroke_size;
            }
            if let Some(stroke_reduction) = stroke_reduction {
                sketcher.stroke_reduction = stroke_reduction;
            }
            if let Some(stroke_jitter) = stroke_jitter {
                sketcher.stroke_jitter = stroke_jitter;
            }
            sketcher.initial_alpha = initial_alpha;
            if let Some(alpha_increase) = alpha_increase {
                sketcher.alpha_increase = alpha_increase;
            }
            sketcher.stroke_inversion_threshold = stroke_inversion_threshold;
            sketcher.min_edge_count = min_edge_count;
            sketcher.max_edge_count = max_edge_count;

            for _ in 0..iterations {
                bar.inc(1);
                sketcher.draw_iter(&in_image);
            }
            sketcher.get_canvas().save(output)?;

            bar.finish();
        }
        Opt::Celestial {
            output,
            width,
            height,
            object_count,
            minimum_mass,
            maximum_mass,
            minimum_initial_velocity,
            maximum_initial_velocity,
            g,
            maximum_radius_from_center,
            increase_mass_with_distance,
            render_count,
            steps,
            step_size,
        } => {
            // Verify that inputs are valid.
            if minimum_mass > maximum_mass {
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

            let mut sketcher = CelestialSketcher::new(
                initial_image,
                object_count,
                minimum_mass..maximum_mass,
                minimum_initial_velocity..maximum_initial_velocity,
                g,
                Rgba::from_channels(255, 255, 255, 255),
                maximum_radius_from_center,
                increase_mass_with_distance,
                render_count.unwrap_or(object_count),
            );

            let bar = ProgressBar::new(steps as u64);

            for i in 0..steps {
                if i % (steps / 1000) == 0 {
                    bar.inc(steps as u64 / 1000);
                }

                // if i % 111 == 0 {
                //     *sketcher.get_canvas_mut() =
                //         generative_art::image::imageops::blur(sketcher.get_canvas(), 1.0);
                //     *sketcher.get_canvas_mut() =
                //         generative_art::image::imageops::brighten(sketcher.get_canvas(), -1);

                //     sketcher
                //         .get_canvas()
                //         .save(format!("images/{}.bmp", i / 111))?;
                // }

                sketcher.draw_iter(step_size);
            }

            sketcher.get_canvas().save(output)?;
        }
    }

    Ok(())
}
