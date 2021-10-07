use image::GenericImageView;

use imageproc::drawing::Blend;
use indicatif::ProgressBar;
use sketch::Sketcher;

mod sketch;

fn main() -> anyhow::Result<()> {
    let in_image = image::open("input.jpg")?;
    let in_image = in_image.into_rgba8();

    let iter_count = 5000;

    let bar = ProgressBar::new(iter_count as u64);

    let mut sketcher = Sketcher::new_preslav(in_image.width(), in_image.height(), iter_count);
    sketcher.image = Blend(in_image.to_owned());

    for _ in 0..iter_count {
        bar.inc(1);
        sketcher.draw_iter(&in_image);
    }
    sketcher.image.0.save("output.png")?;

    bar.finish();
    Ok(())
}
