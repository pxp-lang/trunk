use std::{path::{Path, PathBuf}, process::Command};

use super::Formatter;

pub struct PintFormatter;

impl Formatter for PintFormatter {
    fn name(&self) -> &'static str {
        "pint"
    }

    fn is_installed(&self, cwd: &Path) -> bool {
        cwd.join("vendor/bin/pint").exists()
    }

    fn cmd(&self, path: Option<PathBuf>, dry_run: bool) -> Command {
        let mut cmd = Command::new("vendor/bin/pint");

        if let Some(path) = path {
            cmd.arg(path);
        }

        if dry_run {
            cmd.arg("--test");
        }

        cmd
    }

    fn precedence(&self) -> i32 {
        1
    }
}