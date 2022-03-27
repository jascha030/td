use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(
    name = "Teleport Dir",
    author = "Jascha030 <contact@jaschavanaalst.nl>",
    version = "1.0",
    about = "Navigate into the origin dir of a symlinked file", 
    long_about = None
)]
pub enum App {
    Resolve(Resolve),
    Init(Init),
}

#[derive(Parser)]
pub struct Resolve {
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Parser)]
pub struct Init {}
