use std::{path::{Path, PathBuf}, process::Command};

use super::Checker;

pub struct PhanChecker;

impl Checker for PhanChecker {
    fn name(&self) -> &'static str {
        "phan"
    }

    fn is_installed(&self, cwd: &Path) -> bool {
        cwd.join("vendor/bin/phan").exists()
    }

    fn cmd(&self, path: Option<PathBuf>) -> Command {
        let mut cmd = Command::new("vendor/bin/phan");

        if let Some(path) = path {
            cmd.arg(path);
        }

        cmd
    }
}