use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod cmd {

}

#[derive(Parser)]
#[clap(name = "trunk", version = env!("CARGO_PKG_VERSION"), about = "The all-in-one tool for PHP development.")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,

    #[clap(flatten)]
    verbosity: Verbosity,
}

#[derive(Subcommand)]
enum Command {

}

fn main() {
    let args = Cli::parse();

}
