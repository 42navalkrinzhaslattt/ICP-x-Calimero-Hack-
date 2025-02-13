# Calimero Node Configuration

The Calimero node configuration file (`config.toml`) is structured into several sections, each controlling different aspects of the node's behavior.

## Core Identity

```toml
[identity]
peer_id = "12D3KooW..."     # Unique identifier for the node in the P2P network
keypair = "23jhTdUD..."     # Node's cryptographic keypair for secure communication
```

## Network Configuration

### P2P Network (Swarm)

```toml
[swarm]
listen = [
    "/ip4/0.0.0.0/tcp/2528",         # TCP transport
    "/ip4/0.0.0.0/udp/2528/quic-v1", # QUIC transport
    "/ip6/::/tcp/2528",              # IPv6 TCP
    "/ip6/::/udp/2528/quic-v1"       # IPv6 QUIC
]
```

### API Server

```toml
[server]
listen = [
    "/ip4/127.0.0.1/tcp/2428",       # IPv4 localhost API endpoint
    "/ip6/::1/tcp/2428"              # IPv6 localhost API endpoint
]

[server.admin]
enabled = true                        # Enable admin API

[server.jsonrpc]
enabled = true                        # Enable JSON-RPC API

[server.websocket]
enabled = true                        # Enable WebSocket connections
```

## Network Discovery

### Bootstrap Nodes

```toml
[bootstrap]
nodes = [                            # Initial nodes for network connection
    "/ip4/18.156.18.6/udp/4001/quic-v1/p2p/12D3KooW...",
    "/ip4/18.156.18.6/tcp/4001/p2p/12D3KooW..."
]
```

### Discovery Settings

```toml
[discovery]
mdns = true                          # Enable local network discovery

[discovery.rendezvous]
namespace = "/calimero/devnet/global" # Network namespace
discovery_rpm = 0.5                   # Discovery rate per minute
registrations_limit = 3               # Max concurrent registrations

[discovery.rendezvous.discovery_interval]
secs = 90                            # Discovery interval in seconds
nanos = 0                            # Additional nanoseconds precision

[discovery.relay]
registrations_limit = 3              # Max relay registrations
```

## Synchronization and Storage

```toml
[sync]
timeout_ms = 30000                   # Sync timeout in milliseconds
interval_ms = 30000                  # Sync interval in milliseconds

[datastore]
path = "data"                        # Path for node data storage

[blobstore]
path = "blobs"                       # Path for blob storage
```

## Context Configuration

### Default Context Settings

```toml
[context.config.new]
protocol = "icp"                     # Default protocol for new contexts
network = "ic"                       # Default network
contract_id = "br5f7-..."           # Default contract ID

[context.config.signer]
use = "relayer"                      # Default signer type

[context.config.signer.relayer]
url = "http://3.125.79.112:63529/"  # Relayer service URL
```

### Chain-Specific Configurations

#### NEAR Configuration

```toml
[context.config.signer.self.near.mainnet]
rpc_url = "https://rpc.mainnet.near.org/"
account_id = "509..."
public_key = "ed25519:6RbX..."
secret_key = "ed25519:3FNE..."      # Private key for signing

[context.config.signer.self.near.testnet]
# Similar structure for testnet
```

#### StarkNet Configuration

```toml
[context.config.signer.self.starknet.mainnet]
rpc_url = "https://cloud.argent-api.com/v1/starknet/mainnet/rpc/v0.7"
account_id = "0x7f25..."
public_key = "0x7f25..."
secret_key = "0x2652..."            # Private key for signing

[context.config.signer.self.starknet.sepolia]
# Similar structure for sepolia
```

#### Internet Computer Configuration

```toml
[context.config.signer.self.icp.ic]
rpc_url = "https://ic0.app/"
account_id = "ecw5j-..."
public_key = "ee789..."
secret_key = "df39a..."             # Private key for signing

[context.config.signer.self.icp.local]
# Similar structure for local development
```

## Security Notes

⚠️ **Important**: This configuration file contains sensitive information:

- Private keys for various networks
- Account credentials
- RPC endpoints

It should be:

- Kept secure and not shared
- Backed up safely
- Access restricted to authorized users only

## File Location

Default location: `~/.calimero/node1/config.toml`
