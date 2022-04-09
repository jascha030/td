use clap::Parser;
use std::io::Result;
use teleport_dir::{Command, Run};

fn main() -> Result<()> {
    Command::parse().run()
}
