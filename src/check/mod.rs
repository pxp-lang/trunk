use std::{path::{Path, PathBuf}, process::Command};

use phan::PhanChecker;
use phpstan::PhpstanChecker;
use psalm::PsalmChecker;

pub mod phpstan;
pub mod psalm;
pub mod phan;

pub trait Checker {
    /// Returns the name of the checker.
    fn name(&self) -> &'static str;

    /// Determines whether or not the checker is installed.
    fn is_installed(&self, cwd: &Path) -> bool;

    /// Builds a command for the checker.
    fn cmd(&self, path: Option<PathBuf>) -> Command;
}

pub fn get_supported_checkers() -> Vec<Box<dyn Checker>> {
    vec![
        Box::new(PhpstanChecker),
        Box::new(PsalmChecker),
        Box::new(PhanChecker),
    ]
}