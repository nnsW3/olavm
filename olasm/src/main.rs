use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use subcommands::deploy::Deploy;

mod subcommands;
mod utils;

#[derive(Debug, Parser)]
#[clap(author, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Subcommands>,
    #[clap(long = "version", short = 'V', help = "Print version info and exit")]
    version: bool,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    #[clap(about = "Deploy a ola asm contract locally.")]
    Deploy(Deploy),
}

fn main() {
    if let Err(err) = run_command(Cli::parse()) {
        eprintln!("{}", format!("Error: {err}").red());
        std::process::exit(1);
    }
}

fn run_command(cli: Cli) -> anyhow::Result<()> {
    match (cli.version, cli.command) {
        (false, None) => Ok(Cli::command().print_help()?),
        (true, _) => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        (false, Some(command)) => match command {
            Subcommands::Deploy(cmd) => cmd.run(),
        },
    }
}