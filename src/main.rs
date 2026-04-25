use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::{DynamicImage, ImageError, ImageReader, imageops};

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

struct ImageProcessor {
    img: DynamicImage,
}

impl ImageProcessor {
    fn new(path: &Path) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)?.decode()?;

        Ok(Self { img })
    }

    fn resize(mut self) -> Result<ImageProcessor, ImageError> {
        self.img = self.img.resize(100, 100, imageops::FilterType::CatmullRom);

        Ok(self)
    }

    fn save(&self) -> Result<(), ImageError> {
        Ok(self.img.save("/images")?)
    }
}

fn main() -> Result<(), ImageError> {
    let args: Args = Args::new();

    let my_dir = args.get_dir()?;

    if my_dir.is_empty() {
        return Err(ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidFilename,
            "oh no",
        )));
    }

    for i in my_dir {
        ImageProcessor::new(i.as_path())?.resize()?.save()?;
    }

    Ok(())
}
