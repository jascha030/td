mod _app;
mod init;
mod resolve;

pub use crate::app::_app::*;

pub trait Run {
    fn run(&self) -> Result<()>;
}
