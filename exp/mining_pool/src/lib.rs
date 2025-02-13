/*
*
*   # Start Mining a Block
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> start_mining --args '{"block_data": "block1"}'
*
*   # Miners Join the Pool
*    meroctl --node-name miner1 call --as <EXECUTOR_ID> <CONTEXT_ID> join_mining --args '{"miner_id": "miner1", "hashrate": 100}'
*    meroctl --node-name miner2 call --as <EXECUTOR_ID> <CONTEXT_ID> join_mining --args '{"miner_id": "miner1", "hashrate": 100}'
*
*   # Miners Start Mining
*    meroctl --node-name miner1 call --as <EXECUTOR_ID> <CONTEXT_ID> execute_mining --args '{"miner_id": "miner1"}'
*    meroctl --node-name miner2 call --as <EXECUTOR_ID> <CONTEXT_ID> execute_mining --args '{"miner_id": "miner2"}'
*
*   # Stop Mining
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> stop_mining
*
*   # Retrieve All Mined Blocks
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> get_all_mined_blocks
*
*   # Retrieve Miner Rewards
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> get_miner_rewards --args '{"miner_id": "miner"}'
*/

use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};
use sha2::{Sha256, Digest};
use tokio::sync::broadcast;
use tokio::task;
use std::sync::Arc;
use tokio::sync::Mutex;

/// ✅ **Define Event Types**
#[app::event]
pub enum MiningEvent<'a> {
    MiningStarted { block_data: &'a str },
    MinerJoined { miner_id: &'a str, hashrate: u64 },
    BlockMined { miner_id: &'a str, block_id: &'a str, reward: u64 },
    MiningStopped,
}

/// ✅ **Shared Event Queue for Multi-Node Communication**
type EventBroadcaster = Arc<Mutex<broadcast::Sender<String>>>;

#[app::state(emits = for<'a> MiningEvent<'a>)]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningPool {
    current_block: Option<String>,
    mined_blocks: UnorderedMap<String, String>,
    worker_stats: UnorderedMap<String, (u64, u64, u64)>, // (Hash count, reward, hashrate)
    active_workers: UnorderedMap<String, bool>,
    difficulty: u32,
    is_mining_active: bool,
    total_hashrate: u64,
    event_broadcaster: Option<EventBroadcaster>, // ✅ Event Queue
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        let (sender, _) = broadcast::channel(100);
        let event_broadcaster = Arc::new(Mutex::new(sender));

        // ✅ Start event listener in a background task
        task::spawn(Self::start_event_listener(event_broadcaster.clone()));

        env::log("Mining pool initialized with event listening.");

        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4,
            is_mining_active: false,
            total_hashrate: 0,
            event_broadcaster: Some(event_broadcaster),
        }
    }

    // ================ **Event Listener** ================
    
    /// ✅ **Background Task: Listen for Incoming Events**
    async fn start_event_listener(event_broadcaster: EventBroadcaster) {
        let mut receiver = event_broadcaster.lock().await.subscribe();

        while let Ok(event) = receiver.recv().await {
            env::log(&format!("📥 Received event: {}", event));

            // ✅ React to events
            if event.contains("block_mined") {
                env::log("🚀 New block detected! Stopping mining...");
                let _ = Self::stop_mining();
            }
        }
    }

    // ================ **Mining Functionality** ================

    fn compute_sha256(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn start_mining(&mut self, block_data: String) -> Result<(), Error> {
        if self.is_mining_active {
            return Err(Error::msg("Mining already in progress"));
        }

        self.current_block = Some(block_data.clone());
        self.is_mining_active = true;
        self.worker_stats.clear()?;
        self.active_workers.clear()?;
        self.total_hashrate = 0;

        // ✅ Emit Event: Mining Started
        if let Some(broadcaster) = &self.event_broadcaster {
            let _ = broadcaster.lock().await.send("MiningStarted".to_string());
        }

        env::log(&format!("Mining started for block: {}", block_data));
        Ok(())
    }

    pub fn join_mining(&mut self, miner_id: String, hashrate: u64) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("No active mining session"));
        }

        self.worker_stats.insert(miner_id.clone(), (0, 0, hashrate))?;
        self.active_workers.insert(miner_id.clone(), true)?;
        self.total_hashrate += hashrate;

        // ✅ Emit Event: Miner Joined
        if let Some(broadcaster) = &self.event_broadcaster {
            let _ = broadcaster.lock().await.send(format!("MinerJoined:{}", miner_id));
        }

        env::log(&format!("Miner {} joined with {} H/s", miner_id, hashrate));
        Ok(())
    }

    pub fn execute_mining(&mut self, miner_id: String) -> Result<Option<String>, Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining not active"));
        }

        if !self.active_workers.get(&miner_id)?.unwrap_or(false) {
            return Err(Error::msg("Miner not active"));
        }

        let block_data = match &self.current_block {
            Some(data) => data,
            None => return Err(Error::msg("No active block")),
        };

        if self.mined_blocks.contains(&self.compute_sha256(block_data))? {
            env::log("Block already mined");
            return Ok(None);
        }

        let (mut hash_count, reward, hashrate) = self.worker_stats.get(&miner_id)?.unwrap_or((0, 0, 0));
        let nonce_range = 1_000_000 * hashrate / self.total_hashrate.max(1);
        let mut nonce = 0;

        env::log(&format!("Miner {} starting work on {} hashes", miner_id, nonce_range));

        while nonce < nonce_range {
            let input = format!("{}{}", block_data, nonce);
            let hash = self.compute_sha256(&input);
            hash_count += 1;

            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.mined_blocks.insert(block_id.clone(), block_data.clone())?;
                self.is_mining_active = false;

                let miner_reward = (hash_count * hashrate) / self.total_hashrate.max(1);
                self.worker_stats.insert(miner_id.clone(), (hash_count, miner_reward, hashrate))?;

                // ✅ Emit Event: Block Mined
                if let Some(broadcaster) = &self.event_broadcaster {
                    let _ = broadcaster.lock().await.send(format!("BlockMined:{}", block_id));
                }

                env::log(&format!(
                    "EVENT: block_mined,miner:{},block:{},reward:{}",
                    miner_id, block_id, miner_reward
                ));

                return Ok(Some(block_id));
            }

            nonce += 1;
        }

        self.worker_stats.insert(miner_id, (hash_count, reward, hashrate))?;
        Ok(None)
    }

    pub fn stop_mining(&mut self) -> Result<(), Error> {
        self.is_mining_active = false;

        // ✅ Emit Event: Mining Stopped
        if let Some(broadcaster) = &self.event_broadcaster {
            let _ = broadcaster.lock().await.send("MiningStopped".to_string());
        }

        env::log("Mining stopped");
        Ok(())
    }
}