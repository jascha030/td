use clap::Parser;
use std::path::PathBuf;

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
    let cli = Cli::parse();

    println!("Your argument reads: {:?}", cli.path);
}
