# Local PoW Mining with Calimero Network

## Project Overview

This project implements a decentralized cryptocurrency system where both mining and ledger maintenance happen across Calimero nodes. ICP will be used only as a bridge/gateway for interoperability, not for core ledger functionality.

## Revised Architecture

### Components Overview

```
┌─────────────────────────────┐     ┌──────────────────┐     ┌────────────────┐
│    Calimero Network         │     │     Web App      │     │   ICP Bridge   │
│ - PoW Mining               │<---->│ - User Interface │<--->│ - Cross-chain  │
│ - Distributed Ledger       │      │ - Wallet UI      │     │   Gateway      │
│ - P2P Consensus           │      │ - Mining Stats   │     │ - Asset Lock   │
│ - Transaction Validation   │      └──────────────────┘     └────────────────┘
└─────────────────────────────┘
```

## Team Structure and Work Division

### Developer 1 (Rust/ICP Developer)

**Focus**: ICP Backend Development

- **Core Tasks**:
  - Implement basic ledger canister in Rust
  - Create block validation logic
  - Set up reward distribution
  - Deploy and test on ICP test network

### Developer 2 (Calimero Core)

**Focus**: Mining Module

- **Core Tasks**:
  - Implement basic PoW algorithm
  - Create mining module structure
  - Handle local state management
  - Basic performance monitoring

### Developer 3 (Calimero Integration)

**Focus**: Network Layer

- **Core Tasks**:
  - Set up Calimero node configuration
  - Implement basic P2P for miners
  - Handle ICP proxy integration
  - Manage state synchronization

### Developer 4 (Frontend)

**Focus**: Web Application

- **Core Tasks**:
  - Create mining dashboard
  - Implement wallet connection
  - Build basic mining controls
  - Display real-time status

## 10-Day Development Timeline

### Days 1-2: Foundation

- **Dev 1**:
  - Set up ICP dev environment
  - Create basic canister structure
- **Dev 2**:
  - Implement basic PoW algorithm
  - Test on single node
- **Dev 3**:
  - Configure Calimero node
  - Set up basic proxy
- **Dev 4**:
  - Set up web app project
  - Create basic UI structure

### Days 3-5: Core Features

- **Dev 1**:
  - Basic ledger functionality
  - Simple reward mechanism
- **Dev 2**:
  - Complete mining module
  - Basic difficulty adjustment
- **Dev 3**:
  - Basic P2P implementation
  - ICP proxy communication
- **Dev 4**:
  - Mining controls UI
  - Wallet integration

### Days 6-8: Integration

- **Dev 1**:
  - Test network deployment
  - API documentation
- **Dev 2**:
  - Network integration
  - Performance testing
- **Dev 3**:
  - Multi-node testing
  - State sync testing
- **Dev 4**:
  - Backend integration
  - Real-time updates

### Days 9-10: Polish & Demo

- All team members:
  - Bug fixes
  - Demo preparation
  - Documentation
  - Testing

## MVP Requirements

### Must Have

1. **Mining**

   - Basic PoW mining on local node
   - Difficulty adjustment
   - Mining start/stop

2. **ICP Integration**

   - Simple ledger
   - Block submission
   - Balance checking

3. **User Interface**
   - Mining controls
   - Balance display
   - Mining status

### Nice to Have

- Mining pool functionality
- Performance analytics
- Transaction history

### Out of Scope

- Advanced consensus
- Complex governance
- Multiple mining algorithms

## Daily Schedule

### Morning (30 min)

- Team sync
- Blocker resolution
- Day planning

### Evening (30 min)

- Progress check
- Integration testing
- Next day prep

## Integration Points

1. **ICP ↔ Calimero Bridge**

   - Cross-chain transaction verification
   - Lock/mint protocol implementation
   - State proof validation

2. **Calimero Node ↔ Node**

   - Block propagation
   - Transaction broadcasting
   - Peer discovery and management
   - Consensus messaging

3. **Calimero ↔ Frontend**

   - Mining control API
   - Wallet operations
   - Network statistics
   - Real-time updates

4. **Development Interfaces**
   - API documentation
   - SDK implementations
   - Testing frameworks
   - Monitoring tools

## Development Flow

1. **Local Development**

   - Individual testing
   - Component integration
   - Performance testing

2. **Integration Testing**

   - Multi-node setup
   - End-to-end testing
   - User flow validation

3. **Demo Preparation**
   - Test deployment
   - Demo script
   - Documentation

## Success Criteria

1. **Basic Functionality**

   - Mining works on local node
   - Blocks are submitted to ICP
   - UI shows mining status

2. **Integration**

   - Components work together
   - State stays synchronized
   - User flow is smooth

3. **Performance**
   - Mining is efficient
   - UI is responsive
   - Network is stable

## Revised Architecture Details

### Decentralized Components

1. **Distributed Ledger**

   - Each Calimero node maintains a full copy of the ledger
   - Consensus achieved through PoW and node agreement
   - Transaction validation done by network participants

2. **Mining System**

   - PoW mining distributed across nodes
   - Dynamic difficulty adjustment based on network hashrate
   - Block rewards distributed directly by consensus rules

3. **P2P Network**
   - Direct node-to-node communication
   - Gossip protocol for transaction/block propagation
   - Peer discovery and maintenance

### ICP Integration

- **Bridge Mechanism**

  - Lock/mint mechanism similar to sBTC
  - 1:1 backing of wrapped tokens
  - Cryptographic proof of reserves

- **Bridge Operations**

  - Lock native coins on Calimero network
  - Mint equivalent wrapped tokens on ICP
  - Burn wrapped tokens to release native coins
  - Automated verification of cross-chain transactions

- **Security Model**
  - Native blockchain security not dependent on ICP
  - Bridge security through cryptographic proofs
  - Multiple signature requirements for bridge operations

## Security Benefits

1. **True Decentralization**

   - No central point of failure
   - Network security based on PoW and node distribution
   - Consensus achieved through network participants

2. **Autonomous Operation**

   - Network can function without ICP connection
   - Independent transaction validation
   - Self-sustaining reward mechanism

3. **Trust Minimization**
   - No need to trust ICP network for core operations
   - Cryptographic proof of work
   - Transparent consensus rules

## Detailed Plan - Days 1-2

### Day 1

#### Dev 1 (Rust/ICP Developer)

- Set up dfx environment and configure project structure
- Create initial canister template with basic methods
- Set up unit testing framework
- Implement basic data structures for blocks and transactions
- Create initial state management for the ledger
- Write basic tests for data structures

**Deliverables**

- Working ICP development environment
- Basic canister structure with test coverage
- Initial data models for blocks and transactions

#### Dev 2 (Calimero Core)

- Set up Rust development environment for mining module
- Design basic PoW algorithm structure
- Create initial mining module interfaces

- Implement basic SHA-256 based PoW algorithm
- Create mining state management structure
- Set up basic benchmarking tools

**Deliverables**

- Working PoW algorithm implementation
- Basic mining state management
- Initial performance metrics

#### Dev 3 (Calimero Integration)

- Set up local Calimero node environment
- Configure basic node settings
- Create initial proxy structure
- Implement basic node configuration management
- Set up logging and monitoring
- Create initial proxy endpoints

**Deliverables**

- Working Calimero node setup
- Basic proxy configuration
- Initial logging system

#### Dev 4 (Frontend)

**Morning**

- Set up React/TypeScript project structure
- Create initial component hierarchy
- Set up development tools (ESLint, Prettier)

**Afternoon**

- Implement basic layout components
- Create placeholder mining dashboard
- Set up routing structure

**Deliverables**

- Working frontend development environment
- Basic UI structure
- Initial component library

### Day 2

### Dev 1 (Rust/ICP Developer)

- Implement basic block validation logic
- Create methods for block submission
- Set up basic balance tracking
- Implement query methods for ledger state
- Create basic transaction validation
- Write integration tests

**Deliverables**

- Working block validation
- Basic ledger query functionality
- Integration test suite

#### Dev 2 (Calimero Core)

- Implement difficulty adjustment mechanism
- Create mining job queue
- Set up worker thread management
- Implement basic mining result validation
- Create performance monitoring tools
- Test mining module on single node

**Deliverables**

- Working difficulty adjustment
- Basic mining job management
- Performance monitoring system

#### Dev 3 (Calimero Integration)

- Set up basic network configuration
- Implement initial ICP proxy methods
- Create state synchronization structure
- Implement basic error handling
- Create health check endpoints
- Set up basic metrics collection

**Deliverables**

- Working network configuration
- Basic proxy communication
- Health monitoring system

#### Dev 4 (Frontend)

**Morning**

- Create mining control components
- Implement basic state management
- Set up API service structure

**Afternoon**

- Create basic wallet UI components
- Implement mining statistics display
- Set up WebSocket connection structure

**Deliverables**

- Working mining controls
- Basic wallet interface
- Initial real-time updates structure

### Integration Points for Days 1-2

1. **End of Day 1**

   - Team sync to ensure all development environments are properly set up
   - Review of initial architectures and interfaces
   - Alignment on communication protocols between components

2. **End of Day 2**
   - Integration test of basic mining functionality
   - Review of API endpoints and interfaces
   - Verification of development progress against MVP requirements

### Success Metrics for Days 1-2

1. **Technical Setup**

   - All development environments are properly configured
   - Basic components are implemented and tested individually
   - Initial interfaces are defined and documented

2. **Team Coordination**

   - Clear understanding of integration points
   - Established communication protocols
   - Defined API contracts between components

3. **Progress Tracking**
   - All basic components are implemented
   - Initial integration points are tested
   - Development is on track for core features

### Simplified Consensus Approach

1. **Basic PoW Consensus**

   - Use standard Bitcoin-style PoW
   - Longest chain rule for fork resolution
   - Basic difficulty adjustment

2. **Initial Limitations**
   - Single confirmation (no waiting for multiple blocks)
   - Simple network propagation
   - Basic fork handling
3. **Future Improvements** (post-hackathon)
   - Multiple block confirmations
   - Advanced fork resolution
   - Network partition handling
   - Advanced difficulty adjustment
