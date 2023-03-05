use std::{
    fs, io,
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use rayon::prelude::*;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
}

#[derive(Debug, Parser)]
struct Args {
    /// image paths
    #[arg(required = true)]
    images: Vec<String>,

    /// output directory
    #[arg(short, long)]
    output: Option<String>,
}

impl Args {
    fn build_path(&self, path: &Path) -> Option<PathBuf> {
        path.file_name().map(|file_name| {
            self.output
                .as_deref()
                .map(Path::new)
                .map(|output| output.join(file_name).with_extension("webp"))
                .unwrap_or_else(|| path.with_extension("webp"))
        })
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> Result<()> {
    args.images
        .par_iter()
        .try_for_each(|path| reencode(args, path.as_ref()))
}

fn reencode(args: &Args, path: &Path) -> Result<()> {
    let buffer = fs::read(path)?;
    let image = image::load_from_memory(&buffer)?;
    let target = args
        .build_path(path)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid filename"))?;

    image.save(&target)?;

    println!("{}", target.display());

    Ok(())
}
