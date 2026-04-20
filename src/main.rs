use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::ImageReader;

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

fn main() -> Result<(), std::io::Error> {
    let args: Args = Args::parse();

    let my_dir = fs::read_dir(&Path::new(&args.path))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    let img = ImageReader::open(my_dir[0].as_path())?.decode().unwrap();

    let nimg = img.resize(200, 200, image::imageops::FilterType::Nearest);

    let _ = nimg.save("./images/new.png");

    Ok(())
}
