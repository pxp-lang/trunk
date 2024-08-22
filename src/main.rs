use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cmd::fmt::{fmt, FmtCommand};

mod fmt;
mod utils;
mod cmd {
    pub mod fmt;
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
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Command::Fmt(command) => fmt(command),
    }
}
