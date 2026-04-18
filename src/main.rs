use std::{fs, path::{Path, PathBuf}};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    format: String,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    size: i32,

    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long)]
    locate: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    // let args: Args = Args::parse();

    let my_dir = fs::read_dir(&Path::new("./images"))?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("{:?}", my_dir);

    Ok(())
}
