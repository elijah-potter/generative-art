use std::path::PathBuf;

use indicatif::ProgressBar;
use sketch::Sketcher;
use structopt::StructOpt;

mod sketch;

#[derive(StructOpt, Debug)]
#[structopt(name = "Art")]
struct Opt {
    #[structopt(parse(from_os_str))]
    /// Name of file to use as input
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    /// Name of file to save generated art to
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let in_image = image::open(opt.input)?;
    let in_image = in_image.into_rgba8();

    let iter_count = 5000;

    let bar = ProgressBar::new(iter_count as u64);

    let mut sketcher = Sketcher::new_preslav(in_image.to_owned(), iter_count);
    sketcher.max_edge_count = 6;

    for _ in 0..iter_count {
        bar.inc(1);
        sketcher.draw_iter(&in_image);
    }
    sketcher.get_canvas().save(opt.output)?;

    bar.finish();
    Ok(())
}
