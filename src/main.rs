mod tests;

use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::{DynamicImage, ImageError, ImageReader, imageops};

#[derive(Parser, Debug)]
#[command(
    name = "aht",
    version = "0.1.0",
    about = "A CLI tool for bulk image renaming, resizing, and format conversion.",
    long_about = "A CLI tool for bulk image renaming, resizing, and format conversion."
)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    width: u32,

    #[arg(short = 'e', long)]
    height: u32,

    #[arg(short, long)]
    output: PathBuf,
}

impl Args {
    fn new() -> Self {
        Self::parse()
    }

    fn get_dir(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let dir = fs::read_dir(&Path::new(&self.path))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        if dir.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Input directory is empty. Nothing to process.",
            ));
        }

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

    fn resize(mut self, width: u32, height: u32) -> Result<Self, ImageError> {
        self.img = self
            .img
            .resize(width, height, imageops::FilterType::CatmullRom);

        Ok(self)
    }

    fn save(&self, output: &Path, name: &str) -> Result<(), ImageError> {
        let mut save_path = PathBuf::new();

        if !(output.is_dir()) {
            fs::create_dir_all(output)?
        }

        save_path.push(output);
        save_path.push(Path::new(name).with_extension("png"));

        Ok(self.img.save(save_path)?)
    }
}

fn main() -> Result<(), ImageError> {
    let args: Args = Args::new();

    for (i, path) in args.get_dir()?.iter().enumerate() {
        if !path.is_file() {
            continue;
        }

        let name = format!("{}{}", args.name, i);

        ImageProcessor::new(&path)?
            .resize(args.width, args.height)?
            .save(&args.output, &name)?;
    }

    Ok(())
}
