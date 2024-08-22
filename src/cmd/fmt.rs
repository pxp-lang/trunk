use std::{path::PathBuf, process::exit};

use clap::Parser;

use crate::{fmt::get_supported_formatters, utils::cwd};

#[derive(Parser)]
#[clap(about = "Format code.")]
pub struct FmtCommand {
    #[clap(help = "The path to a file or directory to format.")]
    path: Option<PathBuf>,

    #[clap(short, long, help = "Dry run; don't actually format anything.")]
    dry_run: bool,
}

pub fn fmt(cmd: &FmtCommand) {
    let mut formatters = get_supported_formatters();
    formatters.sort_by(|a, b| b.precedence().cmp(&a.precedence()));

    let cwd = cwd();

    let formatter = formatters.iter().find(|f| f.is_installed(&cwd));

    if let None = formatter {
        eprintln!("No supported formatters found.");
        exit(1);
    }

    let formatter = formatter.unwrap();

    // If we have a path to format, we can join it with the current working directory.
    let path = cmd.path.as_ref().map(|p| cwd.join(p).canonicalize().expect("Failed to canonicalize path."));

    // Grab the command for the formatter.
    let mut cmd = formatter.cmd(path, cmd.dry_run);

    // Set the current working directory for the command.
    cmd.current_dir(cwd);

    match cmd.status() {
        Ok(status) => if ! status.success() {
            eprintln!("Formatter failed with exit code: {}", status);
        },
        Err(e) => {
            eprintln!("Failed to execute formatter: {}", e);
        }
    }
}