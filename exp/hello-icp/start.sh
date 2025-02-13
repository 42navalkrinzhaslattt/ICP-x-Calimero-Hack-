#!/bin/bash

# Define node names and their respective ports
nodes=("node1" "node2" "node3")
server_ports=(2427 2428 2429)
swarm_ports=(2527 2528 2529)

# Base directory for Calimero nodes
base_dir="$HOME/.calimero"

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
      if [[ $choice != "y" && $choice != "Y" ]]; then
        echo "Skipping initialization of $node."
        continue
      fi
      echo "Reinitializing $node..."
    else
      echo "No existing directory found for $node. Proceeding with initialization."
    fi
    initialize_node "$node"
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
  tmux send-keys -t my_session "merod --node-name $node init --server-port $server_port --swarm-port $swarm_port" C-m
}

# Function to run nodes in separate tmux windows
run_nodes() {
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
}

# Start the main script
check_ports

# Start tmux session
tmux new-session -d -s my_session

# Run nodes in separate tmux windows
run_nodes


# Check and initialize nodes
check_node_dirs

# Attach to the session
tmux attach -t my_session
