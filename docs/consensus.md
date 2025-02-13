# Simplified Consensus Mechanism for Hackathon PoC

## Overview

This document outlines a basic Proof-of-Work (PoW) consensus mechanism that can be implemented within hackathon constraints while maintaining essential security properties.

## Core Components

### 1. Block Structure

```rust
struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
    nonce: u64,
    hash: Hash,
}

struct BlockHeader {
    version: u32,
    previous_hash: Hash,
    merkle_root: Hash,
    timestamp: u64,
    difficulty_target: u32,
}
```

### 2. Basic PoW Algorithm

```rust
fn find_nonce(block_header: &BlockHeader) -> u64 {
    let mut nonce = 0;
    loop {
        let hash = calculate_hash(block_header, nonce);
        if meets_difficulty(hash, block_header.difficulty_target) {
            return nonce;
        }
        nonce += 1;
    }
}
```

## Simplified Consensus Rules

### Block Validation

1. **Basic Checks**

   - Valid block structure
   - Correct previous block hash
   - Valid timestamp (within 2 hours of current time)
   - Valid proof-of-work solution

2. **Transaction Validation**
   - No double spending
   - Valid signatures
   - Correct transaction format

### Chain Selection

1. **Longest Chain Rule**

   ```rust
   fn is_better_chain(chain_a: &Chain, chain_b: &Chain) -> bool {
       chain_a.total_work > chain_b.total_work
   }
   ```

2. **Fork Resolution**
   - Keep track of last 6 blocks
   - Switch to longer chain immediately
   - Store competing chains in memory

## Network Protocol

### 1. Block Propagation

```rust
async fn propagate_block(block: Block) {
    // Announce new block
    broadcast_message(NetworkMessage::NewBlock(block.header));

    // Respond to getblock requests
    for peer in connected_peers {
        if peer.requests_block {
            send_block(peer, block);
        }
    }
}
```

### 2. Transaction Broadcasting

```rust
async fn broadcast_transaction(tx: Transaction) {
    // Simple flooding protocol
    for peer in connected_peers {
        peer.send(NetworkMessage::Transaction(tx));
    }
}
```

## Simplified Difficulty Adjustment

```rust
fn adjust_difficulty(last_blocks: &[Block]) -> u32 {
    // Adjust every 10 blocks
    if current_height % 10 != 0 {
        return current_difficulty;
    }

    let time_diff = last_blocks.last().timestamp - last_blocks.first().timestamp;
    let target_time = BLOCK_TIME * 10; // e.g., 10 minutes * 10 blocks

    if time_diff < target_time {
        current_difficulty + 1
    } else {
        current_difficulty - 1
    }
}
```

## State Management

### 1. UTXO Set

```rust
struct UTXOSet {
    utxos: HashMap<OutPoint, TransactionOutput>,
}

impl UTXOSet {
    fn apply_block(&mut self, block: &Block) {
        for tx in block.transactions {
            // Remove spent outputs
            for input in tx.inputs {
                self.utxos.remove(&input.previous_output);
            }
            // Add new outputs
            for (index, output) in tx.outputs.iter().enumerate() {
                self.utxos.insert(OutPoint::new(tx.hash, index), output.clone());
            }
        }
    }
}
```

### 2. Mempool Management

```rust
struct Mempool {
    transactions: HashMap<Hash, Transaction>,
    max_size: usize,
}

impl Mempool {
    fn add_transaction(&mut self, tx: Transaction) -> Result<(), Error> {
        if self.transactions.len() >= self.max_size {
            return Err(Error::MempoolFull);
        }
        // Basic validation
        if !tx.verify() {
            return Err(Error::InvalidTransaction);
        }
        self.transactions.insert(tx.hash(), tx);
        Ok(())
    }
}
```

## Limitations & Future Improvements

### Current Limitations

- Single confirmation finality
- Basic difficulty adjustment
- Simple networking protocol
- No advanced fork handling
- Limited mempool management

### Post-Hackathon Improvements

1. **Security**

   - Multiple confirmation requirements
   - Advanced fork resolution
   - Memory-efficient UTXO tracking

2. **Performance**

   - Optimized block propagation
   - Better peer discovery
   - Improved mempool management

3. **Robustness**
   - Network partition handling
   - Advanced difficulty adjustment
   - Chain reorganization improvements

## Implementation Priority

1. **Day 1-2**

   - Basic block structure
   - Simple PoW mining
   - Basic networking

2. **Day 3-4**

   - UTXO tracking
   - Basic mempool
   - Transaction validation

3. **Day 5+**
   - Chain selection
   - Fork handling
   - Difficulty adjustment
