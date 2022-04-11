use crate::command::{Init, Run};
use std::io::Result;
use askama::Template;

#[derive(Template)]
#[template(path = "zsh.txt")]
struct ShellTemplate<'a> {
    cmd: &'a str,
}

impl Run for Init {
    fn run(&self) -> Result<()> {
        let zsh = ShellTemplate { cmd: &self.cmd };

        println!("{}", zsh.render().unwrap());

        Ok(())
    }
}
