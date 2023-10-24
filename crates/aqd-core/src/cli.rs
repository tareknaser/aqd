// SPDX-License-Identifier: Apache-2.0

use clap::{Parser, Subcommand};

// use aqd_solana::SolanaAction;

use aqd_polkadot::PolkadotAction;

#[derive(Parser)]
#[command(  author = env!("CARGO_PKG_AUTHORS"), 
            about = "Aqd is a versatile CLI tool for interacting with contracts on Solana and Polkadot blockchains.", 
            subcommand_required = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // #[command(about = "Interact with Solana contracts on chain")]
    // Solana {
    //     #[clap(subcommand)]
    //     action: SolanaAction,
    // },
    #[command(about = "Interact with Polkadot contracts on chain")]
    Polkadot {
        #[clap(subcommand)]
        action: PolkadotAction,
    },
}
