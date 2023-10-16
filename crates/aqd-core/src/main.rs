// SPDX-License-Identifier: Apache-2.0

mod cli;
mod subcommands;

use {
    clap::{CommandFactory, FromArgMatches},
    std::process::exit,
    subcommands::SolanaAction,
};

use crate::cli::{Cli, Commands::*};

/// The main entry point for `aqd` command-line application.
fn main() {
    // Parse command-line arguments.
    let matches = Cli::command().get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap();

    match cli.command {
        Solana { action } => match action {
            SolanaAction::Deploy(deploy_args) => match deploy_args.handle() {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            },
        },
    }
}
