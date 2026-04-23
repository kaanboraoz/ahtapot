use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::{DynamicImage, ImageError, ImageReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    size: i32,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, alias = "loc")]
    locate: PathBuf,
}

impl Args {
    fn new() -> Self {
        Self::parse()
    }

    fn get_dir(&self) -> Result<Vec<PathBuf>, ImageError> {
        let dir = fs::read_dir(&Path::new(&self.path))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        Ok(dir)
    }
}

struct ImageProcessor<'a> {
    path: &'a Path,
}

impl<'a> ImageProcessor<'a> {
    fn new(path: &'a Path) -> Self {
        Self { path }
    }

    fn open(&self) -> Result<DynamicImage, ImageError> {
        ImageReader::open(&self.path)?.decode()
    }
}

fn main() -> Result<(), ImageError> {
    let args: Args = Args::new();

    let my_dir = args.get_dir()?;

    let img = ImageProcessor::new(my_dir[0].as_path()).open()?;

    let nimg = img.resize(200, 200, image::imageops::FilterType::Nearest);

    let _ = nimg.save("./images/new2.png");

    Ok(())
}
