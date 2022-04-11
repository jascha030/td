use crate::command::{Run, Teleport};
use std::fs::{canonicalize, metadata, symlink_metadata};
use std::io::Result;
use std::path::{Path, PathBuf};
use path_absolutize::*;

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
