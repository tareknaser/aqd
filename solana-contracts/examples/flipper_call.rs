// SPDX-License-Identifier: Apache-2.0

use {
    anyhow::Result,
    solana_clap_v3_utils::input_validators::normalize_to_url_if_moniker,
    solana_cli_config::{Config, CONFIG_FILE},
    solana_contracts::{print_transaction_information, SolanaCall},
};

/// Example of interacting with Solana programs.
///
/// This example demonstrates how to use the `solana_contracts` crate to interact
/// with Solana programs. It sets up the necessary parameters, including the IDL JSON file, program ID,
/// instruction names, arguments, and submits transactions to the Solana blockchain. The example includes
/// calls to the "new," "get," and "flip" methods of a deployed Flipper smart contract.
///
/// This example demonstrates a sequence of interactions with a Solana program, making it useful
/// for understanding how to work with the `solana_contracts` crate.
///
/// The flipper contract is assumed to be deployed. The contract is defined in the
/// `examples/contracts/flipper.sol` file. The contract is compiled using the Solang compiler
/// and deployed using the Solana CLI.
///
/// To run this example, you need to deploy the flipper program and create the data account.
#[tokio::main]
async fn main() -> Result<()> {
    // Parse the config file to get the RPC URL and payer keypair.
    let config_file = CONFIG_FILE
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Error loading config file"))?;
    let cli_config = Config::load(config_file).unwrap_or_default();
    let rpc_url = normalize_to_url_if_moniker(&cli_config.json_rpc_url);
    let keypair = cli_config.keypair_path.to_string();

    // Define the path to the IDL JSON file, the program ID, and whether to output JSON.
    let idl_json = "examples/contracts/flipper.json";
    // The program ID is the address of the deployed program on the Solana blockchain.
    // Replace this with the address of the deployed flipper program.
    let program_id = "71gxeC5D6bGAUznocUWyXdhWQozhDc72qKL7oZ8zn4kR";
    let output_json = false;

    // Call the `new` method of the flipper program.

    // Define the instruction name, data arguments, and accounts arguments.
    let instruction_name = "new";
    let data_args: Vec<String> = vec!["true".to_string()];
    let accounts_args: Vec<String> =
        vec!["new".to_string(), "self".to_string(), "system".to_string()];

    // Create a `SolanaCall` object with the necessary parameters.
    let flipper_new = SolanaCall::new()
        .rpc_url(rpc_url.clone())
        .idl(idl_json.to_string())
        .program_id(program_id.to_string())
        .instruction(instruction_name.to_string())
        .call_data(data_args)
        .accounts(accounts_args)
        .payer(keypair.clone())
        .done()?;

    // Submit the transaction.
    let _signature = flipper_new.submit_transaction()?;
    // The `new` method does not return any data, so no need to print.
    // It also creates a new account because "new" was given as an account argument.
    // This is needed for other methods.
    let (data_account_pubkey, _data_account_path) = &flipper_new.new_accounts()[0];
    let data_account_pubkey = data_account_pubkey.to_string();

    // Call the `get` method of the flipper program.

    // Define the instruction name, data arguments, and accounts arguments.
    let instruction_name = "get";
    let data_args: Vec<String> = vec![];
    // `get` method requires only one account, the data account.
    let accounts_args: Vec<String> = vec![data_account_pubkey.clone()];

    // Create a `SolanaCall` object with the necessary parameters.
    let call_cmd = SolanaCall::new()
        .rpc_url(rpc_url.clone())
        .idl(idl_json.to_string())
        .program_id(program_id.to_string())
        .instruction(instruction_name.to_string())
        .call_data(data_args)
        .accounts(accounts_args)
        .payer(keypair.clone())
        .done()?;

    // Submit the transaction.
    let signature = call_cmd.submit_transaction()?;

    // Print the transaction information.
    match print_transaction_information(
        call_cmd.rpc_client(),
        &signature,
        call_cmd.instruction(),
        call_cmd.idl().types.as_slice(),
        call_cmd.new_accounts(),
        output_json,
    ) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    // Call the `flip` method of the flipper program.

    // Define the instruction name, data arguments, and accounts arguments.
    let instruction_name = "flip";
    let data_args: Vec<String> = vec![];
    // `flip` method requires only one account, the data account.
    let accounts_args: Vec<String> = vec![data_account_pubkey.clone()];

    // Create a `SolanaCall` object with the necessary parameters.
    let call_cmd = SolanaCall::new()
        .rpc_url(rpc_url.clone())
        .idl(idl_json.to_string())
        .program_id(program_id.to_string())
        .instruction(instruction_name.to_string())
        .call_data(data_args)
        .accounts(accounts_args)
        .payer(keypair.clone())
        .done()?;

    // Submit the transaction.
    let _signature = call_cmd.submit_transaction()?;

    // Call the `get` method of the flipper program.
    // Define the instruction name, data arguments, and accounts arguments.
    let instruction_name = "get";
    let data_args: Vec<String> = vec![];
    // `get` method requires only one account, the data account.
    let accounts_args: Vec<String> = vec![data_account_pubkey.clone()];

    // Create a `SolanaCall` object with the necessary parameters.
    let call_cmd = SolanaCall::new()
        .rpc_url(rpc_url.clone())
        .idl(idl_json.to_string())
        .program_id(program_id.to_string())
        .instruction(instruction_name.to_string())
        .call_data(data_args)
        .accounts(accounts_args)
        .payer(keypair.clone())
        .done()?;

    // Submit the transaction.
    let signature = call_cmd.submit_transaction()?;

    // Print a separator.
    println!("------------------------------------------");

    // Print the transaction information.
    match print_transaction_information(
        call_cmd.rpc_client(),
        &signature,
        call_cmd.instruction(),
        call_cmd.idl().types.as_slice(),
        call_cmd.new_accounts(),
        output_json,
    ) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    Ok(())
}
