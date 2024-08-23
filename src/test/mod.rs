use std::{path::{Path, PathBuf}, process::Command};

use pest::PestTestRunner;
use phpunit::PhpunitTestRunner;

pub mod phpunit;
pub mod pest;

pub trait TestRunner {
    fn name(&self) -> &'static str;

    fn precedence(&self) -> i32 {
        0
    }

    fn is_installed(&self, cwd: &Path) -> bool;

    fn cmd(&self, path: Option<PathBuf>, additional_args: &[String]) -> Command;
}

pub fn get_supported_test_runners() -> Vec<Box<dyn TestRunner>> {
    vec![
        Box::new(PestTestRunner),
        Box::new(PhpunitTestRunner),
    ]
}