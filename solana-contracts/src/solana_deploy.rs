// SPDX-License-Identifier: Apache-2.0

use {
    anyhow::Result,
    solana_cli::{
        cli::{
            process_command, CliCommand, CliCommandInfo, CliConfig,
            DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS, DEFAULT_RPC_TIMEOUT_SECONDS,
        },
        program::ProgramCliCommand,
    },
    solana_cli_config::{Config, CONFIG_FILE},
    solana_cli_output::OutputFormat,
    solana_rpc_client_api::config::RpcSendTransactionConfig,
    solana_sdk::{commitment_config::CommitmentConfig, signer::keypair::read_keypair_file},
    std::{str::FromStr, time::Duration},
};

/// Deploy a Solana program to the blockchain.
///
/// This function facilitates the deployment of a Solana program to the blockchain. It reads
/// the default Solana configuration file on disk to obtain necessary configuration settings.
///
/// # Arguments
///
/// * `program_location`: A string representing the location of the program to be deployed.
/// * `verbose`: A boolean indicating whether to provide verbose output.
/// * `output_json`: A boolean indicating whether to format the output as JSON.
///
/// # Returns
///
/// A `Result` containing a string that represents the result of the deployment operation.
pub fn deploy_program(
    program_location: String,
    verbose: bool,
    output_json: bool,
) -> Result<String> {
    // Get the path to the configuration file (default location)
    let config_file = CONFIG_FILE
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Failed to get configuration file path"))?;

    // Load configuration settings from a file or use defaults if the file is not found
    let config = Config::load(config_file).unwrap_or_default();

    // Create a CLI command for program deployment and define signers
    let CliCommandInfo { command, signers } = CliCommandInfo {
        command: CliCommand::Program(ProgramCliCommand::Deploy {
            program_location: Some(program_location.to_string()),
            program_signer_index: None,
            program_pubkey: None,
            buffer_signer_index: None,
            buffer_pubkey: None,
            upgrade_authority_signer_index: 0,
            is_final: false,
            max_len: None,
            allow_excessive_balance: false,
            skip_fee_check: false,
        }),
        // Load signer keypair from the file specified in the configuration
        signers: vec![read_keypair_file(&config.keypair_path)
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to read keypair file '{}': {}",
                    config.keypair_path,
                    e
                )
            })?
            .into()],
    };

    // Parse the commitment level from the configuration file
    let commitment = CommitmentConfig::from_str(&config.commitment)
        .ok()
        .ok_or_else(|| {
            anyhow::anyhow!("Failed to parse commitment level from configuration file")
        })?;

    // Determine the output format (JSON or Display)
    let output_format = match output_json {
        true => OutputFormat::Json,
        false => {
            if verbose {
                OutputFormat::DisplayVerbose
            } else {
                OutputFormat::Display
            }
        }
    };

    let rpc_timeout = Duration::from_secs(
        DEFAULT_RPC_TIMEOUT_SECONDS
            .parse::<u64>()
            .map_err(|e| anyhow::anyhow!("Failed to parse RPC timeout: {}", e))?,
    );
    let confirm_transaction_initial_timeout = Duration::from_secs(
        DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS
            .parse::<u64>()
            .map_err(|e| anyhow::anyhow!("Failed to parse confirm transaction timeout: {}", e))?,
    );

    // Create a new configuration with modified settings
    let cmd_config = CliConfig {
        command,
        json_rpc_url: config.json_rpc_url,
        websocket_url: config.websocket_url,
        signers: signers.iter().map(|s| s.as_ref()).collect(),
        keypair_path: config.keypair_path,
        rpc_client: None,
        rpc_timeout,
        verbose,
        output_format,
        commitment,
        send_transaction_config: RpcSendTransactionConfig {
            preflight_commitment: Some(commitment.commitment),
            ..RpcSendTransactionConfig::default()
        },
        confirm_transaction_initial_timeout,
        address_labels: config.address_labels,
        use_quic: true,
    };

    // Process the deployment command with the updated configuration
    process_command(&cmd_config)
        .map_err(|e| anyhow::anyhow!("Failed to process deployment command: {}", e))
}
