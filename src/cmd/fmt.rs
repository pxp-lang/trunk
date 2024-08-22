use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct FmtCommand {
    #[clap(help = "The path to a file or directory to format.")]
    path: Option<PathBuf>,
}

pub fn fmt(command: &FmtCommand) {
    
}