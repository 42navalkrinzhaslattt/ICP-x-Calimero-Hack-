# User Onboarding Journey: From Web App to Node

## Overview

This guide explains how users transition from using a regular web application to running their own Calimero node for data sovereignty.

## User Journey

### 1. Web Application Entry
- User visits the web application (e.g., `https://yourapp.com`)
- Application detects no local node is running
- User is prompted to set up their personal node

### 2. Node Setup
- Download [Calimero Node Manager](https://github.com/calimero-network/node-multiplatform-tauri/releases)
  - Supported platforms: macOS and Linux
  - Available for both x86_64 and ARM architectures
- Install the desktop application
- Initialize and start a node through the GUI interface

### 3. Connecting Web App to Node
- Return to the web application
- Configure node URL (default: `http://localhost:2428`)
- Authenticate using a wallet
- Web app connects to local node via JSON-RPC API

### 4. Ongoing Usage
- Desktop app runs in background managing the node
- Web app interacts with local node for:
  - Data storage and retrieval
  - P2P communication
  - Identity management
  - Application state synchronization

## Benefits of This Approach

### For Users
- Complete data ownership
- Privacy and security through local storage
- Control over data sharing
- Decentralized identity management

### For Developers
- No need to manage user data
- Reduced server costs
- Built-in P2P capabilities
- Focus on application logic

## Technical Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  Web App    │ <-> │  Local Node  │ <-> │ P2P Network │
│ (Browser)   │     │(Desktop App) │     │             │
└─────────────┘     └──────────────┘     └─────────────┘
```

## Common Questions

### Q: Why do I need both a web app and a desktop app?
A: The web app provides the user interface and application logic, while the desktop app manages your node which handles data storage, P2P communication, and ensures data sovereignty.

### Q: Is my data still accessible if I'm offline?
A: Yes, since your data is stored locally on your node, you can still access it even when offline. The node will sync with the network when you're back online.

### Q: What happens if I use multiple devices?
A: You can run nodes on multiple devices and sync your data across them through the P2P network.

## Next Steps
- [Node Management Guide](calimero_node_init.md)
- [Configuration Guide](calimero_config.md)
- [CLI Usage](calimero_todo.md) 