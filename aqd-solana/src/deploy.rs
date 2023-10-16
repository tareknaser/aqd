// SPDX-License-Identifier: Apache-2.0

use {anyhow::Result, std::process::exit};
use {aqd_utils::check_target_match, solana_contracts::deploy_program};

#[derive(Clone, Debug, clap::Args)]
#[clap(name = "deploy", about = "Deploy a program to Solana")]
pub struct SolanaDeploy {
    #[clap(help = "Specifies the path to the program file to deploy (.so)")]
    program_location: String,
    #[clap(long, help = "Specifies whether to export the output in JSON format")]
    output_json: bool,
    #[clap(
        short('v'),
        long,
        conflicts_with = "output_json",
        help = "Specifies whether to display verbose program deployment information"
    )]
    verbose: bool,
}

impl SolanaDeploy {
    /// Handle the deployment of a Solana program
    ///
    /// This function is responsible for managing the deployment process,
    /// including checking the current directory, parsing command-line arguments,
    /// configuring settings, and executing the deployment command. It also handles
    /// loading the necessary configuration and signers, defining output formats,
    /// and processing the deployment command using the provided configuration.
    pub fn handle(&self) -> Result<()> {
        // Make sure the command is run in the correct directory
        // Fails if the command is run in a Solang Polkadot project directory
        let target_match = check_target_match("solang", None)
            .map_err(|e| anyhow::anyhow!("Failed to check current directory: {}", e))?;
        if !target_match {
            exit(1);
        }

        // Parse command-line arguments
        let program_location = self.program_location.clone();
        let output_json = self.output_json;
        let verbose = self.verbose;

        // Deploy the program
        let result = deploy_program(program_location, output_json, verbose)?;
        println!("{}", result);

        Ok(())
    }
}
