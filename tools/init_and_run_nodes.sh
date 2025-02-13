#!/bin/bash

# Define node names and their respective ports
nodes=("node1" "node2" "node3")
server_ports=(2427 2428 2429)
swarm_ports=(2527 2528 2529)

# Base directory for Calimero nodes
base_dir="$HOME/.calimero"

save_configuration() {
    local app_id=$1
    local context_id=$2
    local member_public_key=$3
    local output_file=".env"

    echo "Saving configuration to $output_file..."
    
    # Create or overwrite the env file
    cat > "$output_file" << EOL
# Calimero Node Configuration
CALIMERO_APP_ID='${app_id}'
CALIMERO_CONTEXT_ID='${context_id}'
CALIMERO_MEMBER_PUBLIC_KEY='${member_public_key}'

# Node Configuration
CALIMERO_NODE1_NAME='${nodes[0]}'
CALIMERO_NODE2_NAME='${nodes[1]}'
CALIMERO_NODE3_NAME='${nodes[2]}'

# Port Configuration
CALIMERO_SERVER_PORTS='${server_ports[*]}'
CALIMERO_SWARM_PORTS='${swarm_ports[*]}'

# Node Public Keys
CALIMERO_NODE2_PUBLIC_KEY='${node2_public_key}'
CALIMERO_NODE3_PUBLIC_KEY='${node3_public_key}'

# Generated on: $(date)
EOL

    echo "Configuration saved successfully to $output_file"
}

# Function to generate invitation payloads
generate_invitation_payloads() {
    local context_id=$1
    local member_public_key=$2
    
    echo "Generating invitation payloads..."
    for i in {1..2}; do
        echo "Generating invitation payload for ${nodes[$i]}..."
        node_output=$(meroctl --node-name ${nodes[$i]} identity generate)
        node_public_key=$(echo "$node_output" | grep "public_key:" | awk '{print $2}')
        
        invitation_payload=$(meroctl --node-name ${nodes[0]} --output-format json context invite "$context_id" "$member_public_key" "$node_public_key")
        echo "Invitation payload for ${nodes[$i]} generated:"
        echo "$invitation_payload"
        
        # Store the payload and public key for later use
        eval "node${i}_public_key=$node_public_key"
        eval "node${i}_payload=$invitation_payload"
    done
}

# Function to invite nodes to join
invite_nodes() {
    local context_id=$1
    
    echo "Inviting nodes to join..."
    for i in {1..2}; do
        echo "Inviting ${nodes[$i]} to join..."
        node_output=$(meroctl --node-name ${nodes[$i]} identity generate)
        node_private_key=$(echo "$node_output" | grep "private_key:" | awk '{print $2}')
        
        # Get the encoded invitation from the stored payload
        eval "payload=\$node${i}_payload"
        encoded_invitation=$(echo "$payload" | jq -r '.data')
        
        # Join the context
        join_output=$(meroctl --node-name ${nodes[$i]} context join "$node_private_key" "$encoded_invitation")
        echo "${nodes[$i]} join output:"
        echo "$join_output"
    done
}

# Function to install the application
install_application() {
    echo "Installing application on ${nodes[0]}..."
    full_output=$(meroctl --node-name ${nodes[0]} app install -p ./res/blockchain.wasm)
    echo "Full command output:"
    echo "$full_output"

    # Extract the ID
    app_id=$(echo "$full_output" | grep "id:" | awk '{print $2}')
    echo "Application installed successfully!"
    echo "Application ID: $app_id"
    
    echo "$app_id"  # Return app_id
}

# Function to create context
create_context() {
    local app_id=$1
    echo "Creating context..."
    context_output=$(meroctl --node-name ${nodes[0]} context create --application-id "$app_id" --protocol "icp")
    echo "Context creation output:"
    echo "$context_output"

    # Extract context ID and public key
    context_id=$(echo "$context_output" | grep "id:" | awk '{print $2}')
    member_public_key=$(echo "$context_output" | grep "member_public_key:" | awk '{print $2}')
    
    echo "$context_id"  # Return context_id
}

# Function to create identities for other nodes
create_identities() {
    for i in {1..2}; do
        echo "Generating ${nodes[$i]} identity..."
        node_output=$(meroctl --node-name ${nodes[$i]} identity generate)
        node_public_key=$(echo "$node_output" | grep "public_key:" | awk '{print $2}')
        node_private_key=$(echo "$node_output" | grep "private_key:" | awk '{print $2}')
        echo "${nodes[$i]} public key: $node_public_key"
    done
}

# Function to check if tmux is installed
check_tmux() {
    if ! command -v tmux >/dev/null 2>&1; then
        echo "tmux is not installed. Would you like to:"
        echo "1) Exit and install tmux manually (recommended)"
        echo "2) Continue without tmux (will open separate terminal windows)"
        read -p "Enter your choice (1 or 2): " choice
        case $choice in
            1)
                echo "Please install tmux and try again:"
                echo "  - On MacOS: brew install tmux"
                echo "  - On Ubuntu/Debian: sudo apt-get install tmux"
                echo "  - On other systems: use your package manager to install tmux"
                exit 1
                ;;
            2)
                echo "Proceeding without tmux..."
                export USE_TMUX=0
                return
                ;;
            *)
                echo "Invalid choice. Exiting."
                exit 1
                ;;
        esac
    fi
    export USE_TMUX=1
}

# Function to check ports
check_ports() {
  for port in "${server_ports[@]}" "${swarm_ports[@]}"; do
    if lsof -i:"$port" &>/dev/null; then
      echo "Port $port is in use."
      read -p "Kill the process using port $port? (y/n): " choice
      if [[ $choice == "y" || $choice == "Y" ]]; then
        lsof -ti:"$port" | xargs kill -9
        echo "Process on port $port killed."
      else
        echo "Port $port is still in use. Exiting."
        exit 1
      fi
    else
      echo "Port $port is free."
    fi
  done
}

# Function to check if node directories exist
check_node_dirs() {
    for node in "${nodes[@]}"; do
        node_dir="$base_dir/$node"
        if [ -d "$node_dir" ]; then
            echo "Directory for $node already exists at $node_dir."
            read -p "Do you want to reinitialize $node? (y/n): " choice
            if [[ $choice == "y" || $choice == "Y" ]]; then
                echo "Removing and reinitializing $node..."
                rm -rf "$node_dir"
                initialize_node "$node"
            else
                echo "Skipping initialization of $node."
                continue
            fi
        else
            echo "No existing directory found for $node. Proceeding with initialization."
            initialize_node "$node"
        fi
    done
}

# Function to initialize a node
initialize_node() {
    node=$1
    index=$(echo "${nodes[@]}" | tr ' ' '\n' | grep -n "^$node$" | cut -d: -f1)
    server_port=${server_ports[$((index - 1))]}
    swarm_port=${swarm_ports[$((index - 1))]}

    echo "Initializing $node on server port $server_port and swarm port $swarm_port."
    mkdir -p "$base_dir/$node"  # Ensure the directory exists
    
    if [ "$USE_TMUX" -eq 1 ]; then
        tmux send-keys -t my_session "merod --node-name $node init --server-port $server_port --swarm-port $swarm_port" C-m
    else
        # For non-tmux, just run the init command directly
        merod --node-name $node init --server-port $server_port --swarm-port $swarm_port
    fi
}

# Function to run nodes
run_nodes() {
    if [ "$USE_TMUX" -eq 1 ]; then
        # Tmux version
        for node in "${nodes[@]}"; do
            node_dir="$base_dir/$node"
            if [ ! -d "$node_dir" ]; then
                echo "Error: Directory for $node does not exist at $node_dir. Please initialize the node first."
                exit 1
            fi

            echo "Starting $node in a new tmux window..."
            tmux new-window -t my_session -n "$node"
            tmux send-keys -t my_session:"$node" "merod --node-name $node run" C-m
        done
    else
        # Non-tmux version - open new terminal windows
        for node in "${nodes[@]}"; do
            node_dir="$base_dir/$node"
            if [ ! -d "$node_dir" ]; then
                echo "Error: Directory for $node does not exist at $node_dir. Please initialize the node first."
                exit 1
            fi

            echo "Starting $node in a new terminal window..."
            if [[ "$OSTYPE" == "darwin"* ]]; then
                # macOS
                osascript -e "tell app \"Terminal\" to do script \"merod --node-name $node run\""
            else
                # Linux (assuming x-terminal-emulator is available)
                x-terminal-emulator -e "merod --node-name $node run" &
            fi
        done
        
        echo "Nodes started in separate terminal windows."
        echo "Please keep these windows open to maintain node operation."
    fi
}

# Main script execution
check_tmux
check_ports
check_node_dirs

if [ "$USE_TMUX" -eq 1 ]; then
    # Start tmux session only if using tmux
    tmux new-session -d -s my_session
fi

run_nodes

# After nodes are running, setup the application
app_id=$(install_application)
context_id=$(create_context "$app_id")
create_identities
generate_invitation_payloads "$context_id" "$member_public_key"
invite_nodes "$context_id"

# Save the configuration
save_configuration "$app_id" "$context_id" "$member_public_key"

echo "Application setup completed:"
echo "Application ID: $app_id"
echo "Context ID: $context_id"
