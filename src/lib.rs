use askama::Template;
use clap::Parser;
use path_absolutize::*;
use std::fs::{metadata, symlink_metadata, canonicalize};
use std::path::{Path, PathBuf};
use std::io::Result;

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

#[derive(Template)]
#[template(path = "zsh.txt")]
struct ShellTemplate<'a> {
    cmd: &'a str,
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

impl Run for Teleport {
    fn run(&self) -> Result<()> {
        let p = &self.path;
        let m = symlink_metadata(Path::new(p.absolutize().unwrap().to_str().unwrap())).unwrap();

        let mut change_path: PathBuf = match m.is_symlink() {
            true => canonicalize(p)?,
            false => p.absolutize().unwrap().to_path_buf(),
        };
            
        change_path = match metadata(&change_path).unwrap().is_file() {
            true => change_path.clone().parent().unwrap().to_path_buf(),
            false => change_path,
        };

        println!("{}", change_path.to_str().unwrap());

        Ok(())
    }
}

impl Run for Init {
    fn run(&self) -> Result<()> {
        let zsh = ShellTemplate { cmd: &self.cmd };

        println!("{}", zsh.render().unwrap());

        Ok(())
    }
}
