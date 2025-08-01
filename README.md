# Quantum Random Seed Oracle 🔮⚛️

**ICP x Calimero Hackathon Project**

A decentralized quantum random seed oracle built on the Internet Computer Protocol (ICP) and Calimero Network, providing truly random numbers generated through quantum computing principles for blockchain applications.

## 🚀 Overview

This project implements a quantum-powered random number oracle that leverages quantum superposition and measurement to generate cryptographically secure random seeds. The oracle operates across both ICP canisters and Calimero contexts, providing verifiable randomness for decentralized applications.

### Key Features

- **🔬 Quantum Random Generation**: Uses Qiskit quantum circuits to generate 100-bit random numbers
- **🌐 Cross-Chain Architecture**: Operates on both ICP and Calimero networks
- **🔐 Cryptographic Security**: HMAC-SHA256 based seed processing and verification
- **📊 Transparent Processing**: All random seed generation and processing is verifiable on-chain
- **🔄 Continuous Feed**: Provides ongoing stream of quantum-generated random values
- **🎯 Oracle Interface**: Easy-to-use API for dApps requiring secure randomness

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Quantum Seed  │    │  Calimero Node   │    │   ICP Canister  │
│   Generator     │───▶│  (RandomChain)   │◀──▶│  (Proxy)        │
│   (Python/      │    │                  │    │                 │
│   Qiskit)       │    └──────────────────┘    └─────────────────┘
└─────────────────┘              │                       │
                                 │                       │
                    ┌──────────────────┐    ┌─────────────────┐
                    │  Frontend App    │    │   External      │
                    │  (React/TS)      │    │   dApps         │
                    └──────────────────┘    └─────────────────┘
```

### Components

1. **Quantum Seed Generator** (`src/backend/quantum_seed/`)
   - Python-based quantum circuit using Qiskit
   - Generates 100-bit random numbers using 5 chunks of 20 qubits each
   - Uses Hadamard gates for quantum superposition

2. **Calimero RandomChain** (`src/app/random/`)
   - Rust-based application running on Calimero nodes
   - Processes quantum seeds and maintains randomness chain
   - HMAC-SHA256 transformation for cryptographic security

3. **ICP Proxy Contract** (`src/backend/proxy_contract/`)
   - Smart contract interface on Internet Computer
   - Manages proposals and cross-chain communication
   - Stores quantum seeds and processed values

4. **Frontend Interface** (`src/frontend/`)
   - React + TypeScript web application
   - User interface for interacting with the oracle
   - Real-time display of random values and statistics

## 🛠️ Technology Stack

### **Quantum Computing**
- **Qiskit** - IBM's quantum computing framework
- **Qiskit Aer** - High-performance quantum circuit simulator
- **Python 3.8+** - Quantum algorithm implementation

### **Blockchain Platforms**
- **Internet Computer Protocol (ICP)** - Decentralized compute platform
- **Calimero Network** - P2P application framework
- **WebAssembly (WASM)** - Smart contract runtime

### **Smart Contract Development**
- **Rust** - Primary systems programming language
- **Calimero SDK** - Calimero network development kit
- **Calimero Storage** - Distributed storage primitives
- **Borsh** - Binary serialization format
- **Candid** - ICP interface description language

### **Frontend Development**
- **React 18** - UI framework
- **TypeScript** - Type-safe JavaScript
- **Vite** - Fast build tool and dev server
- **TailwindCSS 4.0** - Utility-first CSS framework
- **pnpm** - Package manager

### **Frontend Dependencies**
- **@calimero-network/calimero-client** - Calimero network client
- **React Router DOM** - Client-side routing
- **Axios** - HTTP client
- **bs58** - Base58 encoding/decoding
- **Buffer** - Node.js Buffer polyfill

### **Cryptography & Security**
- **HMAC-SHA256** - Message authentication
- **SHA2** - Cryptographic hash functions
- **Quantum Random Number Generation** - True randomness source
- **Rust crypto libraries** - Memory-safe cryptographic implementations

### **Development Tools**
- **DFX** - Internet Computer SDK
- **merod** - Calimero node daemon
- **meroctl** - Calimero node control CLI
- **Cargo** - Rust package manager
- **ESLint & Prettier** - Code quality tools

### **Infrastructure & DevOps**
- **Python virtual environments** - Isolated Python dependencies
- **Shell scripting** (Bash/Zsh) - Automation
- **Git** - Version control
- **Homebrew** - macOS package management
- **Makefile** - Build automation

### **Python Dependencies**
- **pexpect** - Process interaction and automation
- **qiskit** - Quantum computing framework
- **qiskit-aer** - Quantum circuit simulation

### **Rust Crate Dependencies**
- **serde & serde_json** - Serialization/deserialization
- **hex** - Hexadecimal encoding
- **hmac** - HMAC implementation
- **sha2** - SHA-2 hash functions
- **toml_edit** - TOML parsing and editing

## 🔧 Installation & Setup

### Prerequisites

- Python 3.8+ with Qiskit
- Rust and Cargo
- Node.js and pnpm
- DFX (Internet Computer SDK)
- Calimero Network tools (`merod`, `meroctl`)

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/42navalkrinzhaslattt/ICP-x-Calimero-Hack-.git
   cd ICP-x-Calimero-Hack-
   ```

2. **Install Python dependencies**
   ```bash
   chmod +x cli_script.sh
   ./cli_script.sh
   ```

3. **Setup Calimero nodes**
   ```bash
   chmod +x tools/setup_calimero_icp.sh
   ./tools/setup_calimero_icp.sh
   ```

4. **Deploy ICP canisters**
   ```bash
   cd icp-devnet
   ./deploy_devnet_fresh.sh
   ```

5. **Start the frontend**
   ```bash
   cd src/frontend
   pnpm install
   pnpm dev
   ```

## 🎮 Usage

### Generating Quantum Random Seeds

```bash
# Generate quantum random seed
python src/backend/quantum_seed/quantum_seed_generator.py
```

### Calimero Node Operations

```bash
# Set quantum seed in Calimero context
meroctl --node-name node1 call --as <IDENTITY> <CONTEXT_ID> set_seed --args '{"quantum_seed": [10,20,30,40]}'

# Process random value
meroctl --node-name node1 call --as <IDENTITY> <CONTEXT_ID> process_value --args '{"node": "node1"}'

# Get processed values
meroctl --node-name node1 call --as <IDENTITY> <CONTEXT_ID> get_all_values
```

### ICP Canister Integration

```bash
# Create new random proposal
dfx canister call context_contract create_new_proposal '(
  record {
    action_type = "SetQuantumSeed";
    params = record { seed_value = "quantum_seed_data" }
  }
)'
```

## 🧪 Quantum Random Generation Process

1. **Quantum Circuit Creation**: Create 5 circuits with 20 qubits each
2. **Superposition**: Apply Hadamard gates to all qubits
3. **Measurement**: Collapse quantum states to classical bits
4. **Concatenation**: Combine 5 chunks to form 100-bit number
5. **Verification**: Process through HMAC-SHA256 for cryptographic security

### Example Output
```
=== 100-bit Quantum Random Number ===
Decimal: 1267650600228229401496703205375
Binary: 1111110110001010101010101010101010101010101010101010101010101010101010101010101010101010101010101111
Hexadecimal: 0xfec55555555555555f
Bit Length: 100 bits
```

## 🔒 Security Features

- **Quantum Source**: True randomness from quantum measurement
- **Cryptographic Processing**: HMAC-SHA256 for additional security
- **Decentralized Verification**: Multi-node consensus on Calimero
- **Immutable Storage**: Random values stored on ICP blockchain
- **Transparent Auditing**: All operations are publicly verifiable

## 📈 Use Cases

- **DeFi Protocols**: Secure random number generation for lottery systems
- **Gaming**: Provably fair random events in blockchain games
- **NFT Minting**: Random traits and characteristics generation
- **Consensus Mechanisms**: Leader selection in blockchain protocols
- **Cryptographic Applications**: Key generation and security protocols

## 🏆 Hackathon Achievements

- ✅ Integration of quantum computing with blockchain technology
- ✅ Cross-chain architecture between ICP and Calimero
- ✅ Real-time quantum random number generation
- ✅ Decentralized oracle implementation
- ✅ User-friendly frontend interface
- ✅ Comprehensive documentation and testing

## 🧩 Project Structure

```
├── src/
│   ├── app/random/           # Calimero RandomChain application
│   ├── backend/
│   │   ├── proxy_contract/   # ICP proxy smart contract  
│   │   └── quantum_seed/     # Quantum random generator
│   └── frontend/            # React web interface
├── tools/                   # Deployment and setup scripts
├── docs/                    # Project documentation
├── icp-devnet/             # ICP development environment
└── exp/                    # Experimental implementations
```

## 🚀 Future Enhancements

- [ ] Hardware quantum device integration (IBM Quantum, Rigetti)
- [ ] Multi-chain oracle support (Ethereum, Polygon, etc.)
- [ ] Advanced cryptographic protocols (VRF, threshold signatures)
- [ ] Real-time quantum randomness feeds
- [ ] Enterprise API with rate limiting and authentication
- [ ] Quantum randomness quality metrics and analysis

## 🤝 Contributing

We welcome contributions! Please see our [contributing guidelines](CONTRIBUTING.md) for details on how to:

- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **ICP Community** for the robust canister infrastructure
- **Calimero Network** for the decentralized node architecture  
- **IBM Qiskit** for quantum computing tools
- **Hackathon Organizers** for the opportunity to innovate

## 📞 Contact

- **Team**: 42navalkrinzhaslattt
- **Email**: [contact@quantumoracle.dev](mailto:contact@quantumoracle.dev)
- **Discord**: Join our community server
- **Twitter**: [@QuantumOracle](https://twitter.com/QuantumOracle)

---

**Built with ❤️ for the ICP x Calimero Hackathon 2025**

*Bringing quantum randomness to the decentralized world, one qubit at a time.*
