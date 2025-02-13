# HELLO ICP

- [ ] Install ICP devnet / Staring a Local Devnet
  - [ ] Prerequisites: Cargo, dfx, candid extractor, pnpm
  - [ ] Run the ./deploy_devnet_fresh.sh or ./deploy_devnet_addon.sh
- [ ] Initit and run 3 Nodes
- [ ] Install application

## Init and run 3 nodes

```bash
> `merod --node-name node1 init --server-port 2427 --swarm-port 2527`
2025-02-01T13:57:32.002490Z  INFO merod::cli::init: Generated identity: PeerId("12D3KooWSAkBhJxae7nPiqq7Q5NSKFdTQo4oKexBez4CHDxzLFVj")
2025-02-01T13:57:32.039727Z  INFO merod::cli::init: Initialized a node in "/Users/stefano/.calimero/node1"
```

## Understand the ./deploy_devnet_fresh.sh script

This script is performing a series of operations to set up and deploy canisters on the Internet Computer using `dfx`. Here's a breakdown of the main actions:

1. It defines functions to generate new identities and get account IDs.

2. It checks for required dependencies (`dfx`, `cargo`, and `candid-extractor`).

3. It sets the `dfx` version to 0.24.3 using `dfxvm` [Docs > Current > Developer-docs > Getting-started > Installing developer tools](https://internetcomputer.org/docs/current/developer-docs/getting-started/install#1-install-the-ic-sdk-2).

4. It stops any running `dfx` processes and cleans up the state.

5. It generates several identities: minting, initial, archive, and recipient. (see below for better explanation)

6. It starts `dfx` with a clean state [Docs > Current > Developer-docs > Developer-tools > Cli-tools > Cli-reference > dfx start](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-start).

7. It creates and deploys three canisters: `context_contract`, `ledger`, and `mock_external` [Docs > Current > Developer-docs > Developer-tools > Cli-tools > Cli-reference > dfx canister](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-canister).

8. It transfers cycles to the `context_contract` canister [Docs > Current > Developer-docs > Developer-tools > Cli-tools > Cli-reference > dfx canister](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-canister).

9. It prepares and uses initialization arguments for the `ledger` canister.

10. It builds and installs the canisters with specific arguments [Docs > Current > Developer-docs > Developer-tools > Cli-tools > Cli-reference > dfx canister](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-canister).

11. It sets proxy code for the `context_contract` by reading a WASM file and calling a method on the canister.

12. Finally, it prints a deployment summary with all relevant information, including canister IDs and account information.

This script appears to be setting up a development environment for an Internet Computer project, including a custom ledger implementation and a context contract with proxy functionality. It's automating the process of creating identities, deploying canisters, and configuring them for a specific application setup.

### Understand the 4 identities generated: minting, initial, recipient, archive

Certainly! Let's break down the identities mentioned in point 5 and explain their purposes:

1. Minting identity:

   - Purpose: This identity is used as the minting account for the ledger canister. It has the authority to create new tokens.
   - Need: It's crucial for controlling the token supply and performing minting operations.

2. Initial identity:

   - Purpose: This identity receives an initial allocation of tokens when the ledger is set up.
   - Need: It's used to distribute an initial supply of tokens, often for testing or initializing the system.

3. Archive identity:

   - Purpose: This identity serves as the controller for the archive functionality of the ledger.
   - Need: It's responsible for managing the archiving process of ledger blocks, which is important for maintaining the ledger's efficiency and storage management.

4. Recipient identity:
   - Purpose: This identity is likely used for testing transfer operations.
   - Need: It provides a destination account for testing token transfers within the system.

These identities are created to separate different roles and responsibilities within the ICP ecosystem, enhancing security and providing clear distinctions for various system functions.

The command used to create these identities is `dfx identity new`. Here's how they are created in the script:

```bash
# Generate minting account
dfx identity new minting --storage-mode=plaintext || true

# Generate initial account
dfx identity new initial --storage-mode=plaintext || true

# Generate archive controller account
dfx identity new archive --storage-mode=plaintext || true

# Generate test recipient account
dfx identity new recipient --storage-mode=plaintext || true
```

Each identity is created using the `dfx identity new` command, followed by the identity name. The `--storage-mode=plaintext` flag is used to store the identity in plaintext mode, which is not recommended for production use but can be helpful for development and testing purposes.

The `|| true` at the end of each command is a bash construct that prevents the script from stopping if the identity already exists. It essentially tells the script to continue even if the command fails (which would happen if the identity had been created previously).

For more information on managing identities with dfx, you can refer to the [dfx identity documentation](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-identity).
