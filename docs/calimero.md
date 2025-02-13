# Calimero Network Architecture

## Minimal Operating Unit: Peer

A **Peer** is the minimal meaningful operating unit in Calimero. However, it cannot exist independently and requires three components:

1. A **Node** as its base (as a peer is a specific instance of a node)
2. An **Application** (as all interactions happen through application-specific logic)
3. A **Context** to operate within (which connects the peer to the application and other peers)

A peer represents:

- A specific instance of a node within a P2P network
- A user in the system
- Has a unique Peer ID for identification and message routing
- Can interact with other peers in the network through application-defined rules
- Maintains its own identity and state

### Operational Creation Process

The actual creation process in the system follows this sequence:

1. **Node Setup** (Infrastructure Layer):

```bash
# Initialize node
merod --node-name node1 init --server-port 2428 --swarm-port 2528
# Run node
merod --node-name node1 run
```

2. **Application Installation** (Application Layer):

```bash
# Install application
application install file {path/to/app}
>> Application installed <APPLICATION_ID>
```

3. **Peer Creation** (Identity Layer):

```bash
# Generate peer identity
identity new
>> Private key: <PRIVATE_KEY>
>> Public key: <PUBLIC_KEY>
```

4. **Peer Activation** (Context Layer):

```bash
# Join a context to activate the peer
context join <PRIVATE_KEY> <INVITATION_PAYLOAD>
```

This process shows that while a peer is conceptually an instance of a node, operationally it requires:

- The node must exist first as the infrastructure
- An application must be installed to define the interaction rules
- A peer is created through identity generation
- The peer becomes active only when it joins a context tied to the application

## Core Components: Nodes and Contexts

While nodes are fundamental units in Calimero, they operate within a larger organizational concept called a "Context". Here's how these components work together:

### Understanding Different Types of Nodes: Blockcahin (Ethreum/Bitcoin) vs ICP vs Calimero

#### Traditional Blockchain Nodes

In traditional blockchain networks (like Ethereum or Bitcoin):

- Nodes maintain a complete copy of the blockchain
- Validate and propagate transactions
- Participate in consensus mechanisms
- Run the same protocol and store the same data
- Primary focus is on maintaining network consensus and state

#### Internet Computer Protocol (ICP) Nodes

In the Internet Computer:

- Nodes are organized into subnets that host canisters
- Each subnet is a blockchain running the Internet Computer Protocol
- Nodes participate in consensus within their subnet
- Focus on executing WebAssembly canisters and maintaining subnet state
- Part of a hierarchical infrastructure managed by the Network Nervous System (NNS)

#### Calimero Nodes

In Calimero:

- Nodes are application-agnostic infrastructure providers
- Can host multiple peers and applications
- Run isolated WebAssembly VMs for different contexts
- Focus on P2P communication and application execution
- Don't maintain a global blockchain state
- Provide infrastructure for context-specific networks

Key differences:

1. **State Management**:

   - Blockchain nodes: Global state
   - ICP nodes: Subnet state
   - Calimero nodes: Context-specific states

2. **Purpose**:

   - Blockchain nodes: Consensus and validation
   - ICP nodes: Canister execution and subnet consensus
   - Calimero nodes: Application hosting and P2P communication

3. **Architecture**:
   - Blockchain nodes: Part of a single network
   - ICP nodes: Part of a subnet in a hierarchical system
   - Calimero nodes: Independent infrastructure hosting multiple contexts

### Nodes

A node is any individual device or computer that participates in the Calimero network. There are two main types of nodes:

#### 1. Client Nodes

- Acts as a gateway to run applications and connect with other peers
- Features:
  - Runtime environment for executing DApps (WebAssembly)
  - Local storage and state management
  - Identity management
  - Can be embedded with clients or run as a remote node
  - JSON-RPC API and WebSocket interface for communication

#### 2. Specialized Nodes

- Provide enhanced network capabilities
- Features:
  - Heavy data processing
  - Advanced encryption tasks
  - Additional storage solutions
  - Can be deployed by any participant (Calimero, third-party developers, or users)

### Contexts

Context is the core organizational unit of the Calimero ecosystem:

- Application-specific network for direct user communication
- Eliminates need for intermediaries
- Each application runs in an isolated context on a node
- Nodes can participate in multiple contexts
- Enables context-aware state reconciliation

### Context Creation Process

Creating a context involves several steps:

1. **Application Installation** (if not already installed):

```bash
# Install application
application install file {path/to/app}
>> Application installed <APPLICATION_ID>
```

2. **Context Creation**:

```bash
# Create context with specific protocol (near, starknet, or icp)
context create <APPLICATION_ID> --protocol <protocol_name>
>> Created context <CONTEXT_ID> with identity <CONTEXT_IDENTITY>
```

3. **Peer Invitation Process**:
   - New peers generate their identity:
   ```bash
   identity new
   >> Private key: <PRIVATE_KEY>
   >> Public key: <PUBLIC_KEY>
   ```
   - Context creator invites peers:
   ```bash
   context invite <CONTEXT_ID> <CONTEXT_IDENTITY> <PUBLIC_KEY_OF_PEER>
   >> Invitation payload: <INVITATION_PAYLOAD>
   ```
   - Peers join the context:
   ```bash
   context join <PRIVATE_KEY> <INVITATION_PAYLOAD>
   ```

This process ensures:

- Each context is tied to a specific application
- All peers have unique identities within the context
- Secure peer-to-peer communication through identity verification
- Proper synchronization of the application state across all peers

This architecture allows Calimero to achieve:

- Local-first execution with eventual consistency
- Enhanced privacy through encrypted peer-to-peer communication
- Scalable performance tied to the number of actors
- Flexible deployment options for different use cases

## Principal Actors in Memory

The Calimero Network has four principal actors that exist in memory, each with specific data structures and responsibilities:

### 1. Node

**Location**: `core/crates/node/src/lib.rs`

```rust
pub struct Node {
    sync_config: SyncConfig,          // Synchronization configuration
    store: Store,                     // Persistent storage
    ctx_manager: ContextManager,      // Context management
    network_client: NetworkClient,    // P2P networking
    node_events: broadcast::Sender<NodeEvent> // Event handling
}
```

### 2. Peer/DID

**Location**: `core/crates/primitives/src/identity.rs`

```rust
pub struct Did {
    pub id: String,                 // Unique identifier
    pub root_keys: Vec<RootKey>,    // Authentication keys
    pub client_keys: Vec<ClientKey> // Session keys
}

pub struct RootKey {
    pub signing_key: String,
    pub wallet_type: WalletType,
    pub wallet_address: String,
    pub created_at: u64,
}

pub struct ClientKey {
    pub wallet_type: WalletType,
    pub signing_key: String,
    pub created_at: u64,
    pub context_id: Option<ContextId>,
}
```

### 3. Context

**Location**: `core/crates/primitives/src/context.rs`

```rust
pub struct Context {
    pub id: ContextId,              // Unique identifier
    pub application_id: ApplicationId, // Associated application
    pub root_hash: Hash,            // State verification
}

pub struct ContextConfigParams<'a> {
    pub protocol: Cow<'a, str>,
    pub network_id: Cow<'a, str>,
    pub contract_id: Cow<'a, str>,
    pub proxy_contract: Cow<'a, str>,
    pub application_revision: u64,
    pub members_revision: u64,
}
```

### 4. Application

**Location**: `core/crates/context/config/src/types.rs`

```rust
pub struct Application<'a> {
    pub id: Repr<ApplicationId>,    // Unique identifier
    pub blob: Repr<BlobId>,        // Application code
    pub size: u64,                 // Size in bytes
    pub source: ApplicationSource<'a>, // Source location
    pub metadata: ApplicationMetadata<'a> // Additional info
}
```

### Context Manager

**Location**: `core/crates/context/src/lib.rs`

```rust
pub struct ContextManager {
    store: Store,                    // State storage
    client_config: ClientConfig,     // Configuration
    config_client: ExternalClient<AnyTransport>,
    blob_manager: BlobManager,       // Data blob handling
    network_client: NetworkClient,   // Network access
    server_sender: ServerSender,     // Server communication
    state: Arc<RwLock<State>>,      // Runtime state
}
```

These actors work together in a hierarchical relationship:

1. Node provides the infrastructure and hosts peers
2. Peers (DIDs) represent users and their identities
3. Contexts provide isolated environments for applications
4. Applications define the business logic and rules

Each actor maintains its own state and has clear responsibilities within the system, enabling a modular and secure architecture.

## Runtime Environment Implementation

The Calimero Network uses a WebAssembly (WASM) runtime environment that provides a secure, sandboxed execution environment for applications. Here's how it's structured:

### Runtime Environment

The core runtime implementation provides execution of WebAssembly modules within a sandboxed environment:

```rust
// Runtime execution function
pub fn run(
    code: &[u8],               // WASM bytecode
    method_name: &str,         // Function to execute
    context: RuntimeContext,   // Execution context
    storage: &mut dyn Storage, // State storage
    limits: &RuntimeLimits,    // Resource limits
) -> RuntimeResult<Outcome>
```

### Runtime Components

1. **WASM Runtime** ([Wasmer](https://wasmer.io/))

   - Executes WebAssembly modules
   - Provides memory management and sandboxing
   - Enforces resource limits and security boundaries

2. **RuntimeLogic**

   - Central orchestrator for execution
   - Manages state and resources
   - Controls host function access
   - Collects logs and events

3. **Resource Management**

```rust
struct RuntimeLimits {
    max_memory_pages: u32,        // Memory limits (e.g., 1 KiB)
    max_stack_size: u32,          // Stack size (e.g., 200 KiB)
    max_registers: u32,           // Number of registers
    max_register_size: Constraint, // Register size (e.g., 100 MiB)
    max_logs: u32,                // Maximum log entries
    max_events: u32,              // Maximum events
    // ... other resource constraints
}
```

### Execution Flow

1. Load WASM module
2. Set up runtime environment
3. Link host functions
4. Execute in sandbox
5. Collect results and clean up

This runtime implementation ensures:

- Secure execution of untrusted code
- Resource limitation and metering
- Deterministic execution
- Isolation between applications
- State persistence and management

Each principal actor (Node, Peer, Context, Application) interacts with this runtime layer:

- Nodes provide the runtime infrastructure
- Applications are compiled to WASM modules
- Contexts get their own runtime instance
- Peers interact through the runtime's sandboxed environment

## Node and Virtual Machine Relationship

A Node in Calimero has two aspects:

### 1. Physical Layer

- The actual device (computer, server, etc.)
- Provides physical resources:
  - CPU for computation
  - RAM for memory
  - Disk for storage
  - Network interfaces for communication

### 2. Software Layer

- **Node Software** (`merod`):
  The Node software is implemented as a comprehensive system with several key components:

```rust
pub struct Node {
    sync_config: SyncConfig,          // Synchronization configuration
    store: Store,                     // Persistent storage
    ctx_manager: ContextManager,      // Context management
    network_client: NetworkClient,    // P2P networking
    node_events: broadcast::Sender<NodeEvent>, // Event handling
}
```

Key Components:

1. **Network Identity (PeerId)**:

   - Derived from node's cryptographic keys
   - Used in multiaddresses: `/ip4/192.0.2.0/tcp/443/p2p/QmcEPrat8ShnCph8WjkREzt5CPXF2RwhYxYBALDcLC1iV6`
   - Enables persistent identification in the P2P network

2. **NetworkClient**:

   ```rust
   struct NetworkConfig {
       identity: Keypair,
       swarm: SwarmConfig,        // Network listening
       bootstrap: BootstrapConfig, // Initial connections
       discovery: DiscoveryConfig  // Peer discovery
   }
   ```

   - Manages P2P communication using libp2p
   - Handles peer discovery and connections
   - Implements Gossipsub for message broadcasting

3. **Runtime (WebAssembly VM)**:

   ```rust
   pub fn run(
       code: &[u8],               // WASM bytecode
       method_name: &str,         // Function to execute
       context: VMContext,        // Execution context
       storage: &mut dyn Storage, // State storage
       limits: &VMLimits,        // Resource limits
   ) -> RuntimeResult<Outcome>
   ```

   - Executes application code in sandbox
   - Manages resource limits
   - Provides isolation between applications

4. **Store (RocksDB)**:
   - Provides persistent storage
   - Interfaces with WASM runtime
   - Manages context transactions and metadata

This implementation shows how the node software (`merod`) integrates these components to provide a complete P2P application platform.

### How They Work Together

1. **Resource Allocation**:

   - Physical Node: Provides raw computing resources
   - Node Software: Partitions these resources into isolated VM instances

2. **VM Management**:

   - Physical Node: Runs the Wasmer engine
   - Node Software: Creates and manages VM instances for:
     - Different applications
     - Different contexts
     - Different peers

3. **Security Boundaries**:
   - Physical Node: Provides hardware-level isolation
   - Node Software: Implements software-level sandboxing through the VM

This means when we say "nodes provide the VM infrastructure", we're referring to how the node software (`merod`) running on physical devices manages and orchestrates the virtual machines, creating a secure and isolated environment for applications to run.

## Multiple Nodes on a Single Device

A single physical device can run multiple node instances, each being a separate `merod` process:

### Node Instance Separation

```bash
# Initialize first node
merod --node-name node1 init --server-port 2428 --swarm-port 2528
merod --node-name node1 run

# Initialize second node (different ports)
merod --node-name node2 init --server-port 2429 --swarm-port 2529
merod --node-name node2 run
```

Each node instance:

- Has its own identity (different `peer_id`)
- Uses separate ports for communication
- Maintains its own storage space
- Runs its own set of VMs
- Can host different peers and contexts

### Resource Sharing

- Physical resources (CPU, RAM, disk) are shared between node instances
- Each node instance can be configured with different resource limits
- Operating system handles resource scheduling between node processes

### Use Cases for Multiple Nodes

1. **Development and Testing**:

   - Run production and test nodes side by side
   - Test network interactions locally

2. **Resource Isolation**:

   - Dedicate nodes to specific applications or contexts
   - Separate resource-intensive nodes from lighter ones

3. **Network Simulation**:

   - Test P2P network behavior with multiple local nodes
   - Simulate different network topologies

4. **Security Boundaries**:
   - Isolate different user groups or applications
   - Provide additional security through process separation

## Node CLI Interaction

Similar to how you interact with a VM through SSH, Calimero nodes provide a CLI interface through the `meroctl` command-line tool:

### Basic Node Interaction

```bash
# Connect to a running node
meroctl --node-name node1 connect

# Check node status
meroctl --node-name node1 status

# View node logs
meroctl --node-name node1 logs
```

### Interactive Shell Mode

When a node is running, it provides an interactive shell with commands:

```bash
Usage: [call|peers|pool|gc|store] [args]

> peers                  # Show connected peers
> store                  # Print the DB state
> call <method> <args>   # Call application methods
> pool                   # Show transaction pool
> gc                     # Clean up transaction pool
```

### Remote Access

- Nodes can be accessed remotely through their JSON-RPC API
- Default ports:
  - Server port (e.g., 2428) for API access
  - Swarm port (e.g., 2528) for P2P communication
- Security:
  - Authentication required through client keys
  - TLS encryption for remote connections
  - Access control through node configuration

### Common Operations

1. **Application Management**:

   ```bash
   # Install an application
   meroctl --node-name node1 application install {path}

   # List installed applications
   meroctl --node-name node1 application list
   ```

2. **Context Operations**:

   ```bash
   # Create a new context
   meroctl --node-name node1 context create <APP_ID>

   # List contexts
   meroctl --node-name node1 context list
   ```

3. **Identity Management**:

   ```bash
   # Generate new identity
   meroctl --node-name node1 identity new

   # List identities
   meroctl --node-name node1 identity list
   ```

This CLI interface provides complete control over the node, similar to how SSH provides access to a traditional VM's shell.

## CLI-Node Communication Protocol

The `meroctl` CLI tool communicates with nodes through a layered protocol stack:

### Communication Stack

1. **Transport Layer**:

   - HTTP/HTTPS for transport
   - Default server port: 2428
   - TLS encryption for secure communication

2. **API Layer**:

   ```rust
   // JSON-RPC request format
   struct Request {
       jsonrpc: Version,        // JSON-RPC version
       id: RequestId,           // Request identifier
       method: String,          // Method to call
       params: RequestPayload   // Method parameters
   }
   ```

3. **Authentication Layer**:
   ```rust
   struct Environment {
       args: RootArgs,          // Including node name
       output: Output           // Response handling
   }
   ```

### Protocol Flow

1. **Connection Establishment**:

   ```rust
   // 1. Load node configuration
   let config = load_config(&environment.args.home, &environment.args.node_name)?;

   // 2. Get node address
   let multiaddr = fetch_multiaddr(&config)?;

   // 3. Create client
   let client = Client::new();
   ```

2. **Command Processing**:

   - CLI command → JSON-RPC request
   - Node processes request
   - Response returned as JSON-RPC response

3. **Security**:
   - All communications are authenticated using node's private key
   - TLS encryption for remote connections
   - Access control through node configuration

This means when you run a CLI command:

1. `meroctl` connects to the node's HTTP server (port 2428)
2. Sends a JSON-RPC request with the command
3. Node processes it in its VM
4. Returns response through the same channel

This is different from the P2P communication (port 2528) which is used for node-to-node communication in the network.

## Glossary and Technical Details

### Key Terms

- **JSON-RPC**: A stateless, lightweight remote procedure call (RPC) protocol that uses JSON for data encoding. It allows clients to call methods on remote systems over HTTP, making it easy to interact with the node programmatically.

- **P2P (Peer-to-Peer)**: A decentralized network architecture where participants (peers) communicate directly with each other without requiring a central server.

### Node API Server

Each node runs an internal HTTP server that exposes a JSON-RPC API:

1. **Server Implementation**:

   ```rust
   // Node includes an HTTP server for API access
   struct NodeServer {
       rpc_handler: JsonRpcHandler,
       http_server: HttpServer,
       port: u16,              // Default 2428
   }
   ```

2. **Core API Endpoints**:

   ```json
   // Application Management
   {
     "method": "application_install",
     "params": {"path": "string"}
   }
   {
     "method": "application_list",
     "params": {}
   }

   // Context Management
   {
     "method": "context_create",
     "params": {
       "app_id": "string",
       "protocol": "string"
     }
   }
   {
     "method": "context_list",
     "params": {}
   }

   // Identity Management
   {
     "method": "identity_new",
     "params": {}
   }
   {
     "method": "identity_list",
     "params": {}
   }
   ```

### P2P Communication Architecture

The P2P network in Calimero uses several protocols and techniques:

1. **Network Stack**:

   ```rust
   struct NetworkManager {
       swarm: Swarm,           // libp2p swarm for P2P connections
       port: u16,              // Default 2528
       protocols: Vec<Protocol> // Supported P2P protocols
   }
   ```

2. **Core P2P Protocols**:

   - **Kademlia DHT**: For peer discovery and routing
   - **Gossipsub**: For efficient message broadcasting
   - **DCUtR**: For NAT traversal and direct connections
   - **mDNS**: For local network peer discovery

3. **Communication Flow**:

   ```mermaid
   graph TD
       A[Peer 1] -->|1. Discovery| B[DHT Network]
       B -->|2. Peer Info| A
       A -->|3. Direct Connection| C[Peer 2]
       A -->|4. Message Exchange| C
   ```

4. **Message Types**:
   ```rust
   enum P2PMessage {
       ApplicationData(Vec<u8>),  // Application-specific data
       StateSync(Hash),          // State synchronization
       PeerDiscovery(PeerId),    // Peer discovery
       ContextUpdate(ContextId)  // Context updates
   }
   ```

This architecture means each node is both:

1. A client (through its JSON-RPC API server)
2. A P2P network participant (through its swarm port)

The separation of ports (2428 for API, 2528 for P2P) allows for clear distinction between:

- Client-node communication (JSON-RPC over HTTP)
- Node-node communication (P2P protocols)

## Node Implementation Layers

In Calimero, the term "node" encompasses both physical and software aspects, though the distinction is mainly conceptual:

### Physical Layer (Hardware)

- Any device capable of running the node software
- Managed by the operating system
- Provides:
  - Computing resources (CPU)
  - Memory (RAM)
  - Storage (Disk)
  - Network connectivity

### Software Layer (`merod`)

The Node software is implemented as a comprehensive system with several key components:

```rust
pub struct Node {
    sync_config: SyncConfig,          // Synchronization configuration
    store: Store,                     // Persistent storage
    ctx_manager: ContextManager,      // Context management
    network_client: NetworkClient,    // P2P networking
    node_events: broadcast::Sender<NodeEvent>, // Event handling
}
```

Key Components:

1. **Network Identity (PeerId)**:

   - Derived from node's cryptographic keys
   - Used in multiaddresses: `/ip4/192.0.2.0/tcp/443/p2p/QmcEPrat8ShnCph8WjkREzt5CPXF2RwhYxYBALDcLC1iV6`
   - Enables persistent identification in the P2P network

2. **NetworkClient**:

   ```rust
   struct NetworkConfig {
       identity: Keypair,
       swarm: SwarmConfig,        // Network listening
       bootstrap: BootstrapConfig, // Initial connections
       discovery: DiscoveryConfig  // Peer discovery
   }
   ```

   - Manages P2P communication using libp2p
   - Handles peer discovery and connections
   - Implements Gossipsub for message broadcasting

3. **Runtime (WebAssembly VM)**:

   ```rust
   pub fn run(
       code: &[u8],               // WASM bytecode
       method_name: &str,         // Function to execute
       context: VMContext,        // Execution context
       storage: &mut dyn Storage, // State storage
       limits: &VMLimits,        // Resource limits
   ) -> RuntimeResult<Outcome>
   ```

   - Executes application code in sandbox
   - Manages resource limits
   - Provides isolation between applications

4. **Store (RocksDB)**:
   - Provides persistent storage
   - Interfaces with WASM runtime
   - Manages context transactions and metadata

This implementation shows how the node software (`merod`) integrates these components to provide a complete P2P application platform.

### Layer Interaction

1. **Resource Management**:

   - OS manages physical resources
   - `merod` provides abstractions
   - No direct hardware management in Calimero

2. **Multiple Nodes**:

   - One physical device can run multiple `merod` instances
   - Each instance is a separate node in the network
   - OS handles resource sharing

3. **Implementation Focus**:
   - Calimero focuses on the software layer
   - Physical layer details are abstracted
   - Resource limits are software-defined

This means that while we conceptually distinguish between physical and software layers, the implementation primarily deals with the software layer (`merod`), relying on the operating system to manage the physical resources.

## Node Identity and Hosted Peers

A node in Calimero has two distinct identity concepts:

1. **Node Network Identity**:

   ```rust
   struct Node {
       peer_id: PeerId,            // Node's P2P network identity
       // ... other fields ...
   }
   ```

   - The `peer_id` is used for node-to-node P2P communication
   - Identifies the node in the network infrastructure
   - Used for discovery and routing at the infrastructure level

2. **Hosted Peers**:
   ```rust
   struct Did {
       id: String,                 // Peer's unique identifier
       root_keys: Vec<RootKey>,    // Authentication keys
       client_keys: Vec<ClientKey> // Session keys
   }
   ```
   - Multiple peers (users) can be hosted on a single node
   - Each peer has its own identity separate from the node's `peer_id`
   - Peers represent users and their interactions within contexts

### Example Scenario:

```
Node (peer_id: abc123)
├── Peer 1 (id: user1_did)
│   ├── Context A
│   └── Context B
├── Peer 2 (id: user2_did)
│   └── Context A
└── Peer 3 (id: user3_did)
    └── Context C
```

This shows how a single node (identified by its `peer_id`) can host multiple peers, each with their own identity and context participation.

## Core Implementation Relationships

The core components in Calimero are implemented across several files in the `core/crates` directory:

### Node (Infrastructure Layer)

**Location**: `core/crates/node/src/lib.rs`

```rust
struct Node {
    sync_config: SyncConfig,          // Synchronization configuration
    store: Store,                     // Storage layer
    ctx_manager: ContextManager,      // Manages contexts
    network_client: NetworkClient,    // P2P communication
    node_events: broadcast::Sender<NodeEvent> // Event handling
}
```

- Provides the base infrastructure
- Can host multiple peers (DIDs)
- Manages network connections and storage

### NetworkClient (P2P Layer)

**Location**: `core/crates/network/src/lib.rs`

```rust
// Network behavior implementation
struct Behaviour {
    dcutr: DcutrBehaviour,           // Direct connection upgrades
    gossipsub: GossipsubBehaviour,   // P2P message broadcasting
    identify: IdentifyBehaviour,      // Node identification
    kad: KadBehaviour<MemoryStore>,  // DHT for peer discovery
    mdns: Toggle<MdnsTokioBehaviour>, // Local network discovery
    ping: PingBehaviour,             // Connection health checks
    rendezvous: RendezvousBehaviour, // Peer discovery service
    relay: RelayBehaviour,           // NAT traversal
    stream: StreamBehaviour          // Stream management
}
```

### ContextManager (Context Layer)

**Location**: `core/crates/context/src/lib.rs`

```rust
struct ContextManager {
    store: Store,                    // State storage
    client_config: ClientConfig,     // Configuration
    config_client: ExternalClient<AnyTransport>,
    blob_manager: BlobManager,       // Data blob handling
    network_client: NetworkClient,   // Network access
    server_sender: ServerSender,     // Server communication
    state: Arc<RwLock<State>>,      // Runtime state
}
```

### Store (Storage Layer)

**Location**: `core/crates/store/src/lib.rs`

- Implements RocksDB for persistent storage
- Manages context transactions and metadata
- Interfaces with the WASM runtime

### Server (API Layer)

**Location**: `core/crates/server/src/lib.rs`

- Provides JSON-RPC API endpoints
- Handles WebSocket connections
- Manages admin operations

This implementation structure shows how Calimero:

1. Separates concerns across different crates
2. Uses a modular architecture
3. Maintains clear boundaries between components
4. Enables flexible deployment and scaling

Each component is designed to be:

- Independently maintainable
- Well-documented
- Clearly scoped
- Efficiently testable

## Node Management and CLI Usage

Calimero provides two main command-line tools for managing nodes and interacting with the network:

### 1. Node Management with `merod`

`merod` is used for node initialization, configuration, and runtime management:

```bash
# Initialize a new node
merod --node-name node1 init --server-port 2428 --swarm-port 2528

# Run the node
merod --node-name node1 run

# Configure an existing node
merod --node-name node1 config --server-host 143.34.182.202 --server-port 3000
```

Key features:

- Default configuration directory: `~/.calimero`
- Can specify custom home directory with `--home`
- Each node requires unique server and swarm ports
- Supports multiple nodes on a single machine

### 2. Node Interaction with `meroctl`

`meroctl` provides commands for interacting with running nodes:

:::warning
**Known Issues**:

1. Command Discrepancy: The documentation sometimes refers to `application list` but the command `app list` or `app ls` also fails to execute
2. Configuration Loading: The CLI fails to load the configuration file with the error "Failed to load config file" even when:
   - The file exists at the correct location (`~/.calimero/node1/config.toml`)
   - The node is properly initialized and running
   - The directory permissions are correct
3. Current Status: As of now, we have not been able to successfully execute any `meroctl` commands, suggesting potential issues with:
   - CLI implementation
   - Configuration file format/parsing
   - Connection between CLI and node
     :::

```bash
# List installed applications (correct command)
meroctl --node-name node1 app list
# or
meroctl --node-name node1 app ls

# Create a new context
meroctl --node-name node1 context create --application-id <appId>

# Generate new identity
meroctl --node-name node1 identity new
```

Common operations:

1. **Application Management**:

   ```bash
   # Install an application
   meroctl --node-name node1 app install --path {path/to/app}

   # Get application details
   meroctl --node-name node1 app get <APP_ID>
   ```

2. **Context Management**:

   ```bash
   # List all contexts
   meroctl --node-name node1 context list

   # Create a context
   meroctl --node-name node1 context create <APP_ID>

   # Join a context
   meroctl --node-name node1 context join <PRIVATE_KEY> <INVITATION_PAYLOAD>
   ```

3. **Identity Management**:
   ```bash
   # Generate new identity
   meroctl --node-name node1 identity generate
   ```

### Port Configuration

Each node requires two ports:

1. **Server Port** (default: 2428)

   - Used for JSON-RPC API access
   - CLI-to-node communication
   - Admin operations

2. **Swarm Port** (default: 2528)
   - Used for P2P network communication
   - Node-to-node interactions
   - Network discovery and messaging

### Multiple Nodes Setup

You can run multiple nodes on the same machine by using different ports:

```bash
# First node
merod --node-name node1 init --server-port 2428 --swarm-port 2528
merod --node-name node1 run

# Second node
merod --node-name node2 init --server-port 2429 --swarm-port 2529
merod --node-name node2 run
```

Each node will have:

- Its own identity (different `peer_id`)
- Separate storage space
- Independent set of VMs
- Isolated contexts and applications

This setup is useful for:

- Development and testing
- Running multiple environments
- Network simulation
- Resource isolation

### Getting Help

Both tools provide detailed help information:

```bash
# Get merod help
merod --help

# Get meroctl help
meroctl --help

# Get help for specific commands
meroctl --node-name node1 app --help
meroctl --node-name node1 context --help
```

This CLI interface provides complete control over your nodes, similar to how SSH provides access to traditional VMs.

# Admin Dashboard

Calimero provides a web-based Admin Dashboard that allows users to manage their nodes through a browser interface. The dashboard is:

1. **Locally Available**: Accessible at `http://localhost:NODE_PORT/admin-dashboard/` when running a node
2. **Remotely Available**: Also deployed on Github Pages at `https://calimero-network.github.io/admin-dashboard/`

### Key Features

- Authentication and Identity Management

  - Login with wallet (Metamask, NEAR, etc.)
  - Add and manage root keys
  - List and manage DIDs (Decentralized Identifiers)

- Application Management

  - List installed applications
  - Install new applications
  - Upload and publish applications
  - Manage application contexts

- Node Management
  - View node status and health
  - Monitor storage usage
  - Track network metrics
  - Manage peer connections

### Access and Setup

1. The node serves as the backend for the Admin Dashboard
2. Access requires authentication through a wallet
3. Initial setup involves entering the node URL
4. After setup, users can connect their wallet and sign authentication transactions

# CLI Tools Documentation

Calimero provides multiple interfaces for node management:

1. **Command Line Tools**:

   - `merod` - Node initialization and runtime management
   - `meroctl` - Node interaction and administration

2. **Web Interface**:

   - Admin Dashboard for browser-based management
   - Available both locally and through Github Pages

3. **Desktop Application**:
   - Tauri-based desktop app for node management
   - Available for Linux and macOS
   - Provides GUI access to node operations

## Installation Methods

Both tools can be installed using either Homebrew or installation scripts:

### Homebrew Installation

```bash
# Add Calimero tap
brew tap calimero-network/homebrew-tap

# Install tools
brew install merod
brew install meroctl
```

### Script Installation

```bash
# Install merod
curl -sSf https://raw.githubusercontent.com/calimero-network/core/master/scripts/install-merod.sh | bash

# Install meroctl
curl -sSf https://raw.githubusercontent.com/calimero-network/core/master/scripts/install-meroctl.sh | bash
```

## Documentation Resources

Full documentation for these tools can be found in:

1. `docs/05-developer-tools/01-CLI/01-merod.mdx` - Detailed merod documentation
2. `docs/05-developer-tools/01-CLI/02-meroctl.mdx` - Detailed meroctl documentation
3. Online documentation at [docs.calimero.network](https://docs.calimero.network)

# Node Management Interfaces

Calimero provides multiple interfaces for node management:

1. **Command Line Tools**:

   - `merod` - Node initialization and runtime management
   - `meroctl` - Node interaction and administration

2. **Web Interface**:

   - Admin Dashboard for browser-based management
   - Available both locally and through Github Pages

3. **Desktop Application**:
   - Tauri-based desktop app for node management
   - Available for Linux and macOS
   - Provides GUI access to node operations

# Admin Dashboard

Calimero provides a web-based Admin Dashboard that allows users to manage their nodes through a browser interface. The dashboard is:

1. **Locally Available**: Accessible at `http://localhost:NODE_PORT/admin-dashboard/` when running a node
2. **Remotely Available**: Also deployed on Github Pages at `https://calimero-network.github.io/admin-dashboard/`

### Key Features

- Authentication and Identity Management

  - Login with wallet (Metamask, NEAR, etc.)
  - Add and manage root keys
  - List and manage DIDs (Decentralized Identifiers)

- Application Management

  - List installed applications
  - Install new applications
  - Upload and publish applications
  - Manage application contexts

- Node Management
  - View node status and health
  - Monitor storage usage
  - Track network metrics
  - Manage peer connections

### Access and Setup

1. The node serves as the backend for the Admin Dashboard
2. Access requires authentication through a wallet
3. Initial setup involves entering the node URL
4. After setup, users can connect their wallet and sign authentication transactions

# CLI Tools Documentation

The command-line interface provides two main tools, each with specific responsibilities:

## 1. `merod` - Node Management Tool

`merod` is responsible for node initialization, configuration, and runtime management.

### Key Commands

```bash
merod [OPTIONS] --node-name <n> <COMMAND>
```

Available commands:

- `init` - Initialize node configuration
- `config` - Configure the node
- `run` - Run a node
- `relay` - Relay incoming requests
- `help` - Print help information

### Common Options

- `--home <PATH>` - Directory for config and data (default: `~/.calimero`)
- `--node-name <n>` - Name of node
- `--server-port` - Port for API server (default: 2428)
- `--swarm-port` - Port for P2P communication (default: 2528)

## 2. `meroctl` - Node Interaction Tool

`meroctl` enables interaction with running nodes through various commands.

### Key Commands

```bash
meroctl [OPTIONS] --node-name <n> <COMMAND>
```

Available commands:

- `app` - Manage applications

  - `list` (or `ls`) - List installed applications
  - `install` - Install an application
  - `get` - Fetch application details

- `context` - Manage contexts

  - `list` - List all contexts
  - `create` - Create a new context
  - `join` - Join an application context
  - `invite` - Create invitation to a context
  - `get` - Fetch context details
  - `delete` - Delete a context
  - `watch` - Watch context events
  - `update` - Update app in context

- `identity` - Manage identities

  - `generate` - Generate public/private key pair

- `proxy` - Manage proxy contracts

  - `get` - Fetch proxy contract details

- `call` - Execute RPC calls

### Common Options

- `--home <PATH>` - Directory for config and data
- `--node-name <n>` - Name of node
- `--output-format <FORMAT>` - Output format [json, plain-text]

## Installation Methods

Both tools can be installed using either Homebrew or installation scripts:

### Homebrew Installation

```bash
# Add Calimero tap
brew tap calimero-network/homebrew-tap

# Install tools
brew install merod
brew install meroctl
```

### Script Installation

```bash
# Install merod
curl -sSf https://raw.githubusercontent.com/calimero-network/core/master/scripts/install-merod.sh | bash

# Install meroctl
curl -sSf https://raw.githubusercontent.com/calimero-network/core/master/scripts/install-meroctl.sh | bash
```

## Documentation Resources

Full documentation for these tools can be found in:

1. `docs/05-developer-tools/01-CLI/01-merod.mdx` - Detailed merod documentation
2. `docs/05-developer-tools/01-CLI/02-meroctl.mdx` - Detailed meroctl documentation
3. Online documentation at [docs.calimero.network](https://docs.calimero.network)
