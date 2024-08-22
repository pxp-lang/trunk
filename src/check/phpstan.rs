use std::{path::{Path, PathBuf}, process::Command};

use super::Checker;

pub struct PhpstanChecker;

impl Checker for PhpstanChecker {
    fn name(&self) -> &'static str {
        "phpstan"
    }

    fn is_installed(&self, cwd: &Path) -> bool {
        cwd.join("vendor/bin/phpstan").exists()
    }

    fn cmd(&self, path: Option<PathBuf>) -> Command {
        let mut cmd = Command::new("vendor/bin/phpstan");

        cmd.arg("analyse");

        if let Some(path) = path {
            cmd.arg(path);
        }

        cmd
    }
}