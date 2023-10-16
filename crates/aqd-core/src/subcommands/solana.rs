// SPDX-License-Identifier: Apache-2.0

use {aqd_solana::SolanaDeploy, clap::Subcommand};

/// Available subcommands for the `solana` subcommand.
#[derive(Debug, Subcommand)]
pub enum SolanaAction {
    Deploy(SolanaDeploy),
}
