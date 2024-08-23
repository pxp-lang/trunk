use std::{path::PathBuf, process::Command};

use super::TestRunner;

pub struct PhpunitTestRunner;

impl TestRunner for PhpunitTestRunner {
    fn name(&self) -> &'static str {
        "phpunit"
    }

    fn is_installed(&self, cwd: &std::path::Path) -> bool {
        cwd.join("vendor/bin/phpunit").exists()
    }

    fn cmd(&self, path: Option<PathBuf>, additional_args: &[String]) -> Command {
        let mut cmd = Command::new("vendor/bin/phpunit");

        if let Some(path) = path {
            cmd.arg(path);
        }

        for arg in additional_args {
            cmd.arg(arg);
        }

        cmd
    }
}