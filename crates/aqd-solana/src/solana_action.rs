// SPDX-License-Identifier: Apache-2.0

use {
    crate::{SolanaCall, SolanaDeploy},
    clap::Subcommand,
};

/// Available subcommands for the `solana` subcommand.
#[derive(Debug, Subcommand)]
pub enum SolanaAction {
    Deploy(SolanaDeploy),
    Call(SolanaCall),
}
