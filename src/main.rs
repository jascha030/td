use clap::Parser;
use path_absolutize::*;
use std::fs;
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

fn main() {
    let path = &Cli::parse().path;
    let metadata = fs::symlink_metadata(Path::new(path.absolutize().unwrap().to_str().unwrap())).unwrap();
    let is_symlink = metadata.is_symlink();

    if is_symlink {
        println!("YES");

        return;
    }

    println!("NO")
}
