mod command;
use clap::Parser;
use crate::command::{Command, Run};
use crate::command::error::TeleportError;

fn main() -> Result<(), TeleportError> {
    return match Command::parse().run() {
        Ok(_) => Ok(()),
        Err(e) => Err(TeleportError::new(&e.to_string())),
    };
}
