use clap::Parser;
use std::io::Result;
use std::path::PathBuf;

pub mod error;
pub mod teleport;
pub mod init;

#[derive(Parser)]
#[clap(
    name = "Teleport Dir",
    author = "Jascha030 <contact@jaschavanaalst.nl>",
    version = "1.0",
    about = "Navigate into the origin dir of a symlinked file.", 
    long_about = None
)]
pub enum Command {
    Teleport(Teleport),
    Init(Init),
}

#[derive(Parser)]
pub struct Teleport {
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Parser)]
pub struct Init {
    #[clap(short, long, default_value = "td")]
    cmd: String,
}

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for Command {
    fn run(&self) -> Result<()> {
        match self {
            Command::Teleport(cmd) => cmd.run(),
            Command::Init(cmd) => cmd.run(),
        }
    }
}
