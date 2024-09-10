use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cmd::{check::{check, CheckCommand}, fmt::{fmt, FmtCommand}, info::{info, InfoCommand}, test::{test, TestCommand}};

mod fmt;
mod utils;
mod check;
mod test;
mod info;
mod composer;
mod cmd {
    pub mod fmt;
    pub mod check;
    pub mod test;
    pub mod info;
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
    Test(TestCommand),
    Info(InfoCommand),
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Command::Fmt(command) => fmt(command),
        Command::Check(command) => check(command),
        Command::Test(command) => test(command),
        Command::Info(command) => info(command),
    }
}
