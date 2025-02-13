# Calimero Node Initialization

## Overview
Node initialization in Calimero is the process of setting up the necessary infrastructure for a node to participate in the network. This document details what happens when you run the initialization command:
```bash
merod --node-name <name> init --server-port 2428 --swarm-port 2528
```

## Initialization Process

### 1. Directory Structure Setup
```bash
~/.calimero/<node-name>/
├── config.toml         # Node configuration file
├── blobs/             # Application data storage
└── data/              # Node state and context data
```

### 2. Identity Creation
- Generates ED25519 cryptographic keypair
- Creates unique `peer_id` derived from the keypair
- This identity is used for:
  - Node identification in the P2P network
  - Secure communication
  - Message signing

### 3. Configuration Generation
The `config.toml` file is created with several sections:
```toml
[identity]
# Node's cryptographic identity
peer_id = "..."
keypair = "..."

[swarm]
# P2P network configuration
port = 2528              # Default P2P port
host = "0.0.0.0"        # Listen on all interfaces

[server]
# API and CLI interface
port = 2428             # Default API port
host = "127.0.0.1"      # Local interface only

[bootstrap]
# Network discovery settings
nodes = []              # Initial peers

[discovery]
# Peer discovery configuration
mdns = true             # Local network discovery
rendezvous = false      # Remote peer discovery

[sync]
# State synchronization
timeout = 5000          # Milliseconds

[datastore]
# Node state storage
path = "data"           # Relative to node directory

[blobstore]
# Application data storage
path = "blobs"          # Relative to node directory
```

### 4. Storage Initialization
- Sets up RocksDB database for persistent storage
- Initializes directories for:
  - Application data (blobs)
  - Node state data
  - Context state storage
- Creates necessary database files and metadata

### 5. Network Configuration
- Configures two distinct communication channels:
  1. **API/CLI Interface** (Default port: 2428)
     - JSON-RPC API endpoint
     - CLI tool communication
     - Administrative operations
  
  2. **P2P Network** (Default port: 2528)
     - Peer discovery
     - Inter-node communication
     - State synchronization

### 6. Security Setup
- Implements access controls
- Sets up TLS for secure communication
- Configures authentication mechanisms
- Establishes resource limits

## Post-Initialization

After initialization, the node is ready to be started with:
```bash
merod --node-name <name> run
```

This process creates a complete environment for the node to:
- Participate in the P2P network
- Host and run applications
- Manage contexts
- Store and synchronize data
- Handle secure communications

## Important Notes

1. **Idempotency**:
   - The initialization process is idempotent
   - Running it multiple times on the same node directory will not create duplicate resources
   - Use `--force` flag to override existing configuration

2. **Custom Configuration**:
   - Default home directory: `~/.calimero`
   - Can be changed using `--home` flag
   - Each node requires unique ports

3. **Multiple Nodes**:
   - A single machine can run multiple nodes
   - Each node needs unique ports and name
   - Example for second node:
     ```bash
     merod --node-name node2 init --server-port 2429 --swarm-port 2529
     ```

4. **Resource Requirements**:
   - Sufficient disk space for storage
   - Network access for P2P communication
   - CPU and RAM for running applications
