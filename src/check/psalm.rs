use std::{path::{Path, PathBuf}, process::Command};

use super::Checker;

pub struct PsalmChecker;

impl Checker for PsalmChecker {
    fn name(&self) -> &'static str {
        "psalm"
    }

    fn is_installed(&self, cwd: &Path) -> bool {
        cwd.join("vendor/bin/psalm").exists()
    }

    fn cmd(&self, path: Option<PathBuf>) -> Command {
        let mut cmd = Command::new("vendor/bin/psalm");

        if let Some(path) = path {
            cmd.arg(path);
        }

        cmd
    }
}