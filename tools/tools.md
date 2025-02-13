# TOOLS

## deploy_devnet.sh

A unified deployment script that handles both fresh and addon deployments for the ICP devnet environment. The script:

1. Allows interactive selection of deployment mode:

   - Fresh: Completely clean deployment, removing all existing state
   - Addon: Preserves existing state and adds to current deployment

2. Manages identities:

   - Creates/verifies required identities (minting, initial, archive)
   - Creates recipient identity in fresh mode
   - Handles both naming conventions (with/without \_ledger suffix)
   - Removes old identities and wallets in fresh mode

3. Deploys and configures canisters:

   - context_contract: Main contract for context handling
   - ledger: ICP ledger canister
   - mock_external: Support canister for proxy operations

4. Handles cycles and initialization:

   - Fabricates and transfers cycles to required canisters
   - Initializes ledger with proper configuration
   - Sets up proxy code for context contract

5. Provides detailed deployment summary including:
   - Deployment mode used
   - All canister IDs
   - Account information and principals

Requirements:

- The script requires a `context-proxy` directory in the same location containing the `calimero_context_proxy_icp.wasm` file
- Without this WASM file, the deployment will fail during the proxy code setup phase

Note: This script combines and replaces the previous `deploy_devnet_addon.sh` and `deploy_devnet_fresh.sh` scripts from the [icp-devnet](https://github.com/calimero-network/icp-devnet) Calimero repository.
