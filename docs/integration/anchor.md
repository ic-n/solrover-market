### Verify Anchor CLI Installation

Source: https://www.anchor-lang.com/docs/quickstart/local

Checks if the Anchor CLI is installed and displays its version. This is a crucial first step to ensure the development environment is set up correctly.

```shell
anchor --version
```

---

### Start Local Solana Validator

Source: https://www.anchor-lang.com/docs/quickstart/local

Manually starts a local Solana validator instance. This is useful for keeping the validator running while iterating on program development, allowing for inspection of accounts and logs.

```shell
solana-test-validator
```

---

### Quick Anchor Installation (Mac/Linux)

Source: https://www.anchor-lang.com/docs/installation

Installs Rust, Solana CLI, and Anchor Framework with a single command for Mac and Linux users. Requires WSL for Windows users.

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

---

### Test Anchor Program Locally

Source: https://www.anchor-lang.com/docs/quickstart/local

Executes the tests for the Anchor program. By default, it starts a local Solana validator, deploys the program, runs tests, and then stops the validator.

```shell
anchor test
```

```shell
anchor test --skip-local-validator
```

---

### Install Solana CLI

Source: https://www.anchor-lang.com/docs/installation

Installs the Solana CLI tool suite, essential for building and deploying Solana programs. Supports different release channels like stable, beta, and edge.

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

---

### Build, Deploy, and Test Anchor Programs

Source: https://www.anchor-lang.com/docs/installation

Commands to compile your Anchor project, deploy it to a specified cluster, and run tests. The `anchor test` command automatically handles building, deploying, and testing, including starting and stopping a local validator if configured for localnet.

```shell
anchor build
anchor deploy
anchor test
```

---

### Build Anchor Program

Source: https://www.anchor-lang.com/docs/quickstart/local

Compiles the Solana program written using the Anchor framework. The output is a deployable program file, typically stored in the 'target/deploy' directory.

```shell
anchor build
```

---

### Initialize New Anchor Project

Source: https://www.anchor-lang.com/docs/quickstart/local

Creates a new Anchor project with a default program and test file. You can specify a project name and choose a test template (e.g., Rust, Mollusk).

```shell
anchor init my-project
```

```shell
anchor init --test-template rust my-program
```

---

### Install Rust using rustup

Source: https://www.anchor-lang.com/docs/installation

Installs the Rust programming language, the primary language for Solana programs, using the recommended rustup installer.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

---

### Initialize Anchor Project

Source: https://www.anchor-lang.com/docs/installation

Creates a new Anchor project with a basic Rust program and TypeScript test template. After initialization, navigate into the newly created project directory.

```shell
anchor init <project-name>
cd <project-name>
```

---

### Create Solana Wallet

Source: https://www.anchor-lang.com/docs/installation

Generates a new Solana keypair (wallet) at the default location and displays the public key.

```bash
solana-keygen new
```

```bash
solana address
```

---

### Deploy Anchor Program to Devnet

Source: https://www.anchor-lang.com/docs/quickstart/local

Execute the `anchor deploy` command to deploy your Anchor program to the cluster specified in your `Anchor.toml` file. If configured for Devnet, this command will deploy the program to the Solana Devnet.

```bash
anchor deploy
```

---

### Run Local Solana Validator

Source: https://www.anchor-lang.com/docs/installation

Starts the built-in Solana test validator to allow local deployment and testing of programs. After starting, update the Solana CLI config to localhost for subsequent commands.

```shell
solana-test-validator
solana config set -ul
```

---

### Navigate to Project Directory

Source: https://www.anchor-lang.com/docs/quickstart/local

Changes the current directory to the newly created Anchor project. This is necessary to access project files and run commands within the project context.

```shell
cd my-project
```

---

### Install Anchor CLI using AVM

Source: https://www.anchor-lang.com/docs/installation

Installs the Anchor Version Manager (AVM) and then uses it to install and manage Anchor CLI versions. This is the recommended method for installing Anchor.

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install latest
avm use latest
```

```bash
avm install 0.31.1
avm use 0.31.1
```

---

### Verify Solana CLI Installation

Source: https://www.anchor-lang.com/docs/installation

Checks the installed Solana CLI version to confirm a successful installation.

```bash
solana --version
```

---

### Solana CLI Configuration

Source: https://www.anchor-lang.com/docs/installation

Commands to view and set the Solana CLI configuration, including the RPC URL and keypair path.

```bash
solana config get
```

```bash
solana config set --url mainnet-beta
solana config set --url devnet
solana config set --url localhost
solana config set --url testnet
```

```bash
solana config set -um
solana config set -ud
solana config set -ul
solana config set -ut
```

---

### Verify Anchor CLI Installation

Source: https://www.anchor-lang.com/docs/installation

Checks the installed version of the Anchor CLI to confirm successful installation.

```bash
anchor --version
```

---

### Update Anchor Program

Source: https://www.anchor-lang.com/docs/quickstart/local

To update an existing Solana program deployed with Anchor, first build the updated program using `anchor build`. This generates a new `.so` file. Then, redeploy the program using `anchor deploy` to the same program ID.

```bash
anchor build
```

```bash
anchor deploy
```

---

### Verify Rust Installation

Source: https://www.anchor-lang.com/docs/installation

Checks the installed Rust compiler version to confirm a successful installation.

```bash
rustc --version
```

---

### Anchor Basics: Hello Solana

Source: https://www.anchor-lang.com/docs/references/examples

A basic 'Hello, Solana!' program using Anchor. This is a foundational example for understanding the structure of an Anchor program.

```Rust
/*
  Example: hello-solana
  Description: Basic "Hello, Solana!" program with Anchor
*/
// Placeholder for actual Rust code
fn hello_solana() {
    // ... implementation ...
}
```

---

### Configure Fish Shell Completions for Anchor and AVM

Source: https://www.anchor-lang.com/docs/installation

Installs shell completions for Anchor and AVM in the Fish shell. This requires creating a specific directory and sourcing the configuration file.

```fish
mkdir -p $HOME/.config/fish/completions
anchor completions fish > $HOME/.config/fish/completions/anchor.fish
avm completions fish > $HOME/.config/fish/completions/avm.fish
source $HOME/.config/fish/config.fish
```

---

### Configure Anchor.toml for Mainnet Deployment

Source: https://www.anchor-lang.com/docs/quickstart/local

To deploy your Anchor program to the Solana Mainnet, update the `cluster` value in your `Anchor.toml` file to 'Mainnet'. This ensures that `anchor deploy` commands target the Mainnet cluster.

```toml
[provider]
cluster = "Mainnet"
wallet = "~/.config/solana/id.json"
```

---

### Configure Anchor.toml for Devnet Deployment

Source: https://www.anchor-lang.com/docs/quickstart/local

Modify the `Anchor.toml` configuration file to set the cluster to 'Devnet' for deploying your Anchor program. This change ensures that subsequent `anchor deploy` commands target the Devnet cluster. Ensure your wallet has sufficient SOL for deployment.

```toml
[toolchain]
[features]
resolution = true
skip-lint = false
[programs.localnet]
my_program = "3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg"
[registry]
url = "https://api.apr.dev"
[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"
[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

```diff
-cluster = "Localnet"
+cluster = "Devnet"
```

```toml
[provider]
cluster = "Devnet"
wallet = "~/.config/solana/id.json"
```

---

### Update Solana CLI

Source: https://www.anchor-lang.com/docs/installation

Updates the Solana CLI to the latest available version.

```bash
agave-install update
```

---

### Anchor Basics: Processing Instructions

Source: https://www.anchor-lang.com/docs/references/examples

An example of how to process instructions within an Anchor program. This covers the core logic for handling client requests.

```Rust
/*
  Example: processing-instructions
  Description: Process instructions using Anchor
*/
// Placeholder for actual Rust code
fn process_instructions() {
    // ... implementation ...
}
```

---

### Add Solana CLI to PATH (Linux/WSL)

Source: https://www.anchor-lang.com/docs/installation

Adds the Solana CLI's active release binary directory to the PATH environment variable for Linux or WSL terminals.

```bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

---

### Configure Bash Shell Completions for Anchor and AVM

Source: https://www.anchor-lang.com/docs/installation

Sets up shell completions for the Anchor and AVM CLIs in Bash. This involves creating directories, piping completion commands, and reloading the shell.

```bash
mkdir -p $HOME/.local/share/bash-completion/completions
anchor completions bash > $HOME/.local/share/bash-completion/completions/anchor
avm completions bash > $HOME/.local/share/bash-completion/completions/avm
exec bash
```

---

### Build Documentation

Source: https://www.anchor-lang.com/docs/updates/contribution-guide

Builds the project documentation and opens it in a web browser. This is useful for checking for broken links after code refactoring.

```bash
cargo doc --open
```

---

### Anchor Basics: Favorites Storage

Source: https://www.anchor-lang.com/docs/references/examples

An example program for storing user 'favorites' using Anchor. This showcases data persistence and user-specific state management.

```Rust
/*
  Example: favorites
  Description: Store user "favorites" with Anchor
*/
// Placeholder for actual Rust code
fn favorites_program() {
    // ... implementation ...
}
```

---

### Anchor Basics: Calculate SOL Rent

Source: https://www.anchor-lang.com/docs/references/examples

An example program for calculating SOL rent for accounts using Anchor. Understanding rent is essential for managing on-chain storage costs.

```Rust
/*
  Example: rent
  Description: Calculate account SOL rent with Anchor
*/
// Placeholder for actual Rust code
fn calculate_rent() {
    // ... implementation ...
}
```

---

### Anchor CLI: Installation Progress Display

Source: https://www.anchor-lang.com/docs/updates/changelog

Improves the user experience by showing installation progress when Solana tools are not installed and toolchain overrides are used. This provides visual feedback during the setup process.

```bash
# This fix ensures that if Solana tools are not found,
# the CLI displays progress during the installation/setup.
# No specific command, it's a behavior fix during CLI setup.

```

---

### Anchor AVM: Download Binaries by Default

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Version Manager (AVM) now downloads binaries by default during installation, simplifying the setup process.

```bash
# This is a background change in AVM's installation logic.
```

---

### Anchor Basics: Create Account

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates the process of creating new accounts with Anchor. This is a fundamental operation for storing data on the Solana ledger.

```Rust
/*
  Example: create-account
  Description: Create accounts with Anchor
*/
// Placeholder for actual Rust code
fn create_account() {
    // ... implementation ...
}
```

---

### Anchor Tokens: Token Fundraiser

Source: https://www.anchor-lang.com/docs/references/examples

An example of a token fundraiser program using Anchor. This allows users to contribute to a project by purchasing tokens.

```Rust
/*
  Example: token-fundraiser
  Description: Token fundraiser using Anchor
*/
// Placeholder for actual Rust code
fn token_fundraiser() {
    // ... implementation ...
}
```

---

### Airdrop Devnet SOL

Source: https://www.anchor-lang.com/docs/quickstart/solpg

This command is used to request SOL from the devnet faucet to fund your Solana wallet for testing purposes. Ensure you have the Solana CLI installed and configured for devnet.

```bash
solana airdrop 5
```

---

### Anchor Basics: Counter Program

Source: https://www.anchor-lang.com/docs/references/examples

A simple counter program built with Anchor. This example illustrates basic state management, instruction handling, and account updates.

```Rust
/*
  Example: counter
  Description: Counter program using Anchor
*/
// Placeholder for actual Rust code
fn counter_program() {
    // ... implementation ...
}
```

---

### Anchor Tokens: Escrow Program

Source: https://www.anchor-lang.com/docs/references/examples

An example of an escrow program built with Anchor. This program facilitates secure token exchanges between parties.

```Rust
/*
  Example: escrow
  Description: Escrow program using Anchor
*/
// Placeholder for actual Rust code
fn escrow_program() {
    // ... implementation ...
}
```

---

### Request SOL Airdrop (Solana CLI)

Source: https://www.anchor-lang.com/docs/installation

Requests an airdrop of devnet SOL to fund your wallet for transaction fees and program deployment. The command is limited to 5 SOL per request on devnet due to rate limits. Ensure your cluster is set to devnet.

```shell
solana config set -ud
solana airdrop 2
solana balance
```

---

### Install Recommended Solana Tooling (Agave)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Installs the recommended Solana tooling version 2.1.0, which includes renamed binaries (Agave). This script fetches and installs the necessary binaries.

```sh
sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.0/install)"
```

---

### Close Solana Program Account

Source: https://www.anchor-lang.com/docs/quickstart/local

To reclaim the SOL allocated to a program account, use the `solana program close` command followed by the program ID and the `--bypass-warning` flag. Note that the program ID cannot be reused after closing.

```bash
solana program close 3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg --bypass-warning
```

---

### Anchor Token Extensions: Token Metadata

Source: https://www.anchor-lang.com/docs/references/examples

An example of managing token metadata for Token 2022 extensions using Anchor. This includes setting URIs, names, and symbols.

```Rust
/*
  Example: metadata
  Description: Token metadata with Anchor
*/
// Placeholder for actual Rust code
fn token_metadata() {
    // ... implementation ...
}
```

---

### Run Anchor Program Tests

Source: https://www.anchor-lang.com/docs/quickstart/solpg

Executes the test file for the Anchor program after it has been deployed. This command runs the client-side tests.

```bash
test
```

---

### Anchor Basics: Reallocate Account Data

Source: https://www.anchor-lang.com/docs/references/examples

Shows how to reallocate account data using Anchor. This is useful for resizing accounts when more or less storage is needed.

```Rust
/*
  Example: realloc
  Description: Reallocate account data with Anchor
*/
// Placeholder for actual Rust code
fn realloc_account() {
    // ... implementation ...
}
```

---

### Configure Zsh Shell Completions for Anchor and AVM

Source: https://www.anchor-lang.com/docs/installation

Enables shell completions for Anchor and AVM in Zsh. This involves ensuring `compinit` is loaded in `.zshrc` and then piping the completion commands to the appropriate system directory, followed by reloading the shell.

```zsh
autoload -U compinit
compinit -i
anchor completions zsh | sudo tee /usr/local/share/zsh/site-functions/_anchor
avm completions zsh | sudo tee /usr/local/share/zsh/site-functions/_avm
exec zsh
```

---

### Anchor Basics: Close Account

Source: https://www.anchor-lang.com/docs/references/examples

Provides an example of closing an account using Anchor. This is crucial for managing program state and freeing up resources on the Solana blockchain.

```Rust
/*
  Example: close-account
  Description: Close account example with Anchor
*/
// Placeholder for actual Rust code
fn close_account() {
    // ... implementation ...
}
```

---

### Anchor Token Extensions: Default Account State

Source: https://www.anchor-lang.com/docs/references/examples

Shows how to set up default account states for Token 2022 extensions using Anchor. This simplifies account initialization.

```Rust
/*
  Example: default-account-state
  Description: Default account state setup with Anchor
*/
// Placeholder for actual Rust code
fn default_account_state() {
    // ... implementation ...
}
```

---

### Anchor Version Manager (avm) Usage

Source: https://www.anchor-lang.com/docs/references/avm

This snippet outlines the general usage of the avm command, including available options and subcommands for managing Anchor CLI versions. It shows how to get help, install, list, uninstall, and use specific versions.

```Shell
Anchor version manager
USAGE:
    avm <SUBCOMMAND>
OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
SUBCOMMANDS:
    help         Print this message or the help of the given subcommand(s)
    install      Install a version of Anchor
    list         List available versions of Anchor
    uninstall    Uninstall a version of Anchor
    use          Use a specific version of Anchor
```

---

### Reload PATH for Rust

Source: https://www.anchor-lang.com/docs/installation

Reloads the shell's PATH environment variable to include Cargo's bin directory after Rust installation.

```bash
. "$HOME/.cargo/env"
```

---

### Interact with Anchor Programs using Rust Client

Source: https://www.anchor-lang.com/docs/clients

This guide explains how to use Anchor's Rust client library to interact with Solana programs. It covers the necessary setup and usage patterns for Rust developers.

```Rust
// Example usage of Anchor's Rust client library
// Requires adding anchor_client and anchor_lang to Cargo.toml

use anchor_client::anchor_auto::anchor_client;
use anchor_client::anchor_auto::anchor_client::Program;
use anchor_client::anchor_auto::anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::anchor_auto::anchor_client::solana_sdk::signature::{Keypair, Signer};

fn main() {
    // Configure the client to use the local cluster.
    let payer = Keypair::new(); // Replace with your payer keypair
    let program_id = Pubkey::new_unique(); // Replace with your program ID

    // Load your program's IDL (e.g., from a file)
    let idl = anchor_client::anchor_auto::anchor_client::Idl::load(
        "./path/to/your/program.json"
    ).expect("Failed to load IDL");

    let client = anchor_client::Client::new_with_options(anchor_client::Cluster::Localnet, std::rc::Rc::new(payer), anchor_client::Options::default());
    let program = client.program(program_id, idl);

    // Example: Call an instruction
    // let mut ctx = program.request();
    // ctx.instruction(&anchor_client::anchor_auto::anchor_client::Instruction::Initialize { /* instruction args */ });
    // let signature = ctx.submit().expect("Failed to submit transaction");
    // println!("Transaction signature: {}", signature);
}

```

---

### Anchor Token Extensions: Basics

Source: https://www.anchor-lang.com/docs/references/examples

Introduces the basics of Token 2022 extensions using Anchor. This covers the foundational concepts for extending token functionality.

```Rust
/*
  Example: basics
  Description: Basics of Token 2022 with Anchor
*/
// Placeholder for actual Rust code
fn token_extension_basics() {
    // ... implementation ...
}
```

---

### Anchor Tokens: Token Swap

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates a token swap program using Anchor. This enables users to exchange one type of SPL token for another.

```Rust
/*
  Example: token-swap
  Description: Swap tokens with Anchor
*/
// Placeholder for actual Rust code
fn token_swap() {
    // ... implementation ...
}
```

---

### Anchor Basics: Transfer SOL

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates how to transfer SOL tokens between accounts using Anchor. This is a fundamental token operation on Solana.

```Rust
/*
  Example: transfer-sol
  Description: Transfer SOL tokens with Anchor
*/
// Placeholder for actual Rust code
fn transfer_sol() {
    // ... implementation ...
}
```

---

### Hello Anchor Program (Rust)

Source: https://www.anchor-lang.com/docs/quickstart/solpg

A basic Anchor program written in Rust for Solana. It defines an 'initialize' function that takes a u64 data value, creates a new account, and stores the data. It includes account definitions and program logic.

```rust
use anchor_lang::prelude::*;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()><>
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct NewAccount {
    data: u64
}
```

---

### Anchor Basics: Checking Accounts

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates how to check accounts in Anchor programs. This example covers the fundamental aspects of account validation and interaction within the Anchor framework.

```Rust
/*
  Example: checking-accounts
  Description: Checking account example with Anchor
*/
// Placeholder for actual Rust code
fn check_accounts() {
    // ... implementation ...
}
```

---

### Anchor Tokens: NFT Minter

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates how to mint Non-Fungible Tokens (NFTs) using Anchor. This example covers the process of creating unique digital assets.

```Rust
/*
  Example: nft-minter
  Description: Mint NFTs using Anchor
*/
// Placeholder for actual Rust code
fn nft_minter() {
    // ... implementation ...
}
```

---

### Anchor Tokens: Create SPL Token

Source: https://www.anchor-lang.com/docs/references/examples

Shows how to create a new SPL token using Anchor. This involves defining token metadata and initializing the token account.

```Rust
/*
  Example: create-token
  Description: Create an SPL token with Anchor
*/
// Placeholder for actual Rust code
fn create_spl_token() {
    // ... implementation ...
}
```

---

### Anchor AVM: Prompt for Installation with 'use' Command

Source: https://www.anchor-lang.com/docs/updates/changelog

When using the `anchor avm use` command, if the specified version is not installed, the tool will now prompt the user to install it.

```bash
anchor avm use <version>
```

---

### Anchor Initialize Account Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Creates the account via a CPI to the system program and initializes it by setting its account discriminator.

```rust
#[account(
    init,
    payer = <target_account>,
    space = <num_bytes>
)]
```

---

### Build Anchor Program

Source: https://www.anchor-lang.com/docs/quickstart/solpg

Builds the Anchor program. After building, the program's on-chain address is updated in the `declare_id!()` macro.

```bash
build
```

---

### Interact with Anchor Programs using TypeScript Client

Source: https://www.anchor-lang.com/docs/clients

This guide explains how to use Anchor's TypeScript client library to interact with Solana programs. It details the setup and usage for client-side interactions.

```TypeScript
// Example usage of Anchor's TypeScript client library
// Requires installation of @coral-xyz/anchor

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyProgram } from "./idl/my_program"; // Assuming your IDL is in ./idl/my_program.ts

async function main() {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Replace with your program's IDL and program ID
  const programId = new anchor.web3.PublicKey("YOUR_PROGRAM_ID");
  const idl = /* Your IDL JSON object or imported IDL */;

  const program = new Program<MyProgram>(idl, programId, provider);

  // Example: Call an instruction
  try {
    const tx = await program.methods.initialize(/* instruction args */).rpc();
    console.log("Transaction sent:", tx);
    await provider.connection.confirmTransaction(tx, "confirmed");
    console.log("Transaction confirmed!");
  } catch (error) {
    console.error("Error calling instruction:", error);
  }
}

main();

```

---

### Deploy Anchor Program

Source: https://www.anchor-lang.com/docs/quickstart/solpg

Deploys the built Anchor program to the Solana network (devnet by default). Requires SOL to be allocated to the program's on-chain account. SOL can be obtained via `solana airdrop 5` or the Web Faucet.

```bash
deploy
```

---

### Anchor AVM: Short Alias for 'install' and 'list'

Source: https://www.anchor-lang.com/docs/updates/changelog

Short aliases have been added for the `anchor avm install` and `anchor avm list` commands, streamlining their usage.

```bash
anchor avm i <version> # Alias for install
anchor avm l         # Alias for list
```

---

### Anchor Token Extensions: Token Grouping

Source: https://www.anchor-lang.com/docs/references/examples

An example of token grouping functionality for Token 2022 extensions using Anchor. This allows for managing collections of tokens.

```Rust
/*
  Example: group
  Description: Token grouping example with Anchor
*/
// Placeholder for actual Rust code
fn token_grouping() {
    // ... implementation ...
}
```

---

### Install Anchor CLI Version

Source: https://www.anchor-lang.com/docs/references/avm

This command installs a specified version of the anchor-cli. Versions can be specified using semantic versioning, a commit hash (full or short), or the keyword 'latest' to install the most recent version.

```Shell
avm install <VERSION_OR_COMMIT>
```

```Shell
# <VERSION>-<COMMIT>
avm install 0.30.1-cfe82aa682138f7c6c58bf7a78f48f7d63e9e466
```

```Shell
# Full commit hash
avm install cfe82aa682138f7c6c58bf7a78f48f7d63e9e466
```

```Shell
# Short commit hash
avm install cfe82aa
```

---

### LiteSVM Program Deployment Example (Rust)

Source: https://www.anchor-lang.com/docs/testing/litesvm

Illustrates how to add and test a compiled Solana program (SPL example logging) using LiteSVM in Rust. It shows setting up the program ID, creating an instruction, adding the program to the SVM, airdropping funds, simulating, and sending the transaction, then asserting log output and compute units consumed.

```Rust
use {
    litesvm::LiteSVM,
    solana_instruction::{account_meta::AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_pubkey::{pubkey, Pubkey},
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::VersionedTransaction,
};

fn test_logging() {
    let program_id = pubkey!("Logging111111111111111111111111111111111111");
    let account_meta = AccountMeta {
        pubkey: Pubkey::new_unique(),
        is_signer: false,
        is_writable: true,
    };
    let ix = Instruction {
        program_id,
        accounts: vec![account_meta],
        data: vec![5, 10, 11, 12, 13, 14],
    };

    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    let bytes = include_bytes!("../../node-litesvm/program_bytes/spl_example_logging.so");

    svm.add_program(program_id, bytes);
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    // let's sim it first
    let sim_res = svm.simulate_transaction(tx.clone()).unwrap();
    let meta = svm.send_transaction(tx).unwrap();

    assert_eq!(sim_res.meta, meta);
    assert_eq!(meta.logs[1], "Program log: static string");
    assert!(meta.compute_units_consumed < 10_000) // not being precise here in case it changes
}
```

---

### Anchor Token Extensions: Transfer Hook

Source: https://www.anchor-lang.com/docs/references/examples

An example of implementing a transfer hook for Token 2022 extensions using Anchor. This allows custom logic to be executed during transfers.

```Rust
/*
  Example: transfer-hook
  Description: Transfer hook example with Anchor
*/
// Placeholder for actual Rust code
fn transfer_hook_example() {
    // ... implementation ...
}
```

---

### Anchor Token Extensions: Multiple Extensions

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates how to use multiple Token 2022 extensions simultaneously in an Anchor program. This showcases combining different functionalities.

```Rust
/*
  Example: multiple-extensions
  Description: Multiple extensions example with Anchor
*/
// Placeholder for actual Rust code
fn multiple_extensions() {
    // ... implementation ...
}
```

---

### Anchor Token Extensions: NFT Metadata Pointer

Source: https://www.anchor-lang.com/docs/references/examples

Shows how to implement an NFT metadata pointer for Token 2022 extensions using Anchor. This allows off-chain metadata storage.

```Rust
/*
  Example: nft-meta-data-pointer
  Description: NFT metadata pointer with Anchor
*/
// Placeholder for actual Rust code
fn nft_metadata_pointer() {
    // ... implementation ...
}
```

---

### Initialize Project with Mollusk Test Template

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Initializes a new Anchor project using the 'mollusk' test template, which provides a specific testing environment setup.

```bash
anchor init my-program --test-template mollusk
```

---

### LiteSVM Installation (Rust)

Source: https://www.anchor-lang.com/docs/testing/litesvm

Installs the LiteSVM library as a development dependency for Rust projects using Cargo.

```Rust
cargo add litesvm --dev
```

---

### Anchor Tokens: SPL Token Minter

Source: https://www.anchor-lang.com/docs/references/examples

A program for minting SPL tokens using Anchor. This example focuses on the creation and distribution of fungible tokens.

```Rust
/*
  Example: spl-token-minter
  Description: SPL token minting with Anchor
*/
// Placeholder for actual Rust code
fn spl_token_minter() {
    // ... implementation ...
}
```

---

### Anchor Basics: PDA Rent Payer

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates the use of Program Derived Addresses (PDAs) as rent payers in Anchor programs. This is important for managing account rent.

```Rust
/*
  Example: pda-rent-payer
  Description: PDA rent payer example with Anchor
*/
// Placeholder for actual Rust code
fn pda_rent_payer() {
    // ... implementation ...
}
```

---

### Install Anchor CLI from Commit using AVM

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Demonstrates how to install and use specific commits of the Anchor CLI with avm, including full and short commit hashes.

```bash
# <VERSION>-<COMMIT>
avm install 0.28.0-6cf200493a307c01487c7b492b4893e0d6f6cb23
# Full commit hash
avm install 6cf200493a307c01487c7b492b4893e0d6f6cb23
# Short commit hash
avm install 6cf200
```

```bash
avm use 0.28.0-6cf200493a307c01487c7b492b4893e0d6f6cb23
```

```toml
[toolchain]
anchor_version = "0.28.0-6cf200493a307c01487c7b492b4893e0d6f6cb23"
```

---

### Test Anchor Program (TypeScript)

Source: https://www.anchor-lang.com/docs/quickstart/solpg

Demonstrates how to invoke the 'initialize' instruction of an Anchor program from a client-side TypeScript test file. It includes generating keypairs, sending transactions, confirming them, and fetching account data.

```typescript
// No imports needed: web3, anchor, pg and more are globally available
describe("Test", () => {
  it("initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new web3.Keypair();
    // Send transaction
    const data = new BN(42);
    const txHash = await pg.program.methods
      .initialize(data)
      .accounts({
        newAccount: newAccountKp.publicKey,
        signer: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([newAccountKp])
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);
    // Fetch the created account
    const newAccount = await pg.program.account.newAccount.fetch(
      newAccountKp.publicKey
    );
    console.log("On-chain data is:", newAccount.data.toString());
    // Check whether the data on-chain is equal to local 'data'
    assert(data.eq(newAccount.data));
  });
});
```

---

### Install Anchor CLI (avm)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Installs the latest version of Anchor Version Manager (avm) from source, which is used to manage Anchor CLI installations.

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --force
```

---

### List Available Anchor Versions

Source: https://www.anchor-lang.com/docs/references/avm

This command lists all available versions of the Anchor CLI that can be installed or managed by avm.

```Shell
avm list
```

---

### Install Anchor CLI Version 0.31.0

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Installs a specific version of the Anchor CLI using avm. This command downloads pre-compiled binaries.

```bash
avm install 0.31.0
```

---

### Anchor Tokens: NFT Operations

Source: https://www.anchor-lang.com/docs/references/examples

Covers various operations related to NFTs using Anchor, such as transferring, burning, and updating metadata.

```Rust
/*
  Example: nft-operations
  Description: NFT operations with Anchor
*/
// Placeholder for actual Rust code
fn nft_operations() {
    // ... implementation ...
}
```

---

### Anchor Token Extensions: Interest-Bearing Tokens

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates creating interest-bearing tokens using Token 2022 extensions with Anchor. This allows tokens to accrue interest over time.

```Rust
/*
  Example: interest-bearing
  Description: Interest-bearing tokens using Anchor
*/
// Placeholder for actual Rust code
fn interest_bearing_tokens() {
    // ... implementation ...
}
```

---

### avm install no longer downloads if version exists

Source: https://www.anchor-lang.com/docs/updates/changelog

The `avm install` command in Anchor CLI now checks if a version is already installed on the machine and avoids re-downloading it, optimizing installation.

```bash
`avm install` no longer downloads the version if already installed in the machine (#1670).
```

---

### Anchor Basics: Cross Program Invocation

Source: https://www.anchor-lang.com/docs/references/examples

Illustrates how to perform cross-program invocations (CPI) using Anchor. This allows programs to call and interact with other programs on Solana.

```Rust
/*
  Example: cross-program-invocation
  Description: Cross program invocation with Anchor
*/
// Placeholder for actual Rust code
fn cross_program_invocation() {
    // ... implementation ...
}
```

---

### LiteSVM Minimal Example (Rust)

Source: https://www.anchor-lang.com/docs/testing/litesvm

Demonstrates a minimal example of using LiteSVM in Rust to perform a token transfer between two accounts. It covers setting up the SVM, airdropping funds, creating a transfer instruction, sending a transaction, and verifying account balances.

```Rust
use litesvm::LiteSVM;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_system_interface::instruction::transfer;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_transaction::Transaction;

let from_keypair = Keypair::new();
let from = from_keypair.pubkey();
let to = Pubkey::new_unique();
let mut svm = LiteSVM::new();

svm.airdrop(&from, 10_000).unwrap();

let instruction = transfer(&from, &to, 64);
let tx = Transaction::new(
    &[&from_keypair],
    Message::new(&[instruction], Some(&from)),
    svm.latest_blockhash(),
);

let tx_res = svm.send_transaction(tx).unwrap();

let from_account = svm.get_account(&from);
let to_account = svm.get_account(&to);

assert_eq!(from_account.unwrap().lamports, 4936);
assert_eq!(to_account.unwrap().lamports, 64);
```

---

### Anchor Token Extensions: Transfer Fee

Source: https://www.anchor-lang.com/docs/references/examples

Shows how to implement transfer fees for Token 2022 extensions using Anchor. This allows charging a fee on token transfers.

```Rust
/*
  Example: transfer-fee
  Description: Transfer fees example with Anchor
*/
// Placeholder for actual Rust code
fn transfer_fee_example() {
    // ... implementation ...
}
```

---

### Anchor Token Extensions: Non-Transferable Tokens

Source: https://www.anchor-lang.com/docs/references/examples

An example of creating non-transferable tokens using Token 2022 extensions with Anchor. This restricts token movement.

```Rust
/*
  Example: non-transferable
  Description: Non-transferable tokens using Anchor
*/
// Placeholder for actual Rust code
fn non_transferable_tokens() {
    // ... implementation ...
}
```

---

### Install Solana from anza.xyz Domain in Docker Verifiable Builds (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Configures Docker verifiable builds to install Solana from the anza.xyz domain.

```bash
cli: Install Solana from anza.xyz domain in Docker verifiable builds (#3271).
```

---

### Anchor Tokens: Transfer SPL Tokens

Source: https://www.anchor-lang.com/docs/references/examples

Shows how to transfer SPL tokens between accounts using Anchor. This is a core functionality for managing token balances.

```Rust
/*
  Example: transfer-tokens
  Description: Transfer SPL tokens using Anchor
*/
// Placeholder for actual Rust code
fn transfer_spl_tokens() {
    // ... implementation ...
}
```

---

### Anchor Initialize If Needed Account Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Similar to 'init', but only runs if the account does not already exist. Requires the 'init-if-needed' cargo feature.

```rust
#[account(
    init_if_needed,
    payer = <target_account>
)]
#[account(
    init_if_needed,
    payer = <target_account>,
    space = <num_bytes>
)]
```

---

### Initialize Project with Multiple Files Template

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Commands to create a new Anchor project with a pre-defined multi-file structure for better organization.

```bash
anchor init <NAME> --template multiple
```

```bash
anchor new <NAME> --template multiple
```

---

### Anchor SPL Token Constraints

Source: https://www.anchor-lang.com/docs/references/account-constraints

Create or validate token accounts, specifying mint and authority, and optionally the token program.

```rust
#[account(
    token::mint = <target_account>,
    token::authority = <target_account>
)]
#[account(
    token::mint = <target_account>,
    token::authority = <target_account>,
    token::token_program = <target_account>
)]
```

---

### Anchor AVM: Install from Commit

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Version Manager (AVM) now supports installing `anchor-cli` directly from a Git commit hash, allowing for testing specific development versions.

```bash
# Install Anchor CLI from a specific commit hash
avm install <commit_hash>

# Example:
avm install a1b2c3d4e5f67890abcdef1234567890abcdef12

```

---

### Anchor Basics: Program Derived Addresses

Source: https://www.anchor-lang.com/docs/references/examples

Illustrates the concept and usage of Program Derived Addresses (PDAs) in Anchor. PDAs are crucial for creating deterministic addresses controlled by programs.

```Rust
/*
  Example: program-derived-addresses
  Description: Program-derived addresses with Anchor
*/
// Placeholder for actual Rust code
fn program_derived_addresses() {
    // ... implementation ...
}
```

---

### Anchor Token Extensions: Permanent Delegate

Source: https://www.anchor-lang.com/docs/references/examples

Illustrates setting a permanent delegate for Token 2022 extensions using Anchor. This designates a specific address to manage token accounts.

```Rust
/*
  Example: permanent-delegate
  Description: Permanent delegate setup with Anchor
*/
// Placeholder for actual Rust code
fn permanent_delegate_setup() {
    // ... implementation ...
}
```

---

### Anchor Program Structure Example

Source: https://www.anchor-lang.com/docs/basics/program-structure

Demonstrates a basic Anchor program with an 'initialize' instruction. It showcases the use of `declare_id`, `#[program]`, `#[derive(Accounts)]`, and `#[account]` macros to define program logic, account requirements, and custom account types.

```Rust
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct NewAccount {
    data: u64,
}
```

---

### Anchor Token Extensions: Memo Transfer

Source: https://www.anchor-lang.com/docs/references/examples

Shows how to implement memo transfers for Token 2022 extensions using Anchor. This allows attaching arbitrary data to token transfers.

```Rust
/*
  Example: memo-transfer
  Description: Memo transfer with Anchor
*/
// Placeholder for actual Rust code
fn memo_transfer() {
    // ... implementation ...
}
```

---

### Create Token Mint with Anchor

Source: https://www.anchor-lang.com/docs/tokens/basics

Demonstrates how to create and initialize token mint accounts in Solana programs using Anchor. This includes examples for creating mint accounts with generated keypairs or Program Derived Addresses (PDAs).

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(init,
        payer = payer,
        space = 82, // Mint account size
        mint::authority = authority,
        mint::decimals = 6
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK: Authority can be any account
    #[account(seeds = [b"authority"], bump)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: ProgramId,
}

pub fn handler(ctx: Context<CreateMint>) -> Result<()> {
    Ok(())
}
```

---

### Anchor Token Extensions: CPI Guard

Source: https://www.anchor-lang.com/docs/references/examples

Demonstrates the CPI guard feature for Token 2022 extensions using Anchor. This enhances security by controlling cross-program invocations.

```Rust
/*
  Example: cpi-guard
  Description: CPI guard example with Anchor
*/
// Placeholder for actual Rust code
fn cpi_guard_example() {
    // ... implementation ...
}
```

---

### Anchor Initialize Instruction Handler

Source: https://www.anchor-lang.com/docs/basics/program-structure

An example of an initialize instruction handler in Anchor Lang. It takes a Context and data, updates account data, and logs a message.

```rust
pub fn initialize(ctx:Context<Initialize>, data: u64) -> Result<()> {
    ctx.accounts.new_account.data = data;
    msg!("Changed data to: {}!", data);
    Ok(())
}
```

---

### Customize Installation Location with AVM_HOME (Bash)

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows customization of the Anchor Version Manager (AVM) installation location by setting the `AVM_HOME` environment variable.

```bash
avm: Support customizing the installation location using `AVM_HOME` environment variable (#2917).
```

---

### Valid Usage: Multiple Arguments with #[instruction]

Source: https://www.anchor-lang.com/docs/references/account-constraints

Shows a valid example of using the #[instruction(...)] attribute with multiple arguments. Both `input_one` and `input_two` are correctly declared in the attribute and the struct, matching the order in the instruction handler.

```rust
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, input_one: String, input_two: String) -> Result<()> {
        // --snip--
    }
}
#[derive(Accounts)]

#[instruction(input_one: String, input_two: String)]
pub struct Initialize<'info> {
    // --snip--
}
```

---

### Add --no-install Option to Init Command (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a `--no-install` flag to the `init` command, allowing project initialization without automatic installation.

```bash
cli: Add `--no-install` option to the `init` command (#2945).
```

---

### Anchor Tokens: PDA as Mint Authority

Source: https://www.anchor-lang.com/docs/references/examples

Illustrates using a Program Derived Address (PDA) as the mint authority for SPL tokens with Anchor. This enhances security and control.

```Rust
/*
  Example: pda-mint-authority
  Description: PDA as mint authority with Anchor
*/
// Placeholder for actual Rust code
fn pda_mint_authority() {
    // ... implementation ...
}
```

---

### Install Latest Anchor CLI

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Command to install the latest available version of the Anchor CLI using the avm tool. This ensures you are using the most recent features and bug fixes.

```bash
avm install latest
```

---

### Confirm Solana Transaction

Source: https://www.anchor-lang.com/docs/quickstart/solpg

Confirms a Solana transaction by providing its hash (signature). This command is used to view transaction logs and verify execution.

```bash
solana confirm -v [TxHash]
```

```bash
solana confirm -v 3TewJtiUz1EgtT88pLJHvKFzqrzDNuHVi8CfD2mWmHEBAaMfC5NAaHdmr19qQYfTiBace6XUmADvR4Qrhe8gH5uc
```

---

### Replace Global JS Dependency Installs (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Replaces global JavaScript dependency installations with local ones in the CLI. This improves project isolation and avoids potential conflicts with globally installed packages.

```bash
npm install --save-dev @project-serum/anchor-cli
```

---

### Anchor Address Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks if the account key matches the provided public key expression.

```rust
#[account(address = <expr>)]
#[account(address = <expr> @ <custom_error>)]
```

---

### Create Program Instance (Frontend/Node with Wallet)

Source: https://www.anchor-lang.com/docs/clients/typescript

Sets up an Anchor Program instance for frontend or Node.js applications using a wallet adapter. It requires the program's IDL, connection, and wallet to create an AnchorProvider, which is then used to instantiate the Program. This setup allows for building and sending transactions with a default signer.

```TypeScript
import { Program, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import type { HelloAnchor } from "./idlType";
import idl from "./idl.json";

const { connection } = useConnection();
const wallet = useAnchorWallet();

const provider = new AnchorProvider(connection, wallet, {});
setProvider(provider);

export const program = new Program(idl as HelloAnchor, {
  connection,
});
```

---

### Anchor Token Extensions: Immutable Owner

Source: https://www.anchor-lang.com/docs/references/examples

Illustrates setting up an immutable owner for Token 2022 extensions using Anchor. This prevents the owner from changing after creation.

```Rust
/*
  Example: immutable-owner
  Description: Immutable owner setup with Anchor
*/
// Placeholder for actual Rust code
fn immutable_owner_setup() {
    // ... implementation ...
}
```

---

### Anchor Executable Account Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks if the account is executable, meaning it is a program account.

```rust
#[account(executable)]
```

---

### Ergonomic Lamport Retrieval Method

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

A new, more performant method for getting the lamport balance of an account compared to using `to_account_info()`.

```rust
let lamports = ctx.accounts.my_account.get_lamports();
```

---

### Instruction Discriminator Example (Hex)

Source: https://www.anchor-lang.com/docs/basics/idl

Shows the hexadecimal output of hashing 'global:initialize', highlighting the first 8 bytes which serve as the instruction's discriminator.

```shell
af af 6d 1f 0d 98 9b ed d4 6a 95 07 32 81 ad c2 1b b5 e0 e1 d7 73 b2 fb bd 7a b5 04 cd d4 aa 30
```

---

### Install Anchor CLI Version

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Command to install a specific version of the Anchor CLI using the `avm` (Anchor Version Manager) tool. This is typically used when upgrading or downgrading the Anchor CLI.

```bash
avm install 0.30.1
```

---

### Anchor.toml: Test Genesis Program Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Configures the `solana-test-validator` to start with specific programs pre-loaded. This is achieved using the `address` and `program` fields, with an optional `upgradeable` flag.

```toml
[[test.genesis]]
address = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"
program = "dex.so"
[[test.genesis]]
address = "22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD"
program = "swap.so"
upgradeable = true

```

---

### Create Token Account with Anchor

Source: https://www.anchor-lang.com/docs/tokens/basics

Explains how to create and initialize token accounts in Solana programs using Anchor. It covers creating Associated Token Accounts (ATAs) and Program Derived Address (PDA) token accounts with practical code examples.

```rust
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: Mint can be any account
    pub mint: AccountInfo<'info>,
    #[account(init,
        payer = owner,
        associated_token::authority = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateTokenAccount>) -> Result<()> {
    Ok(())
}
```

---

### Anchor Reallocate Account Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Used to reallocate program account space at the beginning of an instruction, with options for payer and zeroing.

```rust
#[account(
    realloc = <space>,
    realloc::payer = <target>,
    realloc::zero = <bool>
)]
```

---

### Anchor Sysvar<'info, T> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Shows how to use `Sysvar<'info, T>` to validate that an account is a system variable (sysvar) and deserialize it. Examples include `Rent` and `Clock` sysvars.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub rent:Sysvar<'info, Rent>,
    pub clock:Sysvar<'info, Clock>,
}
```

---

### Upgrade Solana Version

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Command to upgrade the local Solana installation to a specific recommended version. This ensures compatibility with the Anchor release and its features.

```bash
solana-install init 1.18.8
```

---

### Create Token Account with PDA (Anchor)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Illustrates creating a token account with a PDA derived from program-defined seeds. This example uses `token::mint`, `token::authority`, and `token::token_program` constraints along with `seeds` and `bump`.

```rust
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
// --snip--
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(


        init_if_needed,
        payer = signer,
        token::mint = mint,
        token::authority = token_account,
        token::token_program = token_program,
seeds= [b"token"],
bump
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
```

---

### Anchor Lang: Permanent Delegate Extension Attribute

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating the permanent delegate extension on a mint account. Specifies the permanent delegate for the account.

```rust
#[account(
    extensions::permanent_delegate::delegate = <target_account>
)]
```

---

### Exporting mpl-token-metadata (Cargo.toml)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Provides an example of how to manage dependencies when using the `mpl-token-metadata` crate with `anchor-spl`. It suggests removing the direct dependency and using the version exported by `anchor-spl`.

```TOML
[dependencies]
anchor-spl = { version = "0.29.0", features = ["metadata"] }
- mpl-token-metadata = "1.13.1"
```

---

### Anchor Owner Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks if the account owner matches the provided expression.

```rust
#[account(owner = <expr>)]
#[account(owner = <expr> @ <custom_error>)]
```

---

### Anchor Stack Overflow Error Example

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

This example illustrates a typical stack overflow error message encountered during Anchor builds when stack usage exceeds the allocated limit (4096 bytes). It highlights the 'Stack offset' and the difference between the used and maximum offset, indicating a need to minimize large stack variables.

```text
Error: Function _ZN71_$LT$pr..Test$u20$as$u20$anchor_lang..Accounts$LT$pr..TestBumps$GT$$GT$12try_accounts17h5572074d55b9e638E Stack offset of 4112 exceeded max offset of 4096 by 16 bytes, please minimize large stack variables.
```

---

### Rust - Create Token Mint Account

Source: https://www.anchor-lang.com/docs/tokens/basics/create-mint

Provides a Rust code example for creating and initializing a token mint account within a Solana program using Anchor. It specifies account constraints for decimals, mint authority, and freeze authority.

```rust
use anchor_spl::token_interface::{Mint, TokenInterface};
// --snip--
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]


    pub mint: InterfaceAccount<'info, Mint>,


    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
```

---

### Anchor Token Extensions: Mint Close Authority

Source: https://www.anchor-lang.com/docs/references/examples

Illustrates setting the mint close authority for Token 2022 extensions using Anchor. This controls who can close the mint.

```Rust
/*
  Example: mint-close-authority
  Description: Mint close authority with Anchor
*/
// Placeholder for actual Rust code
fn mint_close_authority() {
    // ... implementation ...
}
```

---

### Anchor Close Account Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Closes the account by sending its remaining lamports to the target account and resetting its data.

```rust
#[account(close = <target_account>)]
```

---

### Anchor CLI: `migrate` Command Fix

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves an issue where the `migrate` command failed if `ts-node` was not installed globally. This fix allows the `migrate` command to function correctly even without a global `ts-node` installation.

```bash
# This fix ensures the migrate command works correctly.
# It might involve using a locally installed ts-node or a different execution method.
anchor migrate

```

---

### Anchor Custom Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Applies a custom constraint that evaluates a given expression to determine validity.

```rust
#[account(constraint = <expr>)]
#[account(
    constraint = <expr> @ <custom_error>
)]
```

---

### Anchor IDL Instruction Definition

Source: https://www.anchor-lang.com/docs/basics/idl

An example snippet from an Anchor IDL file showing the structure of an instruction definition, including its name and the byte array representing its discriminator.

```json
{
  "name": "initialize",
  "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
  ...
}
```

---

### Anchor SystemAccount<'info> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Demonstrates the `SystemAccount<'info>` type, which validates that the account is owned by the system program. This is commonly used for accounts that will be created or managed by the system program.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub account:SystemAccount<'info>,
}
```

---

### Anchor Option<Account<'info, T>> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Demonstrates the use of `Option<Account<'info, T>>` for handling optional accounts. This allows a program to function even if a particular account is not provided in the transaction.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub account:Option<Account<'info, AccountType>>,
}
```

---

### Anchor Lang: Metadata Pointer Extension Attribute

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating the metadata pointer extension on a mint account. Specifies the authority and metadata address for the pointer.

```rust
#[account(
    extensions::metadata_pointer::authority = <target_account>,
    extensions::metadata_pointer::metadata_address = <target_account>
)]
```

---

### Anchor Has One Account Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks if the target field on the account matches the key of the target field in the Accounts struct.

```rust
#[account(
    has_one = <target_account>
)]
#[account(
    has_one = <target_account> @ <custom_error>
)]
```

---

### Anchor Zero Discriminator Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks if the account discriminator is zero. This is useful for accounts larger than 10 KiB.

```rust
#[account(zero)]
```

---

### Declare Program with `declare_program!` Macro

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Example of using the `declare_program!` macro to generate program clients from an IDL file. This macro allows for dependency-free program interactions, both for on-chain CPI and off-chain RPC calls, by referencing the IDL located in the `idls` directory.

```rust
declare_program!(program_name);
```

---

### Anchor CLI: `--test-template` for `init`

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `--test-template` option to the `init` command in the Anchor CLI. This allows users to specify a template for their test setup when initializing a new project.

```bash
anchor init --test-template <TEMPLATE_NAME>
```

---

### Close Anchor Program

Source: https://www.anchor-lang.com/docs/quickstart/solpg

Closes an on-chain Anchor program, recovering the allocated SOL. Requires the program's address, which can be found in `declare_id!()`.

```bash
solana program close [ProgramID]
```

```bash
solana program close 2VvQ11q8xrn5tkPNyeraRsPaATdiPx8weLAD8aD4dn2r
```

---

### Anchor Init Project

Source: https://www.anchor-lang.com/docs/references/cli

Initializes a new Anchor project workspace, creating essential configuration files and directories for Solana program development, including Anchor.toml, Cargo.toml, package.json, programs/, app/, tests/, and migrations/deploy.js.

```bash
anchor init
```

---

### Anchor Program<'info, T> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Shows how to use `Program<'info, T>` to validate that an account is the specified program, such as the System program or the Token program. This is essential for ensuring correct program interactions.

```rust
use anchor_spl::token::Token;
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub system_program:Program<'info, System>,
    pub token_program:Program<'info, Token>,
}
```

---

### Mint Tokens with Anchor

Source: https://www.anchor-lang.com/docs/tokens/basics

Details how to mint tokens in Solana programs using Anchor. This involves creating new tokens via cross-program invocations (CPIs) to the Token Program, with accompanying code examples.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, MintTo};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    anchor_spl::token::mint_to(CpiContext::new(cpi_program, cpi_accounts), amount)
}
```

---

### Anchor Mutable Account Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks if the given account is mutable, allowing Anchor to persist state changes.

```rust
#[account(mut)]
#[account(mut @ <custom_error>)]
```

---

### Anchor Signer Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks if the given account signed the transaction. Consider using the Signer type for accounts with only this constraint.

```rust
#[account(signer)]
#[account(signer @ <custom_error>)]
```

---

### Upgrade Anchor CLI

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-1

Instructions to upgrade the Anchor CLI to a specific version using the `avm install` command. This is a common step when updating the Anchor framework.

```Shell
avm install 0.31.1
```

---

### Anchor Lang: Mint Account Attributes

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating mint accounts with specified parameters like authority and decimals. Supports optional freeze authority.

```rust
#[account(
    mint::authority = <target_account>,
    mint::decimals = <expr>
)]
```

```rust
#[account(
    mint::authority = <target_account>,
    mint::decimals = <expr>,
    mint::freeze_authority = <target_account>
)]
```

---

### Access Instruction Arguments with #[instruction]

Source: https://www.anchor-lang.com/docs/references/account-constraints

Demonstrates how to use the #[instruction(...)] attribute to access arguments passed to an instruction handler. The arguments must be declared in the same order as they appear in the handler function. Skipping arguments is not allowed.

```rust
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, input: String) -> Result<()> {
        // --snip--
    }
}
#[derive(Accounts)]

#[instruction(input: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4 +input.len(),
    )]
    pub new_account: Account<'info, DataAccount>,
    // --snip--
}
```

---

### Anchor CLI: Test Template for Mollusk

Source: https://www.anchor-lang.com/docs/updates/changelog

A new test template for Mollusk has been added to the Anchor CLI. This provides a starting point for projects utilizing the Mollusk framework.

```bash
anchor init --template mollusk my_mollusk_project
```

---

### Transfer Tokens with Anchor

Source: https://www.anchor-lang.com/docs/tokens/basics

Illustrates how to transfer tokens between token accounts through cross-program invocations (CPIs) in Anchor. This section provides code examples for implementing token transfers.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Transfer};

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)
}
```

---

### Anchor PDA Seeds and Bump Constraint

Source: https://www.anchor-lang.com/docs/references/account-constraints

Checks that the given account is a Program Derived Address (PDA) derived from the currently executing program, the seeds, and optionally the bump.

```rust
#[account(
    seeds = <seeds>,
    bump
)]
#[account(
    seeds = <seeds>,
    bump,
    seeds::program = <expr>
)]
#[account(
    seeds = <seeds>,
    bump = <expr>
)]
#[account(
    seeds = <seeds>,
    bump = <expr>,
    seeds::program = <expr>
)]
```

---

### Configure Solana Version in Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Specifies the Solana toolchain version in the Anchor.toml configuration file. If the version is greater than 1.18.19, it automatically installs and switches to the Agave tooling.

```toml
[toolchain]
solana_version = "2.1.0"
```

---

### Support for Configuration Options in Anchor.toml (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables support for configuration options for `solana-test-validator` directly within the `Anchor.toml` file. This simplifies the setup and management of local development environments.

```toml
[test.validator]
 eth_ws_url = "http://127.0.0.1:8545"

```

---

### Localnet Command for Test Validator (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `localnet` command for easily starting a local `solana-test-validator` with the workspace deployed. This simplifies local development and testing workflows.

```bash
anchor localnet
```

---

### Anchor Lang: Override Token Program Attribute

Source: https://www.anchor-lang.com/docs/references/account-constraints

Allows overriding the default token program for account attributes. This is useful when working with custom token programs.

```rust
#[account(*::token_program = <target_account>)]
```

---

### Rust: IdlBuilder for Programmatic IDL Creation

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Introduces a new method for building IDLs programmatically using 'IdlBuilder', allowing for easier extension of functionality without breaking changes. This example shows how to create an IDL with specific configurations.

```Rust
let idl = IdlBuilder::new().program_path(path).skip_lint(true).build()?;
```

---

### Anchor Lang: Associated Token Account Attributes

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating associated token accounts. Allows specifying the mint, authority, and optionally the token program.

```rust
#[account(
    associated_token::mint = <target_account>,
    associated_token::authority = <target_account>
)]
```

```rust
#[account(
    associated_token::mint = <target_account>,
    associated_token::authority = <target_account>,
    associated_token::token_program = <target_account>
)]
```

---

### Rust Client for Anchor Program Interaction

Source: https://www.anchor-lang.com/docs/clients/rust

Demonstrates a Rust client using `anchor-client` to interact with the Anchor program. It includes setting up the RPC client, generating keypairs, requesting airdrops, and creating a program client to send instructions.

```rust
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        native_token::LAMPORTS_PER_SOL,
        signature::Keypair,
        signer::Signer,
        system_program,
    },
    Client,
    Cluster,
};
use anchor_lang::prelude::*;
use std::rc::Rc;
declare_program!(example);
use example::{accounts::Counter, client::accounts, client::args};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899", // Local validator URL
        CommitmentConfig::confirmed(),
    );
    // Generate Keypairs and request airdrop
    let payer = Keypair::new();
    let counter = Keypair::new();
    println!("Generated Keypairs:");
    println!("   Payer: {}", payer.pubkey());
    println!("   Counter: {}", counter.pubkey());
    println!("\nRequesting 1 SOL airdrop to payer");
    let airdrop_signature = connection.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)?;
    // Wait for airdrop confirmation
    while !connection.confirm_transaction(&airdrop_signature)? {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("   Airdrop confirmed!");
    // Create program client
    let provider = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::confirmed(),
    );
    let program = provider.program(example::ID)?;
    // Build and send instructions
    // ... (further client interaction code would follow here)
    Ok(())
}

```

---

### Anchor CLI: Initialize Git Repository

Source: https://www.anchor-lang.com/docs/updates/changelog

When running `anchor init`, a new Git repository is automatically initialized for the workspace. This feature can be disabled by using the `--no-git` flag, allowing for manual Git setup.

```bash
anchor init --no-git
```

---

### Anchor CLI: Force Init and New

Source: https://www.anchor-lang.com/docs/updates/changelog

This feature allows the 'init' and 'new' commands in the Anchor CLI to be forced, enabling re-initialization or creation of projects even if they already exist. This is useful for resetting project states or starting fresh.

```bash
anchor init --force
anchor new --force
```

---

### Anchor AccountInfo<'info> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Shows how to use `AccountInfo<'info>`, which can be used as a type but recommends using `UncheckedAccount` instead for clarity when no specific checks are intended. This is useful for accounts where ownership or other properties are not validated by Anchor.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    /// CHECK: AccountInfo is an unchecked account


    pub unchecked_account:AccountInfo<'info>,
}
```

---

### Anchor Signer<'info> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Illustrates the `Signer<'info>` type, which validates that the associated account has signed the transaction. This is critical for accounts that need to authorize an action.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub signer:Signer<'info>,
}
```

---

### Valid Usage: Skipping Arguments with #[instruction]

Source: https://www.anchor-lang.com/docs/references/account-constraints

Illustrates a valid scenario where an argument (`input_two`) is skipped in the #[instruction(...)] attribute. This is permissible as long as the skipped argument is not before the last required argument.

```rust
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, input_one: String, input_two: String) -> Result<()> {
        // --snip--
    }
}
#[derive(Accounts)]

#[instruction(input_one: String)]
pub struct Initialize<'info> {
    // --snip--
}
```

---

### Anchor Program Structure

Source: https://www.anchor-lang.com/docs/basics

Explains the structure of Anchor programs, detailing key macros and their functions in streamlining Solana program development.

```Rust
use anchor_lang::prelude::*;

#[program]
mod my_program {
    pub fn initialize(ctx: Context<Initialize>) -> Result {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    pub data_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub value: u64,
}
```

---

### Anchor Test Integration

Source: https://www.anchor-lang.com/docs/references/cli

Runs an integration test suite against the configured blockchain cluster. It automatically deploys new versions of all workspace programs before execution. If connected to a localnet, it starts the local validator. Options exist to skip local validator startup or manage log file output.

```bash
anchor test
```

```bash
anchor test --skip-local-validator
```

---

### Anchor Lang: Group Pointer Extension Attribute

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating the group pointer extension on a mint account. Specifies the authority and group address for the pointer.

```rust
#[account(
    extensions::group_pointer::authority = <target_account>,
    extensions::group_pointer::group_address = <target_account>
)]
```

---

### Fund Anchor.toml Configured Wallet When Testing

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that the wallet configured in Anchor.toml is funded when running tests. This simplifies the testing setup by automatically providing funds to the test wallet.

```bash
anchor test
# Wallet configured in Anchor.toml is automatically funded.

```

---

### Anchor Lang: Close Authority Extension Attribute

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating the close authority extension on a mint account. Specifies the authority responsible for closing the account.

```rust
#[account(
    extensions::close_authority::authority = <target_account>
)]
```

---

### Anchor PDA Constraints: No Optional Seeds

Source: https://www.anchor-lang.com/docs/basics/pda

Demonstrates defining a PDA without any optional seeds using an empty array in the 'seeds' constraint. This is a basic setup for deriving a PDA.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(

        seeds = [],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

---

### Use `rustls-tls` Feature for `reqwest`

Source: https://www.anchor-lang.com/docs/updates/changelog

Switches the `reqwest` dependency to use the `rustls-tls` feature. This eliminates the need for users to have OpenSSL installed, simplifying the build process and improving cross-platform compatibility.

```rust
// This change is typically made in Cargo.toml:
// [dependencies]
// reqwest = { version = "...", features = ["rustls-tls"] }
```

---

### Building an Initialize Instruction (Rust)

Source: https://www.anchor-lang.com/docs/features/declare-program

This snippet illustrates how to build an `initialize` instruction using the generated client module. It uses the `program.request()` builder pattern, specifies the required accounts for the `initialize` instruction (including the `counter` account and `system_program`), and provides the necessary arguments. The `.instructions()?` method generates the instruction, which is then typically removed from the resulting vector.

```rust
// Build initialize instruction
let initialize_ix = program
    .request()
    // Accounts required for initialize instruction
    .accounts(accounts::Initialize {
        counter: counter.pubkey(),
        payer: program.payer(),
        system_program: system_program::ID,
    })
    .args(args::Initialize)
    .instructions()?
        .remove(0);
```

---

### Anchor Lang: Transfer Hook Extension Attribute

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating the transfer hook extension on a mint account. Specifies the authority and program ID for the transfer hook.

```rust
#[account(
    extensions::transfer_hook::authority = <target_account>,
    extensions::transfer_hook::program_id = <target_account>
)]
```

---

### Rust Client Library for Solana Programs

Source: https://www.anchor-lang.com/docs/clients/rust

The 'anchor-client' crate is the official Rust library for interacting with Anchor programs on the Solana blockchain. It simplifies the process of sending transactions and fetching data from your deployed programs. Ensure you have the necessary Solana toolchain and Anchor dependencies installed.

```Rust
use anchor_client::Program;
use solana_sdk::pubkey::Pubkey;

// Example usage (assuming you have a program instance)
// let program_id = Pubkey::new_unique();
// let program = Program::new(anchor_client::Client::new_ref(), program_id);

// Further client interactions would go here...
```

---

### Rust - Update Zero Copy Account

Source: https://www.anchor-lang.com/docs/features/zero-copy

Demonstrates how to update an existing zero-copy account using `AccountLoader` and the `load_mut` method to get mutable access to the account data.

```rust
#[derive(Accounts)]
pub struct Update<'info> {

    #[account(mut)]
    pub data_account: AccountLoader<'info, Data>,
}

pub fn update(ctx: Context<Update>) -> Result<()> {


    let account = &mut ctx.accounts.data_account.load_mut()?;
    account.data = [2; 10232];
    Ok(())
}
```

---

### Invalid Usage: Incorrect Argument Order with #[instruction]

Source: https://www.anchor-lang.com/docs/references/account-constraints

Demonstrates an invalid usage of the #[instruction(...)] attribute where the order of arguments does not match the instruction handler. This will result in an error because `input_two` is declared before `input_one` in the attribute.

```rust
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, input_one: String, input_two: String) -> Result<()> {
        // --snip--
    }
}
#[derive(Accounts)]

#[instruction(input_two: String)]
pub struct Initialize<'info> {
    // --snip--
}
```

---

### Write Arbitrary Account Data with LiteSVM

Source: https://www.anchor-lang.com/docs/testing/litesvm

Illustrates how to write arbitrary account data using LiteSVM in Rust, specifically demonstrating the creation of an account with a large amount of USDC. This bypasses the need for actual USDC mints in tests, simplifying the setup for testing token-related functionalities.

```Rust
use {
    litesvm::LiteSVM,
    solana_account::Account,
    solana_program_option::COption,
    solana_program_pack::Pack,
    solana_pubkey::{pubkey, Pubkey},
    spl_associated_token_account_client::address::get_associated_token_address,
    spl_token::{
        state::{Account as TokenAccount, AccountState},
        ID as TOKEN_PROGRAM_ID,
    },
};
fn test_infinite_usdc_mint() {
    let owner = Pubkey::new_unique();
    let usdc_mint = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    let ata = get_associated_token_address(&owner, &usdc_mint);
    let usdc_to_own = 1_000_000_000_000;
    let token_acc = TokenAccount {
        mint: usdc_mint,
        owner: owner,
        amount: usdc_to_own,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    let mut svm = LiteSVM::new();
    let mut token_acc_bytes = [0u8; TokenAccount::LEN];
    TokenAccount::pack(token_acc, &mut token_acc_bytes).unwrap();
    svm.set_account(
        ata,
        Account {
            lamports: 1_000_000_000,
            data: token_acc_bytes.to_vec(),
            owner: TOKEN_PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
    let raw_account = svm.get_account(&ata).unwrap();
    assert_eq!(
        TokenAccount::unpack(&raw_account.data).unwrap().amount,
        usdc_to_own
    )
}
```

---

### Build Initialize Instruction

Source: https://www.anchor-lang.com/docs/features/declare-program

Builds the instruction to initialize a counter account. It specifies the program, payer, system program, and the Initialize argument.

```rust
let initialize_ix = program
    .request()
    .accounts(accounts::Initialize {
        counter: counter.pubkey(),
        payer: program.payer(),
        system_program: system_program::ID,
    })
    .args(args::Initialize)
    .instructions()?
    .remove(0);
```

---

### InitSpace for Unnamed and Unit Structs

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Provides examples of deriving `InitSpace` for unnamed and unit structs, resolving previous compatibility issues. This enables correct space calculation for these struct types.

```Rust
#[derive(InitSpace)]
pub struct Unnamed(pub u64, #[max_len(64)] pub Vec<u8>);
#[derive(InitSpace)]
pub struct Unit;
```

---

### Rust - Initialize Zero Copy Account (init constraint)

Source: https://www.anchor-lang.com/docs/features/zero-copy

Illustrates initializing a zero-copy account using the `init` constraint with `AccountLoader`. This method is limited to allocating 10240 bytes due to CPI limitations and uses `load_init` to set the discriminator and get mutable data.

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(


init,
        // 10240 bytes is max space to allocate with init constraint
        space = 8 + 10232,
        payer = payer,
    )]
    pub data_account: AccountLoader<'info, Data>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {


    let account = &mut ctx.accounts.data_account.load_init()?;
    account.data = [1; 10232];
    Ok(())
}
```

---

### Anchor Lang: Group Member Pointer Extension Attribute

Source: https://www.anchor-lang.com/docs/references/account-constraints

Defines attributes for creating or validating the group member pointer extension on a mint account. Specifies the authority and member address for the pointer.

```rust
#[account(
    extensions::group_member_pointer::authority = <target_account>,
    extensions::group_member_pointer::member_address = <target_account>
)]
```

---

### Anchor Box<Account<'info, T>> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Demonstrates the use of `Box<Account<'info, T>>` to save stack space. This pattern is useful when dealing with potentially large account structures or when optimizing memory usage within the program.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub account:Box<Account<'info, AccountType>>,
}
```

---

### Anchor AccountLoader<'info, T> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Illustrates the `AccountLoader<'info, T>` type, which facilitates on-demand zero-copy deserialization. This is efficient for accounts that are frequently accessed but may not need to be fully deserialized every time.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub account:AccountLoader<'info, ZeroCopyAccountType>,
}
#[account(zero_copy)]
pub struct ZeroCopyAccountType {
    data: u64,
}
```

---

### Anchor Lang: Version Management Tool (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `avm`, a command-line tool for managing different versions of the Anchor CLI. This facilitates switching between and managing multiple Anchor CLI installations.

```CLI
cli: Add avm, a tool for managing anchor-cli versions (#1385).
```

---

### Anchor Account<'info, T> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Demonstrates the use of Anchor's `Account<'info, T>` type, which acts as an account container that checks ownership upon deserialization. This is a fundamental type for handling accounts in Anchor programs.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub account: Account<'info, CustomAccountType>,
}
#[account]
pub struct CustomAccountType {
    data: u64,
}
```

---

### Anchor Interface<'info, T> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Shows how to use the `Interface<'info, T>` type to validate that an account belongs to a specific set of programs, such as the Token program or Token2022 program. This is crucial for cross-program interactions.

```rust
// Token program or Token2022 program
use anchor_spl::token_interface::TokenInterface;
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub program: Interface<'info, TokenInterface>,
}
```

---

### Anchor Program Entrypoint with Initialize Instruction

Source: https://www.anchor-lang.com/docs/basics/program-structure

The main program entrypoint for a Solana program using Anchor, including the initialize instruction and its associated Accounts struct.

```rust
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

---

### Implement Transfer Hook Interface with `#[interface]`

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Rust code demonstrating the use of the `#[interface]` attribute to specify custom instruction discriminators for transfer hooks. This example shows how to implement the `initialize` and `execute` functions for the `spl_transfer_hook_interface`.

```rust
mod my_hook_program {
    #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize(ctx: Context<Initialize>, metas: Vec<AnchorExtraAccountMeta>) -> Result<()> {
        /* ... */
    }
    #[interface(spl_transfer_hook_interface::execute)]
    pub fn execute(ctx: Context<Execute>, amount: u64) -> Result<()> {
        /* ... */
    }
}
```

---

### Anchor CLI: Deploy with Solana CLI Arguments

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Enables passing arguments directly to `solana program deploy` via the `anchor deploy` command. This allows for more granular control over the deployment process.

```bash
anchor deploy -- --final
```

---

### Markdown: Mollusk Compute Unit Benchmark Report

Source: https://www.anchor-lang.com/docs/testing/mollusk

This is an example of a markdown report generated by the Mollusk Compute Unit Bencher. It displays the benchmark name, the compute units (CUs) consumed, and the delta (change) compared to a previous benchmark run, aiding in tracking performance regressions.

```markdown
| Name   | CUs   | Delta  |
| ------ | ----- | ------ |
| bench0 | 450   | --     |
| bench1 | 579   | -129   |
| bench2 | 1,204 | +754   |
| bench3 | 2,811 | +2,361 |
```

---

### Upgrade Anchor Version Manager (avm)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Command to install or update the Anchor Version Manager (avm) to a specific version, ensuring compatibility with Anchor releases. This is a prerequisite for managing different Anchor versions.

```bash
cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.0 avm --locked
```

---

### Anchor New Program

Source: https://www.anchor-lang.com/docs/references/cli

Creates a new Solana program within the workspace's `programs/` directory, initializing it with a standard boilerplate structure for rapid development.

```bash
anchor new <program-name>
```

---

### Anchor Caller Program: CPI for Initialize and Increment

Source: https://www.anchor-lang.com/docs/features/declare-program

This Rust code demonstrates how a caller program can perform Cross-Program Invocations (CPI) to the 'example' program. It uses the `declare_program!()` macro to generate a CPI module and then invokes the 'initialize' and 'increment' instructions of the callee program.

```rust
use anchor_lang::prelude::*;
declare_id!("GENmb1D59wqCKRwujq4PJ8461EccQ5srLHrXyXp4HMTH");


declare_program!(example);
use example::{
    accounts::Counter,
    cpi::{
        self,
        accounts::{Increment, Initialize},
    },
    program::Example,
};
#[program]
pub mod example_cpi {
    use super::*;
    pub fn initialize_cpi(ctx: Context<InitializeCpi>) -> Result<()> {
        // Create CPI context for initialize
        let cpi_ctx = CpiContext::new(
            ctx.accounts.example_program.to_account_info(),
            Initialize {
                payer: ctx.accounts.payer.to_account_info(),
                counter: ctx.accounts.counter.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );
        // Invoke the initialize instruction
        cpi::initialize(cpi_ctx)?;
        Ok(())
    }
    pub fn increment_cpi(ctx: Context<IncrementCpi>) -> Result<()> {
        // Create CPI context for increment
        let cpi_ctx = CpiContext::new(
            ctx.accounts.example_program.to_account_info(),
            Increment {
                counter: ctx.accounts.counter.to_account_info(),
            },
        );
        // Invoke the increment instruction
        cpi::increment(cpi_ctx)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeCpi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub counter: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub example_program: Program<'info, Example>,
}
#[derive(Accounts)]
pub struct IncrementCpi<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub example_program: Program<'info, Example>,
}
```

---

### Rust: Create Mint Account with PDA Authority

Source: https://www.anchor-lang.com/docs/tokens/basics/mint-tokens

Demonstrates creating a mint account where the mint authority is a Program Derived Address (PDA). This setup allows the program to control token minting. It requires Anchor and SPL Token Interface dependencies.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface};

declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");

#[program]
pub mod token_example {
    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Created Mint Account: {:?}", ctx.accounts.mint.key());
        Ok(())
    }

    // ... mint_tokens function ...
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint,
        mint::freeze_authority = mint,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

```

---

### Anchor IDL Init

Source: https://www.anchor-lang.com/docs/references/cli

Creates an IDL account on the blockchain, writing a specified IDL JSON file into a program-owned account. It defaults to an account size double the IDL size for future upgrades.

```bash
anchor idl init -f <target/idl/program.json> <program-id>
```

---

### Anchor InterfaceAccount<'info, T> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Illustrates `InterfaceAccount<'info, T>`, an account container that checks ownership upon deserialization, specifically for interface accounts like Mints or TokenAccounts within the Token or Token2022 programs.

```rust
// Token program or Token2022 program Mint/TokenAccount
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub mint:InterfaceAccount<'info, Mint>,
    pub token:InterfaceAccount<'info, TokenAccount>,
    pub program: Interface<'info, TokenInterface>,
}
```

---

### Anchor.toml: Test Upgradeable Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Enables deploying the test program using `--upgradeable-program`. This allows testing instructions that should only be executable by the program's upgrade authority, which defaults to the `provider.wallet`. If false or unspecified, the program is deployed with `--bpf-program`, disabling upgrades.

```toml
[test]
upgradeable = true

```

---

### Use InitSpace Macro for Account Space Calculation

Source: https://www.anchor-lang.com/docs/references/space

Shows how to use the `#[derive(InitSpace)]` macro to automatically calculate and add an `INIT_SPACE` constant to a struct. This simplifies space calculation, especially for nested types like `Vec` with `#[max_len]`. The example demonstrates its application with different data types and nested vectors, and how to use the generated `INIT_SPACE` in the `#[account]` macro.

```rust
#[account]


#[derive(InitSpace)]
pub struct ExampleAccount {
    pub data: u64,
    #[max_len(50)]
    pub string_one: String,
    #[max_len(10, 5)]
    pub nested: Vec<Vec<u8>>,
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,


    #[account(init, payer = payer, space = 8 + ExampleAccount::INIT_SPACE)]
    pub data: Account<'info, ExampleAccount>,
}
```

---

### Embed Workspace Programs into Local Validator Genesis

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows embedding workspace programs directly into the local validator's genesis configuration when testing. This simplifies the setup for local development and testing.

```bash
anchor test --embed-programs

```

---

### Rust: SPL initialize_account3 and initialize_mint2 Wrappers

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides wrapper functions for `initialize_account3` and `initialize_mint2` in the Rust SPL module.

```rust
use anchor_spl::token_interface::{initialize_account3, initialize_mint2};

// Call these functions to initialize accounts and mints.
```

---

### Anchor Program Derived Address (PDA)

Source: https://www.anchor-lang.com/docs/basics

Demonstrates how to utilize Program Derived Addresses (PDAs) within Anchor programs to establish deterministic account addresses.

```Rust
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

pub fn get_pda<'info>(program_id: &Pubkey) -> (Pubkey, u8) {
    let seeds = &["my_seed".as_bytes()];
    Pubkey::find_program_address(seeds, program_id)
}
```

---

### CLI Fixes in Anchor Lang

Source: https://www.anchor-lang.com/docs/updates/changelog

This snippet outlines fixes and improvements made to the Command Line Interface (CLI) tools in the Anchor Lang project. It covers issues related to installation, argument handling, path management, and template generation.

```Rust
cli: Fix installation with `--locked` argument using Rust v1.80 due to `time` crate issue (#3143).
cli: Fix template code shouldn't escape (#3210).
cli: Use OS-agnostic paths (#3307).
avm: Use `rustc 1.79.0` when installing versions older than v0.31 (#3315).
cli: Fix priority fee calculation causing panic on localnet (#3318).
cli: Fix `shell` command failing due to outdated program initialization (#3351).
cli: Remove passing the rent sysvar account to IDL instructions (#3372).
cli: Ignore non semver solana/agave releases to avoid panic (#3432).
cli: Fix custom `provider.cluster` (#3428).
cli: Fix altering user-provided lib names (#3467).
cli: Use camelCase for program name in `anchor.workspace` templates (#3581).
```

---

### Example Transaction Data with CPI Event

Source: https://www.anchor-lang.com/docs/features/events

This JSON represents sample transaction data, illustrating how event data emitted via `emit_cpi!()` appears. The encoded event data is found within the `data` field of an inner instruction (CPI) in the `innerInstructions` array.

```json
{
  "blockTime": 1735854530,
  "meta": {
    "computeUnitsConsumed": 13018,
    "err": null,
    "fee": 5000,
    "innerInstructions": [
      {
        "index": 0,
        "instructions": [
          {
            "accounts": [1],
            "data": "6AJcBqZP8afBKheoif1oA6UAiLAcqYr2RaR33pFnEY1taQp",
            "programIdIndex": 2,
            "stackHeight": 2
          }
        ]
      }
    ],
    "loadedAddresses": {
      "readonly": [],
      "writable": []
    },
    "logMessages": [
      "Program 2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1 invoke [1]",
      "Program log: Instruction: EmitEvent",
      "Program 2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1 invoke [2]",
      "Program 2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1 consumed 5000 of 192103 compute units",
      "Program 2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1 success",
      "Program 2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1 consumed 13018 of 200000 compute units",
      "Program 2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1 success"
    ],
    "postBalances": [499999999999995000, 0, 1141440],
    "postTokenBalances": [],
    "preBalances": [500000000000000000, 0, 1141440],
    "preTokenBalances": [],
    "rewards": [],
    "status": {
      "Ok": null
    }
  },
  "slot": 3,
  "transaction": {
    "message": {
      "header": {
        "numReadonlySignedAccounts": 0,
        "numReadonlyUnsignedAccounts": 2,
        "numRequiredSignatures": 1
      },
      "accountKeys": [
        "4kh6HxYZiAebF8HWLsUWod2EaQQ6iWHpHYCz8UcmFbM1",
        "2brZf9PQqEvv17xtbj5WNhZJULgVZuLZT6FgH1Cqpro2",
        "2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1"
      ],
      "recentBlockhash": "2QtnU35RXTo7uuQEVARYJgWYRYtbqUxWQkK8WywUnVdY",
      "instructions": [
        {
          "accounts": [1, 2],
          "data": "3XZZ984toC4WXCLkxBsLimpEGgH75TKXRJnk",
          "programIdIndex": 2,
          "stackHeight": null
        }
      ],
      "indexToProgramIds": {}
    },
    "signatures": [
      "3gFbKahSSbitRSos4MH3cqeVv2FiTNaLCuWaLPo6R98FEbHnTshoYxopGcx74nFLqt1pbZK9i8dnr4NFXayrMndZ"
    ]
  }
}
```

---

### Rust CPI Example: SOL Transfer

Source: https://www.anchor-lang.com/docs/basics/cpi

This Rust code snippet demonstrates how to implement a Cross Program Invocation (CPI) in an Anchor program to transfer SOL. It defines a `sol_transfer` instruction that internally calls the System Program's `transfer` instruction. It requires the `anchor_lang` and `anchor_lang::system_program` crates.

```rust
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("9AvUNHjxscdkiKQ8tUn12QCMXtcnbR9BVGq3ULNzFMRi");
#[program]
pub mod cpi {
    use super::*;


    pub fnsol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();
        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );


transfer(cpi_context, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
```

---

### Importing Generated Client Modules (Rust)

Source: https://www.anchor-lang.com/docs/features/declare-program

This code demonstrates how to import the generated modules from the `declare_program!()` macro. It brings into scope the program's account types (`Counter`), the account structs used for instruction building (`client::accounts`), and the argument structs for instructions (`client::args`). This allows for type-safe and convenient construction of Anchor program instructions.

```rust
use example::{
  accounts::Counter,  // Program Account types
  client::accounts,   // Accounts for program instructions
  client::args,       // Arguments for program instructions
};
```

---

### Calculate Account Space in Rust

Source: https://www.anchor-lang.com/docs/references/space

Demonstrates how to manually calculate the required space for an Anchor account by defining a `MAX_SIZE` constant based on the account's fields and their types. It includes an example of an enum and how to set up the `#[account]` macro with the calculated space, remembering to add 8 bytes for Anchor's internal discriminator.

```rust
#[account]
pub struct MyData {
    pub val: u16,
    pub state: GameState,
    pub players: Vec<Pubkey> // we want to support up to 10 players
}
impl MyData {

    pub const MAX_SIZE: usize = 2 + (1 + 32) + (4 + 10 * 32);
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}
#[derive(Accounts)]
pub struct InitializeMyData<'info> {
    // Note that we have to add 8 to the space for the internal anchor

    #[account(init, payer = signer, space = 8 + MyData::MAX_SIZE)]
    pub acc: Account<'info, MyData>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}
```

---

### Anchor CLI: Recommended Solana Args and Max Retries for Deploy

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor deploy` command now includes recommended Solana arguments by default and introduces a new `--max-retries` option for enhanced deployment control.

```bash
anchor deploy --max-retries 5
```

---

### Type Aliases in Anchor IDL (Rust)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Demonstrates how Anchor IDL now supports type aliases, showing a Rust example defining a type alias `U8Array` and its usage in a program instruction. This feature currently has limitations with `idl-build`.

```Rust
pub type U8Array = [u8; 8];
#[program]
pub mod my_program {
    use super::*;
    pub fn type_alias(ctx: Context<TypeAlias>, u8_array: U8Array) -> Result<()> {
        msg!(/* {:?} */ u8_array);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct TypeAlias {}

```

---

### Rust: Address Constraint with Non-Const Expressions

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Fixes a regression in v0.30 where non-const expressions with the 'address' constraint failed to build. This example shows the corrected usage of the 'address' constraint in a Rust struct.

```Rust
#[derive(Accounts)]
pub struct MyIx {
    #[account(address = my_account.authority())]
    pub authority: Signer<'info>,
    pub my_account: Account<'info, MyAccount>,
}
```

---

### Anchor: Dependency Free Composability

Source: https://www.anchor-lang.com/docs/features

Demonstrates how to use Anchor's `declare_program` macro to interact with other Solana programs without requiring additional dependencies. This simplifies cross-program invocations.

```Rust
use anchor_lang::prelude::*;

#[program_id]
mod my_program {
    // ...
}

#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    // Declare another program to interact with
    /// CHECK: This is a program account, and we don't need to do anything else with it.
    pub other_program: AccountInfo<'info>,
}

pub fn my_instruction(ctx: Context<MyAccounts>) -> Result<()> {
    // Interact with the other program
    // ...
    Ok(())
}
```

---

### Anchor TS: Account Resolution Update

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Demonstrates the updated method for specifying accounts in Anchor transactions. The `accounts` method is now type-safe based on IDL resolution fields. The example shows how to remove resolvable accounts to fix potential errors after the update.

```javascript
await program.methods.init().rpc();
```

---

### Anchor Program Initialization Logic (Rust)

Source: https://www.anchor-lang.com/docs/basics/program-structure

A basic Anchor program demonstrating an initialize function. It sets data on a new account and logs a message. Requires the Initialize account struct and a NewAccount struct.

```rust
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct NewAccount {
    data: u64,
}
```

---

### Anchor UncheckedAccount<'info> Example

Source: https://www.anchor-lang.com/docs/references/account-types

Illustrates the `UncheckedAccount<'info>` type, an explicit wrapper for `AccountInfo` types that emphasizes no checks are performed. This is used when you need to access an account's data without Anchor performing any validation.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    // CHECK: No checks are performed


    pub account:UncheckedAccount<'info>,
}
```

---

### Send Transaction with Instructions

Source: https://www.anchor-lang.com/docs/features/declare-program

Constructs a transaction, adds the initialize and increment instructions, signs it with the counter account, and sends it. Returns the transaction signature.

```rust
let signature = program
    .request()
    .instruction(initialize_ix)
    .instruction(increment_ix)
    .signer(&counter)
    .send()
    .await?;
```

---

### Anchor Program IDL File

Source: https://www.anchor-lang.com/docs/basics

Details the Interface Description Language (IDL) file in Anchor, its purpose, advantages, and how it simplifies interactions between programs and clients.

```JSON
{
  "version": "0.1.0",
  "name": "my_program",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "dataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "myAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "value",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
```

---

### Anchor CLI Usage

Source: https://www.anchor-lang.com/docs/references/cli

Provides the general usage syntax for the Anchor CLI and lists available subcommands. To see detailed help for any subcommand, users can run `anchor -h` followed by the subcommand name.

```bash
anchor <SUBCOMMAND>
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
SUBCOMMANDS:
    account    Fetch and deserialize an account using the IDL provided
    build      Builds the workspace
    cluster    Cluster commands
    deploy     Deploys each program in the workspace
    expand     Expands the macros of a program or the workspace
    help       Prints this message or the help of the given subcommand(s)
    idl        Commands for interacting with interface definitions
    init       Initializes a workspace
    keys       Program keypair commands
    migrate    Runs the deploy migration script
    new        Creates a new program
    shell      Starts a node shell with an Anchor client setup according to the local config
    test       Runs integration tests against a localnetwork
    upgrade    Upgrades a single program. The configured wallet must be the upgrade authority
    verify     Verifies the on-chain bytecode matches the locally compiled artifact. Run this
               command inside a program subdirectory, i.e., in the dir containing the program's
               Cargo.toml
```

---

### Rust Account Structure with Seed Math Expression

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Rust struct definition for an Anchor account that uses a seed expression involving arithmetic operations on account data. This example highlights a scenario that previously caused IDL generation errors but is now handled, though automatic client resolution may not be supported.

```rust
#[derive(Accounts)]
pub struct SeedMathExpr<'info> {
    #[account(seeds = [&(my_account.data + 1).to_le_bytes()], bump)]
    pub math_expr_account: UncheckedAccount<'info>,
    pub my_account: Account<'info, MyAccount>,
}
#[account]
pub struct MyAccount {
    data: u64,
}
```

---

### Build Anchor Workspace

Source: https://www.anchor-lang.com/docs/references/cli

Builds the programs within the Anchor workspace, targeting Solana's BPF runtime and generating IDL files in the `target/idl` directory. The `--verifiable` flag builds within a Docker image for deterministic output.

```bash
anchor build
```

```bash
anchor build --verifiable
```

```bash
anchor build -- --features my-feature
```

---

### Rust: Send Transaction with Initialize and Increment Instructions

Source: https://www.anchor-lang.com/docs/clients/rust

This Rust code snippet demonstrates how to construct and send a transaction containing two instructions: 'initialize' and 'increment'. It utilizes the Anchor client library to build the requests, specify accounts and arguments, and send the transaction. Finally, it fetches and prints the updated counter account data.

```Rust
println!("\nSend transaction with initialize and increment instructions");
    let initialize_ix = program
        .request()
        .accounts(accounts::Initialize {
            counter: counter.pubkey(),
            payer: program.payer(),
            system_program: system_program::ID,
        })
        .args(args::Initialize)
        .instructions()?;
    let increment_ix = program
        .request()
        .accounts(accounts::Increment {
            counter: counter.pubkey(),
        })
        .args(args::Increment)
        .instructions()?;
    let signature = program
        .request()
        .instruction(initialize_ix.remove(0))
        .instruction(increment_ix.remove(0))
        .signer(&counter)
        .send()
        .await?;
    println!("   Transaction confirmed: {}", signature);
    println!("\nFetch counter account data");
    let counter_account: Counter = program.account::<Counter>(counter.pubkey()).await?;
    println!("   Counter value: {}", counter_account.count);
    Ok(())
}
```

---

### Mint Tokens via CPI (Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/mint-tokens

Demonstrates how to mint tokens using a cross-program invocation (CPI) to the Token Program in Rust with Anchor. This involves setting up the necessary accounts and instruction data for the `mint_to` instruction.

```Rust
use anchor_lang::prelude::*;
use anchor_spl::token::{MintTo, mint_to};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>, // The mint account
    #[account(mut)]
    pub to: Account<'info, TokenAccount>, // The token account to mint to
    #[account(constraint = mint.mint_authority == Some(authority.key()))]
    pub authority: Signer<'info>, // The mint authority
    pub token_program: Program<'info, Token>, // The Token Program
}

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();

    // Perform the CPI to mint tokens
    mint_to(cpi_accounts, amount, ctx.accounts.token_program.to_account_info())?;

    Ok(())
}
```

---

### Anchor spl: Init and close open orders instructions

Source: https://www.anchor-lang.com/docs/updates/changelog

Includes instructions for initializing and closing open orders within the SPL module.

```spl
# spl: Add init and close open orders instructions (#245).
```

---

### Invoke Instruction with RPC

Source: https://www.anchor-lang.com/docs/clients/typescript

Demonstrates invoking a Solana program instruction using Anchor's client library. The `.rpc()` method builds, signs (using the provider's wallet), and sends the transaction, returning a transaction signature. It requires instruction data, account configurations, and optional signers.

```TypeScript
// Generate keypair for the new account
const newAccountKp = new Keypair();
const data = new BN(42);
const transactionSignature = await program.methods
  .initialize(data)
  .accounts({
    newAccount: newAccountKp.publicKey,
    signer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([newAccountKp])
  .rpc();
```

---

### Anchor Program Initialization Instruction

Source: https://www.anchor-lang.com/docs/basics/idl

This Rust code defines an Anchor program with an 'initialize' instruction. It takes a u64 data parameter and initializes a new account, setting its data field and logging the change. It requires a 'signer' and the 'system_program'.

```rust
use anchor_lang::prelude::*;
declare_id!("BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd");
#[program]
mod hello_anchor {
    use super::*;



    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct NewAccount {
    data: u64,
}
```

---

### Anchor Custom Error Definition with #[error_code]

Source: https://www.anchor-lang.com/docs/features/errors

Demonstrates defining custom program-specific errors using the `#[error_code]` attribute in Anchor. This attribute automatically assigns error codes starting from 6000 and allows custom messages via `#[msg]`.

```Rust
#[error_code]
pub enum MyError {
    #[msg("My custom error message")]
    MyCustomError,
    #[msg("My second custom error message")]
    MySecondCustomError,
}
```

---

### Rust Account Structure with Address Constraint

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Rust struct definition for an Anchor account using an `address` constraint based on another account's field. This example shows a change where such constraints no longer cause IDL compile errors, but recommends using `has_one` for automatic resolution.

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(address = my_account.authority)]
    pub authority: UncheckedAccount<'info>,
    pub my_account: Account<'info, MyAccount>,
}
#[account]
pub struct MyAccount {
    authority: u64,
}
```

---

### Create Mint Account using Keypair (Anchor Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-mint

This snippet demonstrates creating a new mint account using a generated Keypair in an Anchor program. It utilizes specific account constraints to initialize the mint account, setting decimals, authority, and freeze authority. The 'payer' constraint specifies the account responsible for rent.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");
#[program]
pub mod token_example {
    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Created Mint Account: {:?}", ctx.accounts.mint.key());
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
```

---

### IDL Build Command (Shell)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Illustrates the command to specifically generate the IDL file using the `anchor idl build` command, separate from the main build process.

```Shell
anchor idl build
```

---

### Anchor Init Behavior Change

Source: https://www.anchor-lang.com/docs/updates/changelog

The `#[account(init)]` attribute now creates the account within the same instruction for consistency with PDA initialization. To retain the old behavior, use `#[account(zero)]`.

```rust
lang: `#[account(init)]` now creates the account inside the same instruction to be consistent with initializing PDAs. To maintain the old behavior of `init`, replace it with `#[account(zero)]` (#641).
```

---

### Rust to JS Type Conversion: Enums

Source: https://www.anchor-lang.com/docs/references/type-conversion

This example details the conversion of Rust enums to TypeScript representations. Different enum variants (unit, named, tuple) are mapped to specific JavaScript object structures, allowing for equivalent data representation.

```Rust
// Rust
enum MyEnum {
    One,
    Two { val: u32 },
    Three(u8, i16),
}
```

```TypeScript
// TypeScript Representations
// Unit variant
const one = { one: {} };
// Named variant
const two = {
  two: { val: 99 },
};
// Tuple variant
const three = {
  three: [12, -34],
};
```

---

### Use npm's Default License for New Projects (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Configures the `anchor init` command to use npm's default license for newly created projects.

```bash
cli: Use npm's configured default license for new projects made with `anchor init` (#2929).
```

---

### Anchor Custom Instruction Handler

Source: https://www.anchor-lang.com/docs/features/errors

Example of an instruction handler in Anchor that returns a custom Result. Anchor programs use a custom `Result<T>` type which wraps Rust's standard Result, allowing for custom error handling with Anchor's `Error` type.

```Rust
pub fn custom_instruction(ctx: Context<CustomInstruction>) ->Result<()> {
    // --snip--
    Ok(())
}
```

---

### Create Token Account with Keypair Address (Anchor Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Initializes a standard token account where the account address is derived from a keypair's public key. This uses Anchor's account constraints to interact with the System Program for creation.

```rust
#[account(
    init,
    payer = <payer>,
    token::mint = <mint>,
    token::authority = <authority>,
    token::token_program = <token_program>
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,

```

---

### Anchor Cross Program Invocation (CPI)

Source: https://www.anchor-lang.com/docs/basics

Illustrates how to implement Cross Program Invocations (CPIs) in Anchor programs, enabling composability between different Solana programs.

```Rust
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program};

pub fn call_other_program<'info>(ctx: Context<'_, '_, '_, '_, OtherProgramContext<'info>>) -> Result {
    let cpi_accounts = solana_program::system_program::Transfer {
        from: ctx.accounts.sender.to_account_info(),
        to: ctx.accounts.receiver.to_account_info(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    solana_program::system_program::transfer(cpi_ctx, 1000)
}

#[derive(Accounts)]
pub struct OtherProgramContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
```

---

### Support for Solidity Programs in Anchor CLI

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for Solidity programs in the Anchor CLI. New commands `anchor init --solidity` and `anchor new --solidity` create Solidity-based projects, and `anchor build` and `anchor test` work accordingly.

```bash
# Initialize a new project with Solidity support
anchor init --solidity

# Create a new project with Solidity support
anchor new --solidity

# Build a Solidity program
anchor build

# Test a Solidity program
anchor test
```

---

### Rust: Using pubkey! Macro

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Shows the convenient usage of the `pubkey!` macro from `anchor_lang::prelude` for declaring public keys, comparing it to the `Pubkey::from_str` method. It also notes that `solana-program` dependency can be removed if `pubkey!` is the only reason for its inclusion.

```rust
use anchor_lang::prelude::pubkey;

fn main() {
    let key = pubkey!("11111111111111111111111111111111");
    println!("{}", key);
}
```

```rust
use std::str::FromStr;
use solana_program::pubkey::Pubkey;

fn main() {
    let key = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    println!("{}", key);
}
```

---

### Rust Client: Mock Client for Testing

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Introduces the ability to use a mock RPC client for testing purposes by enabling the 'mock' feature. This facilitates isolated testing of client logic without relying on a live network.

```Rust
anchor-client = { version = "0.31.0", features = ["mock"] }
```

---

### Initialize Anchor Workspace with Package Manager

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

This command demonstrates how to specify a package manager when creating a new Anchor workspace. The '--package-manager' flag allows users to choose between 'npm', 'yarn', or 'pnpm' during the initialization process.

```shell
anchor init <NAME> --package-manager npm
```

---

### Anchor CLI: Build Verifiable Program

Source: https://www.anchor-lang.com/docs/references/verifiable-builds

This command builds your Solana program using Anchor CLI, ensuring a verifiable build by leveraging Docker. It helps in creating consistent executables across different development environments.

```bash
anchor build --verifiable
```

---

### Anchor CLI: Verifiable Deployments

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Supports deploying a verified build instead of the default build using the `--verifiable` flag, similar to verifiable builds. This enhances the integrity of deployments.

```bash
anchor deploy --verifiable
```

---

### Anchor Mint Account Initialization

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds constraints for initializing mint accounts as PDAs, including `#[account(init, seeds = [...], mint::decimals = <expr>, mint::authority = <expr>)]`. This simplifies PDA creation for mint accounts.

```rust
lang: Add constraints for initializing mint accounts as pdas, `#[account(init, seeds = [...], mint::decimals = <expr>, mint::authority = <expr>)]` (#562).
```

---

### Anchor CLI: Test Upgradeable Programs

Source: https://www.anchor-lang.com/docs/updates/changelog

New settings `test.upgradeable` and `test.genesis.upgradeable` in `Anchor.toml` allow for testing upgradeable programs.

```toml
[test]
upgradeable = true
genesis = {"upgradeable" = true}

```

---

### InterfaceAccount Type for Token Accounts (Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Demonstrates the use of `InterfaceAccount` as a wrapper for token accounts, enabling compatibility with both the Token Program and Token Extension Program.

```rust
pub token_account: InterfaceAccount<'info, TokenAccount>,

```

---

### Ergonomic Lamport Transfer Methods

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

New, more performant methods for subtracting and adding lamports to accounts compared to the older `try_borrow_mut_lamports` approach.

```rust
ctx.accounts.from.sub_lamports(amount)?;
ctx.accounts.to.add_lamports(amount)?;
```

---

### Create Program Instance (No Wallet)

Source: https://www.anchor-lang.com/docs/clients/typescript

Creates an Anchor Program instance using only the IDL and a Connection to a Solana cluster. This method is suitable for scenarios where no default wallet is needed, such as fetching accounts or building instructions without a connected wallet. It requires the program's IDL and a Solana Connection object.

```TypeScript
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import type { HelloAnchor } from "./idlType";
import idl from "./idl.json";

const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

export const program = new Program(idl as HelloAnchor, {
  connection,
});
```

---

### Rust: Program Accounts with Full Paths

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Resolves an issue in v0.30 where 'Program' accounts with full paths did not work. This snippet demonstrates the correct way to define 'Program' accounts using their full path.

```Rust
#[derive(Accounts)]
pub struct FullPath<'info> {
    pub external_program: Program<'info, external::program::External>,
}
```

---

### Accept Program Lib Names in `anchor deploy`

Source: https://www.anchor-lang.com/docs/updates/changelog

The `--program-name` argument for the `anchor deploy` command now accepts program library names. This provides more flexibility in specifying which program to deploy when multiple programs are defined in a project.

```bash
# Deploy a program using its library name
anchor deploy --program-name my_program_lib
```

---

### Enable init-if-needed Feature (Cargo.toml)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Configures the Anchor Lang dependency in Cargo.toml to enable the `init-if-needed` feature, allowing the use of the `init_if_needed` account constraint.

```toml
[dependencies]
anchor-lang = { version = "0.31.1", features = ["init-if-needed"] }

```

---

### Fetch and Deserialize Account (with IDL)

Source: https://www.anchor-lang.com/docs/references/cli

Fetches an account by its public key and deserializes its data into JSON format using the specified account type name and program name. If run within a workspace, it uses the workspace's IDL; otherwise, an IDL path must be provided.

```bash
anchor account <program-name>.<AccountTypeName> <account_pubkey>
```

```bash
anchor account <program-name>.<AccountTypeName> <account_pubkey> --idl <path/to/idl.json>
```

---

### SPL Token 2022 Instructions

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Lists new instructions added for the Token 2022 standard, including checked burn, mint, approve, and withdrawal operations.

```Rust
// New instructions:
// burn_checked
// mint_to_checked
// approve_checked
// withdraw_withheld_tokens_from_accounts
```

---

### Add --program-keypair to anchor deploy

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor deploy` command now supports a `--program-keypair` option, enabling users to specify a keypair for the program during deployment.

```bash
`anchor deploy` (#1786).
```

---

### Initialize Large Zero Copy Account (Anchor)

Source: https://www.anchor-lang.com/docs/features/zero-copy

Illustrates initializing a zero-copy account with data exceeding 10,240 bytes. This requires a two-step process: first creating the account via the System Program, then initializing its data within your program. The `#[account(zero)]` constraint is used for this purpose.

```rust
use anchor_lang::prelude::*

declare_id!("CZgWhy3FYPFgKE5v9atSGaiQzbSB7cM38ofwX1XxeCFH");

#[program]
pub mod zero_copy_two {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.data_account.load_init()?;
        account.data = [1; 10_485_752];
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    pub data_account: AccountLoader<'info, Data>,
}

#[account(zero_copy)]
pub struct Data {
    // 10240 bytes - 8 bytes account discriminator
    pub data: [u8; 10_485_752],
}
```

---

### Anchor Initialize Accounts Structure (Rust)

Source: https://www.anchor-lang.com/docs/basics/program-structure

Defines the accounts required for an Anchor program's initialize instruction. It includes constraints for account initialization, payer, space, mutability, and system program.

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

---

### Using Exported mpl-token-metadata (Rust)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Demonstrates the correct way to import and use the `mpl-token-metadata` crate when it's exported via the `metadata` feature of `anchor-spl`.

```Rust
use anchor_spl::metadata::mpl_token_metadata;
```

---

### Anchor CLI: Verify Program Deployment

Source: https://www.anchor-lang.com/docs/references/verifiable-builds

This command verifies a locally built program against a program deployed on the mainnet. It checks the program ID and, if an IDL exists, also verifies the on-chain IDL against the local definition.

```bash
anchor verify -p <lib-name> <program-id>
```

---

### Anchor CLI: Program Template Multi-File

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now supports creating program templates with multiple files for instructions, state, and other components, promoting better project organization.

```bash
# Initialize a new project with a multi-file template
anchor init --template multi-file my-new-program

```

---

### Account Initialization with Discriminator - Rust

Source: https://www.anchor-lang.com/docs/basics/program-structure

This Rust code snippet demonstrates how to initialize a new account in an Anchor program. It shows the #[account] attribute with initialization parameters, including the space allocation for the 8-byte discriminator and the account data.

```rust
#[account(init, payer = signer, space =8 + 8)]
pub new_account: Account<'info, NewAccount>,

```

---

### Build IDL with Cargo Features

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Demonstrates how to pass additional arguments to the underlying cargo build process when building the IDL using the Anchor CLI. This allows enabling specific cargo features.

```bash
anchor build -- --features my-feature
```

---

### Import Anchor Program Modules

Source: https://www.anchor-lang.com/docs/features/declare-program

After declaring a program, its generated modules for account types, cross-program invocation helpers, and program types can be brought into scope using the `use` keyword. This allows for easier access to these components.

```rust
use example::{
    accounts::Counter, // Account types
    cpi::{             // Cross program invocation helpers
        self,
        accounts::{Increment, Initialize},
    },
    program::Example,  // Program type
};

```

---

### Anchor Upgrade Program

Source: https://www.anchor-lang.com/docs/references/cli

Upgrades the on-chain program code using Solana's upgradeable BPF loader. It requires the path to the compiled program artifact (`.so` file) and the program's ID.

```bash
anchor upgrade <target/deploy/program.so> --program-id <program-id>
```

---

### Rust - Initialize Zero Copy Account (zero constraint)

Source: https://www.anchor-lang.com/docs/features/zero-copy

Shows how to initialize a zero-copy account using the `zero` constraint for accounts larger than 10240 bytes (up to 10MB). This bypasses CPI limitations by verifying the account is uninitialized and requires pre-creation via System Program.

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {


    #[account(zero)]
    pub data_account: AccountLoader<'info, Data>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {


    let account = &mut ctx.accounts.data_account.load_init()?;
    account.data = [1; 10_485_752];
    Ok(())
}
```

---

### Anchor lang: Initialize PDAs with accounts constraints

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows initializing Program Derived Addresses (PDAs) using accounts constraints, providing a structured way to define PDA initialization.

```rust
/// lang: Initialize program derived addresses with accounts constraints (#386).
```

---

### Anchor CPI: Initialize Instruction

Source: https://www.anchor-lang.com/docs/features/declare-program

This Rust code demonstrates how to perform a cross-program invocation (CPI) to initialize an account in another Anchor program. It defines an account struct for the CPI context and uses the generated CPI helper functions.

```rust
#[derive(Accounts)]
pub struct InitializeCpi<'info> {
    // Counter type from accounts module
    #[account(mut)]


    pub counter: Account<'info, Counter>,
    // Example type from program module


    pub example_program: Program<'info, Example>,
}

pub fn initialize_cpi(ctx: Context<InitializeCpi>) -> Result<()> {
    // Create CPI context for initialize
    let cpi_ctx = CpiContext::new(
        ctx.accounts.example_program.to_account_info(),
        Initialize {
            payer: ctx.accounts.payer.to_account_info(),
            counter: ctx.accounts.counter.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
    );
    // Invoke the initialize instruction

    cpi::initialize(cpi_ctx)?;
    Ok(())
}

```

---

### Anchor Callee Program: Initialize and Increment Instructions

Source: https://www.anchor-lang.com/docs/features/declare-program

This Rust code defines a simple Anchor program with an 'initialize' and an 'increment' instruction. It includes the necessary account definitions and the Counter struct. This program generates an IDL file that can be used by other programs to invoke its instructions.

```rust
use anchor_lang::prelude::*;
declare_id!("8HupNBr7SBhBLcBsLhbtes3tCarBm6Bvpqp5AfVjHuj8");
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count += 1;
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
#[account]
pub struct Counter {
    pub count: u64,
}
```

---

### InitIfNeede Keyword for Instructions (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `#[account(init_if_needed)]` keyword, allowing the same instruction to be invoked even if the account has already been created. This simplifies idempotent instruction logic.

```rust
#[account(init_if_needed, payer = payer, space = 8)]
struct MyAccount {...}
```

---

### Mint Tokens via CPI (TypeScript)

Source: https://www.anchor-lang.com/docs/tokens/basics/mint-tokens

Illustrates how to mint tokens using a cross-program invocation (CPI) to the Token Program in TypeScript with Anchor. This involves preparing the accounts and instruction data for the `mintTo` instruction.

```TypeScript
import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { MintToParams } from "@solana/spl-token";

// Assuming 'program' is your Anchor program instance
// and 'provider' is your Anchor provider

async function mintTokens(mintAccount: anchor.web3.PublicKey, toAccount: anchor.web3.PublicKey, authority: anchor.web3.Keypair, amount: number): Promise<void> {
    try {
        const tx = await program.methods.mintTokens(new anchor.BN(amount)).accounts({
            mint: mintAccount,
            to: toAccount,
            authority: authority.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([authority]).rpc();

        console.log("Mint transaction signature", tx);
    } catch (error) {
        console.error("Error minting tokens:", error);
        if (error instanceof AnchorError) {
            console.error(`Anchor error: ${error.error.errorMessage}`);
        }
    }
}
```

---

### Create Associated Token Account

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Creates an associated token account for a user using Anchor framework. This function requires the signer, mint account, token program, associated token program, and system program.

```rust
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");
#[program]
pub mod token_example {
    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Created Mint Account: {:?}", ctx.accounts.mint.key());
        Ok(())
    }
    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!(
            "Created Token Account: {:?}",
            ctx.accounts.token_account.key()
        );
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint.key(),
        mint::freeze_authority = mint.key(),
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(

        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
```

---

### Anchor CLI: Set Priority Fees

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Allows setting priority fees for IDL commands in the Anchor CLI to improve transaction landing on mainnet-beta. If not specified, it defaults to the median fee of the last 150 confirmed slots.

```bash
anchor idl erase-authority --program-id <PROGRAM_ID> --priority-fee 9000
```

---

### Initialize and Update Zero Copy Account (Anchor)

Source: https://www.anchor-lang.com/docs/features/zero-copy

Shows how to initialize and update a zero-copy account in Anchor. The `initialize` function uses the `init` constraint for accounts up to 10240 bytes, while the `update` function demonstrates modifying existing account data. Both require the `AccountLoader` and a `Data` struct marked with `#[account(zero_copy)]`.

```rust
use anchor_lang::prelude::*

declare_id!("8B7XpDXjPWodpDUWDSzv4q9k73jB5WdNQXZxNBj1hqw1");

#[program]
pub mod zero_copy {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.data_account.load_init()?;
        account.data = [1; 10232];
        Ok(())
    }
    pub fn update(ctx: Context<Update>) -> Result<()> {
        let account = &mut ctx.accounts.data_account.load_mut()?;
        account.data = [2; 10232];
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        // 10240 bytes is max space to allocate with init constraint
        space = 8 + 10232,
        payer = payer,
    )]
    pub data_account: AccountLoader<'info, Data>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub data_account: AccountLoader<'info, Data>,
}

#[account(zero_copy)]
pub struct Data {
    // 10240 bytes - 8 bytes account discriminator
    pub data: [u8; 10232],
}
```

---

### Anchor Rust: Mint Tokens Instruction

Source: https://www.anchor-lang.com/docs/tokens/basics/mint-tokens

Defines the `mint_tokens` instruction within an Anchor program, which handles the cross-program invocation to mint tokens. It takes an amount and context containing necessary accounts like signer, mint, token account, and the token program.

```rust
use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface},
};
declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");
#[program]
pub mod token_example {
    use super::*;
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token_interface::mint_to(cpi_context, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
```

---

### Anchor.toml: Provider Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Configures the wallet and cluster used for all Anchor commands. Specifies the target cluster (e.g., 'localnet') and the path to the keypair file for wallet authentication.

```toml
[provider]
cluster = "localnet"                    # The cluster used for all commands.
wallet = "~/.config/solana/id.json"     # The keypair used for all commands.

```

---

### Anchor Client Script with `declare_program!()` (Rust)

Source: https://www.anchor-lang.com/docs/features/declare-program

This Rust client script demonstrates how to interact with an Anchor program using the `declare_program!()` macro. It sets up an RPC client, generates keypairs, requests an airdrop, and then uses the `anchor-client` crate to build and send `initialize` and `increment` instructions. The script fetches and prints the final counter value, illustrating a complete off-chain interaction flow.

```rust
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        native_token::LAMPORTS_PER_SOL,
        signature::Keypair,
        signer::Signer,
        system_program,
    },
    Client,
    Cluster,
};
use anchor_lang::prelude::*;
use std::rc::Rc;
declare_program!(example);
use example::{accounts::Counter, client::accounts, client::args};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899", // Local validator URL
        CommitmentConfig::confirmed(),
    );
    // Generate Keypairs and request airdrop
    let payer = Keypair::new();
    let counter = Keypair::new();
    println!("Generated Keypairs:");
    println!("   Payer: {}", payer.pubkey());
    println!("   Counter: {}", counter.pubkey());
    println!("\nRequesting 1 SOL airdrop to payer");
    let airdrop_signature = connection.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)?;
    // Wait for airdrop confirmation
    while !connection.confirm_transaction(&airdrop_signature)? {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("   Airdrop confirmed!");
    // Create program client
    let provider = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::confirmed(),
    );
    let program = provider.program(example::ID)?;
    // Build and send instructions
    println!("\nSend transaction with initialize and increment instructions");
    let initialize_ix = program
        .request()
        .accounts(accounts::Initialize {
            counter: counter.pubkey(),
            payer: program.payer(),
            system_program: system_program::ID,
        })
        .args(args::Initialize)
        .instructions()?
        .remove(0);
    let increment_ix = program
        .request()
        .accounts(accounts::Increment {
            counter: counter.pubkey(),
        })
        .args(args::Increment)
        .instructions()?
        .remove(0);
    let signature = program
        .request()
        .instruction(initialize_ix)
        .instruction(increment_ix)
        .signer(&counter)
        .send()
        .await?;
    println!("   Transaction confirmed: {}", signature);
    println!("\nFetch counter account data");
    let counter_account: Counter = program.account::<Counter>(counter.pubkey()).await?;
    println!("   Counter value: {}", counter_account.count);
    Ok(())
}
```

---

### Anchor #[derive(Accounts)] Macro Usage

Source: https://www.anchor-lang.com/docs/basics/program-structure

Demonstrates the usage of the #[derive(Accounts)] macro to define the accounts required for an instruction, such as the 'initialize' instruction.

```rust
#[derive(Accounts)]

pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]

    pubnew_account: Account<'info, NewAccount>,
    #[account(mut)]

    pubsigner: Signer<'info>,

    pubsystem_program: Program<'info, System>,
}
```

---

### Account Parsing with `declare_program!`

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Demonstrates the use of `declare_program!` to generate an enum for parsing accounts of a specific program. It shows how to use the generated `try_from_bytes` utility.

```Rust
declare_program!(my_program);
// Usage:
// my_program::utils::Account::try_from_bytes(...)
```

---

### Create Associated Token Account (Anchor)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Defines the accounts required for creating an associated token account using Anchor. It includes constraints for initialization, payer, mint, authority, and token programs.

```rust
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
// --snip--
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]

    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
```

---

### Anchor Shell

Source: https://www.anchor-lang.com/docs/references/cli

Launches an interactive Node.js shell with a pre-configured Anchor client. This client is set up according to the local workspace configuration, allowing direct interaction with deployed Solana programs.

```bash
anchor shell
```

---

### Anchor CLI: Test Specific Program

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Tests only a specified program within a multi-program workspace using the `--program-name` (`-p`) argument, which builds and tests the selected program.

```bash
anchor test --program-name example
```

---

### Anchor.toml: Test Startup Wait Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Increases the duration Anchor waits for the `solana-test-validator` to initialize. This is particularly useful when cloning numerous accounts, which can extend the validator's startup time.

```toml
[test]
startup_wait = 10000

```

---

### Anchor IDL Fetch

Source: https://www.anchor-lang.com/docs/references/cli

Fetches the IDL from a configured blockchain cluster and saves it to a specified output file. Ensure your Anchor.toml is configured with the correct cluster endpoint.

```bash
anchor idl fetch -o <out-file.json> <program-id>
```

```bash
anchor idl fetch GrAkKfEpTKQuVHG2Y97Y2FF4i7y7Q5AHLK94JBy7Y5yv
```

---

### Upgrade Solana Version

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Command to upgrade the Solana CLI tools to a specific version using the `solana-install` utility. This ensures compatibility with the Anchor version and other Solana ecosystem tools.

```bash
solana-install init 1.18.17
```

---

### Rust: SPL create_metadata_accounts_v3 and set_collection_size Wrappers

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides new wrapper functions in the Rust SPL module for `create_metadata_accounts_v3` and `set_collection_size`.

```rust
use anchor_spl::token_2022::metadata::{create_metadata_accounts_v3, set_collection_size};

// Example usage would involve calling these functions with appropriate parameters.
```

---

### TOML: Cargo.toml Dependencies for Anchor Lang Project

Source: https://www.anchor-lang.com/docs/clients/rust

This TOML file lists the dependencies required for an Anchor Lang project. It includes the anchor-client and anchor-lang crates, along with anyhow for error handling and tokio for asynchronous operations.

```TOML
[package]
name = "rs"
version = "0.1.0"
edition = "2021"
[dependencies]
anchor-client = { version = "0.31.1", features = ["async"] }
anchor-lang = "0.31.1"
anyhow = "1.0.93"
tokio = { version = "1.0", features = ["full"] }
```

---

### Anchor CLI: Support Fetching Legacy IDLs

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now includes support for fetching legacy IDLs, enhancing compatibility with older projects.

```bash
# This is a background change in the CLI's IDL fetching logic.
```

---

### Configure Program Upgradability in Tests

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Settings in Anchor.toml to enable upgradability for programs during testing, either globally or for specific programs.

```toml
[test]
upgradeable = true
```

```toml
[[test.genesis]]
address = "22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD"
program = "swap.so"
upgradeable = true
```

---

### Add Initialize Account Instruction for SPL Tokens

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `initialize_account` instruction for SPL tokens. This allows for the creation and initialization of new SPL token accounts.

```rust
use anchor_spl::token::{Mint, Token, InitializeAccount};

let cpi_accounts = InitializeAccount {
    account: account.to_account_buf()?,
    mint: mint.to_account_buf()?,
    authority: authority.to_account_buf()?,
};

let (ix, _) = TokenInstruction::initialize_account(cpi_accounts);

```

---

### Cargo.toml: Add Solana Benchmarking Configuration

Source: https://www.anchor-lang.com/docs/testing/mollusk

This configuration snippet for `Cargo.toml` enables benchmarking for a Solana program using Mollusk. It specifies the benchmark name and sets `harness` to `false` to allow custom benchmarking logic, such as that provided by Mollusk.

```toml
[[bench]]
name = "compute_units"
harness = false
```

---

### List Cluster Endpoints

Source: https://www.anchor-lang.com/docs/references/cli

Lists the available Solana cluster endpoints that can be used with Anchor.

```bash
anchor cluster list
```

---

### Anchor: Emit Events

Source: https://www.anchor-lang.com/docs/features

Shows how to emit events from Anchor programs using the `emit!` and `emit_cpi!` macros. Events are crucial for off-chain applications to monitor program state changes.

```Rust
use anchor_lang::prelude::*;

#[event]
pub struct MyEvent {
    pub data: u64,
}

// In your instruction:
// emit!(MyEvent { data: 123 });
```

---

### Deploy Programs

Source: https://www.anchor-lang.com/docs/references/cli

Deploys all programs defined in the Anchor workspace to the currently configured cluster. Note that this command generates a new program address each time it is executed.

```bash
anchor deploy
```

---

### Rust: Implement Solana Transfer CPI with PDA Signer

Source: https://www.anchor-lang.com/docs/basics/cpi

Shows a typical Anchor approach for constructing Cross Program Invocations (CPIs). It creates a CpiContext including the program ID and accounts, then uses `with_signer` to include PDA seeds and the bump seed for signing.

```Rust
pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
    let from_pubkey = ctx.accounts.pda_account.to_account_info();
    let to_pubkey = ctx.accounts.recipient.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();
    let seed = to_pubkey.key();
    let bump_seed = ctx.bumps.pda_account;
    let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

    letcpi_context= CpiContext::new(
        program_id,
        Transfer {
            from: from_pubkey,
            to: to_pubkey,
        },
    )
    .with_signer(signer_seeds);

    transfer(cpi_context, amount)?;
    Ok(())
}

```

---

### Support State Instructions in Client

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for state instructions in the client. This allows for more structured interaction with program state through dedicated instruction types.

```rust
#[program_state]
struct MyState {
    // ...
}

#[instruction(state: MyState)]
fn my_instruction(...) {}

```

---

### Anchor CLI: Build IDL for Single Program

Source: https://www.anchor-lang.com/docs/updates/changelog

If a project contains only one program, the `anchor idl build` command will now automatically build its IDL.

```bash
# This behavior is automatic when only one program exists.
```

---

### Interacting with SPL Tokens using Anchor

Source: https://www.anchor-lang.com/docs/tokens/extensions

This section provides guidance on interacting with SPL Tokens, including their basics and extensions, within the Anchor framework. It's essential for developers building Solana programs that handle token operations.

```Rust
/*
This is a placeholder for Rust code examples related to SPL Token interactions within Anchor.
Actual code would involve defining accounts, instructions, and program logic for token operations.
*/

// Example structure for a token account
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    // ... other accounts like token program, etc.
}

// Example instruction for transferring tokens
pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    // Logic to transfer tokens using Anchor's Token program integration
    Ok(())
}
```

```TypeScript
/*
This is a placeholder for TypeScript code examples related to SPL Token interactions within Anchor.
Actual code would involve using the @solana/web3.js and @project-serum/anchor libraries.
*/

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey, Transaction, SystemProgram } from "@solana/web3.js";

// Example function to transfer tokens using Anchor client
async function transferTokens(program: Program, fromPublicKey: PublicKey, toPublicKey: PublicKey, amount: number): Promise<void> {
    // Get associated token accounts if they don't exist
    // ... logic to get or create associated token accounts

    // Call the Anchor program instruction
    await program.rpc.transferTokens(new anchor.BN(amount), {
        accounts: {
            from: /* from associated token account */,
            to: /* to associated token account */,
            authority: program.provider.wallet.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            // ... other accounts
        },
    });
    console.log("Tokens transferred successfully!");
}
```

---

### Anchor CLI: Build with Alternate Architecture

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Allows preserving the old build behavior by explicitly specifying the architecture, defaulting to `cargo build-sbf`.

```bash
anchor build --arch bpf
```

---

### Anchor Program IDL: Example.json

Source: https://www.anchor-lang.com/docs/clients/typescript

This JSON file represents the Interface Definition Language (IDL) for the Anchor program. It details the program's address, metadata, instructions, accounts, and data types, enabling client-side interaction.

```json
{
  "address": "6khKp4BeJpCjBY1Eh39ybiqbfRnrn2UzWeUARjQLXYRC",
  "metadata": {
    "name": "example",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "increment",
      "discriminator": [11, 18, 104, 9, 104, 174, 59, 33],
      "accounts": [
        {
          "name": "counter",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "counter",
          "writable": true,
          "signer": true
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "Counter",
      "discriminator": [255, 176, 4, 245, 188, 253, 124, 25]
    }
  ],
  "types": [
    {
      "name": "Counter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "count",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
```

---

### Anchor Rust: MintTo CPI Logic

Source: https://www.anchor-lang.com/docs/tokens/basics/mint-tokens

Illustrates the core logic within an Anchor instruction to perform a cross-program invocation (CPI) for minting tokens. It constructs the `MintTo` struct with necessary accounts and context, then calls the `token_interface::mint_to` function.

```rust
pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    // Create the MintTo struct with the accounts required for the CPI
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    // The program being invoked in the CPI
    let cpi_program = ctx.accounts.token_program.to_account_info();
    // Combine the accounts and program into a "CpiContext"
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    // Make CPI to mint_to instruction on the token program
    token_interface::mint_to(cpi_context, amount)?;
    Ok(())
}
```

---

### Rust Anchor Program: Initialize and Increment

Source: https://www.anchor-lang.com/docs/clients/typescript

This Rust code defines a simple Anchor program with two instructions: 'initialize' to create and set a counter account, and 'increment' to increase its value. It includes account definitions and the program's public interface.

```rust
use anchor_lang::prelude::*;
declare_id!("6khKp4BeJpCjBY1Eh39ybiqbfRnrn2UzWeUARjQLXYRC");
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count += 1;
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
#[account]
pub struct Counter {
    pub count: u64,
}
```

---

### Anchor Program IDL (JSON)

Source: https://www.anchor-lang.com/docs/clients/rust

The Interface Description Language (IDL) for the Anchor program. It specifies the program's address, metadata, instructions (including discriminators, accounts, and arguments), and account definitions.

```json
{
  "address": "6khKp4BeJpCjBY1Eh39ybiqbfRnrn2UzWeUARjQLXYRC",
  "metadata": {
    "name": "example",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "increment",
      "discriminator": [11, 18, 104, 9, 104, 174, 59, 33],
      "accounts": [
        {
          "name": "counter",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "counter",
          "writable": true,
          "signer": true
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "Counter",
      "discriminator": [255, 176, 4, 245, 188, 253, 124, 25]
    }
  ],
  "types": [
    {
      "name": "Counter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "count",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
```

---

### Anchor Program Definition (Rust)

Source: https://www.anchor-lang.com/docs/clients/rust

Defines a simple Anchor program with 'initialize' and 'increment' instructions. It includes account structures for 'Counter' and context definitions for the instructions. The program uses `declare_id!` to set its program ID.

```rust
use anchor_lang::prelude::*;
declare_id!("6khKp4BeJpCjBY1Eh39ybiqbfRnrn2UzWeUARjQLXYRC");
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count += 1;
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
#[account]
pub struct Counter {
    pub count: u64,
}
```

---

### Anchor CLI: Workspace Inheritance

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now supports workspace inheritance, allowing for more modular and reusable project configurations.

```bash
# Example Anchor.toml with workspace inheritance
# [programs.local]
# my_program = { url = "http://localhost:8899", deployer = "..." }
#
# [programs.devnet]
# my_program = { url = "https://api.devnet.solana.com", deployer = "..." }
#
# [test]
# program_id = "<your_program_id>"
#
# [scripts]
# test = "anchor test"

```

---

### Fetch and Deserialize Counter Account

Source: https://www.anchor-lang.com/docs/features/declare-program

Fetches the Counter account data from the program using its public key and deserializes it into the Counter struct.

```rust
let counter_account: Counter = program.account::<Counter>(counter.pubkey()).await?;
```

---

### IDL Build Feature (Cargo.toml)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Shows how to enable the `idl-build` feature in a program's `Cargo.toml` to generate Interface Definition Language (IDL) files during compilation. This feature requires listing all used crates with the `idl-build` feature.

```TOML
[features]
idl-build = ["anchor-lang/idl-build"]
```

```TOML
[features]
idl-build = [
    "anchor-lang/idl-build",
    "anchor-spl/idl-build",
    "another-program/idl-build"
]
```

---

### Anchor Lang: Generate Documentation for Constants

Source: https://www.anchor-lang.com/docs/updates/changelog

The `declare_program!` macro in Anchor's Rust library now generates documentation for constants defined within it, improving code clarity.

```rust
declare_program!(MyProgram, "MYPROG", "MYDISC",
    /// This is a constant value.
    const MY_CONSTANT: u64 = 100;
);
```

---

### Create Associated Token Account (Anchor Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Creates a new associated token account using Anchor's account constraints. It leverages CPI to the System Program for initialization and requires specifying the payer, mint, authority, and token program.

```rust
#[account(
    init,
    payer = <payer>,
    associated_token::mint = <mint>,
    associated_token::authority = <authority>,
    associated_token::token_program = <token_program>
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,

```

---

### Anchor CLI: Optional Package Manager Flag in 'init'

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor init` command now accepts an optional `package-manager` flag. This allows users to specify the package manager (e.g., npm, yarn) to be used when setting up a new project.

```bash
anchor init --package-manager yarn my_project
```

---

### Process Single Instruction with Mollusk SVM

Source: https://www.anchor-lang.com/docs/testing/mollusk

Demonstrates how to process a single instruction using the `process_instruction` method in Mollusk SVM. It sets up necessary components like program ID, keys, instruction, and accounts, then executes the instruction and retrieves the result.

```rust
use {
    mollusk_svm::Mollusk,
    solana_sdk::{account::Account, instruction::{AccountMeta, Instruction}, pubkey::Pubkey},
};
let program_id = Pubkey::new_unique();
let key1 = Pubkey::new_unique();
let key2 = Pubkey::new_unique();
let instruction = Instruction::new_with_bytes(
    program_id,
    &[],
    vec![
        AccountMeta::new(key1, false),
        AccountMeta::new_readonly(key2, false),
    ],
);
let accounts = vec![
    (key1, Account::default()),
    (key2, Account::default()),
];
let mollusk = Mollusk::new(&program_id, "my_program");
// Execute the instruction and get the result.
let result = mollusk.process_instruction(&instruction, &accounts);
```

---

### Rust: Benchmark Solana Program Compute Units

Source: https://www.anchor-lang.com/docs/testing/mollusk

This Rust code demonstrates how to use the MolluskComputeUnitBencher to benchmark compute unit usage for Solana programs. It requires setting up the Mollusk SVM and defining instructions and accounts for each benchmark. The output is a markdown file detailing CU usage and changes.

```rust
use {
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    mollusk_svm::Mollusk,
    /* ... */
};
// Optionally disable logging.
solana_logger::setup_with("");
/* Instruction & accounts setup ... */
let mollusk = Mollusk::new(&program_id, "my_program");
MolluskComputeUnitBencher::new(mollusk)
    .bench(("bench0", &instruction0, &accounts0))
    .bench(("bench1", &instruction1, &accounts1))
    .bench(("bench2", &instruction2, &accounts2))
    .bench(("bench3", &instruction3, &accounts3))
    .must_pass(true)
    .out_dir("../target/benches")
    .execute();
```

---

### Anchor Client/Lang/SPL/Syn: Docs.rs Build Features

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables all features for the `docs.rs` build across the client, lang, spl, and syn crates. This ensures that the documentation generated on docs.rs is comprehensive and includes all available features.

```rust
# Cargo.toml snippet for enabling all features for docs.rs
[package]
name = "anchor_client"
version = "0.29.0"

[features]
default = ["all"]
all = [] # Explicitly list all features here if needed, or rely on default behavior

[dependencies]
# ... other dependencies ...

```

---

### Anchor CLI: Priority Fees for IDL Commands

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now supports priority fees for IDL commands. This allows users to specify a priority fee when interacting with IDL-related operations on the Solana network, potentially speeding up transaction confirmation.

```bash
anchor idl upgrade --priority-fee 1000
anchor idl publish --priority-fee 2000
```

---

### Testing Anchor Programs with LiteSVM

Source: https://www.anchor-lang.com/docs/testing

LiteSVM allows you to write tests for Solana programs using Rust, TypeScript/JavaScript, or Python. It provides a framework for interacting with your programs during testing.

```Rust
/* Example Rust test using LiteSVM */
// import necessary LiteSVM modules
// write test functions interacting with Anchor program
```

```TypeScript
/* Example TypeScript test using LiteSVM */
// import necessary LiteSVM modules
// write test functions interacting with Anchor program
```

```Python
'''Example Python test using LiteSVM'''
# import necessary LiteSVM modules
# write test functions interacting with Anchor program
```

---

### Anchor TOML Configuration Updates

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a `[registry]` section in Anchor.toml, adds `anchor login` and `anchor publish` commands, and allows specifying `anchor_version` and `solana_version` for verifiable builds. These changes streamline publishing and environment management.

```rust
cli: Adds a `[registry]` section in the Anchor toml (#570).
cli: Adds the `anchor login <api-token>` command (#570).
cli: Adds the `anchor publish <package>` command (#570).
cli: Adds a root level `anchor_version` field to the Anchor.toml for specifying the anchor docker image to use for verifiable builds (#570).
cli: Adds a root level `solana_version` field to the Anchor.toml for specifying the solana toolchain to use for verifiable builds (#570).
```

---

### Anchor CLI: Use Package Name as Program Name

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Allows using the package name (snake_case) of the program instead of the library name with the `--program-name` (`-p`) argument in various commands.

```bash
anchor build -p my-program
```

---

### Anchor CLI: Initialize Workspace with Rust Test Template

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Initializes a new Anchor workspace using the Rust test template, which prevents a wild TypeScript test from appearing.

```bash
anchor init --test-template rust
```

---

### Deterministic and Verifiable Builds

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for deterministic and verifiable builds. This ensures that builds are reproducible and can be verified for integrity.

```bash
anchor build --verify

```

---

### Instruction Discriminator Generation (Conceptual)

Source: https://www.anchor-lang.com/docs/basics/idl

Demonstrates the conceptual process of generating an instruction discriminator by hashing a prefix and instruction name. The first 8 bytes of the SHA256 hash are used as the discriminator.

```shell
sha256("global:initialize")
```

---

### Anchor Try From Parameter Change

Source: https://www.anchor-lang.com/docs/updates/changelog

The `try_from` methods for `ProgramAccount`, `Loader`, and `ProgramState` now require an additional `program_id: &Pubkey` parameter. This enhances security by explicitly specifying the program ID.

```rust
lang: `try_from` methods for `ProgramAccount`, `Loader`, and `ProgramState` now take in an additional `program_id: &Pubkey` parameter (#660).
```

---

### `idl init` Supports Large IDL Files

Source: https://www.anchor-lang.com/docs/updates/changelog

Enhances the `anchor idl init` command to support very large IDL files. This allows for the initialization of programs with extensive interface definitions.

```bash
# Initialize IDL with a large file
anchor idl init --filepath large_idl.json
```

---

### Rust: Constructing CPI Context with PDA Signers

Source: https://www.anchor-lang.com/docs/basics/cpi

Illustrates the specific steps to include PDA-derived seeds and the bump seed within an Anchor CPI context. This is crucial for allowing a program to sign transactions on behalf of a PDA.

```Rust
let seed = to_pubkey.key();
let bump_seed = ctx.bumps.pda_account;


letsigner_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];
let cpi_context = CpiContext::new(
    program_id,
    Transfer {
        from: from_pubkey,
        to: to_pubkey,
    },
)

.with_signer(signer_seeds);

```

---

### Anchor Sealevel Attacks: Insecure vs. Secure vs. Recommended

Source: https://www.anchor-lang.com/docs/references/security-exploits

Demonstrates common attack patterns in Solana programs developed with Anchor. It provides three variations for each attack: an insecure version, a secure fix, and a recommended idiomatic Anchor solution.

```rust
1. insecure - represents flawed code that may be insecure
2. secure - represents a fix
3. recommended - represents a fix with idiomatic Anchor code
```

---

### Convert Legacy IDLs using CLI

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Anchor CLI command to convert legacy IDL JSON files to the new IDL format. This utility simplifies the process of updating older IDL definitions to be compatible with newer Anchor versions.

```bash
anchor idl convert <PATH_TO_IDL_JSON>
```

---

### Anchor Token Initialization Syntax Change

Source: https://www.anchor-lang.com/docs/updates/changelog

Changes the syntax for token initialization from `#[account(init, seeds = [...], token = <expr>, authority = <expr>)]` to `#[account(init, token::mint = <expr> token::authority = <expr>)]`. This provides a more structured way to specify token parameters.

```rust
lang: Change `#[account(init, seeds = [...], token = <expr>, authority = <expr>)]` to `#[account(init, token::mint = <expr> token::authority = <expr>)]` (#562).
```

---

### Anchor CLI: Run Multiple Commands with `anchor test`

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor test` command can now run multiple commands sequentially. This enhances the testing workflow by allowing users to chain related test operations.

```bash
# Example: Run build, then test
anchor test --commands "build,test"

# Example: Run specific test files
anchor test --commands "test tests/my_test.rs, test tests/another_test.rs"
```

---

### Compile Programs with `cargo build-sbf` using `--arch sbf`

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `--arch sbf` option to Anchor CLI commands, enabling programs to be compiled using `cargo build-sbf`. This is essential for building programs targeting the Solana Virtual Machine (SVM).

```bash
# Build a program for the SBF architecture
anchor build --arch sbf
```

---

### Include prettier formatter in Anchor workspaces

Source: https://www.anchor-lang.com/docs/updates/changelog

Workspaces created using `anchor init` now include the `prettier` formatter and associated scripts, promoting consistent code style.

```bash
`anchor init` (#1741).
```

---

### Anchor CLI/Lang: IDL Generation via Compilation

Source: https://www.anchor-lang.com/docs/updates/changelog

IDL generation is now supported through compilation. `anchor build` uses the parsing method, while `anchor idl build` uses the new build method.

```bash
# Generate IDL using the build method
anchor idl build

# Generate IDL using the parsing method (default for anchor build)
anchor build

```

---

### Anchor Keys List

Source: https://www.anchor-lang.com/docs/references/cli

Lists all program keypairs associated with the current workspace. This is useful for managing cryptographic identities for your Solana programs.

```bash
anchor keys list
```

---

### Anchor Migrate Deploy

Source: https://www.anchor-lang.com/docs/references/cli

Executes the deployment script located at `migrations/deploy.js`. It injects a provider configured from the workspace's Anchor.toml, facilitating the deployment process. Migrations currently support a simple deploy script structure.

```javascript
// File: migrations/deploys.js
const anchor = require("@coral-xyz/anchor");
module.exports = async function (provider) {
  anchor.setProvider(provider);
  // Add your deploy script here.
};
```

---

### Anchor CLI: Run Script with Multiple Commands

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Executes a script defined in `Anchor.toml` that contains multiple commands.

```bash
anchor run test-all
```

---

### Create Token Account using PDA

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Creates a token account using a Program Derived Address (PDA) as its address. This method is useful for managing token accounts deterministically. It requires the signer, mint account, token program, and system program.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");
#[program]
pub mod token_example {
    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Created Mint Account: {:?}", ctx.accounts.mint.key());
        Ok(())
    }
    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!(
            "Created Token Account: {:?}",
            ctx.accounts.token_account.key()
        );
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint.key(),
        mint::freeze_authority = mint.key(),
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(

        init_if_needed,
        payer = signer,
        token::mint = mint,
        token::authority = token_account,
        token::token_program = token_program,
        seeds = [b"token"],
        bump
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
```

---

### Add OS and CPU Fields to NPM Package (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Includes `os` and `cpu` fields in the `@project-serum/anchor-cli` npm package metadata. This provides system information for the package.

```json
"os": "linux",
"cpu": "x64"
```

---

### Anchor Program and Signer Types

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `Program` type for executable accounts and the `Signer` type for accounts that only require signing without data usage. These types improve clarity and safety in account handling.

```rust
lang: `Program` type introduced for executable accounts (#705).
lang: `Signer` type introduced for signing accounts where data is not used (#705).
```

---

### CLI: Anchor Test --run Flag

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `--run` flag to `anchor test` in the CLI, allowing users to execute a subset of test suites.

```bash
# Run only tests tagged with 'my_feature'
anchor test --run my_feature

```

---

### Rust: Mint Tokens via CPI with PDA Signature

Source: https://www.anchor-lang.com/docs/tokens/basics/mint-tokens

Shows how to mint tokens using a Cross Program Invocation (CPI) where the program signs with a PDA's seeds. This function requires the amount of tokens to mint and uses the `mint_to` instruction from the SPL Token Interface.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface};

// ... create_mint function ...

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> TokenExample<'info> {
    pub fn mint_tokens(&self, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[self.bumps.mint]]];
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.mint.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

        token_interface::mint_to(cpi_context, amount)?;
        Ok(())
    }
}

```

---

### TypeScript: Builder Pattern `.prepare()` Method

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `.prepare()` method to the builder pattern in the TypeScript client, allowing for pre-transaction preparation.

```typescript
import { Program } from "@coral-xyz/anchor";

// Example:
const preparedTransaction = await program.methods.myMethod().prepare();
// Use preparedTransaction for further processing or sending.
```

---

### Anchor CLI: Pass Arguments to Solana Program Deploy

Source: https://www.anchor-lang.com/docs/updates/changelog

Enhances the 'anchor deploy' command to support passing arguments directly to the underlying 'solana program deploy' command. This provides greater flexibility when deploying Solana programs.

```bash
anchor deploy -- --some-solana-arg value
```

---

### Anchor CLI: Fix Build --no-docs

Source: https://www.anchor-lang.com/docs/updates/changelog

A fix has been implemented for the `anchor build --no-docs` command to prevent it from incorrectly adding documentation to the IDL.

```bash
# This command should now correctly exclude docs from the IDL
anchor build --no-docs

```

---

### Anchor TS: Optional Provider Options

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Shows how the `opts` parameter for `AnchorProvider` is now optional. This simplifies the instantiation of the provider when default options are sufficient.

```javascript
new AnchorProvider(connection, wallet);
```

---

### Instruction Discriminator Mapping (Decimal)

Source: https://www.anchor-lang.com/docs/basics/idl

Illustrates the conversion of the first 8 hexadecimal bytes of the SHA256 hash into their decimal equivalents, representing the instruction discriminator.

```shell
af = 175
af = 175
6d = 109
1f = 31
0d = 13
98 = 152
9b = 155
ed = 237
```

---

### Add --skip-build to anchor publish

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor publish` command now includes a `--skip-build` flag, allowing users to publish without triggering a build process. This can be useful for specific deployment workflows.

```bash
`anchor publish` (#1786).
```

---

### Anchor.toml: Programs Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Lists the addresses of programs within the workspace, specifically for `localnet`. This configuration is used during localnet testing to load programs at genesis using the `--bpf-program` option with `solana-test-validator`.

```toml
[programs.localnet]
my_program = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"

```

---

### Anchor Lang: Lamports Helper Methods

Source: https://www.anchor-lang.com/docs/updates/changelog

New methods `get_lamports`, `add_lamports`, and `sub_lamports` have been added for all account types, simplifying lamport manipulation.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MyAccount<'info> {
    #[account(mut)]
    pub my_account: Account<'info, SomeData>,
}

#[account]
pub struct SomeData {
    // ... data fields
}

fn example_usage(my_account: &mut Account<'_, SomeData>) {
    let lamports = my_account.get_lamports();
    my_account.add_lamports(1000)?;
    my_account.sub_lamports(500)?;
}
```

---

### Anchor Client: DynSigner Helper

Source: https://www.anchor-lang.com/docs/updates/changelog

A `DynSigner` helper struct is introduced to simplify the use of `Client<C>` with Solana clap CLI utilities when `Signer` is loaded as `Box<dyn Signer>`.

```rust
use anchor_client::solana_client::rpc::client::RpcClient;
use anchor_client::solana_client::signer::keypair::Keypair;
use anchor_client::Client;
use std::ops::Deref;

// Example usage with DynSigner
struct MySigner(Keypair);

impl Deref for MySigner {
    type Target = Keypair;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// let signer = MySigner(Keypair::new());
// let client: Client<impl Signer + Clone + Deref<Target = impl Signer>> = Client::new_with_signer(..., signer);

```

---

### Override Toolchain Versions in Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Configuration in Anchor.toml to specify the desired Anchor CLI and Solana versions for the workspace.

```toml
[toolchain]
anchor_version = "0.29.0" # `anchor-cli` version to use
solana_version = "1.17.0" # Solana version to use
```

---

### Anchor CLI: Short Alias for 'idl build'

Source: https://www.anchor-lang.com/docs/updates/changelog

A short alias has been added for the `anchor idl build` command, making it quicker to invoke.

```bash
anchor idl build
```

---

### Rust Token Account Structure

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Defines the structure of a Token account in Solana programs, including fields for mint, owner, amount, delegate, state, and native token information. This structure is used by both the Token Program and Token Extension Program.

```Rust
/// Account data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u64,
    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate
    pub delegate: COption<Pubkey>,
    /// The account's state
    pub state: AccountState,
    /// If `is_native.is_some`, this is a native token, and the value logs the
    /// rent-exempt reserve. An Account is required to be rent-exempt, so
    /// the value is used by the Processor to ensure that wrapped SOL
    /// accounts do not drop below this threshold.
    pub is_native: COption<u64>,
    /// The amount delegated
    pub delegated_amount: u64,
    /// Optional authority to close the account.
    pub close_authority: COption<Pubkey>,
}
```

---

### Anchor.toml: Run Multiple Commands in Scripts

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Enables running multiple commands sequentially within scripts defined in `Anchor.toml` using the `&&` operator.

```toml
[scripts]
test-all = "cargo test && yarn run ts-mocha tests/**/*.ts"
```

---

### Configure Cargo.toml for anchor-spl

Source: https://www.anchor-lang.com/docs/tokens

This configuration for `Cargo.toml` demonstrates how to include `anchor-spl` as a dependency and enable the `idl-build` feature, which is required for Anchor's IDL generation process when using `anchor-spl`.

```TOML
[features]
idl-build = [
    "anchor-lang/idl-build",
    "anchor-spl/idl-build",
]
[dependencies]
anchor-lang = "0.31.1"
anchor-spl = "0.31.1"
```

---

### Build Increment Instruction

Source: https://www.anchor-lang.com/docs/features/declare-program

Builds the instruction to increment the counter. It requires the counter account and the Increment argument.

```rust
let increment_ix = program
    .request()
    .accounts(accounts::Increment {
        counter: counter.pubkey(),
    })
    .args(args::Increment)
    .instructions()?
    .remove(0);
```

---

### TypeScript: Set Args After Accounts and Retrieve Pubkeys

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables setting arguments after accounts have been specified and retrieving public keys in the TypeScript client.

```typescript
import { Program } from "@coral-xyz/anchor";

// Example:
const builder = program.methods.myMethod().accounts({
  signer: signer.publicKey,
  dataAccount: dataAccount.publicKey,
});

// Set arguments after accounts
const tx = await builder.args({ value: 10 }).rpc();
```

---

### Anchor AVM: .anchorversion File Support

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Version Manager (AVM) now supports the `.anchorversion` file to easily switch between different versions of the `anchor-cli`.

```bash
# Create a .anchorversion file in your project root
# Specify the desired Anchor version
echo "0.29.0" > .anchorversion

# AVM will automatically use this version when running anchor commands

```

---

### Add sync_native CPI wrapper for Token Program

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides a new CPI (Cross-Program Invocation) wrapper function `sync_native` for the Token Program, simplifying interactions with native token accounts.

```rust
// Add `sync_native` token program CPI wrapper function (#1833).
```

---

### Generic ProgramData and UpgradableLoaderState (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows `ProgramData` and `UpgradableLoaderState` to be passed as generics to `Account`. This enhances flexibility when working with upgradeable programs.

```rust
Account<'info, ProgramData>
Account<'info, UpgradableLoaderState>
```

---

### Untitled

Source: https://www.anchor-lang.com/docs/updates/changelog

No description

---

### Anchor CLI: Migrate to agave-install for Solana Versions

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now migrates to `agave-install` when the `solana_version` is greater than or equal to `1.18.19`. This ensures compatibility with newer Solana toolchain management.

```bash
# This is a background change in the CLI's logic.
```

---

### Anchor SPL: Checked Burn, Mint, and Approve Instructions

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds `burn_checked`, `mint_to_checked`, and `approve_checked` instructions to the Anchor SPL library. These instructions provide checked operations for token management.

```rust
use anchor_spl::token::{burn_checked, BurnChecked};
use anchor_spl::token::{mint_to_checked, MintToChecked};
use anchor_spl::token::{approve_checked, ApproveChecked};

// Example usage for burn_checked:
// let cpi_accounts = BurnChecked { ... };
// let cpi_program = token::program::ID;
// let (program_id, authority, mint, to, amount, decimals) = (...);
// let instruction = burn_checked(cpi_program, to, mint, authority, amount, decimals);
```

---

### Add `--skip-build` Option to Verify Command

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `--skip-build` option for the `anchor verify` command. This allows verifying a program's deployment without triggering a build process, useful for quick checks or when the build is already complete.

```bash
# Verify a program without rebuilding
anchor verify --skip-build
```

---

### Anchor lang: Initialize SPL token accounts with PDAs

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables initializing SPL token accounts as Program Derived Addresses (PDAs) using the `#[account(init, token = ...)]` constraint, supporting initialization via `seeds` or `associated`.

```rust
/// lang: Add `#[account(init, token = <mint-target>, authority = <token-owner-target>...)]` constraint for initializing SPL token accounts as program derived addresses for the program. Can be used when initialized via `seeds` or `associated` (#400).
```

---

### TypeScript Client: Interact with Anchor Program

Source: https://www.anchor-lang.com/docs/clients/typescript

This TypeScript code demonstrates how to build and send transactions to the Anchor program using `@coral-xyz/anchor`. It sets up a connection, generates keypairs, airdrops funds, and constructs transactions for initializing and incrementing the counter.

```typescript
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import type { Example } from "./idl/example.ts";
import idl from "./idl/example.json";
// Set up a connection to the cluster
const connection = new Connection("http://127.0.0.1:8899", "confirmed");
// Create a Program instance using the IDL and connection
const program = new Program(idl as Example, {
  connection,
});
// Generate new Keypairs for the payer and the counter account
const payer = Keypair.generate();
const counter = Keypair.generate();
// Airdrop SOL to fund the payer's account for transaction fees
const airdropTransactionSignature = await connection.requestAirdrop(
  payer.publicKey,
  LAMPORTS_PER_SOL
);
await connection.confirmTransaction(airdropTransactionSignature);
// Build the initialize instruction
const initializeInstruction = await program.methods
  .initialize()
  .accounts({
    payer: payer.publicKey,
    counter: counter.publicKey,
  })
  .instruction();
// Build the increment instruction
const incrementInstruction = await program.methods
  .increment()
  .accounts({
    counter: counter.publicKey,
  })
  .instruction();
// Add both instructions to a single transaction
const transaction = new Transaction().add(
  initializeInstruction,
  incrementInstruction
);
// Send the transaction
const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer, counter]
);
console.log("Transaction Signature", transactionSignature);
```

---

### Add `InitSpace` Derive Macro

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides the `#[derive(InitSpace)]` macro for automatically calculating the required space for an account at initialization. This simplifies account creation by removing the need for manual space calculations.

```rust
use anchor_lang::prelude::*;
use anchor_lang::solana_program::borsh::BorshSchema;

#[derive(Accounts, InitSpace)]
pub struct MyAccount {
    pub data: String,
    pub count: u64,
}

// The macro automatically calculates the space needed for MyAccount.
// You can then use this information when creating the account.
```

---

### Anchor IDL Build

Source: https://www.anchor-lang.com/docs/references/cli

Generates the Interface Description Language (IDL) for a Solana program using the compilation method. This is crucial for client-side applications to interact with the program.

```bash
anchor idl build
```

---

### Anchor.toml: Wildcard Pattern in Workspace Members

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Supports simple wildcard patterns in `workspace.members` and `workspace.exclude` within `Anchor.toml` for defining workspace configurations.

```toml
[workspace]
members = ["programs/*"]
```

---

### Conditional Instructions with `cfg`

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Illustrates how to conditionally compile instructions based on features using the `cfg` attribute. This allows for feature-gated functionality within Anchor programs.

```Rust
#[program]
mod my_program {
    #[cfg(feature = "my-feature")]
    pub fn my_feature(ctx: Context<MyFeature>) -> Result<()> {}
}
```

---

### Create Token Account with PDA Address (Anchor Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Creates a token account with a Program Derived Address (PDA) as its address. This method uses Anchor's account constraints and requires specifying seeds and a bump for PDA derivation, along with payer, mint, authority, and token program.

```rust
#[account(
    init,
    payer = <payer>,
    token::mint = <mint>,
    token::authority = <authority>,
    token::token_program = <token_program>,
    seeds = [<seeds>],
    bump
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,

```

---

### Anchor Rust: Create CpiContext for Transfer

Source: https://www.anchor-lang.com/docs/basics/cpi

Demonstrates creating a `CpiContext` for the System Program's transfer instruction. This context bundles the program ID and the specific accounts involved in the transfer operation.

```rust
let cpi_context = CpiContext::new(
    program_id,
    Transfer {
        from: from_pubkey,
        to: to_pubkey,
    },
);

```

---

### Anchor PDA Constraints: Multiple Seeds and Account References

Source: https://www.anchor-lang.com/docs/basics/pda

Illustrates deriving a PDA using multiple seeds, including a static seed and a dynamic seed derived from another account's public key. This is useful for creating PDAs tied to specific on-chain entities.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub signer: Signer<'info>,
    #[account(

        seeds = [b"hello_world", signer.key().as_ref()],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

---

### Support Multiple Programs with `Interface` and `InterfaceAccount`

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `Interface` and `InterfaceAccount` types to support multiple programs with the same interface, particularly relevant for Token-2022 extensions. This allows for more flexible program interactions.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, Token2022}

#[derive(Accounts)]
pub struct MyTokenAccounts<'info> {
    #[account(token::mint = mint.key(), token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: AccountInfo<'info>,
    // Specify the token program interface
    pub token_program: Interface<'info, TokenInterface>,
}
```

---

### TypeScript SDK Program Access (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Illustrates the improved flexibility in accessing programs within the TypeScript SDK, allowing both PascalCase and camelCase naming conventions for program properties.

```TypeScript
const camel = anchor.workspace.myProgram;
const pascal = anchor.workspace.MyProgram;
```

---

### Anchor Init Command Fix

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes a regression in the `anchor init` command that caused a "Workspace not found" error. This ensures that project initialization works correctly.

```rust
cli: Fix `anchor init` command "Workspace not found" regression (#598).
```

---

### Anchor Verify Program

Source: https://www.anchor-lang.com/docs/references/cli

Verifies that the on-chain bytecode of a deployed program matches the locally compiled artifact. This ensures the integrity and correctness of your deployed programs.

```bash
anchor verify <program-id>
```

---

### Create Mint Account using PDA (Anchor Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-mint

This snippet shows how to create a mint account where the address is a Program Derived Address (PDA). This approach uses 'seeds' and 'bump' constraints for deterministic address generation. The mint authority and freeze authority are also set to the PDA itself, enabling the program to sign minting instructions.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");
#[program]
pub mod token_example {
    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Created Mint Account: {:?}", ctx.accounts.mint.key());
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint.key(),
        mint::freeze_authority = mint.key(),
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
```

---

### Rust: Declare Program with Legacy IDLs

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Demonstrates using the `declare_program!` macro with legacy IDLs (pre Anchor v0.30). It highlights potential issues with non-default features and suggests solutions like using `idl convert` or regenerating the IDL with Anchor v0.30.

```rust
use anchor_lang::prelude::*;

#[program]
mod my_program {
    pub fn initialize(ctx: Context<Initialize>) -> Result {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

// Example of a legacy IDL structure that might require conversion or regeneration
// #[account]
// pub struct MyAccount {
//     pub data: u64,
// }
```

---

### PostInstructions and PreInstructions Replacement (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `postInstructions` and `preInstructions` as replacements for the deprecated `instructions` field in the TypeScript client. This offers a more structured way to manage transaction instructions.

```typescript
const tx = await program.methods.myMethod({ ... }).accounts({ ... }).preInstructions([...]).postInstructions([...]).rpc();
```

---

### Configure Anchor Toolchain Versions

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Overrides toolchain versions for Anchor projects, similar to `rust-toolchain.toml`. Specifies the Anchor CLI version, Solana version, and the JavaScript package manager to use.

```toml
[toolchain]
anchor_version = "0.31.1"
solana_version = "2.1.21"
package_manager = "yarn"
```

---

### Anchor lang: Instruction data in accounts constraints

Source: https://www.anchor-lang.com/docs/updates/changelog

Makes instruction data accessible within accounts constraints, enabling more context-aware account validation.

```rust
/// lang: Instruction data is now available to accounts constraints (#386).
```

---

### Add idl convert Command (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a new `idl convert` command for managing Interface Description Language conversions.

```bash
cli: Add `idl convert` command (#3009).
```

---

### Upgrade Solana to v2 and SPL to Latest (Client, Lang, SPL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates Solana dependencies to v2 and SPL dependencies to their latest versions across client, language, and SPL components.

```rust
client, lang, spl: Upgrade Solana to v2 and SPL to the latest (#3219).
```

---

### CLI: Account Subcommand

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a new `account` subcommand to the Anchor CLI for interacting with accounts on the Solana network.

```bash
# Fetch an account using the CLI
anchor account <account_address>

```

---

### Anchor SPL: Memo Program Support

Source: https://www.anchor-lang.com/docs/updates/changelog

Support for the Memo program has been added to the Anchor SPL library, facilitating the use of memo instructions.

```rust
use anchor_lang::prelude::*;
use anchor_spl::memo;

#[derive(Accounts)]
pub struct SendMemo<'info> {
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    pub memo_program: Program<'info, memo::Memo>
}

fn handler(ctx: Context<SendMemo>) -> Result<()> {
    let message = "Hello from Anchor!".to_string();
    let cpi_program = ctx.accounts.memo_program.to_account_info();
    let cpi_accounts = memo::MemoInstruction {
        memo: message.clone(),
    };
    anchor_lang::system_program::transfer(
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.signer.to_account_info(), // Example: transfer to self to attach memo
        },
        1000, // Amount
    )?;
    // The memo is attached to the transaction, not directly called via CPI instruction data
    // This example shows how to use the memo program context
    Ok(())
}

```

---

### Rust Client: Tokio Support

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Enables the use of 'tokio' with 'anchor-client' by fixing a compatibility issue. This allows developers to leverage 'tokio' for asynchronous operations within their Anchor projects.

```Rust
anchor-client = { version = "0.31.0", features = ["tokio"] }
```

---

### Anchor lang/ts: Change domain delimiters for instruction sighash

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the domain delimiters for the instruction sighash pre-image to a single colon `:` for consistency with accounts.

```rust
/// lang, ts: Change domain delimiters for the pre-image of the instruction sighash to be a single colon `:` to be consistent with accounts (#321).
```

---

### Anchor AccountInfo AsRef

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides `AsRef<AccountInfo>` for `AccountInfo` wrappers, allowing easier access to the underlying `AccountInfo` data.

```rust
lang: Add `AsRef<AccountInfo>` for `AccountInfo` wrappers (#652).
```

---

### Anchor Client: Mock RPC Client Option

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides an option to pass a mock RPC client when using the `anchor_client`. This is useful for testing client-side logic without interacting with a live Solana cluster.

```rust
use anchor_client::Client;
use solana_sdk::signature::{Keypair, Signer};

let payer = Keypair::new();
let mock_rpc_client = MockRpcClient::new(); // Assuming MockRpcClient is defined elsewhere
let client = Client::new_with_mock_rpc(mock_rpc_client, payer.pubkey());
```

---

### Anchor IDL Upgrade

Source: https://www.anchor-lang.com/docs/references/cli

Upgrades the on-chain IDL file to a new version specified by a local JSON file. The configured wallet must be the current authority for this operation.

```bash
anchor idl upgrade <program-id> -f <target/idl/program.json>
```

---

### Update Anchor CLI using Cargo

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Instructions to update the Anchor Version Manager (avm) and the Anchor CLI using Cargo, the Rust package manager.

```bash
cargo install --git https://github.com/coral-xyz/anchor --tag v0.29.0 avm --locked
```

```bash
avm install latest
```

---

### Add Anchor IDL Crate for Conversion

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Rust code snippet showing how to add the `anchor-lang-idl` crate with the `convert` feature to your project's `Cargo.toml`. This enables programmatic conversion of legacy IDLs to new IDLs.

```rust
anchor-lang-idl = { version = "0.1.1", features = ["convert"] }
```

---

### Sync Program ID Declarations with `keys sync`

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `anchor keys sync` command to synchronize program ID declarations. This command helps maintain consistency between program ID definitions in your project and their actual deployed addresses.

```bash
# Synchronize program ID declarations
anchor keys sync
```

---

### Derive PDA and Fund with SOL using TypeScript

Source: https://www.anchor-lang.com/docs/basics/cpi

This TypeScript code demonstrates how to derive a Program Derived Address (PDA) using specified seeds and the program's ID. It also shows how to fund this PDA account with SOL via a direct transfer transaction.

```typescript
const [PDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("pda"), wallet.publicKey.toBuffer()],

  program.programId
);

it("Fund PDA with SOL", async () => {
  const transferInstruction = SystemProgram.transfer({
    fromPubkey: wallet.publicKey,

    toPubkey: PDA,

    lamports: transferAmount,
  });

  const transaction = new Transaction().add(transferInstruction);

  const transactionSignature = await sendAndConfirmTransaction(
    connection,

    transaction,

    [wallet.payer] // signer
  );

  console.log(
    `\nTransaction Signature:` +
      `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
  );
});
```

---

### Anchor.toml: Workspace Members Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Defines the paths, relative to `Anchor.toml`, to all programs within the local workspace. This is crucial for programs not using the standard Anchor workflow or for publishing programs not written in Anchor.

```toml
[workspace]
members = [
    "programs/*",
    "other_place/my_program"
]

```

---

### Process Instruction Chain with Anchor Lang

Source: https://www.anchor-lang.com/docs/testing/mollusk

Demonstrates processing a chain of Solana system instructions using Anchor Lang's `process_instruction_chain`. This function executes each instruction sequentially and returns the final result without explicit validation checks.

```rust
use {
    mollusk_svm::Mollusk,
    solana_sdk::{account::Account, pubkey::Pubkey, system_instruction},
};
let mollusk = Mollusk::default();
let alice = Pubkey::new_unique();
let bob = Pubkey::new_unique();
let carol = Pubkey::new_unique();
let dave = Pubkey::new_unique();
let starting_lamports = 500_000_000;
let alice_to_bob = 100_000_000;
let bob_to_carol = 50_000_000;
let bob_to_dave = 50_000_000;
mollusk.process_instruction_chain(
    &[
        system_instruction::transfer(&alice, &bob, alice_to_bob),
        system_instruction::transfer(&bob, &carol, bob_to_carol),
        system_instruction::transfer(&bob, &dave, bob_to_dave),
    ],
    &[
        (alice, system_account_with_lamports(starting_lamports)),
        (bob, system_account_with_lamports(starting_lamports)),
        (carol, system_account_with_lamports(starting_lamports)),
        (dave, system_account_with_lamports(starting_lamports)),
    ],
);
```

---

### Add Additional Solana Arguments to Upgrade Command (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Enhances the `upgrade` command by allowing additional Solana arguments to be passed.

```bash
cli: Add additional solana arguments to the `upgrade` command (#2998).
```

---

### Fetch All Accounts

Source: https://www.anchor-lang.com/docs/clients/typescript

Shows how to fetch all existing accounts of a specific type defined in the Anchor IDL. The `program.account.<accountName>.all()` method deserializes and returns an array of all accounts matching the specified type.

```TypeScript
const accounts = await program.account.newAccount.all();
```

---

### Anchor CLI for Program Management

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor command-line interface (CLI) is a powerful tool for managing Anchor programs. It facilitates tasks such as building, deploying, testing, and generating IDLs for your Solana programs.

```Shell
# Build the program
anchor build

# Deploy the program
anchor deploy

# Run tests
anchor test

# Generate IDL
anchor idl generate

```

---

### Specify Test Files to Run

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows users to specify which test files should be executed by the `anchor test` command. This provides more control over the testing process, enabling targeted test runs.

```bash
anchor test tests/my_specific_test.ts

```

---

### Pulling New Docker Image (Shell)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Provides the command to pull the latest Docker image for building Anchor projects, specifying the new image name `backpackapp/build` and the version tag `v0.29.0`.

```Shell
docker pull backpackapp/build:v0.29.0
```

---

### Remove Deprecated `create_metadata_account_v2` Wrapper

Source: https://www.anchor-lang.com/docs/updates/changelog

The deprecated `metadata::create_metadata_account_v2` wrapper has been removed from `anchor-spl`. This aligns with the removal of the function from the underlying Token Metadata program.

```rust
// Usage of the removed wrapper would now result in a compile-time error.
// Previously:
// use anchor_spl::token_metadata::create_metadata_account_v2;
// create_metadata_account_v2(...);
```

---

### Using solana-test-validator for RPC Testing

Source: https://www.anchor-lang.com/docs/testing/litesvm

This snippet illustrates the scenario where `solana-test-validator` is necessary due to `litesvm`'s limitations in supporting certain RPC methods or when testing behavior dependent on real validator interactions.

```bash
# Example command to start solana-test-validator
solana-test-validator
```

---

### Anchor.toml: Scripts Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Defines scripts that can be executed using `anchor run <script>`. The `test` script is automatically executed by `anchor test` for running unit tests.

```toml
[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

```

---

### CLI: Custom Cluster Configuration

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows users to specify custom cluster configurations in the Anchor CLI, enabling connections to non-standard Solana clusters.

```bash
# Example: Specify a custom cluster configuration file
anchor build --cluster-config /path/to/custom-cluster.json

```

---

### Anchor: Zero Copy Deserialization

Source: https://www.anchor-lang.com/docs/features

Details how to utilize Anchor's zero-copy deserialization feature for efficient handling of large account data in Solana programs. This avoids unnecessary data copying, improving performance.

```Rust
use anchor_lang::prelude::*;
use anchor_lang::zero_copy::ZeroCopy;

#[account]
#[derive(ZeroCopy)]
pub struct LargeAccountData {
    pub data: [u8; 1024],
}

#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(zero_copy)]
    pub large_account: Account<'info, LargeAccountData>,
}
```

---

### Mint Freeze Authority Keyword (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `mint::freeze_authority` keyword for specifying the freeze authority during mint initialization within `#[derive(Accounts)]`. This simplifies mint configuration.

```rust
#[account(init, payer = payer, space = 8, mint::freeze_authority = freeze_authority)]
struct MyMint {...}
```

---

### SystemAccount Type (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `SystemAccount<'info>` type for generic wallet addresses or accounts owned by the system program. This simplifies handling of system-level accounts.

```rust
#[account(address = SYSTEM_PROGRAM_ID)]
struct SystemAccount<'info> {...}
```

---

### Target/Types Directory Creation (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

The `target/types` directory is now created during the build process to store TypeScript type files for each program's IDL. This automates the generation of type definitions for easier integration.

```bash
anchor build
```

---

### Override Rustup Toolchain for IDL Build

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Environment variable setting to specify a particular Rust toolchain (e.g., a specific nightly version) for Anchor's IDL build process. This helps manage potential compatibility issues with different Rust compiler versions.

```bash
RUSTUP_TOOLCHAIN="nightly-2024-05-09" anchor idl build
```

---

### Add return values to Anchor Lang CPI client

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor Lang CPI (Cross-Program Invocation) client now supports return values, allowing programs to receive data from invoked programs.

```rust
// Add return values to CPI client (#1598).
```

---

### Derive Associated Token Account Address (Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/create-token-account

Calculates the deterministic address and bump seed for an associated token account. This function takes the owner's wallet address, the token mint address, and program IDs as input.

```rust
pub fn get_associated_token_address_and_bump_seed_internal(
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    program_id: &Pubkey,
    token_program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &wallet_address.to_bytes(), // Owner's public key
            &token_program_id.to_bytes(), // Token Program or Token Extension Program
            &token_mint_address.to_bytes(), // Token mint address
        ],
        program_id, // Associated Token Program ID
    )
}
```

---

### Anchor CLI: Version Compatibility Check

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now checks for compatibility between the `anchor-lang` crate version and the CLI version. This helps prevent issues arising from using mismatched versions of the Anchor ecosystem.

```bash
# The check is performed automatically during CLI operations.
# If incompatible versions are detected, an error message will be displayed.
anchor --version

```

---

### Anchor Lang: Account Reference to AccountInfo

Source: https://www.anchor-lang.com/docs/updates/changelog

All accounts in Anchor Lang now have a reference to `AccountInfo`, providing a consistent way to access account data and metadata.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MyAccount<'info> {
    #[account(mut)]
    pub my_account: Account<'info, SomeData>,
}

#[account]
pub struct SomeData {
    // ... data fields
}
```

---

### Add aliases for anchor build and test commands

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces shorthand aliases `b` for `anchor build` and `t` for `anchor test`, streamlining command-line usage.

```bash
`b` and `t` aliases for `build` and `test` respectively (#1823).
```

---

### Anchor Program with IDL Generation (Rust)

Source: https://www.anchor-lang.com/docs/features/declare-program

This Rust code defines an Anchor program with `initialize` and `increment` functions. It includes the necessary structs for accounts and the `Counter` data model. The `declare_id!()` macro specifies the program's unique identifier, and the `#[program]` attribute marks the module containing the program's logic. This program generates an IDL file (e.g., `example.json`) which is crucial for off-chain clients.

```rust
use anchor_lang::prelude::*;
declare_id!("6khKp4BeJpCjBY1Eh39ybiqbfRnrn2UzWeUARjQLXYRC");
#[program]
pub mod example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<() ак>
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<() ак>
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count += 1;
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
#[account]
pub struct Counter {
    pub count: u64,
}
```

---

### Upload Accounts for Test Validator

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Specifies accounts to upload from local `.json` files to the test cluster. Each entry requires an account address and the filename of the JSON data.

```toml
[[test.validator.account]]
address = "Ev8WSPQsGb4wfjybqff5eZNcS3n6HaMsBkMk9suAiuM"
filename = "some_account.json"
[[test.validator.account]]
address = "Ev8WSPQsGb4wfjybqff5eZNcS3n6HaMsBkMk9suAiuM"
filename = "some_other_account.json"
```

---

### Process and Validate Solana Instruction with Mollusk (Rust)

Source: https://www.anchor-lang.com/docs/testing/mollusk

Demonstrates how to use Mollusk to process a Solana instruction and validate its execution results. Mollusk provides a fast testing environment by directly executing program ELFs using the BPF Loader within a minified SVM. It requires explicit account lists as it does not interact with AccountsDB.

```Rust
mollusk.process_and_validate_instruction(
    &instruction,   // <-- Instruction to test
    &accounts,      // <-- Account states
    &checks,        // <-- Checks to run on the instruction result
);
```

---

### Anchor build adds docs to IDL

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor build` command now automatically includes documentation in the generated IDL. This behavior can be disabled using the `--no-docs` flag.

```bash
`anchor build` now adds docs to idl. This can be turned off with `--no-docs` (#1561).
```

---

### Export spl-associated-token-account Crate (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Makes the `spl-associated-token-account` crate publicly available for use.

```rust
spl: Export `spl-associated-token-account` crate (#2999).
```

---

### Enable IDL Build Feature in Cargo.toml

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Configuration snippet for `Cargo.toml` to enable the `idl-build` feature, which is now required for Anchor's IDL generation to function correctly. This feature must be enabled for the main `anchor-lang` crate and any other crates used for IDL type definitions.

```plaintext
[features]
idl-build = ["anchor-lang/idl-build"]
```

---

### Anchor Client: Compile with Solana 1.14

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor client library has been updated to compile successfully with Solana toolchain version `1.14`.

```rust
// Ensure your project's Solana dependencies are compatible with 1.14
// For example, in Cargo.toml:
solana-sdk = "1.14.x"

```

---

### Preload Instructions for State RPC Transactions

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows preloading instructions for state RPC transactions in TypeScript. This can optimize transaction construction by preparing instructions in advance.

```ts
const instructions = await program.state.instruction.initialize(...);
const transaction = new Transaction().add(...instructions);

```

---

### Using `declare_program!()` Macro (Rust)

Source: https://www.anchor-lang.com/docs/features/declare-program

This snippet shows the basic usage of the `declare_program!()` macro in an Anchor client. It takes the program's name (derived from its IDL file) as an argument, which allows the client to generate a module for interacting with that specific program. The macro automatically generates types for accounts, arguments, and client-side functions based on the IDL.

```rust
declare_program!(example);  // Looks for /idls/example.json
```

---

### Process and Validate Instruction with Mollusk SVM Checks

Source: https://www.anchor-lang.com/docs/testing/mollusk

Illustrates processing an instruction and validating the result using `process_and_validate_instruction` in Mollusk SVM. It includes setting up a system transfer instruction, accounts, and a series of validation checks using the `Check` enum.

```rust
use {
    mollusk_svm::{Mollusk, result::Check},
    solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey
        system_instruction,
        system_program,
    },
};
let sender = Pubkey::new_unique();
let recipient = Pubkey::new_unique();
let base_lamports = 100_000_000u64;
let transfer_amount = 42_000u64;
let instruction = system_instruction::transfer(&sender, &recipient, transfer_amount);
let accounts = [
    (
        sender,
        Account::new(base_lamports, 0, &system_program::id()),
    ),
    (
        recipient,
        Account::new(base_lamports, 0, &system_program::id()),
    ),
];
let checks = vec![
    Check::success(),
    Check::compute_units(system_processor::DEFAULT_COMPUTE_UNITS),
    Check::account(&sender)
        .lamports(base_lamports - transfer_amount)
        .build(),
    Check::account(&recipient)
        .lamports(base_lamports + transfer_amount)
        .build(),
];
Mollusk::default().process_and_validate_instruction(
    &instruction,
    &accounts,
    &checks,
);
```

---

### Add anchor-spl Dependency

Source: https://www.anchor-lang.com/docs/tokens

This snippet shows how to add the `anchor-spl` crate as a dependency to your Rust project using the `cargo add` command. This is a necessary step for interacting with Solana's Token Programs from an Anchor program.

```Shell
cargo add anchor-spl
```

---

### Transfer Tokens with Anchor (Rust)

Source: https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens

Demonstrates how to transfer tokens using the `transfer_checked` instruction from within an Anchor program in Rust. This involves making a cross-program invocation to the Token Program.

```rust
use anchor_lang::prelude::*
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked, transfer_checked};

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut, address = authority.key())]
    pub authority: Signer<'info>,
    #[account(mut, token::mint = mint.key(), token::authority = authority.key())]
    pub from: TokenAccount<'info>,
    #[account(token::mint = mint.key())]
    pub to: TokenAccount<'info>,
    pub mint: Mint<'info>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferTokens>, amount: u64, decimals: u8) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer_checked(cpi_ctx, amount, decimals)
}
```

---

### Anchor Rust: MintTokens Account Definition

Source: https://www.anchor-lang.com/docs/tokens/basics/mint-tokens

Defines the `MintTokens` struct, which specifies the accounts required for the minting instruction. This includes the signer, mint account, destination token account, and the token program, all with appropriate constraints.

```rust
#[derive(Accounts)]
pub struct MintTokens<'info> {
    // The mint authority
    #[account(mut)]
    pub signer: Signer<'info>,
    // The mint account
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    // The destination token account
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    // The token program
    pub token_program: Interface<'info, TokenInterface>,
}
```

---

### Anchor TS: Load Workspace Programs On-Demand

Source: https://www.anchor-lang.com/docs/updates/changelog

Workspace programs in the Anchor TypeScript SDK are now loaded on-demand instead of loading all of them at once, improving startup performance.

```typescript
import * as anchor from "@coral-xyz/anchor";

// Program is loaded only when accessed
const myProgram = anchor.workspace.MyProgram;

// This access triggers the loading of MyProgram if not already loaded.
```

---

### Anchor Test Command Program Address

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows program address configuration for the test command through `clusters.localnet`. This provides flexibility in setting up local test environments.

```rust
cli: Allow program address configuration for test command through `clusters.localnet` (#554).
```

---

### Anchor: Declare Program Macro for CPIs and Client Transactions

Source: https://www.anchor-lang.com/docs/features/declare-program

Demonstrates using the `declare_program!()` macro to interact with Anchor programs. It generates Rust modules for cross-program invocations (CPIs) and building client-side transactions, simplifying interactions for both on-chain and off-chain code.

```rust
/// Macro to declare a program and generate modules for interaction.
/// This simplifies Cross-Program Invocations (CPIs) and client-side transaction building.
#[macro_export]
macro_rules! declare_program {
    ( $name:ident, $id:expr ) => {
        pub mod $name {
            pub mod cpi {
                // Helper functions for making cross-program invocations (CPIs)
            }
            pub mod client {
                // Accounts and arguments for client-side transactions
            }
            pub mod account {
                // Account data types (program state)
            }
            pub mod program {
                // Program ID constant
                pub const ID: ::anchor_lang::prelude::Pubkey = $id;
            }
            pub mod constants {
                // Program constants
            }
            pub mod events {
                // Program events
            }
            pub mod types {
                // Program types
            }
            pub mod errors {
                // Program errors
            }
        }
    };
}
```

---

### Add `async_rpc` Method to Solana RPC Client

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `async_rpc` method, which returns a non-blocking Solana RPC client. This enables asynchronous interaction with the Solana cluster, improving client application responsiveness.

```rust
use anchor_client::anchor_rpc::Client as AnchorRpcClient;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcConfig;

// Assuming 'provider' is an instance of AnchorProvider
// let rpc_client: RpcClient = provider.async_rpc();
```

---

### Anchor Solana Toolchain Update

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Solana toolchain to v1.7.11 for `cli`, `client`, and `lang`. This ensures compatibility with the latest Solana features and improvements.

```rust
cli, client, lang: Update solana toolchain to v1.7.11 (#653).
```

---

### Anchor CLI: Build and Test Specific Program

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the ability to build and test only a specified program within a multi-program workspace. This speeds up development cycles by focusing on the program currently being worked on.

```bash
anchor build --program <program_name>
anchor test --program <program_name>
```

---

### Configure Solana Test Validator

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Sets options for the `solana-test-validator` CLI, used in Anchor's `anchor test` command. Options control cluster URL, warp slot, RPC ports, ledger size, ledger location, gossip ports, faucet SOL, faucet port, dynamic port range, and bind address.

```toml
[test.validator]
url = "https://api.mainnet-beta.solana.com"
warp_slot = 1337
slots_per_epoch = 5
rpc_port = 1337
limit_ledger_size = 1337
ledger = "test-ledger"
gossip_port = 1337
gossip_host = "127.0.0.1"
faucet_sol = 1337
faucet_port = 1337
dynamic_port_range = "1337 - 1337"
bind_address = "0.0.0.0"
```

---

### Anchor Launch Command Removal

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the `anchor launch` command. Users should now use alternative deployment methods.

```rust
cli: Removed `anchor launch` command (#634).
```

---

### Anchor Lang: System Program CPI Wrappers

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang introduces system program CPI (Cross-Program Invocation) wrapper functions. The `system_program` module is now public, providing a more structured way to interact with the System Program.

```rust
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

#[program]
mod my_program {
    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()>
}
```

---

### Specify Multiple With Targets for Associated Accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows specifying multiple `with` targets when creating associated accounts. This enhances flexibility in managing associated token accounts or other program-derived addresses.

```rust
#[account(init, associated = token_program, payer = payer, owner = owner, with = mint)]
struct MyAccount<'info> {
    // ...
}

```

---

### Anchor Rent Sysvar Dynamic Fetch

Source: https://www.anchor-lang.com/docs/updates/changelog

Dynamically fetches the rent sysvar when using the `init` instruction. This ensures correct rent calculation for newly initialized accounts.

```rust
lang: Dynamically fetch rent sysvar for when using `init` (#587).
```

---

### CPI Clients for State Instructions

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides Cross-Program Invocation (CPI) clients specifically for program state instructions. This allows one program to call state-related instructions in another program.

```rust
use anchor_lang::solana_program::program;

// Assuming a state instruction exists in another program
// program::invoke_signed(...)

```

---

### Anchor cli: Build single program option

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `--program-name` option to the build command, allowing developers to build a single program within a workspace.

```cli
# cli: Add `--program-name` option for build command to build a single program at a time (#362).
```

---

### Anchor CLI/Client/Lang/SPL: Solana Toolchain Update

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor components have been updated to support Solana toolchain version `1.17.0`. Version `1.16` remains supported.

```bash
# Ensure your Solana toolchain is updated:
solana --version
# Expected output should indicate 1.17.0 or compatible
```

---

### Anchor CLI: User Specific Wallet Path

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes an issue where the `provider.wallet` path specified in `Anchor.toml` was not correctly used. This ensures that custom wallet configurations are respected by the CLI.

```toml
[programs.localnet]
my_program = { address = "...", wallet = "~/.config/solana/id.json" }
```

---

### Anchor Keys Sync

Source: https://www.anchor-lang.com/docs/references/cli

Synchronizes program public keys declared via `declare_id!` in the Rust code with the actual public keys of the deployed programs. This ensures consistency between your code and on-chain deployments.

```bash
anchor keys sync
```

---

### Anchor CLI: Fetch IDL Outside Workspace

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor idl fetch` command now supports operations outside of an Anchor workspace. This enhances flexibility when interacting with deployed programs.

```bash
anchor idl fetch
```

---

### Update Anchor.toml Provider Table

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: The `wallet` and `cluster` settings in Anchor.toml must now be placed under the `[provider]` table. This change standardizes configuration structure.

```toml
[provider]
wallet = "..."
cluster = "..."

```

---

### Testing Anchor Programs with Mollusk

Source: https://www.anchor-lang.com/docs/testing

Mollusk is a testing framework specifically for writing tests for Solana programs in Rust. It integrates with Anchor for a seamless testing experience.

```Rust
/* Example Rust test using Mollusk */
// import necessary Mollusk modules
// write test functions interacting with Anchor program
```

---

### Anchor IDL Proc-Macro2 Usage Update

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses an issue in the Anchor IDL (Interface Description Language) by updating its usage of the `proc-macro2` crate to be compatible with the latest nightly Rust compiler, as noted in version 0.31.1.

```text
idl: Update `proc-macro2` usage for latest nightly (#3663)
```

---

### Support Legacy IDLs with declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for legacy Interface Description Languages (IDLs) when using the `declare_program!` macro.

```rust
lang: Support legacy IDLs with `declare_program!` (#2997).
```

---

### Anchor Context Struct Definition

Source: https://www.anchor-lang.com/docs/basics/program-structure

Defines the generic Context struct used in Anchor Lang instructions. It provides access to program ID, accounts, remaining accounts, and bump seeds.

```rust
pub struct Context<'a, 'b, 'c, 'info, T: Bumps> {
    /// Currently executing program id.
    pub program_id: &'a Pubkey,
    /// Deserialized accounts.
    pub accounts: &'b mut T,
    /// Remaining accounts given but not deserialized or validated.
    /// Be very careful when using this directly.
    pub remaining_accounts: &'c [AccountInfo<'info>],
    /// Bump seeds found during constraint validation. This is provided as a
    /// convenience so that handlers don't have to recalculate bump seeds or
    /// pass them in as arguments.
    /// Type is the bumps struct generated by #[derive(Accounts)]
    pub bumps: T::Bumps,
}
```

---

### Set Package Manager for Anchor

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Configures the JavaScript package manager (e.g., `npm`, `yarn`, `pnpm`, `bun`) that Anchor will use for client and workspace commands. Defaults to `yarn` if not specified. Values should be lowercase.

```toml
[toolchain]
package_manager = "pnpm"
```

---

### Anchor CLI: Wildcard Patterns in Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for simple wildcard patterns in the `workspace.members` and `workspace.exclude` sections of the `Anchor.toml` configuration file. This simplifies managing multiple programs within a workspace.

```toml
[workspace]
members = [
    "programs/*",
    "other_programs/specific_program",
]
exclude = [
    "tests/fixtures/*",
]

```

---

### Version Verifiable Docker Builder

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces version verification for the Docker builder. This ensures that the correct and intended Docker image is used for building, enhancing reproducibility and security.

```bash
anchor build --docker-image <image_name>:<tag>

```

---

### Rust - Use AccountLoader for Zero Copy

Source: https://www.anchor-lang.com/docs/features/zero-copy

Demonstrates how to use `AccountLoader<'info, T>` to deserialize a zero-copy account, where `T` is the struct annotated with `#[account(zero_copy)]`. This is used within the `Accounts` struct for program instructions.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {


    pub zero_copy_account:AccountLoader<'info, Data>,
}
```

---

### Fix Using Optional Accounts with declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses an issue concerning the use of optional accounts with the `declare_program!` macro.

```rust
lang: Fix using optional accounts with `declare_program!` (#2967).
```

---

### Invoke Anchor Program's SOL Transfer CPI with TypeScript

Source: https://www.anchor-lang.com/docs/basics/cpi

This TypeScript code snippet shows how to invoke the `sol_transfer` instruction of the Anchor program. It specifies the PDA as the `pdaAccount` and the wallet's public key as the `recipient`, initiating a CPI for the SOL transfer.

```typescript
it("SOL Transfer with PDA signer", async () => {
  const transactionSignature = await program.methods

    .solTransfer(new BN(transferAmount))

    .accounts({
      pdaAccount: PDA,

      recipient: wallet.publicKey,
    })

    .rpc();

  console.log(
    `\nTransaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
  );
});
```

---

### Anchor Lang: InitIfNedded Rent Check (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that `init_if_needed` now checks for rent exemption when initialization is not required. This prevents potential issues with accounts not meeting rent requirements.

```Rust
lang: `init_if_needed` now checks rent exemption when init is not needed (#1250).
```

---

### Anchor CLI: Warn if anchor-spl/idl-build is Missing

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now issues a warning if the `anchor-spl/idl-build` feature is missing when it's expected, helping to prevent build-related issues.

```bash
# Example of a scenario that might trigger the warning
```

---

### Update Solana Version

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Command to update the Solana CLI to a supported version, specifically 1.17.0, to resolve compatibility issues.

```bash
solana-install init 1.17.0
```

---

### Sync Anchor Program ID

Source: https://www.anchor-lang.com/docs/basics/program-structure

Synchronizes the program ID in the `declare_id` macro with the public key of the keypair generated locally. This command is useful when working with cloned repositories to ensure the program ID matches the locally built version.

```bash
anchor keys sync
```

---

### Anchor lang: Generic support for Accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds generic support to the `Accounts` struct, enabling more flexible and reusable account definitions.

```rust
/// lang: Add generic support to Accounts (#496).
```

---

### Anchor Client: Public Solana Account Decoder Dependency

Source: https://www.anchor-lang.com/docs/updates/changelog

This feature makes the `solana_account_decoder` dependency public in the Anchor client, allowing for greater flexibility and integration with Solana's account decoding mechanisms.

```rust
pub use solana_account_decoder;
```

---

### Sync Program IDs on Initial Build (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures program IDs are synchronized during the initial build process.

```bash
cli: Sync program ids on the initial build (#3023).
```

---

### Rust - Zero Copy Account Traits

Source: https://www.anchor-lang.com/docs/features/zero-copy

Shows the necessary traits (`Copy`, `Clone`, `bytemuck::Zeroable`, `bytemuck::Pod`, `repr(C)`) that are automatically implemented by the `#[account(zero_copy)]` attribute for zero-copy deserialization.

```rust
#[derive(Copy, Clone)]
#[derive(bytemuck::Zeroable)]
#[derive(bytemuck::Pod)]
#[repr(C)]
struct Data {
  // --snip--
}
```

---

### Anchor SPL: MPL Token Metadata Export

Source: https://www.anchor-lang.com/docs/updates/changelog

The `mpl-token-metadata` crate is now exported by the Anchor SPL library, making it easier to integrate with the Metaplex token metadata program.

```rust
use anchor_spl::token_metadata::mpl_token_metadata;

// Now you can use types and functions from mpl_token_metadata directly
// For example:
// let metadata_program = AccountInfo::new(...);
// let metadata_account = mpl_token_metadata::accounts::Metadata::new(...);

```

---

### Anchor Expand Command (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `anchor expand` command, which acts as a wrapper around `cargo expand`. This utility helps in debugging and understanding macro expansions within Anchor projects.

```bash
anchor expand
```

---

### Anchor Program for PDA SOL Transfer

Source: https://www.anchor-lang.com/docs/basics/cpi

This Rust code defines an Anchor program with a `sol_transfer` instruction. It facilitates a CPI to the System Program to transfer SOL from a PDA, which is signed for by the program itself using provided seeds.

```rust
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("3455LkCS85a4aYmSeNbRrJsduNQfYRY82A7eCD3yQfyR");
#[program]
pub mod cpi {
    use super::*;


    pub fnsol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();
        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];
        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        )
        .with_signer(signer_seeds);
        transfer(cpi_context, amount)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(
        mut,
        seeds = [b"pda", recipient.key().as_ref()],
        bump,
    )]
    pda_account: SystemAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
```

---

### Allow passing arguments to scripts with anchor run

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor run` command now supports passing arguments to the underlying scripts it executes, offering more flexibility in script execution.

```bash
`anchor run` (#1914).
```

---

### Add compilation optimizations to Anchor CLI template

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI template now includes compilation optimizations, potentially improving build times and performance for generated programs.

```bash
`anchor init` template (#1807).
```

---

### Read Zero Copy Account Data

Source: https://www.anchor-lang.com/docs/features/zero-copy

Demonstrates how to read data from a zero-copy account using the `load` method. This is useful for read-only operations where you only need to access the account's data without modifying it. It requires defining the account structure with `AccountLoader`.

```rust
use anchor_lang::prelude::*

#[derive(Accounts)]
pub struct ReadOnly<'info> {
    pub data_account: AccountLoader<'info, Data>,
}

pub fn read_only(ctx: Context<ReadOnly>) -> Result<()> {
    let account = &ctx.accounts.data_account.load()?;
    msg!("First 10 bytes: {:?}", &account.data[..10]);
    Ok(())
}
```

---

### Add ProgramDataAddress to Program Account (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `programdata_address` field to the `Program` account, which will be populated if the account is owned by the upgradable BPF loader. This enhances program management capabilities.

```rust
pub programdata_address: Option<Pubkey>
```

---

### Create and Mint Tokens with PDA Authority

Source: https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens

This function initializes a new token mint and a token account owned by a PDA. It then mints tokens to this account. The PDA is derived using the 'mint' seed, and its authority is used for minting.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface};

declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");

#[program]
pub mod token_example {
    use super::*;
    pub fn create_and_mint_tokens(ctx: Context<CreateAndMintTokens>, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint]]];
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
        token_interface::mint_to(cpi_context, amount)?;
        Ok(())
    }
    // ... other functions
}

#[derive(Accounts)]
pub struct CreateAndMintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint,
        mint::freeze_authority = mint,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = token_account,
        seeds = [b"token"],
        bump
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
```

---

### Expand Macros

Source: https://www.anchor-lang.com/docs/references/cli

Expands the macros for a specific program or the entire workspace. This command can be run within a program's directory or the workspace root, with an option to specify a program name.

```bash
anchor expand
```

---

### Anchor Try From Init Removal

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes `try_from_init` from `Loader`, `ProgramAccount`, and `CpiAccount`, replacing it with `try_from_unchecked`. This simplifies account deserialization.

```rust
lang: `try_from_init` has been removed from `Loader`, `ProgramAccount`, and `CpiAccount` and replaced with `try_from_unchecked` (#641).
```

---

### Rust: SPL freeze_delegated_account and thaw_delegated_account Wrappers

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces wrapper functions for `freeze_delegated_account` and `thaw_delegated_account` in the Rust SPL module.

```rust
use anchor_spl::token_interface::{freeze_delegated_account, thaw_delegated_account};

// Use these wrappers to manage delegated account states.
```

---

### Docker: Pull Anchor Image

Source: https://www.anchor-lang.com/docs/references/verifiable-builds

This command pulls a specific version of the Anchor Docker image from Docker Hub. These images are used to create reproducible build environments for Solana programs.

```bash
docker pull solanafoundation/anchor:v0.31.1
```

---

### Anchor CPI: Increment Instruction

Source: https://www.anchor-lang.com/docs/features/declare-program

This Rust code shows how to execute the `increment` instruction of another Anchor program via cross-program invocation (CPI). It defines the necessary account struct for the CPI context and utilizes the generated CPI functions.

```rust
#[derive(Accounts)]
pub struct IncrementCpi<'info> {
    // Counter type from accounts module
    #[account(mut)]


    pub counter: Account<'info, Counter>,
    // Example type from program module


    pub example_program: Program<'info, Example>,
}

pub fn increment_cpi(ctx: Context<IncrementCpi>) -> Result<()> {
    // Create CPI context for increment
    let cpi_ctx = CpiContext::new(
        ctx.accounts.example_program.to_account_info(),
        Increment {
            counter: ctx.accounts.counter.to_account_info(),
        },
    );
    // Invoke the increment instruction

    cpi::increment(cpi_ctx)?;
    Ok(())
}

```

---

### Anchor Custom Account Structure (Rust)

Source: https://www.anchor-lang.com/docs/basics/program-structure

Defines a custom account structure 'NewAccount' for Anchor programs. It includes a 'data' field of type u64 and is decorated with the #[account] attribute for Anchor's data handling.

```rust
#[account]
pub struct NewAccount {
    data: u64,
}
```

---

### Clone Accounts for Test Validator

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Specifies accounts to clone from a remote cluster to the test cluster. If an account is a program owned by the 'BPF upgradeable loader', Anchor automatically clones its program data.

```toml
[[test.validator.clone]]
address = "7NL2qWArf2BbEBBH1vTRZCsoNqFATTddH6h8GkVvrLpG"
[[test.validator.clone]]
address = "2RaN5auQwMdg5efgCaVqpETBV8sacWGR8tkK4m9kjo5r"
[[test.validator.clone]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" # implicitly also clones PwDiXFxQsGra4sFFTT8r1QWRMd4vfumiWC1jfWNfdYT
```

---

### Anchor.toml: Features Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Enables specific features for the IDL. The `resolution` feature, when set to `true`, allows for account resolution, with `true` being the default value.

```toml
[features]
resolution = true

```

---

### Account Wrapper Improvements (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Account wrappers for non-Anchor programs no longer require a `serialize` function due to a default implementation. Similarly, `try_deserialize` now delegates to `try_deserialize_unchecked` by default, simplifying wrapper creation.

```rust
impl<'a, T> Account<'a, T> {
    pub fn set_inner(&mut self, inner: T) {
        self.0.set_inner(inner);
    }
}
```

---

### Anchor CLI: --program-id Option for 'idl convert'

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor idl convert` command now supports a `--program-id` option, allowing users to specify the program ID when converting IDLs.

```bash
anchor idl convert --program-id <program_id>
```

---

### Zero Copy Deserialization for Accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables zero-copy deserialization for accounts in the Anchor language. This optimization avoids unnecessary data copying, improving performance for large accounts.

```rust
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct MyAccount {
    pub data: [u8; 100],
}

```

---

### Customize Anchor.toml Clients per Network

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows generated `anchor.workspace` clients to be customized per network using `[cluster.<slug>]` in Anchor.toml. This provides flexibility in configuring clients for different deployment environments.

```toml
[cluster.<slug>]

```

---

### Change from #[repr(packed)] to #[repr(C)] for Zero Copy Accounts (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Switches the representation for zero-copy accounts from `#[repr(packed)]` to `#[repr(C)]`. This change ensures more predictable memory layout and compatibility.

```rust
#[repr(C)]
struct MyAccount {...}
```

---

### TypeScript: Enhanced Transaction Confirmation

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Enhances transaction confirmation by allowing 'Provider.sendAndConfirm' to accept an optional 'blockhash'. This provides better control and reliability for transaction confirmations.

```TypeScript
await program.provider.sendAndConfirm(tx, signers, { blockhash: ... });
```

---

### AccountLoader for Zero Copy Accounts (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `AccountLoader` type for zero-copy accounts, including support for Cross-Program Invocations (CPI). This provides a robust way to load and interact with zero-copy accounts.

```rust
let loader = AccountLoader::<MyZeroCopyAccount>::new(account_info)?;
let data = loader.load()?;
```

---

### Namespace Account Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows namespacing account discriminators in the Anchor language. This helps prevent collisions when multiple programs use similar account structures.

```rust
#[account(discriminator = "my_program::MyAccount")]
struct MyAccount {
    // ...
}

```

---

### Anchor AVM: Windows Support for Renaming Binary

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Version Manager (AVM) now supports renaming the Anchor binary on Windows, improving cross-platform compatibility.

```bash
# This is a background change in AVM's installation logic on Windows.
```

---

### Anchor cli: Specify workspace programs in Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows specifying workspace programs using `{ address = <base58-str>, idl = <filepath-str> }` within the `[clusters.<network>]` table in `Anchor.toml`.

```cli
# cli: `[clusters.<network>]` table entries can now also use `{ address = <base58-str>, idl = <filepath-str> }` to specify workspace programs (#400).
```

---

### Anchor Program Instruction Module

Source: https://www.anchor-lang.com/docs/basics/program-structure

Defines the module containing all instruction handlers for an Anchor program using the `#[program]` attribute. Each public function within this module represents an invokable instruction.

```Rust
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct NewAccount {
    data: u64,
}
```

---

### Anchor Program Account Definition

Source: https://www.anchor-lang.com/docs/basics/idl

This Rust code defines an Anchor program and a 'NewAccount' struct annotated with #[account]. This struct represents data stored on accounts created by the program, featuring a single 'data' field of type u64.

```rust
use anchor_lang::prelude::*;
declare_id!("BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd");
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct NewAccount {
    data: u64,
}
```

---

### Anchor PDA Constraints: Single Static Seed

Source: https://www.anchor-lang.com/docs/basics/pda

Shows how to define a PDA using a single static seed. The 'seeds' constraint is populated with a byte array representing the static seed.

```rust
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(


        seeds = [b"hello_world"],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

---

### Anchor ts: Fetch API for deserialized accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Requires explicitly invoking the `fetch` API to retrieve deserialized accounts from `<program>.account.<my-account>` and `<program>.state` namespaces.

```typescript
// ts: Retrieving deserialized accounts from the `<program>.account.<my-account>` and `<program>.state` namespaces now require explicitly invoking the `fetch` API. For example, `program.account.myAccount(<address>)` and `program.state()` is now `program.account.myAccount.fetch(<address>)` and `program.state.fetch()` (#322).
```

---

### Anchor cli/client/lang: Update Solana toolchain

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Solana toolchain to v1.7.1 across CLI, client, and language components.

```cli
# cli, client, lang: Update solana toolchain to v1.7.1 (#368).
```

---

### Anchor PDA with Seeds in IDL

Source: https://www.anchor-lang.com/docs/basics/pda

This Rust program defines a PDA using both a static seed and a dynamic seed from a signer account. The PDA constraints are included in the IDL, allowing the Anchor client to automatically resolve the account address.

```rust
use anchor_lang::prelude::*;
declare_id!("BZLiJ62bzRryYp9mRobz47uA66WDgtfTXhhgM25tJyx5");
#[program]
mod hello_anchor {
    use super::*;
    pub fn test_instruction(ctx: Context<InstructionAccounts>) -> Result<() > {
        msg!("PDA: {}", ctx.accounts.pda_account.key());
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {

    pubsigner: Signer<'info>,
    #[account(

        seeds = [b"hello_world", signer.key().as_ref()],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

---

### Add realloc constraints in Anchor Lang

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces new constraint groups `realloc`, `realloc::payer`, and `realloc::zero` for program accounts. Also adds a `&mut reallocs: BTreeSet<Pubkey>` argument to `Accounts::try_accounts` for managing reallocations.

```rust
/// Add `realloc`, `realloc::payer`, and `realloc::zero` as a new constraint group for program accounts (#1986).
/// Adds a new `&mut reallocs: BTreeSet<Pubkey>` argument to `Accounts::try_accounts` (#1986).
```

---

### Anchor-Client Rust Clients

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor-client` crate provides Rust clients for interacting with Anchor programs. It simplifies the process of sending transactions and querying program state from a Rust application.

```Rust
use anchor_client::anchor_gen::idl;
use anchor_client::solana_client::rpc::Client;
use anchor_client::Program;

// let program_id = "...";
// let rpc_url = "...";
// let client = Client::new_rpc(rpc_url.to_string());
// let program = Program::load_with_params(
//     client.get_rpc(),
//     program_id.parse().unwrap(),
//     idl() // Assuming IDL is available
// );
// let ix = program.request() // Build instruction
//     .args(my_program::instruction::Initialize {}) // Pass instruction args
//     .send_with_context(None)?;

```

---

### Anchor IDL: PDA Resolution for Call Expressions Without Arguments

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor IDL now supports PDA resolution for call expressions that do not have any arguments, improving flexibility in defining program-derived addresses.

```rust
#[account(seeds = [b"my_pda"], bump)]
```

---

### Allow Wider Range of Dependency Versions

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables a wider range of dependency versions to be used, reducing potential dependency conflicts and issues. This improves the flexibility and stability of projects using Anchor.

```rust
// Cargo.toml might have updated dependency version constraints
// [dependencies]
// anchor-lang = { version = "0.28.0", features = ["event-cpi"] }
// anchor-spl = "0.28.0"
// solana-sdk = "~1.16.0"
```

---

### CLI: Validator Geyser Plugin Configuration Support

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for `test.validator.geyser_plugin_config` in the Anchor CLI, allowing configuration of Geyser plugins for local validators.

```bash
# Configure Geyser plugin for local validator
anchor localnet --geyser-plugin-config path/to/config.json

```

---

### Anchor Lang: Bump Target Usage (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the usage of `bump = <target>` with `init`. For `init`, `bump` should be used without a target, and the seed accessed via `ctx.bumps.get("<pda-account-name")`. For subsequent seeds, storing the bump on the account is recommended.

```Rust
lang: Providing `bump = <target>` targets with `init` will now error. On `init` only, it is required to use `bump` without a target and access the seed inside function handlers via `ctx.bumps.get("<pda-account-name")`. For subsequent seeds constraints (without init), it is recommended to store the bump on your account and use it as a `bump = <target>` target to minimize compute units used (#1380).
```

---

### Anchor lang: `init` required for `#[account(associated)]`

Source: https://www.anchor-lang.com/docs/updates/changelog

Requires `init` to be provided when using `#[account(associated)]` to create an associated account. If omitted, the address is assumed to exist and a constraint is added.

```rust
/// lang: `#[account(associated)]` now requires `init` to be provided to create an associated account. If not provided, then the address will be assumed to exist, and a constraint will be added to ensure the correctness of the address (#318).
```

---

### Anchor Lang: Optional Account Bumps

Source: https://www.anchor-lang.com/docs/updates/changelog

Bumps for optional accounts are now represented as `Option<u8>` instead of `u8`, providing better type safety and handling for optional account data.

```rust
struct MyAccount {
    #[account(bump)]
    optional_bump: Option<u8>,
}
```

---

### Use Anchor CLI Version

Source: https://www.anchor-lang.com/docs/references/avm

This command sets a specific version of the Anchor CLI as the active version. This setting persists until changed by calling the command again. 'latest' can be used to switch to the most recent version.

```Shell
avm use <version>
```

---

### GetAccountInfo Helper Method (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `getAccountInfo` helper method to the account namespace/client in the TypeScript client. This simplifies fetching account information.

```typescript
const accountInfo = await program.account.myAccount.getAccountInfo(
  accountAddress
);
```

---

### Return proper error on account length mismatch

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes Anchor Lang to return a proper error instead of panicking when an account's length is smaller than the discriminator in functions of `(Account)Loader`.

```rust
// Return proper error instead of panicking if account length is smaller than discriminator in functions of `(Account)Loader` (#1678).
```

---

### Anchor CLI: Default Build Command

Source: https://www.anchor-lang.com/docs/updates/changelog

This change makes `cargo build-sbf` the default build command for the Anchor CLI, simplifying the build process for Solana programs.

```bash
cargo build-sbf
```

---

### Anchor.toml: Workspace Types Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Specifies a directory where the generated `<idl>.ts` file will be copied upon running `anchor build`. This is useful for frontend integration or version control, keeping the IDL file outside the `target` directory.

```toml
[workspace]
types = "app/src/idl/"

```

---

### Anchor CLI: Package Name as Program Name

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now accepts the package name as the program name. This simplifies project configuration, especially when the package name accurately reflects the program's identity.

```bash
# Assuming package.json or Cargo.toml defines the package name
anchor build --program-name <package_name>

```

---

### Update Solana Toolchain and Dependencies

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Solana toolchain and dependencies to version `1.16.0`, with a maximum specified version of `<1.17.0`. This ensures compatibility with the latest Solana features and improvements.

```bash
# Anchor.toml might specify the Solana version constraint
# solana_version = "1.16.0"
```

---

### Anchor SPL: IDL Build Feature

Source: https://www.anchor-lang.com/docs/updates/changelog

The `idl-build` feature is added to `anchor-spl`. This feature must be enabled for the IDL build method to function when using `anchor-spl`.

```rust
# In your program's Cargo.toml:
[dependencies]
anchor-spl = { version = "0.29.0", features = ["idl-build"] }

```

---

### Fix missing account name on deserialization failure

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses an issue where account name information was missing during deserialization failures when using `init` or `zero` in Anchor Lang.

```rust
// Fix missing account name information when deserialization fails when using `init` or `zero` (#1800).
```

---

### Anchor CLI: Warn if Manifest has solana-program Dependency

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now warns if a project's manifest file includes `solana-program` as a direct dependency, as this can sometimes lead to conflicts.

```bash
# Example Cargo.toml snippet that might trigger the warning:
```

---

### Rust: IDL PDA Seed Generation for Consts

Source: https://www.anchor-lang.com/docs/updates/changelog

Enhances the Anchor Rust IDL generation to parse constants from `impl` blocks for PDA seed generation.

```rust
use anchor_lang::prelude::*;

#[account]
pub struct MyAccount {
    pub data: u64,
}

impl MyAccount {
    pub const SEED_PREFIX: &'static [u8] = b"my_account";
}

// The IDL will now correctly generate PDA seeds using MyAccount::SEED_PREFIX.
```

---

### Implement AsRef for Account (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Implements the `AsRef<T>` trait for `Account<'a, T>`. This allows seamless conversion and access to the inner data of an Anchor account.

```rust
impl<'a, T> AsRef<T> for Account<'a, T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
```

---

### Anchor CLI: Skip IDL Generation

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Skips IDL generation during the build process when the program API has not changed, using the `--no-idl` flag. This optimizes build times by avoiding unnecessary IDL creation.

```bash
anchor build --no-idl
```

---

### Anchor Lang: Comparison Macros

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang introduces new comparison macros for enhanced validation within programs. These include `require_neq`, `require_keys_neq`, `require_gt`, and `require_gte` for checking inequalities and greater-than conditions.

```rust
use anchor_lang::prelude::*;

#[program]
mod my_program {
    pub fn my_instruction(ctx: Context<MyInstruction>) {
        require_neq!(ctx.accounts.a, ctx.accounts.b, "Values must not be equal");
        require_keys_neq!(ctx.accounts.p1, ctx.accounts.p2, "Pubkeys must not be equal");
        require_gt!(ctx.accounts.x, 10, "Value must be greater than 10");
        require_gte!(ctx.accounts.y, 5, "Value must be greater than or equal to 5");
    }
}
```

---

### Fix Upgradeable Program Clones (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves an issue with cloning upgradeable programs in the command-line interface.

```bash
cli: Fix upgradeable program clones (#3010).
```

---

### Anchor cli: `[scripts]` section in Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `[scripts]` section to `Anchor.toml` for defining and running workspace scripts using the `anchor run <script>` command.

```cli
# cli: Add `[scripts]` section to the Anchor.toml for specifying workspace scripts that can be run via `anchor run <script>` (#400).
```

---

### Update Solana Toolchain (Rust, CI, CLI, Docs)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Solana toolchain to version 1.8.5. This ensures compatibility and leverages the latest features and improvements from the Solana ecosystem.

```bash
solana toolchain install 1.8.5
```

---

### Type Safe Context Bumps (Rust)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Demonstrates the transition from a `BTreeMap<String, u8>` to a type-safe struct for accessing context bump values in Anchor programs. The new method is more intuitive and performant.

```Rust
let bump = *ctx.bumps.get("my_account").unwrap();
```

```Rust
let bump = ctx.bumps.my_account;
```

---

### Create and Use CPI Program Interfaces

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the ability to create and use Cross-Program Invocation (CPI) program interfaces in the Anchor language. This allows for defining and interacting with other programs in a structured, interface-based manner.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CallOtherProgram<'info> {
    #[account(mut)]
    pub other_program: Program<'info, OtherProgramInterface>,
}

#[program_interface]
pub trait OtherProgramInterface {
    fn some_instruction(&self, data: u64);
}

```

---

### Anchor IDL: Disallow Conflicting Zero Constraint Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor IDL tooling now disallows account discriminators that could conflict with the `zero` constraint, preventing potential ambiguities.

```bash
# Example of an invalid discriminator that would be caught:
```

---

### Fetch Counter Account (JavaScript)

Source: https://www.anchor-lang.com/docs/clients/typescript

Fetches a counter account from the program and logs its current count. Requires the program object and the public key of the counter account.

```javascript
const counterAccount = await program.account.counter.fetch(counter.publicKey);
console.log("Count:", counterAccount.count);
```

---

### Anchor Account Type Replacement

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a new `Account` type to replace the deprecated `ProgramAccount` and `CpiAccount`. The `Account` type offers improved flexibility and is compatible with `Box` for reduced stack usage.

```rust
lang: Add new `Account` type to replace `ProgramAccount` and `CpiAccount`, both of which are deprecated (#686).
lang: `Box` can be used with `Account` types to reduce stack usage (#686).
```

---

### Anchor CLI: Check for idl-build Feature in 'idl build'

Source: https://www.anchor-lang.com/docs/updates/changelog

When using the `anchor idl build` command, the CLI now checks if the `idl-build` feature exists, providing better error handling for missing features.

```bash
anchor idl build
```

---

### Anchor IDL: Disallow All Zero Account Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor IDL tooling now disallows account discriminators that are all zeros, promoting better practice and avoiding potential issues.

```bash
# Example of an invalid discriminator that would be caught:
```

---

### Anchor IDL Parsing

Source: https://www.anchor-lang.com/docs/updates/changelog

IDLs are now parsed from the entire crate, not just specific files. This ensures comprehensive IDL generation.

```rust
lang: IDLs are now parsed from the entire crate (#517).
```

---

### Rust - Define Mint Account Structure

Source: https://www.anchor-lang.com/docs/tokens/basics/create-mint

Defines the Rust struct representing a Mint account in Solana's Token Programs. This structure includes fields for the mint authority, supply, decimals, initialization status, and freeze authority.

```rust
/// Mint data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mint {
  /// Optional authority used to mint new tokens. The mint authority may only
  /// be provided during mint creation. If no mint authority is present
  /// then the mint has a fixed supply and no further tokens may be
  /// minted.
  pub mint_authority: COption<Pubkey>,
  /// Total supply of tokens.
  pub supply: u64,
  /// Number of base 10 digits to the right of the decimal place.
  pub decimals: u8,
  /// Is `true` if this structure has been initialized
  pub is_initialized: bool,
  /// Optional authority to freeze token accounts.
  pub freeze_authority: COption<Pubkey>,
}
```

---

### Rust: SPL sign_metadata and remove_creator_verification Wrappers

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds wrapper functions for `sign_metadata` and `remove_creator_verification` in the Rust SPL module.

```rust
use anchor_spl::token_metadata::{sign_metadata, remove_creator_verification};

// Use these wrappers for signing metadata and managing creator verification.
```

---

### Add optional accounts syntax in Anchor Lang

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for optionally passing accounts using the `Optional<Account<'info, T>>` syntax. This change may affect tools that use the Anchor generated IDL but should not impact existing programs.

```rust
// Add support for optionally passing in accounts using the syntax `Optional<Account<'info, T>>`. Shouldn't affect existing programs but may be a breaking change to tools that use the anchor generated IDL. #2101.
```

---

### Anchor IDL: IdlBuilder Introduction

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `IdlBuilder` in Anchor's IDL tooling. This provides a programmatic way to construct and manipulate IDL definitions.

```rust
use anchor_idl::IdlBuilder;

let mut builder = IdlBuilder::new();
builder.with_name("my_program");
// ... add accounts, instructions, etc.
let idl = builder.build();
```

---

### Anchor-SPL CPI Clients

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor-spl` crate offers clients for performing Cross-Program Invocations (CPI) to Solana Program Library (SPL) programs. This allows Anchor programs to interact with standard SPL functionalities.

```Rust
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_lang::system_program;

// Example usage within an Anchor program
// let cpi_accounts = system_program::Transfer {
//     from: ctx.accounts.user_ata.to_account_info(),
//     to: ctx.accounts.program_ata.to_account_info(),
// };
// let cpi_program = ctx.accounts.token_program.to_account_info();
// let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
// system_program::transfer(cpi_ctx, amount)?;

```

---

### Client Signer Type Change (Client)

Source: https://www.anchor-lang.com/docs/updates/changelog

Modifies `Client::new` and `Client::new_with_options` to accept `Rc<dyn Signer>` instead of `Keypair`. This change provides more flexibility in managing signers for client interactions.

```rust
pub fn new(program: AccountAddress, provider: Provider<C>) -> Self;
pub fn new_with_options(program: AccountAddress, provider: Provider<C>, options: ClientOptions) -> Self;
```

---

### Add Metadata Wrappers for SPL Token Metadata

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds new metadata wrappers for various operations within the SPL Token Metadata program. These wrappers simplify common tasks like managing collection authorities, burning NFTs, and verifying collection items.

```rust
use anchor_spl::token_metadata::mpl_token_metadata::utils::assert_signer;
use anchor_spl::token_metadata::mpl_token_metadata::state::{Collection, Creator, Metadata, TokenMetadataConfig};

// Example: approve_collection_authority wrapper
// pub fn approve_collection_authority(ctx: Context<ApproveCollectionAuthority>, new_collection_authority: Option<Pubkey>) -> Result<()> { ... }
```

---

### Declare Anchor Program

Source: https://www.anchor-lang.com/docs/features/declare-program

The `declare_program!()` macro is used to bring the generated modules for a program's Interface Definition Language (IDL) into scope. It takes the name of the IDL file as an argument, and Anchor looks for it in the `/idls/` directory.

```rust
declare_program!(example);  // Looks for /idls/example.json
```

---

### Anchor TS: Workspace Program Casing

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor TypeScript SDK now supports accessing workspace programs regardless of casing, e.g., `anchor.workspace.myProgram` or `anchor.workspace.MyProgram`.

```typescript
import * as anchor from "@coral-xyz/anchor";

// Assuming 'myProgram' is in your workspace
const program1 = anchor.workspace.myProgram;
const program2 = anchor.workspace.MyProgram;

console.log(program1 === program2); // true
```

---

### Anchor Lang: Logging and Require Macros

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang enhances its logging capabilities by supporting the logging of expected and actual values and public keys. It also introduces `require_eq` and `require_keys_eq` macros, along with a default error code for the `require` macro.

```rust
use anchor_lang::prelude::*;

#[program]
mod my_program {
    pub fn check_values(ctx: Context<MyAccounts>, value: u64, pubkey: Pubkey) -> Result<()>
}
```

---

### AccountNotInitialized Error (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `AccountNotInitialized` error code to distinguish between accounts with the wrong owner and accounts that do not exist. This improves error handling clarity.

```rust
enum ProgramError {
    AccountNotInitialized,
    WrongOwner,
}
```

---

### Anchor Rust: Transfer Tokens CPI Logic

Source: https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens

Implements the logic for transferring tokens via CPI using Anchor. It prepares the `TransferChecked` struct and `CpiContext`, then calls the `token_interface::transfer_checked` function with the specified amount and decimals.

```rust
pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    // Get the number of decimals for this mint
    let decimals = ctx.accounts.mint.decimals;
    // Create the TransferChecked struct with required accounts
    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    // The program being invoked in the CPI
    let cpi_program = ctx.accounts.token_program.to_account_info();
     // Combine the accounts and program into a "CpiContext"
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    // Make CPI to transfer_checked instruction on token program
    token_interface::transfer_checked(cpi_context, amount, decimals)?;
    Ok(())
}
```

---

### Anchor lang: `#[account(close = <destination>)]` constraint

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `#[account(close = <destination>)]` constraint for closing accounts and directing rent exemption lamports to a specified destination account.

```rust
/// lang: Add `#[account(close = <destination>)]` constraint for closing accounts and sending the rent exemption lamports to a specified destination account (#371).
```

---

### InitIfNeede Attribute Validation (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

The `init_if_needed` attribute now validates provided attributes (e.g., space, owner, token::authority) even when initialization is not required. This ensures consistency and correctness.

```rust
#[account(init_if_needed, space = 8, owner = OWNER_PUBKEY, token::authority = AUTHORITY_PUBKEY)]
struct MyAccount {...}
```

---

### Anchor AccountsInit Trait Removal

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the `AccountsInit` trait. Account initialization is now handled through other mechanisms.

```rust
lang: Remove `AccountsInit` trait (#641).
```

---

### Rust: Serum-Dex to Openbook-Dex Change

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the dependency from `serum-dex` to `openbook-dex` in the Rust Anchor SPL module.

```rust
// The Serum DEX program ID has been replaced with OpenBook DEX.
// Ensure your program uses the correct program ID for OpenBook.
```

---

### Anchor CLI: Verifiable Option for Deploy

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a verifiable option to the 'deploy' command in the Anchor CLI. This allows users to specify whether the deployment should be verifiable, which can be important for auditing and trust.

```bash
anchor deploy --verifiable
```

---

### Anchor Bump Constraint

Source: https://www.anchor-lang.com/docs/updates/changelog

Requires the `bump` to be provided when using the `seeds` constraint. This adds a safety measure to ensure PDAs are initialized with the correct bump seed.

```rust
lang: `bump` must be provided when using the `seeds` constraint. This has been added as an extra safety constraint to ensure that whenever a PDA is initialized via a constraint the bump used is the one created by `Pubkey::find_program_address` (#641).
```

---

### Add Tokio Support to RequestBuilder (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `tokio` support to the `RequestBuilder` in the Anchor client library, enabling asynchronous operations with the `async` feature.

```rust
client: Add `tokio` support to `RequestBuilder` with `async` feature (#3057).
```

---

### Add Ability to Convert Legacy IDLs (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides functionality to convert legacy Interface Description Languages (IDLs) to the current format.

```rust
idl: Add ability to convert legacy IDLs (#2986).
```

---

### Anchor Declare ID Requirement

Source: https://www.anchor-lang.com/docs/updates/changelog

All programs must now define their program ID in source using `declare_id!`. This centralizes program ID management and improves clarity.

```rust
lang: All programs must now define their program id in source via `declare_id!` (#686).
```

---

### Anchor TypeScript Clients

Source: https://www.anchor-lang.com/docs/updates/changelog

The `@project-serum/anchor` package is used for generating TypeScript clients for Anchor programs. This allows frontend applications to easily interact with deployed Solana programs.

```TypeScript
import * as anchor from "@project-serum/anchor";
import { Program, AnchorProvider, web3 } from "@project-serum/anchor";
import { MyProgram } from "./my_program"; // Generated client

// const provider = new AnchorProvider(connection, wallet, opts);
// anchor.setProvider(provider);
// const program = anchor.workspace.MyProgram as Program<MyProgram>;

// async function callInitialize() {
//   const tx = await program.methods.initialize().rpc();
//   console.log("Transaction signature", tx);
// }

```

---

### Add #[account(owner = <program>)] Constraint

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `#[account(owner = <program>)]` constraint in the Anchor language. This attribute allows specifying that an account must be owned by a particular program.

```rust
#[account(owner = other_program_id)]
struct MyAccount {
    // ...
}

```

---

### Configure Package Manager in Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

This snippet shows how to configure the default JavaScript package manager for Anchor projects by setting the 'package_manager' field within the '[toolchain]' section of the Anchor.toml file. Supported values include 'npm', 'yarn', and 'pnpm'.

```toml
[toolchain]
package_manager = "npm"
```

---

### Add Event Emission and Subscriptions

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for event emission and subscriptions in the Anchor language, client, and TypeScript. This allows programs to emit events that can be subscribed to by clients.

```rust
use anchor_lang::emit;

#[event]
struct MyEvent { data: u64 }

// In instruction:
emit!(MyEvent { data: 123 });

```

---

### Anchor-Lang Rust eDSL for Solana

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor-lang` crate provides a Rust eDSL (embedded Domain Specific Language) for developing smart contracts on the Solana blockchain. It simplifies common Solana programming tasks.

```Rust
use anchor_lang::prelude::*;

declare_id!("YourProgramId");

#[program]
mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result {}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // other accounts
}

```

---

### TOML - Add bytemuck Dependency

Source: https://www.anchor-lang.com/docs/features/zero-copy

Specifies the `bytemuck` crate with the `min_const_generics` feature in `Cargo.toml` to enable working with arrays of any size in zero-copy types, along with the `anchor-lang` dependency.

```toml
[dependencies]
bytemuck = { version = "1.20.0", features = ["min_const_generics"] }
anchor-lang = "0.31.1"
```

---

### Add #[account(executable)] Attribute

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `#[account(executable)]` attribute in the Anchor language. This attribute marks an account as executable, typically used for program accounts.

```rust
#[account(executable)]
struct MyProgramAccount {
    // ...
}

```

---

### Add Token Program Constraint to SPL Accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables specifying a `token_program` constraint for `Token`, `Mint`, and `AssociatedToken` accounts. This allows overriding the default token program and using different token interface implementations within the same instruction.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(token::token_program = token_program)]
    pub source: Account<'info, TokenAccount>,
    #[account(token::token_program = token_program)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
```

---

### Anchor TOML Workspace Paths

Source: https://www.anchor-lang.com/docs/updates/changelog

Requires `[workspace]` member and exclude arrays to be filepaths relative to the workspace root. This standardizes workspace configuration.

```rust
cli: `[workspace]` member and exclude arrays must now be filepaths relative to the workspace root (#570).
```

---

### IDL Commands with --print-only Option

Source: https://www.anchor-lang.com/docs/updates/changelog

IDL commands like `idl set-buffer`, `idl set-authority`, and `idl close` now support a `--print-only` option. This option prints the transaction in a base64 Borsh compatible format without sending it to the cluster, useful for multisig management.

```bash
# Print transaction for setting IDL buffer without sending
anchor idl set-buffer --print-only

# Print transaction for setting IDL authority without sending
anchor idl set-authority --print-only

# Print transaction for closing IDL account without sending
anchor idl close --print-only
```

---

### Fix Incorrect `metadata.address` Generation

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects the generation of `metadata.address` in the IDL when deploying with a custom keypair. This ensures the IDL accurately reflects the deployed program's metadata address.

```rust
// This fix likely involves internal logic within Anchor's CLI or build process
// related to IDL generation and keypair handling.
```

---

### Add `idl close` Command

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `anchor idl close` command, which allows developers to close a program's IDL account on the Solana cluster. This is useful for managing on-chain IDL storage.

```bash
# Close the IDL account for a deployed program
anchor idl close <program_id>
```

---

### Anchor IDL & TS: Generics Support

Source: https://www.anchor-lang.com/docs/updates/changelog

Generics are now supported in the IDL and the TypeScript client. This enables the creation of more reusable and abstract program components.

```json
{
  "name": "GenericProgram",
  "types": [
    {
      "name": "Pair",
      "generics": ["T", "U"],
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "first", "type": "T" },
          { "name": "second", "type": "U" }
        ]
      }
    }
  ]
}
```

---

### Rust: SPL MetadataAccount Deserialization

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for deserializing `MetadataAccount` in the Rust SPL module.

```rust
use anchor_spl::token_metadata::MetadataAccount;

// Deserialize metadata account data
// let metadata: MetadataAccount = ...;

```

---

### Support Tests with TypeScript

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds first-class support for writing and running tests using TypeScript with the Anchor CLI. This streamlines the testing workflow for TypeScript developers.

```bash
anchor test
# Tests written in .ts files will be executed.

```

---

### Anchor CLI: IDL Upgrade Command

Source: https://www.anchor-lang.com/docs/updates/changelog

The `idl upgrade` command in the Anchor CLI now closes the IDL buffer account, ensuring proper resource management during IDL updates.

```bash
anchor idl upgrade
```

---

### Override IDL Build Toolchain with RUSTUP_TOOLCHAIN (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables overriding the IDL build toolchain by utilizing the `RUSTUP_TOOLCHAIN` environment variable.

```rust
idl: Allow overriding the idl build toolchain with the `RUSTUP_TOOLCHAIN` environment variable (#2941).
```

---

### Update Solana Toolchain to v1.8.0 (Rust, CLI, SPL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Solana toolchain to version 1.8.0 across Rust, CLI, and SPL components. This ensures consistency and leverages the latest Solana platform features.

```bash
solana toolchain install 1.8.0
```

---

### Anchor Config Keys

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds `members` and `exclude` keys to the config `programs` section. This allows finer control over program inclusion in the workspace.

```rust
cli: Add keys `members` / `exclude` in config `programs` section (#546).
```

---

### Anchor CLI: --no-idl Option for Test Command

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `--no-idl` option for the `anchor test` command. This allows users to skip IDL generation and testing, potentially speeding up the test execution.

```bash
anchor test --no-idl
```

---

### Anchor lang: Specify arbitrary programs as PDA owner

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows specifying arbitrary programs as the owner when creating Program Derived Addresses (PDAs). This provides flexibility in managing account ownership.

```rust
/// Allow one to specify arbitrary programs as the owner when creating PDA (#483).
```

---

### Anchor Client: ThreadSafeSigner Trait Public

Source: https://www.anchor-lang.com/docs/updates/changelog

The `ThreadSafeSigner` trait in the Anchor client library has been made public, allowing developers to implement and use it more broadly for secure transaction signing.

```rust
use anchor_client::signer::ThreadSafeSigner;
```

---

### Anchor CLI: `idl close` Optional Parameter

Source: https://www.anchor-lang.com/docs/updates/changelog

The `idl close` command in the Anchor CLI now accepts an optional `--idl-address` parameter. This allows users to specify the address of the IDL account to close, providing more control over IDL management.

```bash
anchor idl close --idl-address <ADDRESS>
# Or without the address, if it can be inferred or is not needed
```

---

### Anchor Lang: Const in InitSpace Macro

Source: https://www.anchor-lang.com/docs/updates/changelog

The `InitSpace` macro in Anchor Lang now supports using `const` values for specifying account space, improving flexibility.

```rust
use anchor_lang::prelude::*;

const MY_ACCOUNT_SPACE: usize = 100;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, space = MY_ACCOUNT_SPACE, payer = payer)]
    pub my_account: Account<'info, MyData>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct MyData {
    // ... data fields
}

```

---

### Anchor Lang: Program ID Check

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang now includes a check to ensure that the declared program ID matches the actual program ID. This helps prevent deployment errors and ensures correct program identification.

```rust
use anchor_lang::prelude::*;

declare_id!("MyProgramID111111111111111111111111");

#[program]
mod my_program {
    pub fn initialize(ctx: Context<Initialize>) -> Result<()>
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, address = MyProgramID)]
    pub program: AccountInfo<'info>,
}
```

---

### Anchor CLI: Support Non-8-Byte Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now supports non-8-byte discriminators, aligning with the language's flexibility in defining custom discriminator lengths.

```bash
anchor build --custom-discriminator-len 16
```

---

### Anchor Build Error: proc_macro2::Span

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-1

Illustrates a common build error in Anchor related to the `source_file` method of `proc_macro2::Span` due to changes in the Rust nightly compiler. This snippet shows the error message that users might encounter.

```Rust
error[E0599]: no method named `source_file` found for struct `proc_macro2::Span` in the current scope
```

---

### Anchor IDL: Disallow Empty Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor IDL tooling now disallows empty discriminators, ensuring that all accounts and events have a defined identifier.

```bash
# Example of an invalid discriminator that would be caught:
```

---

### Anchor TOML Test Command Requirement

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor.toml now requires an explicit `[scripts]` test command. This standardizes how test commands are defined.

```rust
cli: Anchor.toml now requires an explicit `[scripts]` test command (#550).
```

---

### CpiContext Account Struct Requirement (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Requires `CpiContext` accounts to be used with account structs generated in the `crate::cpi::accounts::*` module. These structs contain `AccountInfo` fields, improving CPI safety.

```rust
mod cpi {
    pub mod accounts {
        #[derive(Accounts)]
        pub struct MyCpiAccounts<'a, 'b, T: Owner {
            // ...
        }
    }
}

use anchor_lang::CpiContext;
use crate::cpi::accounts::MyCpiAccounts;

let cpi_accounts = MyCpiAccounts {
    // ...
};
let cpi_ctx = CpiContext::new(program_id, cpi_accounts);
```

---

### Anchor Publish Cargo.lock Handling

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows `Cargo.lock` to exist in workspace subdirectories when publishing. This improves the handling of dependencies in monorepos.

```rust
cli: Allows Cargo.lock to exist in workspace subdirectories when publishing (#593).
```

---

### Anchor CLI: Always Convert IDLs

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now consistently converts IDLs, ensuring a standardized format for program interfaces.

```bash
# This is a background change in the CLI's behavior.
```

---

### Anchor Clean Command

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

The `anchor clean` command is used to clean the build artifacts of an Anchor project. In version 0.31.0, it now also removes the generated `.anchor` directory, similar to `cargo clean` but preserves program keypairs.

```bash
anchor clean
```

---

### Anchor IDL: Store Deployment Addresses

Source: https://www.anchor-lang.com/docs/updates/changelog

The IDL now supports storing deployment addresses for different clusters. This is useful for tracking where programs have been deployed across various networks (e.g., devnet, mainnet).

```json
{
  "name": "MyProgram",
  "metadata": {
    "addresses": {
      "devnet": "<devnet_address>",
      "mainnet": "<mainnet_address>"
    }
  }
}
```

---

### Upgrade SPL Dependencies to Latest (SPL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates all SPL dependencies to their most recent versions.

```rust
spl: Upgrade SPL deps to latest (#3346).
```

---

### Update JS/TS templates to use new program.methods syntax

Source: https://www.anchor-lang.com/docs/updates/changelog

The JavaScript/TypeScript templates generated by Anchor CLI have been updated to adopt the new `program.methods` syntax for defining and interacting with program instructions.

```typescript
// Update js/ts templates to use new `program.methods` syntax (#1732).
```

---

### Anchor IDL: `docs` Field for Constants

Source: https://www.anchor-lang.com/docs/updates/changelog

The IDL specification now includes a `docs` field for constants. This allows developers to add documentation strings directly to constants within the IDL, improving clarity.

```json
{
  "name": "MyConstants",
  "constants": [
    {
      "name": "DEFAULT_AMOUNT",
      "type": "u64",
      "value": "1000",
      "docs": "The default amount for new accounts."
    }
  ]
}
```

---

### Add Async Feature Flag to Anchor Client

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces an `async` feature flag for the Anchor client, enabling asynchronous operations. This enhances performance and responsiveness in client-side interactions with Anchor programs.

```rust
#[cfg(feature = "async")]
use anchor_client::Client; // Example usage
```

---

### Anchor SPL Dex Proxy

Source: https://www.anchor-lang.com/docs/updates/changelog

Includes a proxy for permissioned markets in the SPL (Solana Program Library) module. This facilitates interaction with specific SPL DEX functionalities.

```rust
spl: Dex permissioned markets proxy (#519, #543).
```

---

### Rust PodStateWithExtensions for Token Extensions

Source: https://www.anchor-lang.com/docs/tokens/extensions

Defines the `PodStateWithExtensions` struct in Rust, used to encapsulate immutable base state data for mints or accounts, including potential extensions. The `tlv_data` field holds extension-specific state that requires deserialization based on enabled extension types.

```rust
/// Encapsulates immutable base state data (mint or account) with possible
/// extensions, where the base state is Pod for zero-copy serde.
#[derive(Debug, PartialEq)]
pub struct PodStateWithExtensions<'data, S: BaseState + Pod> {
    /// Unpacked base data
    pub base: &'data S,
    /// Slice of data containing all TLV data, deserialized on demand

    tlv_data: &'data [u8],
}
```

---

### Rust: Define SolTransfer Accounts for CPI

Source: https://www.anchor-lang.com/docs/basics/cpi

Defines the accounts required for a Solana transfer instruction within the Anchor framework. It specifies a PDA account derived using seeds including the recipient's public key, ensuring the program signs for it.

```Rust
#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(
        mut,

        seeds = [b"pda", recipient.key().as_ref()],
        bump,
    )]

pda_account: SystemAccount<'info>,
    #[account(mut)]

recipient: SystemAccount<'info>,

system_program: Program<'info, System>,
}
```

---

### Anchor CLI: Remove Jest Option

Source: https://www.anchor-lang.com/docs/updates/changelog

The `--jest` option has been removed from the `init` command in the Anchor CLI, simplifying project initialization by removing Jest-specific configurations.

```bash
anchor init --no-jest
```

---

### Add --skip-build Flag to Test Command

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `--skip-build` flag for the `anchor test` command. This allows users to bypass the build step when running tests, which can speed up the testing process if the program has already been built.

```bash
anchor test --skip-build

```

---

### Upgrade Solana Toolchain

Source: https://www.anchor-lang.com/docs/updates/changelog

Upgrades the Solana toolchain to version 1.6.6 across Anchor language, SPL, CLI, and client components. This update includes performance improvements and new features from the Solana SDK.

```bash
# Update solana-cli, solana-program-library, etc.
solana --version

```

---

### Move Permissioned Markets (SPL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Relocates permissioned markets into a separate DEX repository. This organizational change aims to streamline development and maintenance of the DEX components.

```git
git mv projects/anchor/spl/permissioned_markets ../dex/permissioned_markets
```

---

### Allow Specifying Multiple Modifier Functions

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables specifying multiple modifier functions for access control within the Anchor language. This allows for more complex authorization logic.

```rust
#[account]
#[access_control(modifier1, modifier2)]
struct MyAccount {
    // ...
}

```

---

### Rust: Integer Conversion Error Propagation

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Illustrates how to propagate integer conversion errors using the `?` operator in Rust. This allows for cleaner error handling when converting between integer types, such as from `u32` to `i32`.

```rust
use anchor_lang::prelude::*;

fn convert_u32_to_i32(value: u32) -> Result<i32> {
    let n: i32 = value.try_into()?;
    Ok(n)
}

// Example usage within an Anchor instruction
#[derive(Accounts)]
pub struct MyAccounts {}

#[derive(Accounts)]
pub struct MyAccounts {}

#[instruction(value: u32)]
fn my_instruction(ctx: Context<MyAccounts>, value: u32) -> Result {
    let _n: i32 = value.try_into()?;
    Ok(())
}
```

---

### Anchor Lang: InstructionData::write_to Implementation

Source: https://www.anchor-lang.com/docs/updates/changelog

Implements the `write_to` method for `InstructionData` in Anchor Lang. This allows for efficient serialization of instruction data, potentially reducing overhead and improving performance.

```rust
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
struct MyInstructionData { /* fields */ }

impl MyInstructionData {
    fn write_to(&self, writer: &mut impl Write) -> Result<()>
}

```

---

### IDL Commands No Longer Hang on Insufficient Funds

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes an issue where IDL commands would hang if the payer account lacked sufficient funds for transaction fees. The commands now handle this scenario gracefully, preventing indefinite hangs.

```bash
# Example command that might have hung previously:
# anchor idl init --filepath idl.json
```

---

### TypeScript: Solana Web3 Library Update

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Solana web3 library used by Anchor's TypeScript client to version 1.64.0.

```typescript
// Ensure your project depends on @solana/web3.js version 1.64.0 or higher.
// npm install @solana/web3.js@^1.64.0
```

---

### Anchor Account Owner Constraint

Source: https://www.anchor-lang.com/docs/updates/changelog

The `#[account(owner = <pubkey>)]` constraint now requires a `Pubkey` instead of an account. This change enforces stricter type checking for account ownership.

```rust
lang: `#[account(owner = <pubkey>)]` now requires a `Pubkey` instead of an account (#691).
```

---

### Anchor Key Trait Optimization

Source: https://www.anchor-lang.com/docs/updates/changelog

Optimizes the `trait Key` by removing `AccountInfo` cloning. This improves performance by avoiding unnecessary data copying.

```rust
lang: Optimize `trait Key` by removing `AccountInfo` cloning (#652).
```

---

### Anchor IDL: New Specification and Features

Source: https://www.anchor-lang.com/docs/updates/changelog

A new IDL specification has been introduced, bringing support for `repr`s, expression evaluation, and the use of external types during IDL generation. This enhances the expressiveness and capabilities of the IDL.

```json
{
  "version": "2",
  "name": "my_program",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "authority",
          "isSigner": true,
          "pda": {
            "seeds": [
              {
                "kind": "context",
                "account": "authority",
                "property": "authority"
              },
              {
                "kind": "literal",
                "value": "authority_seed"
              }
            ],
            "programId": "<program_id>"
          }
        }
      ],
      "args": [
        {
          "name": "initial_value",
          "type": "u64"
        }
      ]
    }
  ],
  "types": [
    {
      "name": "MyStruct",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "value",
            "type": {
              "defined": "u64",
              "repr": "transparent"
            }
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "MAX_VALUE",
      "type": "u64",
      "value": "1000 * 1000"
    }
  ]
}
```

---

### Anchor Program Account Writable Check

Source: https://www.anchor-lang.com/docs/updates/changelog

Includes a check to ensure `ProgramAccount` is writable before a mutable borrow (in `anchor-debug` only). This prevents potential runtime errors related to mutable access.

```rust
lang: Check that ProgramAccount writable before mut borrow (`anchor-debug` only) (#681).
```

---

### Anchor Lang: Account Utility Type for Bytes

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces an `Account` utility type in Anchor's Rust library that allows retrieving accounts from raw bytes. This simplifies deserialization and handling of account data.

```rust
use anchor_lang::Account;
```

---

### Anchor cli/client: Parse custom cluster URLs

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables parsing custom cluster URLs from strings for both the CLI and client components.

```cli
# cli, client: Parse custom cluster urls from str (#369).
```

---

### Anchor Lang: Interface CPI Calls

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang now allows CPI calls matching an interface without pinning the program ID, enhancing flexibility in cross-program interactions.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CallInterface<'info> {
    // AccountInfo for the program implementing the interface
    // No specific program_id needed if matching by interface
    pub target_program: AccountInfo<'info>,
}

fn call_interface_program(ctx: Context<CallInterface>) -> Result<()> {
    // Call a function on the target program via CPI
    // ...
    Ok(())
}
```

---

### Add support for multiple test suites in Anchor Lang

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the capability to run multiple test suites, each with its own isolated local validator, enhancing test organization and execution.

```rust
// Add support for multiple test suites with separate local validators (#1681).
```

---

### TypeScript: Simpler Program Construction

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Simplifies the 'Program' constructor in TypeScript by removing automatic IDL type inference. This allows specifying the actual IDL type directly, avoiding the need for casting to 'unknown' or 'any'.

```TypeScript
import * as idl from "./idl/my_program.json";
import type { MyProgram } from "./types/my_program";
const program = new Program<MyProgram>(idl, provider);
```

---

### Anchor CLI: Add anchor clean command

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `anchor clean` command, which functions similarly to `cargo clean` but preserves keypairs located within the `target/deploy` directory. This is useful for maintaining deployment configurations.

```bash
anchor clean
```

---

### TypeScript CPI Invocation: SOL Transfer

Source: https://www.anchor-lang.com/docs/basics/cpi

This TypeScript code snippet shows how to invoke the `sol_transfer` instruction of an Anchor program. It uses the Anchor client library to send the transaction and logs a link to the transaction details on SolanaFM. It requires the `BN` object for the amount.

```typescript
it("SOL Transfer Anchor", async () => {
  const transactionSignature = await program.methods
    .solTransfer(new BN(transferAmount))
    .accounts({
      sender: sender.publicKey,
      recipient: recipient.publicKey,
    })
    .rpc();
  console.log(
    `\nTransaction Signature:` +
      `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
  );
});
```

---

### Anchor CLI: Rename Seeds to Resolution

Source: https://www.anchor-lang.com/docs/updates/changelog

The `seeds` feature has been renamed to `resolution` and is now enabled by default in the Anchor CLI, improving clarity and usability for account resolution.

```bash
# Feature 'resolution' is now enabled by default
```

---

### Parse Unused Accounts Correctly in IDL

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes an issue where unused `#[account]`s were not being parsed correctly into the Interface Definition Language (IDL). This ensures the IDL accurately reflects all declared accounts.

```rust
// Ensure all #[account] definitions are included in the IDL

```

---

### Accept Integers for warp_slot (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows integer values to be accepted for the `warp_slot` parameter in the command-line interface.

```bash
cli: Accept integers for `warp_slot` (#3235).
```

---

### TypeScript: Provider.simulate Transaction Signature Verification

Source: https://www.anchor-lang.com/docs/updates/changelog

Modifies `provider.simulate` in TypeScript to send transactions with `sigVerify: false` when no signers are present, optimizing simulation calls.

```typescript
import { AnchorProvider } from "@coral-xyz/anchor";

// If no signers are provided, the transaction will be sent with sigVerify: false
const tx = await provider.simulate(ix, [], { sigVerify: false });
```

---

### Fix bytemuckunsafe Account Serialization with declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses an issue with `bytemuckunsafe` account serialization when using the `declare_program!` macro.

```rust
lang: Fix using `bytemuckunsafe` account serialization with `declare_program!` (#3037).
```

---

### Anchor CLI: Pass Cargo Args to IDL Generation

Source: https://www.anchor-lang.com/docs/updates/changelog

This enhancement allows passing `cargo` arguments to the IDL generation process when building a program or its IDL. This provides more control over the build process for IDLs.

```bash
anchor build --idl-args "--features some_feature"
```

---

### Anchor Lang: Key for Pubkey

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Key` trait is now implemented for `Pubkey` in Anchor Lang. This enables the use of `Pubkey` targets with `associated_token::*` constraints, facilitating associated token account operations.

```rust
use anchor_lang::prelude::*;
use anchor_spl::associated_token;

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[program]
mod my_program {
    pub fn transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()>
}
```

---

### Anchor Lang: Doc Comments for Account Types (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Requires documentation comments when using `AccountInfo` or `UncheckedAccount` types. This promotes better code documentation and understanding of account handling.

```Rust
lang: Require doc comments when using AccountInfo or UncheckedAccount types (#1452).
```

---

### Anchor Lang: LazyAccount Introduction

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `LazyAccount` in Anchor's Rust library. This allows for accounts to be loaded lazily, potentially improving performance in certain scenarios.

```rust
use anchor_lang::LazyAccount;
```

---

### Anchor CLI: Shell Completions Command

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `completions` command to the Anchor CLI, which generates shell completions for various shells (e.g., bash, zsh) using the `clap_complete` crate.

```bash
anchor completions <shell>
```

---

### Upgrade Solana Toolchain to 1.6.3

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: Upgrades the Solana toolchain to version 1.6.3. This major version increment allows for the removal of the `proc_macro_hygiene` feature, simplifying the build process.

```bash
# Update solana-cli, solana-program-library, etc.
solana --version

```

---

### Add anchor shell Command

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `anchor shell` command, which spawns a Node.js shell pre-populated with an environment based on the Anchor.toml configuration. This is useful for interactive development and debugging.

```bash
anchor shell

```

---

### Anchor lang: `bump` keyword for account constraints

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `bump` keyword within account constraints. It adds an optional bump seed to the `seeds` array and, when used with `init` and `seeds`, utilizes `find_program_address` to validate the canonical bump.

```rust
/// A new `bump` keyword is added to the accounts constraints, which is used to add an optional bump seed to the end of a `seeds` array. When used in conjunction with _both_ `init` and `seeds`, then the program executes `find_program_address` to assert that the given bump is the canonical bump (#483).
```

---

### Anchor Rust: Import System Program Transfer Instruction

Source: https://www.anchor-lang.com/docs/basics/cpi

Imports the necessary transfer instruction and related types from the Anchor System Program library. This is a prerequisite for performing CPIs to the System Program's transfer functionality.

```rust
use anchor_lang::system_program::{transfer, Transfer};

```

---

### Skip IDL Build During Tests

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Allows skipping the IDL build process when running tests if the program's API has not changed. This can speed up test execution.

```bash
anchor test --no-idl
```

---

### Specify Output Directory for TypeScript IDL (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows specifying an output directory for the TypeScript IDL when using the `idl parse` subcommand. This provides control over where generated type definitions are saved.

```bash
anchor idl parse path/to/idl.json --output-dir ./types
```

---

### Custom Errors for Constraints (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for custom errors with various constraints including `signer`, `mut`, `has_one`, `owner`, raw constraints, and `address`. This enhances error reporting granularity.

```rust
#[account(signer @ MyError::NotSigner)]
struct MyAccount {...}

enum MyError { NotSigner }
```

---

### Anchor: Custom Errors

Source: https://www.anchor-lang.com/docs/features

Explains how to implement custom error handling within Anchor programs. This allows for more specific and informative error messages during program execution.

```Rust
use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Invalid input provided.")]
    InvalidInput,
    #[msg("Operation failed.")]
    OperationFailed,
}

// In your instruction:
// if some_condition {
//     return Err(MyError::InvalidInput.into());
// }
```

---

### Anchor lang/ts: Change account discriminator pre-image

Source: https://www.anchor-lang.com/docs/updates/changelog

Modifies the pre-image of the `#[state]` account discriminator to be namespaced by "state:" for consistency.

```rust
/// lang, ts: Change account discriminator pre-image of the `#[state]` account discriminator to be namespaced by "state:" (#320).
```

---

### Anchor Lang: `declare_program!` Macro

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `declare_program!` macro in Anchor Lang. This macro simplifies the process of declaring and defining programs, making the code more concise and readable.

```rust
use anchor_lang::prelude::*;

#[program]
mod my_program {
    // ... program logic ...
}

// Using the macro (hypothetical)
#[program]
mod my_program {
    // ... program logic ...
}

declare_program!(my_program);

```

---

### TypeScript Account Resolver Wallet Requirement

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-1

Highlights a change in the TypeScript client library for Anchor. Previously, resolving signers required a wallet, leading to specific errors. Now, only the `publicKey` field of the `Provider` is necessary.

```TypeScript
This function requires the `Provider` interface implementor to have a `wallet` field.
```

---

### Add send_with_spinner_and_config to RequestBuilder

Source: https://www.anchor-lang.com/docs/updates/changelog

The `RequestBuilder` in the Anchor client now includes a `send_with_spinner_and_config` function, providing enhanced control over transaction submission with visual feedback.

```typescript
// Add `send_with_spinner_and_config` function to RequestBuilder (#1926).
```

---

### JavaScript: Derive PDA for Solana Transfer

Source: https://www.anchor-lang.com/docs/basics/cpi

Demonstrates how to derive a Program Derived Address (PDA) in JavaScript using PublicKey.findProgramAddressSync. This is used to find the address of an account that the program must sign for, based on specific seeds and the program's ID.

```JavaScript
const [PDA] = PublicKey.findProgramAddressSync(

  [Buffer.from("pda"), wallet.publicKey.toBuffer()],
  program.programId,
);

```

---

### Anchor Lang: Error Code Handling

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang now supports using `#[error_code]` by simply importing `anchor_lang::error_code`. This simplifies error definition and usage within programs.

```rust
use anchor_lang::error_code;

#[error_code]
enum MyError {
    InvalidState,
    InsufficientFunds,
}

```

---

### Stream Program Logs to .anchor/program-logs

Source: https://www.anchor-lang.com/docs/updates/changelog

Streams program logs generated during testing to a `.anchor/program-logs` directory. This provides a centralized location for inspecting program output.

```bash
anchor test
# Logs will be available in .anchor/program-logs/

```

---

### Anchor Lang: Test Validator Account Flag (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Exposes the `solana-test-validator --account` flag within Anchor.toml configuration using the `[[test.validator.account]]` key. This allows for easier configuration of test validator accounts.

```TOML
cli: Expose the solana-test-validator --account flag in Anchor.toml via [[test.validator.account]] (#1366).
```

---

### Add anchor_lang::pubkey Macro for Pubkey Const Values (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `anchor_lang::pubkey` macro, simplifying the declaration of `Pubkey` constant values.

```rust
lang: Add `anchor_lang::pubkey` macro for declaring `Pubkey` const values (#3021).
```

---

### Transfer Tokens with Anchor (TypeScript)

Source: https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens

Illustrates how to transfer tokens using the `transferChecked` instruction from within an Anchor program in TypeScript. This involves making a cross-program invocation to the Token Program.

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

async function transferTokens(
  program: Program,
  from: Keypair,
  to: PublicKey,
  mint: PublicKey,
  amount: number,
  decimals: number
) {
  const fromTokenAccount = await getTokenAccount(
    program.provider.connection,
    from.publicKey,
    mint
  );
  const toTokenAccount = await getTokenAccount(
    program.provider.connection,
    to,
    mint
  );

  await program.methods
    .transferChecked(new anchor.BN(amount), decimals)
    .accounts({
      authority: from.publicKey,
      from: fromTokenAccount.address,
      to: toTokenAccount.address,
      mint: mint,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([from])
    .rpc();
}

async function getTokenAccount(
  connection: anchor.web3.Connection,
  owner: PublicKey,
  mint: PublicKey
): Promise<{
  address: PublicKey;
  mint: PublicKey;
  owner: PublicKey;
  amount: number;
}> {
  // Placeholder for actual getTokenAccount logic
  // This would typically involve fetching account info from the blockchain
  console.log("Fetching token account for owner:", owner.toBase58());
  // For demonstration, returning a dummy object
  return {
    address: Keypair.generate().publicKey, // Dummy address
    mint: mint,
    owner: owner,
    amount: 100, // Dummy amount
  };
}
```

---

### Anchor lang: Add fallback functions

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for fallback functions in Anchor programs, allowing for more flexible instruction handling.

```rust
/// lang: Add fallback functions (#457).
```

---

### Emit Custom Event in Rust

Source: https://www.anchor-lang.com/docs/features/events

This Rust code demonstrates how to define and emit a custom event using the `emit!()` macro in an Anchor program. The event is logged directly to program logs and includes a string message.

```Rust
use anchor_lang::prelude::*;
declare_id!("8T7MsCZyzxboviPJg5Rc7d8iqEcDReYR2pkQKrmbg7dy");
#[program]
pub mod event {
    use super::*;
    pub fn emit_event(_ctx: Context<EmitEvent>, input: String) -> Result<()> {


emit!(CustomEvent { message: input });
        Ok(())
    }
}
#[derive(Accounts)]
pub struct EmitEvent {}

#[event]
pub struct CustomEvent {
    pub message: String,
}
```

---

### Anchor AVM: Graceful Error Handling

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Version Manager (AVM) has been updated to remove excessive panics and handle errors more gracefully.

```bash
# Example of a command that might have previously panicked but now handles errors gracefully:
avm install invalid_version
# Expected: An informative error message, not a panic.

```

---

### Rust: SPL update_primary_sale_happened_via_token Wrapper

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a wrapper function for `update_primary_sale_happened_via_token` in the Rust SPL module.

```rust
use anchor_spl::token_metadata::update_primary_sale_happened_via_token;

// Call the wrapper function with necessary arguments.
```

---

### Anchor AccountMeta isSigner Field

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that generated `AccountMeta`s for Rust clients correctly set the `isSigner` field. This is crucial for accurate transaction construction and security.

```rust
lang: Generated `AccountMeta`s for Rust clients now properly set the `isSigner` field (#762).
```

---

### Anchor Lang: PDA Seeds Constraint (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `seeds::program` constraint for specifying the program ID when deriving PDAs. This enhances control over PDA generation by explicitly linking them to a specific program.

```Rust
lang: Add `seeds::program` constraint for specifying which program_id to use when deriving PDAs (#1197).
```

---

### Anchor cli: Global options for Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds global options to the CLI for overriding values defined in `Anchor.toml`.

```cli
# cli: Add global options for override Anchor.toml values (#313).
```

---

### Rust: Executing Solana Transfer CPI

Source: https://www.anchor-lang.com/docs/basics/cpi

Details the final step in executing a Cross Program Invocation (CPI) in Anchor. The prepared CPI context and the transfer amount are passed to the `transfer` function to complete the on-chain instruction.

```Rust
transfer(cpi_context, amount)?;

```

---

### Anchor Rust: Transfer Tokens CPI Accounts

Source: https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens

Defines the required accounts for a token transfer CPI operation within an Anchor program. It includes the signer, mint, sender token account, recipient token account, and the token program.

```rust
use anchor_lang::prelude::*
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
declare_id!("3pX5NKLru1UBDVckynWQxsgnJeUN3N1viy36Gk9TSn8d");
#[program]
pub mod token_example {
    use super::*;
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let decimals = ctx.accounts.mint.decimals;
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        token_interface::transfer_checked(cpi_context, amount, decimals)?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
```

---

### Add Filename to Keypair File Errors (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Improves error reporting for keypair file issues by including the filename in the error message.

```bash
cli: add filename to 'Unable to read keypair file' errors (#2932).
```

---

### Enable event-cpi Feature in Cargo.toml

Source: https://www.anchor-lang.com/docs/features/events

To use the `emit_cpi!()` macro for emitting events via Cross Program Invocations (CPIs), you need to add the `event-cpi` feature to the `anchor-lang` dependency in your `Cargo.toml` file.

```toml
[dependencies]
anchor-lang = { version = "0.31.1", features = ["event-cpi"] }
```

---

### Rust: IDL Seed Generation for Byte String Literals

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects the IDL `seed` generation for byte string literals in the Rust implementation.

```rust
// PDA seeds derived from byte string literals should now be generated accurately.
```

---

### Anchor Lang: InitIfNedded ATA Check (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a check to verify that an account is an Associated Token Account (ATA) when `init_if_needed` is used and initialization is not necessary. This prevents incorrect usage with non-ATA accounts.

```Rust
lang: add missing check that verifies that account is ATA when using `init_if_needed` and init is not needed(#1221)
```

---

### Anchor lang: `remaining_accounts` with `CpiContext`

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows the use of `remaining_accounts` with `CpiContext` by implementing the `ToAccountMetas` trait on `CpiContext`.

```rust
/// lang: Allows one to use `remaining_accounts` with `CpiContext` by implementing the `ToAccountMetas` trait on `CpiContext` (#351).
```

---

### Add Serum DEX CPI Client

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a Cross-Program Invocation (CPI) client specifically for the Serum Decentralized Exchange (DEX). This facilitates interactions with Serum's market and order book programs.

```rust
use anchor_spl::dex::SerumDEX;

// ... use SerumDEX::new_ixn(...)

```

---

### Add program.simulate Namespace in TypeScript

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `program.simulate` namespace in the TypeScript client, enhancing simulation capabilities for Anchor programs. This feature allows for more granular control and testing of program interactions.

```ts
program.simulate;
```

---

### Rust Account Structure with Has One Constraint

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Rust struct definition for an Anchor account using the `has_one` constraint. This is recommended as an alternative to `address` constraints with non-constant values for better automatic resolution in IDL generation.

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    pub authority: UncheckedAccount<'info>,
    #[account(has_one = authority)]
    pub my_account: Account<'info, MyAccount>,
}
```

---

### Anchor IDL Set Authority

Source: https://www.anchor-lang.com/docs/references/cli

Assigns a new authority to the IDL account. Both the new authority's public key and the program ID must be encoded in base 58.

```bash
anchor idl set-authority -n <new-authority> -p <program-id>
```

---

### Anchor Lang: Associated Token Program Constraint

Source: https://www.anchor-lang.com/docs/updates/changelog

A fix has been applied to the `associated_token::token_program` constraint in Anchor Lang to ensure correct validation.

```rust
use anchor_lang::prelude::*;
use anchor_spl::associated_token;

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(address = associated_token::ID)] // Correct constraint for associated token program ID
    pub associated_token_program: AccountInfo<'info>,
    // ... other accounts
}

```

---

### Anchor Lang: Instruction Data Arrays

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang now supports arrays with constant sizes in instruction data. This simplifies the handling of structured data passed to program instructions.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}

#[program]
mod my_program {
    pub fn process_array(ctx: Context<MyAccounts>, data: [u8; 4]) -> Result<()>
}

```

---

### Add pubkeys function to methods builder in Anchor TS

Source: https://www.anchor-lang.com/docs/updates/changelog

The `pubkeys` function has been added to the methods builder in the Anchor TypeScript client, allowing retrieval of all instruction account addresses.

```typescript
// Add `pubkeys` function to methods builder to get all instruction account addresses (#1733).
```

---

### Rust: Exporting SPL Associated Token Account Crate

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Explains that the `spl-associated-token-account` crate is now re-exported from `anchor_spl::associated_token`. This allows developers to remove the direct dependency on `spl-associated-token-account` from their `Cargo.toml` if they are using Anchor's re-export.

```toml
[dependencies]
anchor-spl = "0.30.1"
# spl-associated-token-account = "3.0.2" # Can be removed
```

```rust
use anchor_spl::associated_token::AssociatedTokenAccount;

// Example usage within an Anchor instruction
#[derive(Accounts)]
pub struct MyAccounts {
    #[account(mut)]
    pub owner: Signer<'_>,
    #[account(mut)]
    pub token_account: Account<'_, AssociatedTokenAccount>,
}
```

---

### Anchor Rust: Execute System Program Transfer CPI

Source: https://www.anchor-lang.com/docs/basics/cpi

Implements a Cross Program Invocation (CPI) to the System Program's transfer instruction using Anchor. It constructs a `CpiContext` with the program ID and transfer accounts, then calls the `transfer` helper function.

```rust
pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
    let from_pubkey = ctx.accounts.sender.to_account_info();
    let to_pubkey = ctx.accounts.recipient.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();

    letcpi_context= CpiContext::new(
        program_id,
        Transfer {
            from: from_pubkey,
            to: to_pubkey,
        },
    );

    transfer(cpi_context, amount)?;
    Ok(())
}

```

---

### Output TypeScript IDL in IDL Parse (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

The `idl parse` subcommand now outputs the TypeScript IDL. This provides a convenient way to generate TypeScript definitions from a program's IDL.

```bash
anchor idl parse path/to/idl.json --output-ts path/to/idl.ts
```

---

### Add idl type Command (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a new `idl type` command for inspecting and managing IDL types.

```bash
cli: Add `idl type` command (#3017).
```

---

### Anchor TS: Optional `opts` in `AnchorProvider`

Source: https://www.anchor-lang.com/docs/updates/changelog

The `opts` parameter in the `AnchorProvider` constructor is now optional in the TypeScript client. This simplifies provider instantiation when default options are sufficient.

```typescript
import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider } from "@coral-xyz/anchor";

// Previously might have required options:
// const provider = new AnchorProvider(connection, wallet, { commitment: "processed" });

// Now optional:
const provider = new AnchorProvider(connection, wallet);
```

---

### Add Associated Account Attributes

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds `#[account(associated = <target>)]` and `#[associated]` attributes for creating associated program accounts. The TypeScript package provides methods like `<program>.account.<account-name>.associated` to fetch these.

```rust
#[account(init, associated = token_program, payer = payer, owner = owner)]
struct MyAssociatedAccount {
    // ...
}

```

---

### Anchor lang: Associated constraints no longer implement `mut`

Source: https://www.anchor-lang.com/docs/updates/changelog

Associated constraints no longer automatically implement `mut`, requiring explicit specification.

```rust
/// lang: Associated constraints no longer automatically implement `mut` (#341).
```

---

### Anchor Lang: Zero Copy Account Repr Overrides (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows representation overrides for zero-copy accounts in Rust. This provides more flexibility in how zero-copy accounts are serialized and deserialized.

```Rust
lang: Allow repr overrides for zero copy accounts (#1273).
```

---

### Anchor Lang: InitIfNedded Feature Flag (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Places the `init_if_needed` functionality behind a feature flag to reduce incorrect usage. This change aims to improve the robustness and prevent common errors related to account initialization.

```Rust
lang: Put `init_if_needed` behind a feature flag to decrease wrong usage (#1258).
```

---

### Make anchor test fail with --skip-deploy and running validator

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that `anchor test` fails when used with the `--skip-deploy` option without `--skip-local-validator` if a validator is already running, preventing potential conflicts.

```bash
`anchor test` fail when used with `--skip-deploy` option and without `--skip-local-validator` option but there already is a running validator (#1675).
```

---

### TypeScript: AccountClient.fetchMultiple Return Type

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures the return type of `AccountClient.fetchMultiple` accurately reflects the account type being fetched in TypeScript.

```typescript
import { AccountClient } from "@coral-xyz/anchor";

// Assuming 'AccountType' is the type of the account being fetched
const accounts: AccountType[] = await accountClient.fetchMultiple(
  accountAddresses
);
```

---

### Generate TypeScript IDL Types using CLI

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Anchor CLI command to generate TypeScript type definitions from an IDL JSON file. This is useful for client-side development, providing strongly-typed interfaces for interacting with Anchor programs.

```bash
anchor idl type <PATH_TO_IDL_JSON>
```

---

### Anchor CLI: Require IDL Build Feature

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now requires the `idl-build` feature to be explicitly enabled in the program's `Cargo.toml` for IDL generation.

```toml
[package]
name = "my-program"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
idl-build = []

[dependencies]
anchor-lang = "0.29.0"

```

---

### Anchor IDL & TS: Unit and Tuple Struct Support

Source: https://www.anchor-lang.com/docs/updates/changelog

The IDL specification and its TypeScript counterpart now support unit and tuple structs. This allows for more flexible data structure definitions within Anchor programs.

```json
{
  "name": "MyTypes",
  "types": [
    {
      "name": "UnitStruct",
      "type": { "kind": "struct", "fields": [] }
    },
    {
      "name": "TupleStruct",
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "field1", "type": "u64" },
          { "name": "field2", "type": "string" }
        ]
      }
    }
  ]
}
```

---

### Anchor SPL: Compilation with `shmem` Feature

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves a compilation error that occurred when the `shmem` feature was enabled in the SPL (Solana Program Library) dependencies. This ensures that SPL programs can be built correctly with this feature.

```rust
# Cargo.toml snippet demonstrating the feature
[dependencies]
solana-program = {
    version = "1.18.0",
    features = ["shmem"]
}

```

---

### Fix returns serialization in IDL

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects an issue where `returns` was being serialized as `null` instead of `undefined` in the IDL for Anchor Lang.

```rust
// Fix `returns` being serialized as `null` instead of `undefined` in IDL (#1782).
```

---

### Anchor Client: Internal RPC Method for Mock Feature

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds an `internal_rpc` method to the Anchor client, specifically for use with the `mock` feature. This facilitates internal RPC calls during mock client operations.

```rust
client.internal_rpc().get_account(&account_pubkey);
```

---

### Add Separate Spec Crate (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a dedicated specification crate for better organization and management of project specifications.

```rust
idl: Add separate spec crate (#3036).
```

---

### Anchor Fix: `syn::Hash` Missing Method

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses a missing `new_from_array` method in the `syn::Hash` type. This fix ensures that hash values can be correctly constructed from byte arrays, which is crucial for various cryptographic operations.

```rust
use syn::Hash;

let byte_array: [u8; 32] = [0u8; 32];
let hash = Hash::new_from_array(byte_array);

```

---

### Anchor lang: `associated_seeds!` macro for CPIs

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `associated_seeds!` macro for generating signer seeds required for Cross-Program Invocations (CPIs) when signing with an `#[account(associated = <target>)]` account.

```rust
/// lang: Add `associated_seeds!` macro for generating signer seeds for CPIs signed by an `#[account(associated = <target>)]` account (#400).
```

---

### Anchor CLI: Toolchain Property in Anchor.toml

Source: https://www.anchor-lang.com/docs/updates/changelog

The `toolchain` property can now be specified in `Anchor.toml` to override the default Anchor and Solana versions used by the CLI.

```toml
[toolchain]
anchor = "0.29.0"
solana = "1.17.0"

```

---

### Anchor Lang & SPL: Token Extensions Support

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for token extensions in Anchor Lang and the SPL (Solana Program Library) token module. This enables developers to leverage advanced features of the Token 2022 program.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_2022::token_extensions::memo::MemoInstruction;

// Example of using token extensions (hypothetical)
let memo_instruction = MemoInstruction {
    program_id: spl_token_2022::id(),
    // ... other fields ...
};

```

---

### Add Accounts Resolution for Associated Token Accounts (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Implements accounts resolution for associated token accounts, enhancing IDL and TypeScript integration.

```typescript
idl, ts: Add accounts resolution for associated token accounts (#2927).
```

---

### Anchor lang: `#[account(address = <expr>)]` constraint

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `#[account(address = <expr>)]` constraint for asserting the specific address of an account within Anchor programs.

```rust
/// lang: Add `#[account(address = <expr>)]` constraint for asserting the address of an account (#400).
```

---

### Fix anchor keys list path issue

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects an issue where `anchor keys list` was reading the `target` folder from an incorrect path, ensuring proper key management.

```bash
Fix `anchor keys list` reading the `target` folder in the wrong path (#2063).
```

---

### TypeScript: Accessing Payer Keypair in Tests

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Improves type safety for accessing the payer keypair in tests by adding optional 'wallet' and 'payer' fields to the 'Provider' and 'Wallet' interfaces respectively. This eliminates the need for '@ts-ignore'.

```TypeScript
const payer = program.provider.wallet!.payer!;
```

---

### Anchor TOML Section Renaming

Source: https://www.anchor-lang.com/docs/updates/changelog

Renames the `[clusters.<network>]` section in Anchor.toml to `[programs.<network>]`. This change improves clarity in configuration.

```rust
cli: `[clusters.<network>]` Anchor.toml section has been renamed to `[programs.<network>]` (#570).
```

---

### Implement TryFromIntError for Error Propagation (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Implements `TryFromIntError` for the `Error` type, enabling the propagation of integer conversion errors within the Anchor language.

```rust
lang: Implement `TryFromIntError` for `Error` to be able to propagate integer conversion errors (#2950).
```

---

### TypeScript Migrations

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces TypeScript migrations to help manage and update TypeScript codebases related to Anchor projects. This likely includes tooling for refactoring or updating dependencies.

```bash
anchor migrate ts

```

---

### Fix anchor build failing with relative paths in Test.toml

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves a bug where `anchor build` would fail if `Test.toml` included a relative path that did not yet exist, as it is created by the build process itself.

```rust
// Fix `anchor build` failing if `Test.toml` included a relative path that didn't exist yet because it's created by `anchor build` (#1772).
```

---

### Uninstall Anchor CLI Version

Source: https://www.anchor-lang.com/docs/references/avm

This command uninstalls a specific version of the Anchor CLI from your system.

```Shell
avm uninstall <version>
```

---

### Migrate from Enum-Based Method Dispatch to Sighash

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: Migrates the method dispatch mechanism from Rust enums to a sighash-based approach across the Anchor language, client, and TypeScript. This change impacts how instructions are identified and called.

```rust
// Instruction dispatch is now based on sighashes instead of enum variants.

```

---

### TypeScript: Associated Token Account Resolution

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-1

Details how Anchor's TypeScript client now supports Associated Token Account (ATA) resolution. If ATAs are used in instructions, calling the `accounts` method with them specified will result in a type error, requiring their removal from the `accounts` call.

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyProgram } from "./idl/my_program"; // Assuming your IDL is here

async function example() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new Program<MyProgram>(idl, programId, provider);

  // Example of how ATA resolution might affect account calls
  // If 'userAta' is an associated token account, it should be removed from this call
  // const tx = await program.methods.someInstruction().accounts({
  //     user: provider.wallet.publicKey,
  //     userAta: associatedTokenAddress, // This might cause a type error now
  // }).rpc();

  // Corrected call (assuming ATA is not explicitly passed if handled internally or not needed)
  const tx = await program.methods
    .someInstruction()
    .accounts({
      user: provider.wallet.publicKey,
    })
    .rpc();
}
```

---

### Custom Errors for Raw Constraints (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables support for custom errors with raw constraints in Anchor. This allows for more specific and informative error handling within constraint checks.

```rust
#[account(address = MY_ADDRESS @ MyError::InvalidAddress)]
struct MyAccount {...}

enum MyError { InvalidAddress }
```

---

### Anchor CLI: Clean Command Removes .anchor Directory

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor clean` command has been updated to also remove the `.anchor` directory. This provides a more thorough cleanup of build artifacts.

```bash
anchor clean
```

---

### Anchor CLI: Skip Lint Option

Source: https://www.anchor-lang.com/docs/updates/changelog

A `--skip-lint` option has been added to the Anchor CLI. This option disables the check linting introduced in a previous version, allowing for faster prototyping when linting is not desired.

```bash
anchor build --skip-lint
```

---

### Rust: Zero-Copy Attribute Changes

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates `account(zero_copy)` and `zero_copy` attributes in Rust to derive `bytemuck::Pod` and `bytemuck::Zeroable` traits, imposing stricter type requirements.

```rust
use anchor_lang::prelude::*;

#[account]
#[derive(Debug, Clone, Copy, Pod, Zeroable)] // Derive Pod and Zeroable
pub struct MyZeroCopyAccount {
    pub data: u64,
}

// Add to Cargo.toml:
// bytemuck = { version = "1.4.0", features = ["derive", "min_const_generics"] }

```

---

### Fix unexpected_cfgs Build Warning (IDL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses a build warning related to `unexpected_cfgs` during IDL generation.

```rust
idl: Fix `unexpected_cfgs` build warning (#2992).
```

---

### TypeScript: AnchorProvider `feePayer` Check

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `feePayer` check to `AnchorProvider` methods in TypeScript. If the `feePayer` is not set, it defaults to the provider's wallet.

```typescript
import { AnchorProvider } from "@coral-xyz/anchor";

// If provider.wallet is set and feePayer is not explicitly provided,
// the provider's wallet will be used as the fee payer.
```

---

### Anchor Associated Token Accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `associated_token` keyword for initializing associated token accounts within `#[derive(Accounts)]`. This simplifies the process of managing SPL Token accounts associated with a program or user.

```rust
lang: Add `associated_token` keyword for initializing associated token accounts within `#[derive(Accounts)]` (#790).
```

---

### Anchor Lang: Test Clone Key Rename (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Renames the `[[test.clone]]` key in Anchor.toml to `[[test.validator.clone]]`. This change standardizes the naming convention for test validator configurations.

```TOML
cli: [[test.clone]] key in Anchor.toml is renamed to [[test.validator.clone]] (#1366).
```

---

### Anchor Lang: Array Size Handling in Events and IDL

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang now handles array sizes with variable sizes in events and performs array size casting during IDL parsing. This improves the flexibility and robustness of event data and IDL definitions.

```rust
use anchor_lang::prelude::*;

#[event]
pub struct DataArray {
    pub data: Vec<u8>,
}

#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}

#[program]
mod my_program {
    pub fn emit_data_array(ctx: Context<MyAccounts>, data: Vec<u8>) -> Result<()>
}
```

---

### Anchor Lang: Wallet/NodeWallet Type Mapping (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Improves type mapping for `Wallet` and `NodeWallet` classes in TypeScript, adding support for `Option<T>` and `Vec<String>` types. This enhances type safety and usability.

```TypeScript
ts: Fix the root type declaration of the `Wallet` / `NodeWallet` class (#1363).
ts: Improve type mapping of Account fields into Typescript with additional support for `Option<T>` and `Vec<String>` types (#1393).
```

---

### Upgrade TypeScript Version of Templates to v5 (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the TypeScript version used in project templates generated by the CLI to v5.

```bash
cli: Upgrade `typescript` version of templates to v5 (#3480).
```

---

### Anchor IDL Type Alias Definition (JSON)

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-29-0

Shows the JSON representation of the IDL generated for the `U8Array` type alias, illustrating how it's defined with a kind 'alias' and its underlying value.

```JSON
{
  "version": "0.1.0",
  "name": "my_program",
  "instructions": [
    {
      "name": "typeAlias",
      "accounts": [],
      "args": [
        {
          "name": "u8Array",
          "type": {
            "defined": "U8Array"
          }
        }
      ]
    }
  ],
  "types": [
    {
      "name": "U8Array",
      "type": {
        "kind": "alias",
        "value": {
          "array": ["u8", 8]
        }
      }
    }
  ]
}
```

---

### Anchor lang/ts: Framework defined error codes

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces framework-defined error codes, reserving 0-300 for Anchor and 300+ for user-defined errors.

```rust
/// lang, ts: Framework defined error codes are introduced, reserving error codes 0-300 for Anchor, and 300 and up for user defined error codes (#354).
```

---

### Rename Metadata Interface Instruction Fields (SPL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Renames the `token_program_id` fields in the metadata interface instructions to `program_id` for consistency.

```rust
spl: Rename metadata interface instruction fields from `token_program_id` to `program_id` (#3076).
```

---

### Throwing Custom Errors with err! Macro

Source: https://www.anchor-lang.com/docs/features/errors

Shows how to use the `err!` macro to throw custom errors from an Anchor program. The `err!` macro simplifies the process of returning `AnchorError` instances, often used within instruction handlers.

```Rust
#[program]
mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: MyAccount) - Result<()> {
        if data.data = 100 {


            return err!(MyError::DataTooLarge);
        }
        ctx.accounts.my_account.set_inner(data);
        Ok(())
    }
}
#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge
}
```

---

### Rust: Account Closing Reassigns and Reallocates

Source: https://www.anchor-lang.com/docs/updates/changelog

Modifies account closing behavior in Anchor Rust to reassign to the system program and perform reallocations.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut, close = destination)]
    pub account_to_close: AccountInfo<'info>,
    pub destination: AccountInfo<'info>,
}

// When `account_to_close` is closed, its remaining lamports are sent to `destination`,
// and the account data is reallocated to zero.
```

---

### Add `MasterEditionAccount` Deserialization for SPL Metadata

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the ability to deserialize the `MasterEditionAccount` for SPL Token Metadata. This allows developers to easily access and interact with the master edition data associated with NFTs.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_metadata::MasterEditionAccount;

#[derive(Accounts)]
pub struct ReadMasterEdition<'info> {
    #[account(owner = Pubkey::from_str("metaqbxxUJo1ctLBqifpndxUysr5yZ4r74z7a3a7f").unwrap())] // Token Metadata Program ID
    pub master_edition: Account<'info, MasterEditionAccount>,
}
```

---

### Emit CPI Macros for Event Logs

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `emit_cpi!` and `#[event_cpi]` macros, available via the `event-cpi` feature flag. These macros allow storing event logs in transaction metadata, facilitating event tracking in cross-program invocations (CPIs).

```rust
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
struct MyEvent {
    value: u64,
}

#[derive(Accounts)]
pub struct MyAccount<'info> {
    // ... accounts ...
}

#[derive(Accounts)]
pub struct InvokeMyProgram<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

pub fn handler(ctx: Context<InvokeMyProgram>) -> Result<()>
{
    // Example of emitting an event using the macro
    // emit_cpi!(ctx.accounts.my_account.emit_event(MyEvent { value: 10 }));
    Ok(())
}
```

---

### `#[account]` Attribute Arguments No Longer Parse Identifiers as Namespaces (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the `#[account]` attribute to no longer parse identifiers as namespaces, changing how accounts are defined.

```rust
lang: `#[account]` attribute arguments no longer parses identifiers as namespaces (#3140).
```

---

### Transfer Tokens via CPI in Anchor

Source: https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens

This Rust function, `transfer_tokens`, is part of an Anchor program. It performs a token transfer using Cross-Program Invocation (CPI) to the `token_program`. It requires specific accounts like the sender's token account, recipient's token account, and the mint account. The function also handles signer seeds for Program Derived Addresses (PDAs).

```rust
pub fn transfer_tokens(ctx: Context<TransferTokens>) -> Result<()> {


    letsigner_seeds: &[&[&[u8]]] = &[&[b"token", &[ctx.bumps.sender_token_account]]];
    let amount = ctx.accounts.sender_token_account.amount;
    let decimals = ctx.accounts.mint.decimals;
    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.sender_token_account.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
    token_interface::transfer_checked(cpi_context, amount, decimals)?;
    Ok(())
}
```

---

### Anchor CpiAccount Reload Behavior

Source: https://www.anchor-lang.com/docs/updates/changelog

The `CpiAccount::reload` method now mutates the existing struct instead of returning a new one. This optimizes memory usage by modifying the account in place.

```rust
lang: `CpiAccount::reload` mutates the existing struct instead of returning a new one (#526).
```

---

### Anchor lang: Add `require` macro for assertions

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `require` macro in Anchor's Rust DSL for specifying assertions that return error codes upon failure. This enhances error handling within Solana programs.

```rust
/// Adds `require` macro for specifying assertions that return error codes on failure (#483).
#[macro_use] extern crate anchor_lang;
```

---

### Anchor TS: `prepend` Option for `preInstructions`

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a `prepend` option to the `preInstructions` method of the `MethodBuilder` in the Anchor TypeScript client. This allows users to control whether new pre-instructions are added to the beginning or end of the existing list.

```typescript
import * as anchor from "@coral-xyz/anchor";

// Assuming builder is an instance of MethodBuilder
builder.preInstructions.push(someInstruction);

// Using the new option (hypothetical)
builder.preInstructions(someInstruction, { prepend: true });
```

---

### Add `VersionedTransaction` Support in TypeScript Client

Source: https://www.anchor-lang.com/docs/updates/changelog

Integrates support for `VersionedTransaction` in the TypeScript client. Methods in the `Provider` class and `Wallet` interface now accept `tx: Transaction | VersionedTransaction` as an argument.

```typescript
import { Transaction, VersionedTransaction } from "@solana/web3.js";
import { Provider, Wallet } from "@project-serum/anchor";

// Example:
// async sendTransaction(tx: Transaction | VersionedTransaction, ...)
// async signTransaction(tx: Transaction | VersionedTransaction): Promise<Transaction | VersionedTransaction>
```

---

### Anchor lang: Preserve instruction data for fallback functions

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that all instruction data is preserved for fallback functions, improving robustness and error handling in program execution.

```rust
/// Preserve all instruction data for fallback functions (#483).
```

---

### Anchor CLI: Toolchain Override Error Display

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that errors are displayed when the restoration of toolchain overrides fails in the Anchor CLI. This provides better feedback to the user about potential configuration issues.

```bash
# This fix ensures that if toolchain override restoration fails,
# the error message is clearly presented to the user.
# No specific command to trigger this, it's a behavior fix.

```

---

### Process and Validate Instruction Chain with Anchor Lang

Source: https://www.anchor-lang.com/docs/testing/mollusk

Illustrates using Anchor Lang's `process_and_validate_instruction_chain` to process and validate a sequence of Solana instructions. This function allows developers to specify custom validation checks for each instruction in the chain using the `Check` enum.

```rust
use {
    mollusk_svm::{Mollusk, result::Check},
    solana_sdk::{account::Account, pubkey::Pubkey, system_instruction},
};
let mollusk = Mollusk::default();
let alice = Pubkey::new_unique();
let bob = Pubkey::new_unique();
let carol = Pubkey::new_unique();
let dave = Pubkey::new_unique();
let starting_lamports = 500_000_000;
let alice_to_bob = 100_000_000;
let bob_to_carol = 50_000_000;
let bob_to_dave = 50_000_000;
mollusk.process_and_validate_instruction_chain(
    &[
        (
            // 0: Alice to Bob
            &system_instruction::transfer(&alice, &bob, alice_to_bob),
            &[
                Check::success(),
                Check::account(&alice)
                    .lamports(starting_lamports - alice_to_bob) // Alice pays
                    .build(),
                Check::account(&bob)
                    .lamports(starting_lamports + alice_to_bob) // Bob receives
                    .build(),
                Check::account(&carol)
                    .lamports(starting_lamports) // Unchanged
                    .build(),
                Check::account(&dave)
                    .lamports(starting_lamports) // Unchanged
                    .build(),
            ],
        ),
        (
            // 1: Bob to Carol
            &system_instruction::transfer(&bob, &carol, bob_to_carol),
            &[
                Check::success(),
                Check::account(&alice)
                    .lamports(starting_lamports - alice_to_bob) // Unchanged
                    .build(),
                Check::account(&bob)
                    .lamports(starting_lamports + alice_to_bob - bob_to_carol) // Bob pays
                    .build(),
                Check::account(&carol)
                    .lamports(starting_lamports + bob_to_carol) // Carol receives
                    .build(),
                Check::account(&dave)
                    .lamports(starting_lamports) // Unchanged
                    .build(),
            ],
        ),
        (
            // 2: Bob to Dave
            &system_instruction::transfer(&bob, &dave, bob_to_dave),
            &[
                Check::success(),
                Check::account(&alice)
                    .lamports(starting_lamports - alice_to_bob) // Unchanged
                    .build(),
                Check::account(&bob)
                    .lamports(starting_lamports + alice_to_bob - bob_to_carol - bob_to_dave) // Bob pays
                    .build(),
                Check::account(&carol)
                    .lamports(starting_lamports + bob_to_carol) // Unchanged
                    .build(),
                Check::account(&dave)
                    .lamports(starting_lamports + bob_to_dave) // Dave receives
                    .build(),
            ],
        ),
    ],
    &[
        (alice, system_account_with_lamports(starting_lamports)),
        (bob, system_account_with_lamports(starting_lamports)),
        (carol, system_account_with_lamports(starting_lamports)),
        (dave, system_account_with_lamports(starting_lamports)),
    ],
);
```

---

### Expose wallet public key on Provider in Anchor TS

Source: https://www.anchor-lang.com/docs/updates/changelog

The public key of the wallet is now exposed on the `Provider` object in the Anchor TypeScript client, providing easier access to wallet information.

```typescript
// Expose the wallet's publickey on the Provider (#1845).
```

---

### Emit Custom Event via CPI in Anchor Program (Rust)

Source: https://www.anchor-lang.com/docs/features/events

This Rust code demonstrates how to emit a custom event using the `emit_cpi!()` macro within an Anchor program. The `#[event_cpi]` attribute must be applied to the `#[derive(Accounts)]` struct for the instruction that emits events.

```rust
use anchor_lang::prelude::*;
declare_id!("2cDQ2LxKwQ8fnFUz4LLrZ157QzBnhPNeQrTSmWcpVin1");
#[program]
pub mod event_cpi {
    use super::*;
    pub fn emit_event(ctx: Context<EmitEvent>, input: String) -> Result<()> {


emit_cpi!(CustomEvent { message: input });
        Ok(())
    }
}

#[event_cpi]
#[derive(Accounts)]
pub struct EmitEvent {}

#[event]
pub struct CustomEvent {
    pub message: String,
}
```

---

### Anchor Test Program Logs

Source: https://www.anchor-lang.com/docs/updates/changelog

Programs embedded into genesis during tests will now produce program logs. This aids in debugging test scenarios.

```rust
cli: Programs embedded into genesis during tests will produce program logs (#594).
```

---

### Verbose error for missing ANCHOR_WALLET in Anchor TS

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides more verbose error messages when the `ANCHOR_WALLET` environment variable is missing while using `NodeWallet.local()` in the Anchor TypeScript client.

```typescript
// verbose error for missing `ANCHOR_WALLET` variable when using `NodeWallet.local()` (#1958).
```

---

### Embed Programs into Local Validator Genesis

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows specifying programs to embed into the local validator's genesis configuration via Anchor.toml when testing. This ensures that necessary programs are available in the local environment.

```toml
[test]
programs = ["./path/to/program"]

```

---

### Upgrade TypeScript and Remove Generic Parameters of SimulateResponse (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Upgrades the `typescript` dependency to `5.5.4` and removes generic parameters from `SimulateResponse`.

```typescript
ts: Upgrade `typescript` to `5.5.4` and remove the generic parameters of `SimulateResponse` (#3221).
```

---

### Replace web3.Account with web3.Signer in TypeScript

Source: https://www.anchor-lang.com/docs/updates/changelog

Replaces the deprecated `web3.Account` with `web3.Signer` in the TypeScript public APIs. This update aligns with newer Solana web3.js conventions for handling signers.

```ts
web3.Signer;
```

---

### TypeScript: `.fetchNullable()` Robustness

Source: https://www.anchor-lang.com/docs/updates/changelog

Makes `.fetchNullable()` in TypeScript more robust, especially for accounts that only hold a balance and might not have full account data.

```typescript
// .fetchNullable() should return null or undefined for accounts without data,
// rather than throwing an error.
```

---

### Fix IDL Write Corruption from Retries (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves an issue where IDL writes could become corrupted due to retries.

```bash
cli: Fix IDL write getting corrupted from retries (#2964).
```

---

### Add avm update command for Anchor CLI

Source: https://www.anchor-lang.com/docs/updates/changelog

A new command `avm update` has been added to the Anchor CLI's `avm` (Anchor Version Manager) tool, allowing users to update the Anchor CLI to the latest version.

```bash
New `avm update` command to update the Anchor CLI to the latest version (#1670).
```

---

### CLI: Exit Status for Failing Commands

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that failing commands in the Anchor CLI return the correct exit status, improving scriptability and error handling.

```bash
# Example of a failing command
anchor build --force
# The exit status will indicate failure (e.g., non-zero)
```

---

### Fix Internal Identifiers in Instruction Arguments

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves an issue where internal identifiers like `program_id`, `accounts`, `ix_data`, and `remaining_accounts` could not be used in instruction arguments. These identifiers are now correctly handled.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MyInstructionAccounts<'info> {
    // ... accounts ...
}

pub fn my_instruction(ctx: Context<MyInstructionAccounts>, /* other args */) -> Result<()>
{
    // Previously, using 'program_id' here might have caused issues.
    // let program_id = ctx.program_id;
    Ok(())
}
```

---

### Anchor Rust: Define Accounts for System Program Transfer CPI

Source: https://www.anchor-lang.com/docs/basics/cpi

Defines the accounts required for a Cross Program Invocation (CPI) to the System Program's transfer instruction using Anchor. It includes the sender (signer), recipient, and the system program itself.

```rust
#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
sender: Signer<'info>, // from account
    #[account(mut)]
recipient: SystemAccount<'info>, // to account
system_program: Program<'info, System>, // program ID
}
```

---

### Anchor Lang: Non-8-Byte Discriminator Support in declare_program!

Source: https://www.anchor-lang.com/docs/updates/changelog

The `declare_program!` macro in Anchor's Rust library now supports non-8-byte discriminators, enhancing flexibility for custom program structures.

```rust
declare_program!(MyProgram, "MYPROG", "MYDISC");
```

---

### Fix Using address Constraint with Field Expressions (IDL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects the usage of the `address` constraint when field expressions are involved in IDL generation.

```rust
idl: Fix using `address` constraint with field expressions (#3034).
```

---

### Anchor Lang: Program Client Methods (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a new `methods` namespace within the program client, providing a more ergonomic builder API for interacting with Anchor programs. This improves the developer experience for client-side interactions.

```TypeScript
ts: Add new `methods` namespace to the program client, introducing a more ergonomic builder API (#1324).
```

---

### Rust to JS Type Conversion: Primitive Types

Source: https://www.anchor-lang.com/docs/references/type-conversion

This snippet illustrates the conversion of primitive Rust types to their TypeScript equivalents. It covers booleans, various integer and floating-point number types, and strings, showing the direct mapping or the use of Anchor's BN for larger integers.

```Rust
bool
u8/u16/u32/i8/i16/i32
u64/u128/i64/i128
f32/f64
String
```

```TypeScript
boolean
number
anchor.BN
number
string
```

---

### Add Shared Memory API

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces a shared memory API, likely for efficient data sharing between programs or components. This can improve performance by reducing data serialization and deserialization overhead.

```rust
// Usage of shared memory API

```

---

### TypeScript: Null/Undefined Wallet Error Fix

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes a breaking change in TypeScript where providing a null or undefined wallet would throw an error.

```typescript
// The provider should handle null or undefined wallets gracefully.
```

---

### Anchor Lang: Registry Utility (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a registry utility for fetching the latest verified build of Anchor Lang. This helps ensure developers are using the most stable and up-to-date versions.

```TypeScript
ts: Add registry utility for fetching the latest verified build (#1371).
```

---

### TypeScript: Remove createProgramAddressSync and findProgramAddressSync

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes `createProgramAddressSync` and `findProgramAddressSync` from Anchor's TypeScript client, directing users to `@solana/web3.js` for these functionalities.

```typescript
import { PublicKey } from "@solana/web3.js";

// Use @solana/web3.js for these functions:
// const [programAddress, bump] = await PublicKey.findProgramAddress(
//   [Buffer.from("seed")],
//   programId
// );
```

---

### Implement System Program coder in Anchor TypeScript

Source: https://www.anchor-lang.com/docs/updates/changelog

A new coder has been implemented for the System Program within the Anchor TypeScript tooling, enabling better encoding and decoding of system instructions.

```typescript
// Implement a coder for system program (#1920).
```

---

### Fix `anchor new` Not Updating `Anchor.toml`

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves a bug where the `anchor new` command failed to update the `Anchor.toml` configuration file. This ensures that new projects are created with the correct and updated configuration.

```bash
# Creating a new project should now correctly update Anchor.toml
anchor new my_solana_project
```

---

### Anchor lang: Accounts trait parameter

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Accounts` trait now accepts an additional `&[u8]` parameter, modifying how accounts are processed.

```rust
/// lang: Accounts trait now accepts an additional `&[u8]` parameter (#386).
```

---

### Anchor Lang: Error Module Consolidation (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Consolidates all error-related code into a dedicated error module. This improves code organization and maintainability for error handling.

```Rust
lang: All error-related code is now in the error module (#1426).
```

---

### Rust: Payer Account Initialization Prevention

Source: https://www.anchor-lang.com/docs/updates/changelog

Prevents the payer account from being incorrectly initialized as a program account in the Rust implementation.

```rust
// The payer account should not be marked as #[account] if it's just for signing.
```

---

### Anchor CLI Docker Image Update

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Docker image used by the Anchor CLI from `backpackapp/build` to `solanafoundation/anchor` as part of version 0.31.1 release. This change aims to standardize the build environment.

```text
cli, docker: Replace `backpackapp/build` Docker image with `solanafoundation/anchor` (#3619).
```

---

### Anchor lang: Associated `space` constraints must be literal integers

Source: https://www.anchor-lang.com/docs/updates/changelog

Requires associated `space` constraints to be literal integers instead of literal strings.

```rust
/// lang: Associated `space` constraints must now be literal integers instead of literal strings (#341).
```

---

### Support Multithreading in Rust Client

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for multithreading in the Rust client via the `--multithreaded` flag. This can improve the performance of client-side operations by utilizing multiple CPU cores.

```rust
// Example of how the flag might be used when initializing the client or provider
// let provider = AnchorProvider::new(connection, wallet, opts.with_multithreaded());
```

---

### Anchor Lang: Improved Init Error Messages (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides improved error messages when required programs are missing during the `init` constraint. This helps developers diagnose and resolve initialization issues more effectively.

```Rust
lang: Improved error msgs when required programs are missing when using the `init` constraint(#1257)
```

---

### Anchor ts: Instruction decoding and formatting

Source: https://www.anchor-lang.com/docs/updates/changelog

Implements instruction decoding and formatting capabilities for the TypeScript client.

```typescript
// ts: Instruction decoding and formatting (#372).
```

---

### Anchor ts: Event listener fix

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses an issue where event listeners were not firing correctly when creating associated accounts in the TypeScript client.

```typescript
// ts: Event listener not firing when creating associated accounts (#356).
```

---

### Rust Enum for Token Extensions

Source: https://www.anchor-lang.com/docs/tokens/extensions

Defines the `ExtensionType` enum used in the Token Extensions Program. This enum specifies all available extensions that can be added to a token mint or account, each representing a different functionality. It includes various extensions like Transfer Fee, Non-Transferable, Interest Bearing, and more, along with test-specific padding extensions.

```rust
/// Extensions that can be applied to mints or accounts.  Mint extensions must
/// only be applied to mint accounts, and account extensions must only be
/// applied to token holding accounts.
#[repr(u16)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum ExtensionType {
    /// Used as padding if the account size would otherwise be 355, same as a
    /// multisig
    Uninitialized,
    /// Includes transfer fee rate info and accompanying authorities to withdraw
    /// and set the fee
    TransferFeeConfig,
    /// Includes withheld transfer fees
    TransferFeeAmount,
    /// Includes an optional mint close authority
    MintCloseAuthority,
    /// Auditor configuration for confidential transfers
    ConfidentialTransferMint,
    /// State for confidential transfers
    ConfidentialTransferAccount,
    /// Specifies the default Account::state for new Accounts
    DefaultAccountState,
    /// Indicates that the Account owner authority cannot be changed
    ImmutableOwner,
    /// Require inbound transfers to have memo
    MemoTransfer,
    /// Indicates that the tokens from this mint can't be transferred
    NonTransferable,
    /// Tokens accrue interest over time,
    InterestBearingConfig,
    /// Locks privileged token operations from happening via CPI
    CpiGuard,
    /// Includes an optional permanent delegate
    PermanentDelegate,
    /// Indicates that the tokens in this account belong to a non-transferable
    /// mint
    NonTransferableAccount,
    /// Mint requires a CPI to a program implementing the "transfer hook"
    /// interface
    TransferHook,
    /// Indicates that the tokens in this account belong to a mint with a
    /// transfer hook
    TransferHookAccount,
    /// Includes encrypted withheld fees and the encryption public that they are
    /// encrypted under
    ConfidentialTransferFeeConfig,
    /// Includes confidential withheld transfer fees
    ConfidentialTransferFeeAmount,
    /// Mint contains a pointer to another account (or the same account) that
    /// holds metadata
    MetadataPointer,
    /// Mint contains token-metadata
    TokenMetadata,
    /// Mint contains a pointer to another account (or the same account) that
    /// holds group configurations
    GroupPointer,
    /// Mint contains token group configurations
    TokenGroup,
    /// Mint contains a pointer to another account (or the same account) that
    /// holds group member configurations
    GroupMemberPointer,
    /// Mint contains token group member configurations
    TokenGroupMember,
    /// Mint allowing the minting and burning of confidential tokens
    ConfidentialMintBurn,
    /// Tokens whose UI amount is scaled by a given amount
    ScaledUiAmount,
    /// Tokens where minting / burning / transferring can be paused
    Pausable,
    /// Indicates that the account belongs to a pausable mint
    PausableAccount,
    /// Test variable-length mint extension
    #[cfg(test)]
    VariableLenMintTest = u16::MAX - 2,
    /// Padding extension used to make an account exactly Multisig::LEN, used
    /// for testing
    #[cfg(test)]
    AccountPaddingTest,
    /// Padding extension used to make a mint exactly Multisig::LEN, used for
    /// testing
    #[cfg(test)]
    MintPaddingTest,
}
```

---

### Fix Using Const Generics with declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves an issue related to the usage of const generics with the `declare_program!` macro.

```rust
lang: Fix using const generics with `declare_program!` (#2965).
```

---

### Fix Using IDLs with Defined Types as Generic Arguments (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects the handling of Interface Description Languages (IDLs) that use defined types as generic arguments.

```typescript
ts: Fix using IDLs that have defined types as generic arguments (#3016).
```

---

### Add Write Buffers for IDL Upgrades

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds write buffer functionality for IDL upgrades. This feature likely assists in managing changes to the Interface Definition Language (IDL) across different versions.

```bash
# Command related to IDL management and upgrades

```

---

### Anchor CLI: `--no-idl` Flag for `build`

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `--no-idl` flag to the `build` command in the Anchor CLI. This flag prevents the generation of the IDL file during the build process, which can be useful for optimizing build times or when the IDL is not needed.

```bash
anchor build --no-idl
```

---

### Optional Commitment Argument for Fetch (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds an optional `commitment` argument to the `fetch` and `fetchMultiple` methods in the TypeScript client. This allows users to specify the desired confirmation level for data retrieval.

```typescript
async fetch(address: PublicKey, commitment?: Commitment): Promise<AccountInfo | null>;
async fetchMultiple(addresses: PublicKey[], commitment?: Commitment): Promise<(AccountInfo | null)[]>;

```

---

### Discriminator Trait Usage

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Shows how to replace calls to the removed `discriminator()` method with the `DISCRIMINATOR` associated constant in the `Discriminator` trait. This is useful for calculating account space dynamically.

```Rust
space = MyAccount::DISCRIMINATOR.len() + ...
```

---

### Anchor Rust: Invoke Transfer CPI

Source: https://www.anchor-lang.com/docs/basics/cpi

Shows the final step of invoking a Cross Program Invocation (CPI) to the System Program's transfer instruction. It passes the prepared `CpiContext` and the transfer amount to the `transfer` function.

```rust
transfer(cpi_context, amount)?;

```

---

### Rename instruction::state::Ctor to instruction::state::New

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: Renames the generated `instruction::state::Ctor` struct to `instruction::state::New`. This affects how state constructors are referenced in generated code.

```rust
use anchor_lang::solana_program::borsh::BorshSchema;

#[derive(BorshSchema)]
struct MyState;

// Previously: instruction::state::Ctor::new(...)
// Now: instruction::state::New::new(...)

```

---

### Retrieve and Decode Event Data from Transaction (TypeScript)

Source: https://www.anchor-lang.com/docs/features/events

This TypeScript code snippet shows how to fetch transaction data using its signature, extract the CPI instruction containing event data, and then decode that data using Anchor's utilities. This is necessary because events emitted via `emit_cpi!()` are embedded in CPI instruction data.

```typescript
// 1. Fetch the full transaction data using the transaction signature
const transactionData = await program.provider.connection.getTransaction(
  transactionSignature,
  { commitment: "confirmed" }
);
// 2. Extract the CPI (inner instruction) that contains the event data
const eventIx = transactionData.meta.innerInstructions[0].instructions[0];
// 3. Decode the event data
const rawData = anchor.utils.bytes.bs58.decode(eventIx.data);
const base64Data = anchor.utils.bytes.base64.encode(rawData.subarray(8));
const event = program.coder.events.decode(base64Data);
console.log(event);
```

---

### Anchor Lang: Payer Mutability Enforcement (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Enforces that the payer for an initialized account must be marked as mutable (`mut`). This ensures that the payer account has the necessary permissions for initialization operations.

```Rust
lang: Enforce that the payer for an init-ed account be marked `mut` (#1271).
```

---

### Better Error Messages in TypeScript Client (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides improved error messages in the TypeScript client when incorrect account types (e.g., not a `pubkey` or string) are provided in an instruction's accounts object.

```typescript
const tx = await program.methods.myMethod({ ... }).accounts({ myAccount: "invalid-address" }).rpc();
```

---

### Fix Instruction Return Type Generation with declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects the generation of instruction return types when using the `declare_program!` macro.

```rust
lang: Fix instruction return type generation with `declare_program!` (#2977).
```

---

### Add Instruction Method to State Namespace

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds an `instruction` method to the state namespace in TypeScript. This provides a convenient way to access and construct instructions related to program state.

```ts
const ix = await program.state.instruction.initialize(...);

```

---

### Anchor Lang: Context Bumps Argument (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a `bumps: BTree<String, u8>` argument to the `Context` struct. This allows direct access to bump seeds mapped by account name within the accounts context, simplifying bump seed management.

```Rust
lang: `Context` now has a new `bumps: BTree<String, u8>` argument, mapping account name to bump seed "found" by the accounts context. This allows one to access bump seeds without having to pass them in from the client or recalculate them in the handler (#1367).
```

---

### Anchor SPL: anchor-debug Feature

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `anchor-debug` feature for the `anchor-spl` crate. This feature can be enabled for enhanced debugging capabilities when working with SPL programs.

```rust
# In Cargo.toml:
# anchor-spl = { version = "...", features = ["anchor-debug"] }
```

---

### Anchor TS: `accountsPartial` Method

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `accountsPartial` method in the Anchor TypeScript client. This method preserves the behavior of the older `accounts` method, providing backward compatibility for existing projects.

```typescript
import * as anchor from "@coral-xyz/anchor";

// Assuming program is an Anchor program instance
const accounts = await program.account.myAccount.all();

// Using the new method (hypothetical)
const partialAccounts = await program.account.myAccount.allPartial();
```

---

### Add `approve_checked` Function for SPL Tokens

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `approve_checked` function for SPL tokens. This function enables setting token allowances with additional checks, ensuring the approved amount and decimals are valid.

```rust
use anchor_spl::token::approve_checked;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveChecked<'info> {
    #[account(mut)]
    pub owner: Account<'info, TokenAccount>,
    #[account(mut)]
    pub delegate: AccountInfo<'info>,
    #[account(token::mint = owner.mint)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ApproveChecked>, amount: u64, decimals: u8) -> Result<()>
{
    approve_checked(
        ctx.accounts.owner.to_account_info(),
        ctx.accounts.delegate.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.owner.to_account_info(),
        amount,
        decimals,
    )
}
```

---

### Rust: Repository Rust Version Update

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Rust version used within the Anchor repository to 1.62.

```rust
# Ensure your Rust toolchain is updated:
# rustup update stable
# rustup default stable

```

---

### Anchor Cargo Flags Passthrough

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows passing through Cargo flags for build, test, verify, and publish commands. This provides greater flexibility when customizing the build or test process using Cargo.

```rust
cli: Allow passing through cargo flags for build command (#719).
cli: Allow passing through cargo flags for test, verify, and publish commands (#804).
```

---

### Allow Overriding State Account Size

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables overriding the default size of a state account in the Anchor language. This is useful for optimizing storage or accommodating specific data structures.

```rust
#[account(size = 1024)]
struct MyState {
    // ...
}

```

---

### Anchor lang: Feature flag for old state account discriminator

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides a temporary feature flag to use the old state account discriminator for compatibility with programs built before v0.7.0.

```rust
/// lang: Add feature flag for using the old state account discriminator. This is a temporary flag for those with programs built prior to v0.7.0 but want to use the latest Anchor version. Expect this to be removed in a future version (#446).
```

---

### Update Anchor TypeScript package to @coral-xyz/anchor

Source: https://www.anchor-lang.com/docs/updates/changelog

The TypeScript package for Anchor has been updated from `@project-serum/anchor` to `@coral-xyz/anchor`. This change is part of the monorepo migration and requires building the borsh package before the anchor package.

```typescript
// @coral-xyz/borsh' package is now part of the yarn monorepo (#2290).
// The borsh package needs to be built before the anchor package can be built but this should happen automatically when running `yarn build` in packages/anchor, see #2299 and #2306.
// Switch from `@project-serum/anchor` to the `@coral-xyz/anchor` package #2318.
```

---

### Fix IDL Generation with Unsupported Expressions (IDL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves issues encountered during IDL generation when using unsupported expressions.

```rust
idl: Fix generation with unsupported expressions (#3033).
```

---

### Anchor CLI: Fix Account Command Panic

Source: https://www.anchor-lang.com/docs/updates/changelog

The `anchor account` command no longer panics when executed outside of a workspace directory.

```bash
# This command should now execute gracefully even outside a workspace
anchor account

```

---

### Rust: AccountsClose Manual Call Safety

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates `AccountsClose` in Anchor Rust to be safe for manual calling, providing more control over account closing.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut, close = destination)]
    pub account_to_close: AccountInfo<'info>,
    pub destination: AccountInfo<'info>,
}

// Now it's safer to manually invoke the closing logic if needed.
```

---

### Anchor Rust: Set Data with require! Validation

Source: https://www.anchor-lang.com/docs/features/errors

This Rust code snippet demonstrates using the `require!` macro in Anchor to validate data before setting it. It checks if the input data's value is less than 100, returning a custom error `MyError::DataTooLarge` if the condition is false. It also shows how to define custom errors using an enum.

```Rust
#[program]
mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: MyAccount) -> Result<()> {


        require!(data.data < 100, MyError::DataTooLarge);
        ctx.accounts.my_account.set_inner(data);
        Ok(())
    }
}
#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge
}
```

---

### Anchor.toml: Workspace Exclude Configuration

Source: https://www.anchor-lang.com/docs/references/anchor-toml

Specifies programs to be excluded from the workspace, acting as the opposite of `workspace.members`. This is useful for excluding specific programs from being compiled or recognized by the Anchor CLI.

```toml
[workspace]
exclude = [
    "programs/my_program"
]

```

---

### Rust - Define Zero Copy Account

Source: https://www.anchor-lang.com/docs/features/zero-copy

Defines a struct for a zero-copy account in Anchor, annotated with `#[account(zero_copy)]`. This struct will hold the account data and is designed for direct memory access.

```rust
#[account(zero_copy)]
pub struct Data {
    // 10240 bytes - 8 bytes account discriminator
    pub data: [u8; 10232],
}
```

---

### Anchor UncheckedAccount Alias

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `UncheckedAccount` as a preferred alias for `AccountInfo`. This provides a more explicit way to handle accounts when the exact type is not immediately known or relevant.

```rust
lang: `UncheckedAccount` type introduced as a preferred alias for `AccountInfo` (#745).
```

---

### Anchor TS: Optional Wallet Property for Provider

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces an optional `wallet` property to the `Provider` interface in the Anchor TypeScript client. This allows for more flexible configuration of wallet management.

```typescript
interface Provider {
  // ... other properties
  wallet?: any; // Or a specific Wallet interface
}
```

---

### Anchor SPL: Revoke Instruction

Source: https://www.anchor-lang.com/docs/updates/changelog

Support for the revoke instruction has been added to the Anchor SPL token module. This allows programs to revoke delegated authority over tokens.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token;

#[derive(Accounts)]
pub struct Revoke<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub delegate: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[program]
mod my_program {
    pub fn revoke_tokens(ctx: Context<Revoke>) -> Result<()>
}
```

---

### Declare Program ID

Source: https://www.anchor-lang.com/docs/basics/program-structure

Specifies the on-chain address of the Anchor program using the `declare_id` macro. This macro sets the program's unique identifier on the Solana blockchain.

```Rust
use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");
```

---

### Anchor IDL: Check Ambiguous Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor IDL tooling now checks for ambiguous discriminators, helping to prevent conflicts and ensure clarity in program interfaces.

```bash
# Example command that might trigger this check:
```

---

### TypeScript: Seeds Inference for Nested User-Defined Structs

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates seeds inference in TypeScript to correctly handle nested user-defined structs within seeds.

```typescript
// Example of nested struct for seeds:
// interface MyNestedStruct { value: number; }
// const seeds = [Buffer.from('prefix'), Buffer.from(JSON.stringify(myNestedStruct))];
```

---

### Update JS/TS template to use new AnchorProvider class

Source: https://www.anchor-lang.com/docs/updates/changelog

The JavaScript/TypeScript templates generated by Anchor CLI have been updated to utilize the new `AnchorProvider` class.

```typescript
// Update js/ts template to use new `AnchorProvider` class (#1770).
```

---

### Add Version Number to Extracted IDL (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Includes the version number from the program's `Cargo.toml` into the extracted IDL. This helps in tracking the specific version of a program associated with its IDL.

```bash
anchor build -- --crate-version 1.2.3
```

---

### Fix Using Vec<u8> Type with declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects an issue with using the `Vec<u8>` type when employing the `declare_program!` macro.

```rust
lang: Fix using `Vec<u8>` type with `declare_program!` (#2966).
```

---

### Skip Local Validator Creation When Testing

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds an option to skip the creation of a local validator when running tests against `localnet`. This is useful if a local validator is already running or if testing against a remote localnet instance.

```bash
anchor test --skip-local-validator

```

---

### Make Tuple Struct Fields Public in declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Ensures that tuple struct fields are public when using the `declare_program!` macro.

```rust
lang: Make tuple struct fields public in `declare_program!` (#2994).
```

---

### Remove anchor-syn Dependency (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the `anchor-syn` dependency from the project, streamlining build processes.

```rust
idl: Remove `anchor-syn` dependency (#3030).
```

---

### Anchor Lang: Program Data Address Deserialization

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Program` type in Anchor Lang now deserializes the `programdata_address` only when it's needed. This lazy deserialization can improve performance by avoiding unnecessary computations.

```rust
Program::new(program_id).programdata_address()
```

---

### TypeScript: Custom Resolver for Missing Accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the ability to resolve missing accounts using a custom resolver function in the TypeScript client.

```typescript
import { Program } from "@coral-xyz/anchor";

const customResolver = async (
  programId: PublicKey,
  seeds: Buffer[]
): Promise<PublicKey> => {
  // Custom logic to find or derive the account address
  return derivedAddress;
};

// Use the custom resolver when fetching or building transactions
// const account = await program.account.myAccount.fetch(address, { resolver: customResolver });
```

---

### Anchor CLI: Overflow Checks Flag

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI now requires an explicit `--overflow-checks` flag, allowing users to control overflow checking behavior during builds.

```bash
anchor build --overflow-checks
```

---

### Enforce Signer Constraint with CPI Feature (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Accounts marked with the `#[account(signer)]` constraint now enforce the signer requirement when the `"cpi"` feature is enabled. This adds an extra layer of security for CPI operations.

```rust
#[account(signer)]
struct MyAccount {...}
```

---

### Test Clock Sysvar Manipulation with LiteSVM

Source: https://www.anchor-lang.com/docs/testing/litesvm

Demonstrates how to dynamically overwrite the Clock sysvar using LiteSVM in Rust. This allows testing program behavior based on specific timestamps, simulating time travel by setting the clock to different values and verifying program responses.

```Rust
use {
    litesvm::LiteSVM,
    solana_clock::Clock,
    solana_instruction::Instruction,
    use solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::VersionedTransaction,
};
fn test_set_clock() {
    let program_id = Pubkey::new_unique();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../node-litesvm/program_bytes/litesvm_clock_example.so");
    svm.add_program(program_id, bytes);
    let payer = Keypair::new();
    let payer_address = payer.pubkey();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    let blockhash = svm.latest_blockhash();
    let ixs = [Instruction {
        program_id,
        data: vec![],
        accounts: vec![],
    }];
    let msg = Message::new_with_blockhash(&ixs, Some(&payer_address), &blockhash);
    let versioned_msg = VersionedMessage::Legacy(msg);
    let tx = VersionedTransaction::try_new(versioned_msg, &[&payer]).unwrap();
    // set the time to January 1st 2000
    let mut initial_clock = svm.get_sysvar::<Clock>();
    initial_clock.unix_timestamp = 1735689600;
    svm.set_sysvar::<Clock>(&initial_clock);
    // this will fail because it's not January 1970 anymore
    svm.send_transaction(tx).unwrap_err();
    // so let's turn back time
    let mut clock = svm.get_sysvar::<Clock>();
    clock.unix_timestamp = 50;
    svm.set_sysvar::<Clock>(&clock);
    let ixs2 = [Instruction {
        program_id,
        data: vec![1], // unused, this is just to dedup the transaction
        accounts: vec![],
    }];
    let msg2 = Message::new_with_blockhash(&ixs2, Some(&payer_address), &blockhash);
    let versioned_msg2 = VersionedMessage::Legacy(msg2);
    let tx2 = VersionedTransaction::try_new(versioned_msg2, &[&payer]).unwrap();
    // now the transaction goes through
    svm.send_transaction(tx2).unwrap();
}
```

---

### Account Type Location Change (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Account types are now located either in the `prelude` module or the `accounts` module, and no longer directly under the root. Deprecated account types are also no longer imported by the prelude.

```rust
use anchor_lang::prelude::*;
// or
use anchor_lang::accounts::*;
```

---

### Remove bpf Target Support in Hash Feature (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Discontinues support for the `bpf` target within the `hash` feature.

```rust
syn: Remove `bpf` target support in `hash` feature (#3078).
```

---

### Rust Fixes in Anchor Lang

Source: https://www.anchor-lang.com/docs/updates/changelog

This snippet summarizes fixes related to Rust code within the Anchor Lang project. It includes improvements to safety checks, IDL generation, program declaration syntax, account handling, and dependency management.

```Rust
lang: Fix `align` repr support in `declare-program!` (#3056).
lang: Make stack frames slimmer on ATA creation (#3065).
lang: Remove `getrandom` dependency (#3072).
lang: Make `InitSpace` support unnamed & unit structs (#3084).
lang: Fix using `owner` constraint with `Box`ed accounts (#3087).
lang: Add a sanity check for unimplemented token extensions (#3090).
lang: Remove unnecessary clone in account exit routine (#3139).
lang: Fix compilation warnings due to unused deprecated program id macros (#3170).
lang: Remove `arrayref` dependency (#3201).
lang: Use closures for `init` constraints to reduce the stack usage of `try_accounts` (#2939).
lang: Allow the `cfg` attribute above the instructions (#2339).
lang: Fix constant bytes declarations when using `declare_program!` (#3287).
lang: Fix using non-instruction composite accounts with `declare_program!` (#3290).
lang: Remove a potential panic while getting the IDL in `declare_program!` (#3458).
lang: Fix adding `derive`s and `repr`s to type alias definitions in `declare_program!` (#3504).
lang: Fix instructions with no accounts causing compilation errors when using `declare_program!` (#3567).
lang: Fix using `data` as an instruction parameter name in `declare_program!` (#3574).
lang: Require `zero` accounts to be unique (#3409).
lang: Deduplicate `zero` accounts against `init` accounts (#3422).
lang: Fix `cpi` feature instructions not accounting for discriminator overrides (#3376).
```

---

### Anchor TypeScript Provider Update

Source: https://www.anchor-lang.com/docs/updates/changelog

Modifies the Anchor TypeScript client library's `Provider` to require a `publicKey` instead of a `wallet` in its accounts resolver, introduced in version 0.31.1. This change affects how accounts are managed in the client.

```text
ts: Make `Provider` require publicKey instead of wallet in accounts resolver (#3613)
```

---

### Anchor Fix: IDL Constant Seeds Parsing

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects an issue with parsing constant seeds within the IDL. This ensures that program-derived addresses (PDAs) using constant seeds are generated correctly.

```json
{
  "name": "MyProgram",
  "constants": [
    {
      "name": "MY_SEED",
      "type": "string",
      "value": "my_seed_value"
    }
  ],
  "instructions": [
    {
      "name": "create_pda",
      "accounts": [
        {
          "name": "pda_account",
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "account": "authority",
                "property": "authority"
              },
              { "kind": "constant", "value": "MY_SEED" }
            ],
            "programId": "<program_id>"
          }
        }
      ]
    }
  ]
}
```

---

### Set Inner Method for Account Updates (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `set_inner` method to `Account<'a, T>`, facilitating easy updates to the account's data. This is useful for modifying account state within instructions.

```rust
impl<'a, T> Account<'a, T> {
    pub fn set_inner(&mut self, inner: T) {
        self.0.set_inner(inner);
    }
}
```

---

### CLI: Validator Ticks Per Slot Option

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `ticks_per_slot` option to Validator arguments in the Anchor CLI, allowing tuning of the local validator's performance.

```bash
# Start local validator with custom ticks per slot
anchor localnet --ticks-per-slot 1000

```

---

### Anchor TS: Program.addEventListener Type Support

Source: https://www.anchor-lang.com/docs/updates/changelog

Strong type support has been added for the `Program.addEventListener` method in the TypeScript SDK, improving type safety when listening for program events.

```typescript
import * as anchor from "@coral-xyz/anchor";

interface MyEvent {
  data: string;
  value: number;
}

// Assuming 'program' is an Anchor Program instance
const listener = program.addEventListener(
  "MyEventName",
  (event: MyEvent, slot) => {
    console.log(
      `Received event: ${event.data}, value: ${event.value} at slot ${slot}`
    );
  }
);

// To remove the listener:
// program.removeEventListener(listener);
```

---

### Anchor Lang: Type Aliases in IDLs

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor Lang now supports type aliases in IDLs, allowing for more readable and maintainable IDL definitions.

```rust
use anchor_lang::prelude::*;

type MyUint64 = u64;

#[derive(Accounts)]
pub struct MyAccount<'info> {
    #[account(owner = MyProgram::id())]
    pub my_account: Account<'info, MyData>,
}

#[account]
pub struct MyData {
    pub value: MyUint64, // Using the type alias
}

```

---

### Anchor Lang: Update dispatch Function for Dynamic Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

The `dispatch` function in Anchor's Rust library has been updated to accommodate dynamic discriminators, improving its adaptability to various program designs.

```rust
pub fn dispatch(ctx: &mut CpiContext<'_, '_>, data: &[u8]) -> Result<()> { ... }
```

---

### Transfer Tokens via CPI with PDA Authority

Source: https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens

This function facilitates token transfers from a PDA-owned account to a recipient. It uses Cross Program Invocation (CPI) and requires the PDA's seeds to sign the transaction, ensuring the program controls the transfer.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};

#[program]
pub mod token_example {
    // ... other functions
    pub fn transfer_tokens(ctx: Context<TransferTokens>) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"token", &[ctx.bumps.sender_token_account]]];
        let amount = ctx.accounts.sender_token_account.amount;
        let decimals = ctx.accounts.mint.decimals;
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.sender_token_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
        token_interface::transfer_checked(cpi_context, amount, decimals)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = sender_token_account,
        seeds = [b"token"],
        bump
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
```

---

### Custom Discriminators in Anchor

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-31-0

Demonstrates how to override default 8-byte discriminators using the `discriminator` argument in Anchor attribute macros. Supports constant expressions for custom discriminator values.

```Rust
#[account(discriminator = 1)]
#[event(discriminator = [1, 2])]
#[instruction(discriminator = MY_CONST)]
```

---

### Change Program Constructor's idl Parameter Type to any (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Modifies the `Program` constructor in TypeScript to accept the `idl` parameter as type `any`.

```typescript
ts: Change the `Program` constructor's `idl` parameter type to `any` (#3181).
```

---

### TypeScript: Nested PDA Inference

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables nested PDA inference in the TypeScript client, simplifying the process of deriving program-derived addresses for nested structures.

```typescript
// Example of nested PDA inference:
// const nestedPda = await program.methods.getNestedPda().pre ઓlve();
```

---

### Change Event Coder Decode API

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: The event coder's `decode` API now accepts strings directly instead of buffers. The `encode` API has also been removed.

```ts
eventCoder.decode(string);
```

---

### Anchor Lang: `Event` Utility Type

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces an `Event` utility type in Anchor Lang. This type helps in retrieving and parsing events emitted by Anchor programs from raw byte data.

```rust
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
struct MyEvent {
    // event fields
}

// Example usage (hypothetical)
let event_data: Vec<u8> = vec![...]; // Raw event data
let event = MyEvent::try_from_bytes(&event_data)?;
println!("Parsed event: {:?}", event);

```

---

### Anchor SPL: Withdraw Withheld Tokens Instruction

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `withdraw_withheld_tokens_from_accounts` instruction to the Anchor SPL library. This instruction facilitates the withdrawal of withheld tokens from specified accounts.

```rust
use anchor_spl::token::WithdrawWithheldTokensFromAccounts;
```

---

### Namespace Account Discriminator to "internal"

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: Changes the account discriminator namespace for `IdlAccount` to "internal". This affects how account metadata is represented in the IDL.

```rust
// The discriminator for IdlAccount is now namespaced as "internal"

```

---

### Log Instruction Name by Default (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Called instructions now log their names by default. This behavior can be disabled using the `no-log-ix-name` flag, aiding in debugging and tracing instruction execution.

```rust
invoke_signed(&ix, &[&signer1, &signer2], None).await?;
// or with flag
invoke_signed(&ix, &[&signer1, &signer2], Some(&[b"no-log-ix-name"])).await?;
```

---

### Rename RequestBuilder::new to RequestBuilder::from

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: Renames the `RequestBuilder::new` constructor to `RequestBuilder::from` in the client. This change might reflect a shift in how request builders are initialized.

```rust
let builder = RequestBuilder::from(program_id);

```

---

### Anchor Lang: Custom Lifetime in Accounts

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows the use of custom lifetimes within `Accounts` structures in Anchor Lang. This provides more flexibility when defining account structures that may involve references with specific lifetimes.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MyAccounts<'info, 'a> {
    #[account(mut)]
    pub owner: &'a Signer<'info>,
    // other accounts...
}

```

---

### Anchor ts: Optional address metadata for workspace clients

Source: https://www.anchor-lang.com/docs/updates/changelog

Makes address metadata optional for `anchor.workspace` clients in the TypeScript SDK.

```typescript
// ts: Address metadata is now optional for `anchor.workspace` clients (#310).
```

---

### Anchor Rust: Validate Amount with Custom Errors

Source: https://www.anchor-lang.com/docs/features/errors

This Rust code snippet shows how to validate an amount within a specified range (10 to 100) using the `require!` macro in Anchor. It defines custom errors `AmountTooSmall` and `AmountTooLarge` with specific messages and demonstrates their usage within a Solana program.

```Rust
use anchor_lang::prelude::*;
declare_id!("9oECKMeeyf1fWNPKzyrB2x1AbLjHDFjs139kEyFwBpoV");
#[program]
pub mod custom_error {
    use super::*;
    pub fn validate_amount(_ctx: Context<ValidateAmount>, amount: u64) -> Result<()> {


        require!(amount >= 10, CustomError::AmountTooSmall);
        require!(amount <= 100, CustomError::AmountTooLarge);
        msg!("Amount validated successfully: {}", amount);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct ValidateAmount {}
#[error_code]
pub enum CustomError {
    #[msg("Amount must be greater than or equal to 10")]
    AmountTooSmall,
    #[msg("Amount must be less than or equal to 100")]
    AmountTooLarge,
}
```

---

### Eliminate Variable Allocations for Token Extension Code Generation (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Reduces stack space usage by eliminating unnecessary variable allocations during token extension code generation.

```rust
lang: Eliminate variable allocations that build up stack space for token extension code generation (#2913).
```

---

### Client Fixes in Anchor Lang

Source: https://www.anchor-lang.com/docs/updates/changelog

This snippet addresses a specific fix within the client-side implementation of the Anchor Lang project, focusing on the removal of direct usage of `std::process::exit`.

```Rust
client: Remove `std::process::exit` usage (#3544).
```

---

### Rust: Heap Intensive Error Mapping Fix

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses an issue with heap-intensive error mapping in the Rust implementation, improving performance and stability.

```rust
// Error handling and mapping should be optimized to prevent excessive memory usage.
```

---

### Upgrade Node to 20.16.0 LTS (Docker)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the Node.js version used in Docker builds to 20.16.0 LTS.

```docker
docker: Upgrade `node` to 20.16.0 LTS (#3179).
```

---

### Anchor Lang: Automatic PDA Address Inference (Rust, TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Enables automatic inference of PDA addresses, simplifying the process of generating and managing program-derived addresses. This feature benefits both Rust and TypeScript development.

```Rust
lang, ts: Automatically infer PDA addresses (#1331).
```

---

### Anchor Signer Type IDL

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Signer` type now correctly sets `isSigner` to true in the Interface Description Language (IDL). This ensures that off-chain clients can correctly identify signer accounts.

```rust
lang: `Signer` type now sets isSigner to true in the IDL (#750).
```

---

### Anchor Lang: Eliminate Temporary Vec Allocations

Source: https://www.anchor-lang.com/docs/updates/changelog

Reduces heap memory usage by eliminating temporary `Vec` allocations when serializing data with a discriminant. The default capacity for these allocations has been set to 256 bytes.

```rust
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
struct MyData {
    value: u64,
}

// The serialization process is optimized to avoid unnecessary Vec allocations.
// This is an internal optimization within anchor_lang.

```

---

### Allow State Structs Without Ctor or Impl Block

Source: https://www.anchor-lang.com/docs/updates/changelog

Permits state structs in the Anchor language that do not have an explicit constructor (`ctor`) or implementation block, as long as they implement necessary traits. This offers more flexibility in defining state.

```rust
#[account]
struct MyState {
    // ...
}

// Trait implementations can be defined elsewhere

```

---

### Export Sighash Coder Function

Source: https://www.anchor-lang.com/docs/updates/changelog

Exports the sighash coder function from the TypeScript client. This allows developers to manually generate or work with sighashes for instructions.

```ts
import { sighash } from "@project-serum/anchor/dist/cjs/utils/sighash";

const sighashValue = sighash(instructionName, instructionData);
```

---

### Add anchor-debug Feature Flag

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `anchor-debug` feature flag for the Anchor language. This flag enables enhanced logging capabilities, aiding in debugging and understanding program execution flow.

```rust
#[cfg(feature = "anchor-debug")]
println!("Debug message");

```

---

### Anchor Lang: Dynamic Discriminator Length Support

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor's Rust library now supports dynamic discriminator lengths, moving away from the fixed 8-byte standard. This allows for more flexibility in program design.

```rust
#[account(discriminator = "MYDISC")]
```

---

### Rename Internal Identifiers with `__` Prefix

Source: https://www.anchor-lang.com/docs/updates/changelog

Internal identifiers intended for usage within Anchor's framework, such as `program_id`, `accounts`, `ix_data`, and `remaining_accounts`, have been renamed with a `__` prefix. This prevents naming collisions with user-defined variables.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MyInstructionAccounts<'info> {
    // Internal identifiers are now prefixed with '__'
    // pub __program_id: AccountInfo<'info>,
    // pub __remaining_accounts: Vec<AccountInfo<'info>>,
}

pub fn my_instruction(ctx: Context<MyInstructionAccounts>) -> Result<()>
{
    // Accessing internal context variables might now look like:
    // let program_id = ctx.program_id;
    Ok(())
}
```

---

### Anchor TS: Nullable Types for Mapped Types

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor TypeScript now allows nullable types for `Option<T>` mapped types. This improves flexibility when dealing with optional data fields in accounts.

```typescript
import { Account, BorshCoder } from "@project-serum/anchor";

// Assuming an account structure like this:
// interface MyAccount {
//   optionalField: string | null;
// }

// The coder will handle the mapping of Option<String> to string | null
```

---

### Anchor Lang: Export Discriminator Trait from Prelude

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Discriminator` trait is now exported from the `prelude` in Anchor's Rust library. This makes it readily available for use in various program development contexts.

```rust
use anchor_lang::prelude::Discriminator;
```

---

### Anchor CLI: Cargo Feature Resolver

Source: https://www.anchor-lang.com/docs/updates/changelog

The Anchor CLI has switched to using Cargo feature resolver version 2 (`resolver = "2"`). This change impacts how Cargo handles feature dependencies, potentially leading to more predictable and stable builds.

```toml
[package]
name = "anchor_cli"
version = "0.29.0"
resolver = "2"

```

---

### Typed Program<T> with IDL (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Program<T>` type in TypeScript can now be parameterized with an IDL type. This enhances type safety and developer experience when interacting with Anchor programs.

```typescript
interface MyProgramIDL { ... }
const program = new anchor.Program<MyProgramIDL>(idl, programId, provider);
```

---

### Introduce Address Type in TypeScript

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `Address` type in the TypeScript client, enabling the use of Base58 encoded strings in public APIs. This improves usability and consistency when handling account addresses.

```ts
Address;
```

---

### Add `transfer_checked` Function for SPL Tokens

Source: https://www.anchor-lang.com/docs/updates/changelog

Implements the `transfer_checked` function for SPL tokens. This function allows for token transfers with additional checks, such as verifying the token amount and decimals.

```rust
use anchor_spl::token::transfer_checked;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferChecked<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(token::mint = from.mint)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferChecked>, amount: u64, decimals: u8) -> Result<()>
{
    transfer_checked(
        ctx.accounts.from.to_account_info(),
        ctx.accounts.to.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.from.to_account_info(),
        amount,
        decimals,
    )
}
```

---

### TypeScript: RequestBuilder Transaction Functions

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds `transaction` functions to the `RequestBuilder` in the TypeScript client for more streamlined transaction handling.

```typescript
import { RequestBuilder } from "@coral-xyz/anchor";

// Example usage:
const requestBuilder = new RequestBuilder(ix, provider);
const transaction = requestBuilder.transaction();
```

---

### Add Deprecated Attribute to ProgramAccount (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `#[deprecated]` attribute to `ProgramAccount`. This signals that `ProgramAccount` is deprecated and encourages the use of newer alternatives.

```rust
#[account(ProgramAccount)]
struct MyProgramAccount {...}
```

---

### Anchor TS: Optional Blockhash for Provider.sendAndConfirm

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds an optional `blockhash` parameter to the `Provider.sendAndConfirm` method in the Anchor TypeScript client. This allows users to specify a particular blockhash for transaction confirmation.

```typescript
provider.sendAndConfirm(ix, { blockhash: "some_blockhash" });
```

---

### Fix Using Defined Types in Instruction Parameters (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects an issue with using defined types within instruction parameters when using the `declare_program!` macro.

```rust
lang: Fix using defined types in instruction parameters with `declare_program!` (#2959).
```

---

### Anchor SPL: TokenRecordAccount for pNFTs

Source: https://www.anchor-lang.com/docs/updates/changelog

The `TokenRecordAccount` is added for pNFTs (Programmable NFTs), providing specific support for managing token records in the Metaplex ecosystem.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token_metadata::TokenRecordAccount;

#[derive(Accounts)]
pub struct UpdatePnft<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_record: Account<'info, TokenRecordAccount>,
    // ... other accounts
}

```

---

### Rust: Literal Constraint Deprecation

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the deprecated literal constraint syntax in Anchor Rust, encouraging the use of `#[account(constraint = {})]`.

```rust
// Old syntax (deprecated):
// #[account(constraint = "some_condition")]

// New syntax:
// #[account(constraint = { some_condition })]

```

---

### Implement SPL Associated Token Program coder in Anchor TS

Source: https://www.anchor-lang.com/docs/updates/changelog

A coder for the SPL Associated Token Program has been implemented in the Anchor TypeScript tooling, facilitating easier interaction with associated token accounts.

```typescript
// Implement a coder for SPL associated token program (#1939).
```

---

### Anchor Lang: Remove `try_to_vec` Usage

Source: https://www.anchor-lang.com/docs/updates/changelog

Reduces heap memory usage by removing the usage of `try_to_vec` when setting return data. This change aims to improve performance and reduce memory footprint.

```rust
use anchor_lang::prelude::*;

// The change involves internal implementation details of Anchor Lang's serialization.
// Example of where it might have been used:
// let data = MyData { value: 10 };
// let serialized_data = data.try_to_vec()?; // This usage is being removed/optimized.

```

---

### Anchor TS: Spl.token Factory Method

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Spl.token` factory method in Anchor TypeScript now accepts a provider parameter. This allows for specifying the provider when creating token-related objects.

```typescript
import { Provider, Spl } from "@project-serum/anchor";

const provider = new Provider(connection, wallet, {});
const spl = new Spl(provider);

// Example usage:
// const token = spl.token(tokenMint);
```

---

### Anchor Syn: IdlBuild Trait

Source: https://www.anchor-lang.com/docs/updates/changelog

The `IdlBuild` trait is introduced in the `syn` crate to implement IDL support for custom types, enabling more flexible IDL generation.

```rust
use anchor_syn::idl_build::IdlBuild;
use anchor_syn::idl_build::IdlType;

struct MyCustomType {
    field1: u32,
    field2: String,
}

impl IdlBuild for MyCustomType {
    fn idl_type() -> IdlType {
        IdlType::Struct {
            name: "MyCustomType".to_string(),
            fields: vec![
                IdlType::U32.with_name("field1".to_string()),
                IdlType::String.with_name("field2".to_string()),
            ],
        }
    }
}

```

---

### Anchor Lang: Accounts Trait Bump Argument (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Modifies the `try_accounts` method of the `Accounts` trait to include an additional `bumps: &mut BTreeMap<String, u8>` argument. This argument accumulates bump seeds during account processing.

```Rust
lang: The `Accounts` trait's `try_accounts` method now has an additional `bumps: &mut BTreeMap<String, u8>` argument, which accumulates bump seeds (#1367).
```

---

### Anchor CLI: Localhost Test Validator Address

Source: https://www.anchor-lang.com/docs/updates/changelog

Changes the default test validator address from `localhost` to `127.0.0.1`. This addresses potential IP resolution issues, especially with NodeJS 17's IPv6 handling changes.

```bash
# This is a configuration change, typically handled internally or via Anchor.toml
# No direct command, but affects how the CLI connects to the validator.
# Example Anchor.toml configuration:
[test.validator]
address = "127.0.0.1"

```

---

### Move overflow-checks to workspace Cargo.toml

Source: https://www.anchor-lang.com/docs/updates/changelog

The `overflow-checks` setting has been moved to the workspace's `Cargo.toml` file to ensure it is not ignored by the compiler, potentially affecting arithmetic operations.

```rust
// Move `overflow-checks` into workspace `Cargo.toml` so that it will not be ignored by compiler (#1806).
```

---

### Anchor TS: Field Layouts for Aliased Arguments

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes the construction of field layouts for instruction arguments that are type aliases. This ensures that data serialization and deserialization work correctly with aliased types.

```typescript
import * as anchor from "@coral-xyz/anchor";

type MyAmount = u64;

interface InstructionArgs {
  amount: MyAmount;
}

// The fix ensures that 'amount' is correctly handled as a u64.
```

---

### TypeScript: SPL Program Coders

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds coders for SPL programs in the TypeScript client, facilitating easier interaction with SPL token functionalities.

```typescript
import { Program, SplTokenCoder } from "@coral-xyz/anchor";

// Initialize a coder for SPL token programs
const coder = new SplTokenCoder(program.idl);
```

---

### Add MethodsBuilder#accountsStrict for strict account typing in Anchor TS

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `MethodsBuilder#accountsStrict` in the Anchor TypeScript client for enforcing strict typing on instruction account inputs.

```typescript
// Add `MethodsBuilder#accountsStrict` for strict typing on ix account input (#2019).
```

---

### IDL Fixes in Anchor Lang

Source: https://www.anchor-lang.com/docs/updates/changelog

This snippet details fixes applied to the Interface Definition Language (IDL) generation and handling within the Anchor Lang project. It addresses issues with silent failures, test interference, panics, and constraint resolution.

```Rust
idl: Make safety comment checks fail silently when program path env is not set (#3045).
idl: Avoid interference from rust tests during IDL generation (#3058).
idl: Skip IDL checks if `--no-idl` option is passed (#3093).
idl: Fix panicking on tests (#3197).
idl: Fix using `address` constraint with non-const expressions (#3216).
idl: Fix using full path types with `Program` (#3228).
idl: Log output with `ANCHOR_LOG` on failure and improve build error message (#3284).
idl: Fix instructions with tuple parameters not producing an error(#3294).
idl: Fix detecting false-positives from doc comments during module path conversion (#3359).
idl: Ignore compiler warnings during builds (#3396).
idl: Avoid extra IDL generation during `verify` (#3398).
idl: Fix missing `program::seed` resolution (#3474).
idl: Fix using constant identifiers as generic arguments (#3522).
idl: Fix using `Pubkey` constants with `seeds::program` (#3559).
idl: Fix using account or arg values for `seeds::program` (#3570).
```

---

### Add transaction signature to EventCallback parameters

Source: https://www.anchor-lang.com/docs/updates/changelog

The `EventCallback` parameters in the Anchor TypeScript client now include the transaction signature, providing more context for event handling.

```typescript
// Add transaction signature to `EventCallback` parameters (#1851).
```

---

### Fix DNS in NODE_OPTIONS (CLI)

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses an issue with DNS resolution within the `NODE_OPTIONS` environment variable in the CLI. This ensures proper network connectivity for CLI operations.

```bash
export NODE_OPTIONS="--dns-result-order=ipv4first"
```

---

### Anchor spl: `SetAuthority` instruction

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `SetAuthority` instruction for managing account authority within the SPL module.

```spl
# spl: Add `SetAuthority` instruction (#307).
```

---

### Anchor IDL Authority

Source: https://www.anchor-lang.com/docs/references/cli

Outputs the current authority of the IDL account, which is the wallet authorized to update the IDL on the blockchain.

```bash
anchor idl authority <program-id>
```

---

### Require Discriminator Trait Impl for Zero Constraint (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Mandates the implementation of the `Discriminator` trait when using the `zero` constraint.

```rust
lang: Require `Discriminator` trait impl when using the `zero` constraint (#3118).
```

---

### Fix Incorrect maxSupportedTransactionVersion in Provider Methods (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects the `maxSupportedTransactionVersion` value used in the `AnchorProvider.send*()` methods.

```typescript
ts: Fix incorrect `maxSupportedTransactionVersion` in `AnchorProvider.send*()` methods (#2922).
```

---

### Anchor TS: No Assert Dependency

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor TypeScript packages no longer depend on the `assert` module, reducing bundle size and potential conflicts.

```typescript
// No longer requires:
// import assert from 'assert';
```

---

### Anchor TS: Program Stack for Errors

Source: https://www.anchor-lang.com/docs/updates/changelog

Anchor TypeScript now includes an `AnchorError` that contains a program stack, providing more context for errors. For non-`AnchorError` errors, a program stack is also included. Note that `AnchorError` is not returned for processed transactions with `skipPreflight` set to `true`, falling back to `ProgramError` or raw Solana library errors.

```typescript
import { Program, Provider, BN } from "@project-serum/anchor";

// Example of handling potential errors:
// try {
//   await program.methods.someInstruction().rpc();
// } catch (error) {
//   console.error(error);
//   // error object might contain program stack information
// }
```

---

### Anchor Unnamed Struct Handling

Source: https://www.anchor-lang.com/docs/updates/changelog

Ignores `Unnamed` structs instead of panicking. This improves the robustness of Anchor when encountering un-named struct definitions.

```rust
lang: Ignore `Unnamed` structs instead of panic (#605).
```

---

### Anchor Lang: Workspace and Wallet Exports (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds type declarations for conditional `workspace` and `Wallet` exports in TypeScript. This improves type safety and developer experience when working with these exports.

```TypeScript
ts: Add type declarations for conditional `workspace` and `Wallet` exports (#1137).
```

---

### Anchor Lang: #[instruction] Attribute for Custom Discriminators

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `#[instruction]` attribute proc-macro in Anchor's Rust library. This allows developers to override default instruction discriminators, providing greater control over instruction identification.

```rust
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MyInstruction<'info> { ... }

#[instruction(discriminator = "MYINST")]
pub fn my_instruction(ctx: Context<MyInstruction>) -> Result<()> { ... }
```

---

### Replace Cluster URL String with Cluster Struct

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: Replaces the use of URL strings with a `Cluster` struct when constructing clients. This provides a more robust and type-safe way to specify network endpoints.

```rust
use anchor_client::Cluster;

let cluster = Cluster::Devnet;
let client = Client::new_with_options(cluster, keypair, options);

```

---

### Fix Path Resolution of Cargo.lock for External Types (IDL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves an issue with path resolution for `Cargo.lock` when generating IDLs for external types.

```rust
idl: Fix path resolution of the `Cargo.lock` of the project when generating idls for external types (#2946).
```

---

### Anchor TS: Program ID Removal

Source: https://www.anchor-lang.com/docs/updates/release-notes/0-30-0

Illustrates the removal of the `programId` parameter when instantiating the `Program` class. The program ID is now expected to be stored in the IDL's `address` field.

```javascript
new Program(idl);
```

---

### Anchor TypeScript IDL Parsing Fix

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves a bug in the Anchor TypeScript client where the IDL parser failed when encountering multiple constant generics, fixed in version 0.31.1.

```text
ts: Fix parsing IDL with multiple const generics (#3665)
```

---

### Anchor Lang: Loader Account Module Rename (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Renames the `loader_account` module to `account_loader` in Rust. This change improves the clarity and organization of account-related modules.

```Rust
lang: rename `loader_account` module to `account_loader` module (#1279)
```

---

### Error Enum Name and Message Update (Rust, TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Updates the error enum name and message for 'wrong program ownership' account validation. This provides clearer error reporting for account validation failures.

```rust
enum AccountError {
    WrongProgramOwner,
}

```

---

### Anchor Fix: IDL Consistency

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses inconsistencies within the IDL specification. This includes ensuring consistent casing and correct representation of data types and structures.

```json
{
  "name": "MyProgram",
  "types": [
    {
      "name": "MyStruct",
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "someField", "type": "u64" } // Ensuring consistent casing
        ]
      }
    }
  ]
}
```

---

### ProgramErrorWithOrigin Struct

Source: https://www.anchor-lang.com/docs/features/errors

Defines the `ProgramErrorWithOrigin` struct, a variant of Anchor's `Error` type. It wraps a standard Solana `ProgramError` and includes optional origin information and compared values.

```Rust
#[derive(Debug)]
pub struct ProgramErrorWithOrigin {
    pub program_error: ProgramError,
    pub error_origin: Option<ErrorOrigin>,
    pub compared_values: Option<ComparedValues>,
}
```

---

### Fix Potential Panic on External Type Resolution (IDL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Addresses a potential panic that could occur during the resolution of external types in IDL generation.

```rust
idl: Fix potential panic on external type resolution (#2954).
```

---

### Rust: IDL Bytes Literal Parsing Fix

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes an issue in the Rust IDL generation related to parsing bytes literals.

```rust
// Ensure byte literals like `b"data"` are parsed correctly in the IDL.
```

---

### Anchor cli: Remove `--yarn` flag

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the `--yarn` flag from the Anchor CLI, recommending the use of `npx` instead.

```cli
# cli: Remove `--yarn` flag in favor of using `npx` (#432).
```

---

### Add @types/bn.js to devDependencies in CLI template

Source: https://www.anchor-lang.com/docs/updates/changelog

Includes `@types/bn.js` in the `devDependencies` of the Anchor CLI template, ensuring proper TypeScript type definitions for BN.js.

```bash
Add `@types/bn.js` to `devDependencies` in cli template (#1712).
```

---

### Remove async_rpc Method (Client)

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the `async_rpc` method from the Anchor client library.

```rust
client: Remove `async_rpc` method (#3053).
```

---

### Anchor TS: Case Conversion Fixes

Source: https://www.anchor-lang.com/docs/updates/changelog

Corrects issues related to case conversion in the TypeScript client, specifically when using numbers in instruction, account, or event names. This ensures that names are handled correctly regardless of their casing.

```typescript
// Example: Instruction name might be "process_123"
// The
```

---

### Fix ProgramError::ArithmeticOverflow Not Found Error (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Resolves an error where `ProgramError::ArithmeticOverflow` was not found.

```rust
lang: Fix `ProgramError::ArithmeticOverflow` not found error (#2975).
```

---

### Add view functions in Anchor TypeScript

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for 'view functions' in the Anchor TypeScript client, enabling read-only operations on programs.

```typescript
// Add view functions (#1695).
```

---

### Anchor Lang: Discriminator Argument in #[account] Attribute

Source: https://www.anchor-lang.com/docs/updates/changelog

The `#[account]` attribute in Anchor's Rust library now accepts a `discriminator` argument. This allows explicit specification of the discriminator for an account.

```rust
#[account(discriminator = "CUSTOMDISC")]
```

---

### Anchor Lang: Error Handling Upgrade (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Upgrades error handling in Anchor Lang programs by introducing new macros and a Result type. This involves changes to program result types, error attribute names, and error invocation patterns.

```Rust
change all `ProgramResult`'s to `Result<()>`
change `#[error]` to `#[error_code]`
change all `Err(MyError::SomeError.into())` to `Err(error!(MyError::SomeError))` and all `Err(ProgramError::SomeProgramError)` to `Err(ProgramError::SomeProgramError.into())` or `Err(Error::from(ProgramError::SomeProgramError).with_source(source!()))` to provide file and line source of the error (`with_source` is most useful with `ProgramError`s. `error!` already adds source information for custom and anchor internal errors).
change all `solana_program::program::invoke()` to `solana_program::program::invoke().map_err(Into::into)` and `solana_program::program::invoke_signed()` to `solana_program::program::invoke_signed().map_err(Into::into)`
```

---

### Docker: Remove Anchor Container

Source: https://www.anchor-lang.com/docs/references/verifiable-builds

This command forcefully removes a running Docker container named 'anchor-program'. This is useful for cleaning up background processes if a verifiable build was exited prematurely.

```bash
docker rm -f anchor-program
```

---

### Anchor TS: Provider Interface

Source: https://www.anchor-lang.com/docs/updates/changelog

The `Provider` in Anchor TypeScript is now an interface, with `AnchorProvider` serving as an implementor class. This change standardizes provider interactions and allows for custom implementations.

```typescript
import { AnchorProvider, Provider } from "@project-serum/anchor";
```

---

### Anchor Associated and Account Associated Removal

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the `#[associated]` and `#[account(associated = <target>, with = <target>)]` attributes. This simplifies account association management.

```rust
lang: `#[associated]` and `#[account(associated = <target>, with = <target>)]` are both removed (#612).
```

---

### TypeScript: `has_one` Relations Inference

Source: https://www.anchor-lang.com/docs/updates/changelog

Improves `has_one` relation inference in TypeScript, removing the need to explicitly provide accounts mapped via `has_one` relationships.

```typescript
import { Program, AnchorProvider } from "@coral-xyz/anchor";

// Assuming 'relatedAccount' is linked via 'has_one'
// Previously: const tx = await program.methods.myMethod({ ... }).accounts({
//   relatedAccount: relatedAccount.publicKey
// }).rpc();

// Now, if relatedAccount is implicitly known, it might not need explicit passing.
```

---

### Anchor TS: Dynamic Discriminator Lengths

Source: https://www.anchor-lang.com/docs/updates/changelog

This update enables the retrieval of discriminator lengths dynamically within the Anchor TypeScript client, providing more flexibility in handling program instructions.

```typescript
const discriminatorLength = program.instruction.getDiscriminatorLength();
```

---

### Anchor lang: `constraint = <expression>` replacement

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces `constraint = <expression>` as a replacement for deprecated string literal constraints in the Anchor language.

```rust
/// lang: `constraint = <expression>` added as a replacement for (the now deprecated) string literal constraints (#341).
```

---

### Anchor CLI: Remove IDL Parse Command

Source: https://www.anchor-lang.com/docs/updates/changelog

The `idl parse` command has been removed from the Anchor CLI, as IDL generation is now handled through compilation.

```bash
# Command 'anchor idl parse' removed
```

---

### Anchor TS: Optional Commitment Parameter for Program.addEventListener

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces an optional `commitment` parameter to the `Program.addEventListener` method in the Anchor TypeScript client. This enables users to specify the desired commitment level for event listeners.

```typescript
program.addEventListener(event, { commitment: "confirmed" });
```

---

### Anchor Lang: Box Inner Enums

Source: https://www.anchor-lang.com/docs/updates/changelog

Inner enums within `anchor_lang::error::Error` are now boxed to optimize `anchor_lang::Result`, potentially improving performance and reducing stack usage.

```rust
use anchor_lang::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MyError {
    Variant1,
    Variant2(String),
}

// Internally, these enums might be boxed for Result optimization
// type Result<T> = std::result::Result<T, Box<MyError>>;

```

---

### Handle Arrays with Const Length (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Provides support for handling arrays where the length is defined as a constant. This improves type safety and compile-time checks for array operations.

```rust
let arr: [u8; 32];
```

---

### Add const of Program ID to declare_id! and declare_program! (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Allows adding a constant of the program ID directly within the `declare_id!` and `declare_program!` macros.

```rust
lang: Add `const` of program ID to `declare_id!` and `declare_program!` (#3019).
```

---

### Anchor TS Instruction Decoding

Source: https://www.anchor-lang.com/docs/updates/changelog

Uses `hex` by default for decoding instructions in TypeScript clients. This ensures consistent handling of instruction data.

```typescript
ts: Use `hex` by default for decoding Instruction (#547).
```

---

### Anchor Lang: Associated Token Authority Check (Rust)

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds a missing owner check when `associated_token::authority` is used. This enhances security by verifying the correct authority for associated token accounts.

```Rust
lang: Add missing owner check when `associated_token::authority` is used (#1240).
```

---

### Rust: Account Type Deprecation Removal

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes deprecated account types like `CpiAccount`, `Loader`, and `ProgramAccount` from the Anchor Rust library.

```rust
// Deprecated account types have been removed.
// Use alternative methods or types as per the latest Anchor documentation.
```

---

### Anchor CLI: Commit Based Version Override

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes the `anchor_version` override when it's based on a Git commit hash. This ensures that the CLI correctly uses the specified commit for versioning.

```bash
anchor --version <commit_hash>
# Or via Anchor.toml
[programs.localnet]
my_program = { url = "...", anchor_version = "<commit_hash>" }
```

---

### Anchor Owner Trait

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds the `Owner` trait, which is automatically implemented for all `#[account]` structs. This trait simplifies ownership checks and management within Anchor programs.

```rust
lang: Add `Owner` trait, which is automatically implemented by all `#[account]` structs (#686).
```

---

### Remove borsh 0.9 Support (Lang, SPL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Discontinues support for `borsh` version 0.9 in both the language and SPL components.

```rust
lang, spl: Remove `borsh 0.9` support (#3199).
```

---

### Anchor TS: Unnamed Enum Support

Source: https://www.anchor-lang.com/docs/updates/changelog

The TypeScript SDK now supports unnamed (tuple) enums in accounts, allowing for more flexible data structures.

```typescript
import * as anchor from "@coral-xyz/anchor";

// Example of an unnamed enum in an account
interface MyAccountData {
  myEnum: {
    variant1: [];
    variant2: [number, string];
  };
}

// Usage in TypeScript client:
// const accountData: MyAccountData = ...;
// if (accountData.myEnum.variant1) { ... }
// if (accountData.myEnum.variant2) { const [num, str] = accountData.myEnum.variant2; ... }
```

---

### Anchor Lang: TranslateAddress Fix (TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes the `translateAddress` function in TypeScript to correctly use the `PublicKey` constructor, resolving issues caused by prototype chain constructor name checking in minified code.

```TypeScript
ts: fix `translateAddress` which currently leads to failing browser code. Now uses `PublicKey` constructor instead of prototype chain constructor name checking which doesn't work in the presence of code minifying/mangling(#1138)
```

---

### Remove dex Feature (SPL)

Source: https://www.anchor-lang.com/docs/updates/changelog

Discontinues the `dex` feature within the SPL components.

```rust
spl: Remove `dex` feature (#3257).
```

---

### Anchor Test --detach Flag

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces the `--detach` flag to the `anchor test` command, allowing tests to run detached from the main process. This can be useful for managing test execution environments.

```rust
lang: Add `--detach` flag to `anchor test` (#770).
```

---

### Rust to JS Type Conversion: Collections

Source: https://www.anchor-lang.com/docs/references/type-conversion

This section demonstrates how Rust collections like fixed-size arrays, vectors, and optional values are converted to TypeScript. Fixed arrays and vectors map to TypeScript arrays, while Rust's Option<T> is represented as a union type including null or undefined.

```Rust
[T; N] (fixed array)
Vec<T> (vector)
Option<T>
```

```TypeScript
Array<T>
Array<T>
T | null | undefined
```

---

### Add Support for u16 in TypeScript

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for the `u16` integer type in the TypeScript client. This allows for handling unsigned 16-bit integers in program interactions.

```ts
let value: number = 100;
// ... use value in Anchor client
```

---

### Anchor Lang: Discriminator Argument in #[event] Attribute

Source: https://www.anchor-lang.com/docs/updates/changelog

The `#[event]` attribute in Anchor's Rust library now accepts a `discriminator` argument, enabling custom discriminators for emitted events.

```rust
#[event(discriminator = "CUSTOMEVENT")]
```

---

### Propagate Mocha Test Exit Status on Error

Source: https://www.anchor-lang.com/docs/updates/changelog

Fixes an issue where the Mocha test runner's exit status was not correctly propagated on error. This ensures that CI/CD pipelines and other tools accurately reflect test failures.

```bash
# When a test fails, the anchor cli will now exit with the correct status code.

```

---

### Anchor lang: Preserve span information

Source: https://www.anchor-lang.com/docs/updates/changelog

Preserves span information in the Anchor language, leading to more informative compiler error messages.

```rust
/// lang: Span information is now preserved, providing informative compiler error messages (#341).
```

---

### Remove IdlInstruction::Clear

Source: https://www.anchor-lang.com/docs/updates/changelog

Breaking change: Removes the `IdlInstruction::Clear` variant from the Anchor language's IDL representation. This simplifies the instruction set or reflects a change in how instructions are handled.

```rust
// The IdlInstruction enum no longer contains a Clear variant.

```

---

### Anchor lang: Event field names in IDLs

Source: https://www.anchor-lang.com/docs/updates/changelog

Changes event field names in IDLs to use mixed case, a breaking change impacting how event data is accessed.

```rust
/// lang: Event field names in IDLs are now mixed case. (#379).
```

---

### Add Support for u128 and i128

Source: https://www.anchor-lang.com/docs/updates/changelog

Adds support for `u128` and `i128` integer types in both the Anchor language and its TypeScript client. This expands the range of numerical data that can be handled.

```rust
#[account]
struct MyAccount {
    large_number: u128,
}

```

---

### Anchor IDL Erase Authority

Source: https://www.anchor-lang.com/docs/references/cli

Removes the authority from the IDL account, preventing further upgrades. The wallet executing this command must be the current authority.

```bash
anchor idl erase-authority -p <program-id>
```

---

### Anchor Lang: Associated Discriminator Constants in #[account]

Source: https://www.anchor-lang.com/docs/updates/changelog

The `#[account]` attribute in Anchor's Rust library now uses associated discriminator constants instead of hardcoding them. This improves maintainability and reduces errors.

```rust
#[account]
pub struct MyAccount {
    #[account(discriminator = MyAccount::DISCRIMINATOR)]
    // ... fields
}

impl MyAccount {
    const DISCRIMINATOR: &'static [u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
}
```

---

### Add Array Support in Lang and TS

Source: https://www.anchor-lang.com/docs/updates/changelog

Introduces support for arrays in both the Anchor language and its TypeScript client. This enables programs to handle collections of data more effectively.

```rust
#[account]
struct MyAccount {
    data: [u8; 32],
}

```

---

### Rust: State and Interface Attribute Removal

Source: https://www.anchor-lang.com/docs/updates/changelog

Removes the `state` and `interface` attributes from the Anchor Rust framework.

```rust
// The `#[state]` and `#[interface]` attributes are no longer supported.
// Refer to Anchor's documentation for updated patterns.
```

---

### Error Code Mapping Update (Rust, TypeScript)

Source: https://www.anchor-lang.com/docs/updates/changelog

Error codes have been remapped to new numbers to accommodate more errors per namespace. This improves the granularity and organization of error reporting.

```rust
enum MyProgramError {
    Error1 = 1000,
    Error2 = 1001,
}
```
