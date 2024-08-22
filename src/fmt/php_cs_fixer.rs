use std::{path::{Path, PathBuf}, process::Command};

use super::Formatter;

pub struct PhpCsFixerFormatter;

impl Formatter for PhpCsFixerFormatter {
    fn name(&self) -> &'static str {
        "php-cs-fixer"
    }

    fn is_installed(&self, cwd: &Path) -> bool {
        cwd.join("vendor/bin/php-cs-fixer").exists()
    }

    fn cmd(&self, path: Option<PathBuf>, dry_run: bool) -> Command {
        let mut cmd = Command::new("vendor/bin/php-cs-fixer");

        cmd.arg("fix");
        
        if let Some(path) = path {
            cmd.arg(path);
        }

        if dry_run {
            cmd.arg("--dry-run");
        }

        cmd
    }
}