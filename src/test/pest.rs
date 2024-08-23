use std::{path::PathBuf, process::Command};

use super::TestRunner;

pub struct PestTestRunner;

impl TestRunner for PestTestRunner {
    fn name(&self) -> &'static str {
        "pest"
    }

    fn is_installed(&self, cwd: &std::path::Path) -> bool {
        cwd.join("vendor/bin/pest").exists()
    }

    fn cmd(&self, path: Option<PathBuf>, additional_args: &[String]) -> Command {
        let mut cmd = Command::new("vendor/bin/pest");

        if let Some(path) = path {
            cmd.arg(path);
        }

        for arg in additional_args {
            cmd.arg(arg);
        }

        cmd
    }
}