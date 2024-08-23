use std::{path::PathBuf, process::exit};

use clap::Parser;

use crate::{check::get_supported_checkers, utils::cwd};

#[derive(Parser)]
#[clap(about = "Run static analysis.")]
pub struct CheckCommand {
    #[clap(help = "The path to a file or directory to check.")]
    path: Option<PathBuf>,

    #[clap(short, long, help = "A comma-separated list of tools to run.", value_delimiter = ',', num_args = 1..)]
    using: Vec<String>,
}

pub fn check(cmd: &CheckCommand) {
    let cwd = cwd();

    let mut checkers = get_supported_checkers().into_iter().filter(|c| c.is_installed(&cwd)).collect::<Vec<_>>();

    if ! cmd.using.is_empty() {
        checkers = checkers.into_iter().filter(|c| cmd.using.contains(&c.name().to_string())).collect::<Vec<_>>();
    } else {
        checkers = checkers.into_iter().take(1).collect::<Vec<_>>();
    }

    if checkers.is_empty() {
        eprintln!("No supported checkers found.");
        exit(1);
    }

    let path = cmd.path.as_ref().map(|p| cwd.join(p).canonicalize().expect("Failed to canonicalize path."));

    for checker in checkers {
        let mut cmd = checker.cmd(path.clone());

        cmd.current_dir(&cwd);

        match cmd.status() {
            Ok(status) => if ! status.success() {
                eprintln!("Checker {} failed with exit code: {}", checker.name(), status);
            },
            Err(e) => {
                eprintln!("Failed to execute {}: {}", checker.name(), e);
            }
        }
    }
}