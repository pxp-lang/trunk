use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cmd::{check::{check, CheckCommand}, fmt::{fmt, FmtCommand}};

mod fmt;
mod utils;
mod check;
mod cmd {
    pub mod fmt;
    pub mod check;
}

#[derive(Parser)]
#[clap(name = "trunk", version = env!("CARGO_PKG_VERSION"), about = "The all-in-one tool for PHP development.")]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(flatten)]
    verbosity: Verbosity,
}

#[derive(Subcommand)]
enum Command {
    Fmt(FmtCommand),
    Check(CheckCommand),
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Command::Fmt(command) => fmt(command),
        Command::Check(command) => check(command),
    }
}
