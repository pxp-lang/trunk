use std::{path::{Path, PathBuf}, process::Command};

use php_cs_fixer::PhpCsFixerFormatter;
use pint::PintFormatter;

mod php_cs_fixer;
mod pint;

pub trait Formatter {
    /// Returns the name of the formatter.
    fn name(&self) -> &'static str;

    /// Determines whether or not the formatter is installed.
    fn is_installed(&self, cwd: &Path) -> bool;

    /// Returns the precedence of the formatter.
    /// 
    /// Some formatters rely on other tools internally, so using a precedence
    /// system allows us to determine which formatter to use when multiple
    /// formatters are installed.
    fn precedence(&self) -> i32 {
        0
    }

    /// Builds a command for the formatter.
    fn cmd(&self, path: Option<PathBuf>, dry_run: bool) -> Command;
}

pub fn get_supported_formatters() -> Vec<Box<dyn Formatter>> {
    vec![
        Box::new(PhpCsFixerFormatter),
        Box::new(PintFormatter),
    ]
}