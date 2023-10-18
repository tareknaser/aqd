#!/bin/bash
set -e

# Add Aqd to PATH
export PATH="$PATH:$(pwd)/target/release"

# Add Solana CLI to PATH
export PATH="$PATH:$HOME/.local/share/solana/install/active_release/bin"

# Ensure nohup writes output to a file, not the terminal.
# If we don't, solana-test-validator might not be able to write its
# output and exit
# We run it early so that it can start up while the rest of the
# workflow is running
nohup solana-test-validator -q > validator.out &

# Wait for the validator to start up
sleep 10

# Generate a new keypair
solana-keygen new --no-bip39-passphrase

# Set Solana URL to localhost
solana config set --url localhost

# Air drop 50 SOL (required to send transactions)
solana airdrop 50



# Deploy the program (assumes a localnet cluster is running)
deploy_result=$(aqd solana deploy --output-json flipper.so)

# Extract the program id from the deploy result
program_id=$(echo $deploy_result | jq -r .program_id)

# Assert the program is deployed successfully
# (the program id is not null)
if [ "$program_id" == "null" ]; then
  echo "Error: Solana program deployment failed"
  exit 1
fi

# Wait for the program to be available
sleep 3

# Call the "new" instruction on the flipper program
flipper_new=$(aqd solana call --output-json --idl flipper.json --program "$program_id" --instruction new --data true --accounts new self system)

# Extract the newly created data account pubkey from the call result
data_account_pubkey=$(echo $flipper_new | jq -r '.new_accounts[0].pubkey')

# Assert the data account pubkey is not null
if [ "$data_account_pubkey" == "null" ]; then
  echo "Error: Solana program call failed"
  exit 1
fi

# Call the "get" instruction on the flipper program
flipper_get=$(aqd solana call --output-json --idl flipper.json --program "$program_id" --instruction get --accounts "$data_account_pubkey")

# Extract the decoded return value from the call result
decoded_return_value=$(echo $flipper_get | jq -r .decoded_return_data)

# Assert that the decoded return value is "true"
if [ "$decoded_return_value" != "true" ]; then
  echo "Error: Solana program call failed"
  exit 1
fi

# Call the "flip" instruction on the flipper program
flipper_flip=$(aqd solana call --output-json --idl flipper.json --program "$program_id" --instruction flip --accounts "$data_account_pubkey")

# Call the "get" instruction on the flipper program
flipper_get=$(aqd solana call --output-json --idl flipper.json --program "$program_id" --instruction get --accounts "$data_account_pubkey")

# Extract the decoded return value from the call result
decoded_return_value=$(echo $flipper_get | jq -r .decoded_return_data)

# Assert that the decoded return value is "false"
if [ "$decoded_return_value" != "false" ]; then
  echo "Error: Solana program call failed"
  exit 1
fi

exit 0
