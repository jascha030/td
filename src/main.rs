use clap::Parser;
use path_absolutize::*;
use std::fs::{metadata, read_link, symlink_metadata};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(name = "Teleport Dir")]
#[clap(author = "Jascha030 <contact@jaschavanaalst.nl>")]
#[clap(version = "1.0")]
#[clap(about = "Navigate into the origin dir of a symlinked file", long_about = None)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let p = Cli::parse().path;
    let m = symlink_metadata(Path::new(p.absolutize().unwrap().to_str().unwrap())).unwrap();

    let mut change_path: PathBuf = match m.is_symlink() {
        true => read_link(p.absolutize().unwrap())?,
        false => p.absolutize().unwrap().to_path_buf(),
    };

    change_path = match metadata(&change_path).unwrap().is_file() {
        true => change_path.clone().parent().unwrap().to_path_buf(),
        false => change_path,
    };
    
    println!("{}", change_path.to_str().unwrap());

    Ok(())
}
