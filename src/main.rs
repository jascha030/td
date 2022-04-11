use clap::Parser;
use teleport_dir::{Command, TeleportError, Run};

fn main() -> Result<(), TeleportError> {
    return match Command::parse().run() {
        Ok(_) => Ok(()),
        Err(e) => Err(TeleportError::new(&e.to_string())),
    };
}
