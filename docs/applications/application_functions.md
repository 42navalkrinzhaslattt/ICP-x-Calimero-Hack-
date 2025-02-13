## Calimero Blockchain Setup Guide

### Context Apps: Key-Value Store, Mining (One Node), Mining Pool

### Create and Run 2 or More Nodes
```sh
merod --node-name node1 init --server-port 2427 --swarm-port 2527
merod --node-name node2 init --server-port 2428 --swarm-port 2528

merod --node-name node1 run
merod --node-name node2 run
```

### Install an App on One of the Nodes
```sh
application install file <PATH_TO_blockchain.wasm_FILE>
```
Installed application: `<APPLICATION_ID>`

[GitHub Repository](https://github.com/552020/calimero_x_icp_hackathon/tree/antonsplavnik/cal-65-build-letter-exchange-app/basic_setup/context/key_value_store)

### Create a New Context
```sh
context create <APPLICATION_ID> --protocol icp
```
Created context `<CONTEXT_ID>` with identity `<CONTEXT_IDENTITY>`

### Create a New Identity for Node2
```sh
identity new
```
Private key: `<PRIVATE_KEY>`  
Public key: `<PUBLIC_KEY>`  

### Invite Node2 from Node1 to the Context
```sh
context invite <CONTEXT_ID> <CONTEXT_IDENTITY> <PUBLIC_KEY_OF_NODE2>
```
Invitation payload: `<INVITATION_PAYLOAD>`

### Accept an Invitation from Node2
```sh
context join <PRIVATE_KEY_OF_NODE2> <INVITATION_PAYLOAD>
```

### Retrieve Values After Setting Nodes and Context
```sh
context ls
identity ls <CONTEXT_ID>
```

## Key-Value Store App
### Call Functions from Another (New) Terminal
```sh
meroctl --node-name <NODE NAME> call --as <EXECUTOR_IDENTITY_ID> <CONTEXT_ID> set --args '{"key": "username", "value": "Alice"}'

meroctl --node-name <NODE NAME> call --as <EXECUTOR_IDENTITY_ID> <CONTEXT_ID> get --args '{"key": "username"}'

meroctl --node-name <NODE NAME> call --as <EXECUTOR_IDENTITY_ID> <CONTEXT_ID> entries
```

## Miner App
### Basic Mining Application on Calimero Blockchain
#### Call Functions from Another (New) Terminal

#### Start Mining a Block
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> mine_block --args '{"miner_id": "node1", "data": "tx-data"}'
```

#### Stop All Mining Processes
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> stop_mining
```

#### Resume Mining
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> start_mining
```

#### Retrieve All Mined Blocks
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> get_latest_block
```

## Mining Pool App
### Basic Mining Pool Application on Calimero Blockchain

#### Start Mining a Block
```sh
meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> start_mining --args '{"block_data": "block1"}'
```

#### Miners Join the Pool
```sh
meroctl --node-name miner1 call --as <EXECUTOR_ID> <CONTEXT_ID> join_mining --args '{"miner_id": "miner1", "hashrate": 100}'
meroctl --node-name miner2 call --as <EXECUTOR_ID> <CONTEXT_ID> join_mining --args '{"miner_id": "miner2", "hashrate": 100}'
```

#### Miners Start Mining
```sh
meroctl --node-name miner1 call --as <EXECUTOR_ID> <CONTEXT_ID> execute_mining --args '{"miner_id": "miner1"}'
meroctl --node-name miner2 call --as <EXECUTOR_ID> <CONTEXT_ID> execute_mining --args '{"miner_id": "miner2"}'
```

#### Stop Mining
```sh
meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> stop_mining
```

#### Retrieve All Mined Blocks
```sh
meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> get_all_mined_blocks
```

#### Retrieve Miner Rewards
```sh
meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> get_miner_rewards --args '{"miner_id": "miner"}'
```

## Chat App
### Simple Chat Application on Calimero Blockchain

#### Send a Message to Chat
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> send_message --args '{"sender": "id", "content": "message text"}'
```

#### Get the Latest Messages from the Chat
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> get_all_messages
