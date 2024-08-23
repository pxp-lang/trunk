use std::{path::PathBuf, process::exit};

use clap::Parser;

use crate::{test::get_supported_test_runners, utils::cwd};

#[derive(Parser)]
#[clap(about = "Run tests.")]
pub struct TestCommand {
    #[clap(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    args: Vec<String>,
}

pub fn test(cmd: &TestCommand) {
    let mut runners = get_supported_test_runners();
    runners.sort_by(|a, b| b.precedence().cmp(&a.precedence()));

    let cwd = cwd();
    let runner = runners.iter().find(|r| r.is_installed(&cwd));

    if let None = runner {
        eprintln!("No supported test runners found.");
        exit(1);
    }

    let runner = runner.unwrap();

    let (path, additional_args): (Option<PathBuf>, Vec<String>) = if let Some(first) = cmd.args.first() {
        match cwd.join(first).canonicalize() {
            Ok(path) => (Some(path), cmd.args[1..].to_vec()),
            Err(_) => (None, cmd.args.clone())
        }
    } else {
        (None, cmd.args.to_vec())
    };

    let mut cmd = runner.cmd(path, &additional_args);

    cmd.current_dir(cwd);

    match cmd.status() {
        Ok(status) => if ! status.success() {
            eprintln!("Test runner {} failed with exit code: {}", runner.name(), status);
        },
        Err(e) => {
            eprintln!("Failed to execute {}: {}", runner.name(), e);
        }
    }
}