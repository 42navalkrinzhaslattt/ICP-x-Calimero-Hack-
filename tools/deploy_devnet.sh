#!/bin/bash
set -e

# Set DFX version
dfxvm default 0.24.3

# Function to update env file
update_env_file() {
    local env_file=".env"
    
    # Create or clear the env file
    cat > "$env_file" << EOL
# DFX CANISTER ENVIRONMENT VARIABLES
DFX_VERSION='0.24.3'
DFX_NETWORK='local'
CANISTER_ID_LEDGER='${LEDGER_ID}'
CANISTER_ID_CONTEXT_CONTRACT='${CONTEXT_ID}'
CANISTER_ID_MOCK_EXTERNAL='${MOCK_EXTERNAL_ID}'
CANISTER_ID='${MOCK_EXTERNAL_ID}'
MINTING_ACCOUNT='${MINTING_ACCOUNT}'
INITIAL_ACCOUNT='${INITIAL_ACCOUNT}'
ARCHIVE_PRINCIPAL='${ARCHIVE_PRINCIPAL}'
EOL

    # Add recipient principal only in fresh mode
    if [ $FRESH_DEPLOY -eq 0 ]; then
        echo "RECIPIENT_PRINCIPAL='${RECIPIENT_PRINCIPAL}'" >> "$env_file"
    fi

    echo "# END DFX CANISTER ENVIRONMENT VARIABLES" >> "$env_file"
    
    echo "Environment variables saved to $env_file"
}

# Function to remove identity if it exists
remove_identity() {
    local name=$1
    if check_identity_exists "$name"; then
        echo "Removing identity '$name'..."
        dfx identity remove "$name" --drop-wallets || true
    fi
}

# Function to clean up identities
cleanup_identities() {
    echo "Cleaning up existing identities..."
    
    # Remove identities with both naming conventions
    remove_identity "minting"
    remove_identity "minting_ledger"
    remove_identity "initial"
    remove_identity "initial_ledger"
    remove_identity "archive"
    remove_identity "archive_ledger"
    remove_identity "recipient"
    
    echo "Identity cleanup completed."
}

# Function to check if identity exists
check_identity_exists() {
    local name=$1
    dfx identity list | grep -q "^$name"
    return $?
}

# Function to handle identity creation and setup
setup_identities() {
    local is_fresh=$1

	dfxvm default 0.24.3
    
    echo "Setting up identities..."
    
    # Create required identities if they don't exist
    for identity in "minting" "initial" "archive"; do
        if ! check_identity_exists "$identity"; then
            echo "Creating $identity identity..."
            dfx identity new "$identity" --storage-mode=plaintext
        else
            echo "Identity '$identity' already exists, skipping creation..."
        fi
    done
    
    # Handle recipient identity only in fresh mode
    if [ $is_fresh -eq 0 ]; then
        if ! check_identity_exists "recipient"; then
            echo "Creating recipient identity..."
            dfx identity new recipient --storage-mode=plaintext
        else
            echo "Identity 'recipient' already exists, skipping creation..."
        fi
        dfx identity use recipient
        RECIPIENT_PRINCIPAL=$(dfx identity get-principal)
    fi
	    # Get principals and accounts
    dfx identity use minting
    MINTING_PRINCIPAL=$(dfx identity get-principal)
    MINTING_ACCOUNT=$(get_account_id "$MINTING_PRINCIPAL")
    
    dfx identity use initial
    INITIAL_PRINCIPAL=$(dfx identity get-principal)
    INITIAL_ACCOUNT=$(get_account_id "$INITIAL_PRINCIPAL")
    
    dfx identity use archive
    ARCHIVE_PRINCIPAL=$(dfx identity get-principal)
    
    # Switch back to default identity
    dfx identity use default
    
    echo "Identity setup completed."
}

# Function to stop all dfx processes
stop_all_dfx() {
    echo "Stopping all dfx processes..."
    dfx stop || true
    
    # Kill any process using dfx port 4943
    local max_attempts=5
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if lsof -ti:4943 >/dev/null 2>&1; then
            echo "Attempt $attempt: Found process using dfx port 4943, killing it..."
            lsof -ti:4943 | xargs kill -9
            echo "Waiting for port to be released..."
            sleep 3  # Increased sleep time to allow proper cleanup
        else
            echo "Port 4943 is free"
            break
        fi
        
        attempt=$((attempt + 1))
        
        if [ $attempt -gt $max_attempts ]; then
            echo "Failed to free port 4943 after $max_attempts attempts. Please try again in a few moments."
            exit 1
        fi
    done
    
    # Try pgrep first (preferred method)
    if command -v pgrep >/dev/null 2>&1; then
        while pgrep -f "[d]fx" > /dev/null; do
            echo "Waiting for dfx processes to stop..."
            pkill -f "[d]fx" || true
            sleep 2
        done
    else
        # Fallback to ps aux if pgrep is not available
        while ps aux | grep "[d]fx" > /dev/null; do
            echo "Waiting for dfx processes to stop..."
            ps aux | grep "[d]fx" | awk '{print $2}' | xargs kill -9 2>/dev/null || true
            sleep 2
        done
    fi
    echo "All dfx processes stopped"
}

# Function to generate a new identity and return its principal
generate_identity() {
    local name=$1
    dfx identity new "$name" --storage-mode=plaintext || true
    dfx identity use "$name"
    dfx identity get-principal
}

# Function to get account ID from principal
get_account_id() {
    local principal=$1
    dfx ledger account-id --of-principal "$principal"
}

# Function to ask for deployment mode
ask_deployment_mode() {
    while true; do
        echo -e "\nPlease select deployment mode:"
        echo "1) Fresh (Clean state, remove all existing data)"
        echo "2) Addon (Preserve existing state)"
        read -p "Enter your choice (1 or 2): " choice
        case $choice in
            1)
                echo "Starting fresh deployment..."
                return 0
                ;;
            2)
                echo "Starting addon deployment..."
                return 1
                ;;
            *)
                echo "Invalid choice. Please enter 1 or 2."
                ;;
        esac
    done
}



# Ask for deployment mode
ask_deployment_mode
FRESH_DEPLOY=$?

if [ $FRESH_DEPLOY -eq 0 ]; then
	stop_all_dfx
    echo "Cleaning up previous state..."
    rm -rf .dfx
    rm -rf ~/.config/dfx/replica-configuration/
    rm -rf ~/.config/dfx/identity/minting
    rm -rf ~/.config/dfx/identity/initial
    rm -rf ~/.config/dfx/identity/archive
    rm -rf ~/.cache/dfinity/
    rm -rf ~/.config/dfx/
    
    # Remove canister_ids.json if it exists
    if [ -f "canister_ids.json" ]; then
        rm canister_ids.json
    fi
else
    echo "Proceeding with addon deployment (preserving existing state)..."
fi

# Setup identities based on deployment mode
setup_identities $FRESH_DEPLOY

# Start dfx with clean state if fresh deployment
if [ $FRESH_DEPLOY -eq 0 ]; then
    dfx start --clean --background
else
    dfx start --background || true
fi

# NOTE: Previously in fresh deployment, 'default' identity was used.
# We now consistently use 'initial' identity for both modes as it's
# the one that receives the initial tokens and is more appropriate
# for deployment operations.
dfx identity use initial

echo "Creating and deploying canisters..."
dfx canister create context_contract
dfx canister create ledger
dfx canister create mock_external

# Get the context ID
CONTEXT_ID=$(dfx canister id context_contract)
# Get the wallet ID and seed it
WALLET_ID=$(dfx identity get-wallet)

# Fabricate cycles for the wallet
echo "Fabricating cycles for wallet..."
dfx ledger fabricate-cycles --canister $WALLET_ID --amount 200000 || { echo "Failed to fabricate cycles"; exit 1; }

# Transfer cycles from wallet to context contract
echo "Transferring cycles to context contract..."
dfx canister deposit-cycles 1000000000000000000 $CONTEXT_ID || { echo "Failed to deposit cycles"; exit 1; }

echo "Done! Cycles transferred to context contract: $CONTEXT_ID"

# Get the IDs
CONTEXT_ID=$(dfx canister id context_contract)
LEDGER_ID=$(dfx canister id ledger)

# Prepare ledger initialization argument
LEDGER_INIT_ARG="(variant { Init = record { \
    minting_account = \"${MINTING_ACCOUNT}\"; \
    initial_values = vec { \
        record { \"${INITIAL_ACCOUNT}\"; record { e8s = 100_000_000_000 } } \
    }; \
    send_whitelist = vec {}; \
    transfer_fee = opt record { e8s = 10_000 }; \
    token_symbol = opt \"LICP\"; \
    token_name = opt \"Local Internet Computer Protocol Token\"; \
    archive_options = opt record { \
        trigger_threshold = 2000; \
        num_blocks_to_archive = 1000; \
        controller_id = principal \"${ARCHIVE_PRINCIPAL}\" \
    } \
} })"

# Build and install canisters
echo "Building canisters..."
dfx build

# Install mode depends on deployment type
INSTALL_MODE=$([ $FRESH_DEPLOY -eq 0 ] && echo "install" || echo "reinstall")

# First install the ledger canister
echo "Installing ledger canister..."
dfx canister install ledger --mode=$INSTALL_MODE --argument "$LEDGER_INIT_ARG"

# Get the ledger ID and install context contract with it
LEDGER_ID=$(dfx canister id ledger)
echo "Installing context contract..."
dfx canister install context_contract --mode=$INSTALL_MODE --argument "(principal \"${LEDGER_ID}\")"

# Install mock external canister
echo "Installing mock external canister..."
dfx canister install mock_external --mode=$INSTALL_MODE --argument "(principal \"${LEDGER_ID}\")"
MOCK_EXTERNAL_ID=$(dfx canister id mock_external)

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Build path relative to the script location
WASM_FILE="${SCRIPT_DIR}/context-proxy/calimero_context_proxy_icp.wasm"

# Verify file exists
if [ ! -f "$WASM_FILE" ]; then
    echo "Error: WASM file not found at: $WASM_FILE"
    exit 1
fi

echo "Setting proxy code..."
# Then modify the script to use a consistent reading method
WASM_CONTENTS=$(xxd -p "$WASM_FILE" | tr -d '\n' | sed 's/\(..\)/\\\1/g')

# Execute the command using the temporary file
dfx canister call context_contract set_proxy_code --argument-file <(
  echo "(
    blob \"${WASM_CONTENTS}\"
  )"
)

# Update .env file
update_env_file

# Print all relevant information at the end
echo -e "\n=== Deployment Summary ==="
echo "Deployment Mode: $([ $FRESH_DEPLOY -eq 0 ] && echo "Fresh" || echo "Addon")"
echo "Context Contract ID: ${CONTEXT_ID}"
echo "Ledger Contract ID: ${LEDGER_ID}"
echo "Demo External Contract ID: ${MOCK_EXTERNAL_ID}"
echo -e "\nAccount Information:"
echo "Minting Account: ${MINTING_ACCOUNT}"
echo "Initial Account: ${INITIAL_ACCOUNT}"
echo "Archive Principal: ${ARCHIVE_PRINCIPAL}"
if [ $FRESH_DEPLOY -eq 0 ]; then
    echo "Recipient Principal: ${RECIPIENT_PRINCIPAL}"
fi
echo -e "\nDeployment completed successfully!"