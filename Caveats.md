- The function `decode_instruction_return_data` in `crates/solana-contracts/src/printing_utils.rs` currently retrieves the return data from the logs due to a known bug in the Solana SDK, which is expected to be fixed in [#33639](https://github.com/solana-labs/solana/pull/33639). Once this fix is officially released, the function should be updated to fetch the return data directly from the transaction itself.
- The file `crates/aqd-utils/src/borsh_encoding.rs` has been copied from `solang`. While `solang` provides the `discriminator` function, it doesn't expose `BorshToken`. Ideally, to reduce its reliance on `solang` (a heavy dependency), `aqd` should aim to be a lightweight crate. Instead, `solang` can depend on `aqd-utils`.
- Currently, a dependency issue between `solana` crates and `cargo-contract` (as reported in issue [#26688](https://github.com/solana-labs/solana/issues/26688)) prevents `aqd` from including both `solana` and `cargo-contract` dependencies in the same workspace. This is why the `solana` crates are placed in the project's home directory, while the other crates are located in the `crates/` directory.

The desired structure is:
```
aqd
├── crates
│   └── aqd-core
│   └── aqd-polkadot
│   └── aqd-solana 
│   ├── aqd-utils
│   ├── solana-contracts
```
