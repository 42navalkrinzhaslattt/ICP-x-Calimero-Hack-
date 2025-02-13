
// ================ Chat ================
/*
use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::Vector;
use serde::{Serialize, Deserialize};

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct ChatApp {
    messages: Vector<Message>,
    message_sequence: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Serialize, Deserialize)]
struct Message {
    content: String,
    sender: String,
    sequence: u64,
}

#[app::logic]
impl ChatApp {
    #[app::init]
    pub fn init() -> ChatApp {
        env::log("Initializing minimal chat application");
        ChatApp {
            messages: Vector::new(),
            message_sequence: 0,
        }
    }

    /// Send a text message - CORRECTED ACCOUNT ID METHOD
    pub fn send_message(&mut self, sender: String, content: String) -> Result<u64, Error> {
        if content.len() > 280 {
            return Err(Error::msg("Message too long (max 280 characters)"));
        }
    
        let sequence = self.message_sequence;
        let message = Message {
            content: Self::sanitize_input(&content),
            sender, 
            sequence,
        };
        
        self.messages.push(message.clone())?;
        self.message_sequence += 1;
        
        env::log(&format!(
            "EVENT: message_sent,sender:{},sequence:{}",
            message.sender, message.sequence
        ));
    
        Ok(sequence)
    }

    /// Get all messages
    pub fn get_all_messages(&self) -> Result<Vec<Message>, Error> {
        let mut result = Vec::new();
        let len = self.messages.len()?;

        for i in 0..len {
            if let Ok(msg_opt) = self.messages.get(i) {
                if let Some(msg) = msg_opt {
                    result.push(msg);
                }
            }
        }
        Ok(result)
    }

    /// Sanitize input
    fn sanitize_input(input: &str) -> String {
        input
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('&', "&amp;")
            .replace('\n', "<br>")
    }
}
*/

// ================ Miner ================
/*
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;
use sha2::{Sha256, Digest}; // Import SHA-256 hashing functionality


/// **Simulated Blockchain Miner**
#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MinerState {
    blocks: UnorderedMap<String, String>, // Store mined blocks
    difficulty: u32,  // The mining difficulty level
}

#[app::logic]
impl MinerState {
    /// **Initialize the Miner for All Nodes**
    #[app::init]
    pub fn init() -> MinerState {
        env::log("Initializing miner simulation...");
        MinerState {
            blocks: UnorderedMap::new(),
            difficulty: 4, // Set mining difficulty (higher = slower mining)
        }
    }

    /// **Computes a SHA-256 hash of the given input string**
    pub fn compute_sha256(input: &str) -> String {  // 👈 Add `pub` so it can be called
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result) // Convert bytes to hex string
    }

    /// **Simulate a Mining Process**
    // pub fn mine_block(&mut self, miner_id: String, data: String) -> Result<String, Error> {
    //     env::log(&format!("Miner {} is attempting to mine a block...", miner_id));

    //     let mut nonce = 0;
    //     loop {
    //         let input = format!("{}{}{}", miner_id, data, nonce);
    //         let hash = Self::compute_sha256(&input);

    //         // Check if hash meets difficulty criteria (leading zeros)
    //         if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
    //             let block_id = format!("block-{}", hash);
    //             self.blocks.insert(block_id.clone(), data.clone())?;

    //             // Emit event so other nodes know a block was mined
    //             env::log(&format!("EVENT: block_mined, miner: {}, block: {}", miner_id, block_id));

    //             return Ok(block_id);
    //         }

    //         nonce += 1;
    //     }
    // }   

    pub fn mine_block(&mut self, miner_id: String, data: String) -> Result<String, Error> {
        env::log(&format!("Miner {} is starting mining...", miner_id));
    
        let mut nonce = 0;
        let mut blocks_mined = 0;
    
        loop {
            let input = format!("{}{}{}", miner_id, data, nonce);
            let hash = Self::compute_sha256(&input);
    
            // **Only log every 10,000 iterations to prevent overflow**
            if nonce % 10_000 == 0 {
                env::log(&format!("Miner {} progressing... Nonce: {}", miner_id, nonce));
            }
    
            // **Check if hash meets difficulty criteria**
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.blocks.insert(block_id.clone(), data.clone())?;
                blocks_mined += 1;
    
                // ✅ **Only log when a block is mined**
                env::log(&format!("EVENT: block_mined, miner: {}, block: {}, total blocks: {}", miner_id, block_id, blocks_mined));
    
                if blocks_mined >= 5 {
                    env::log(&format!("Miner {} stopping after mining {} blocks.", miner_id, blocks_mined));
                    break;
                }
            }
    
            nonce += 1;
    
            // ✅ **Stop mining after a max nonce to avoid infinite mining**
            if nonce > 1_000_000 {
                env::log(&format!("Miner {} reached max nonce limit. Stopping mining.", miner_id));
                break;
            }
        }
    
        Ok(format!("Miner {} completed mining {} blocks.", miner_id, blocks_mined))
    }
    

    /// **Retrieve Latest Mined Block**
    pub fn get_latest_block(&self) -> Result<Option<String>, Error> {
        env::log("Fetching latest mined block...");
        let latest_block = self.blocks.entries()?.last();    
        Ok(latest_block.map(|(block_id, _data)| block_id))
    }
    
}*/







// ================ Mining Pool latest version================
/*
use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};
use sha2::{Sha256, Digest};

/// ✅ **Define Emitted Events for Real-Time Monitoring**
#[app::event]
pub enum MiningEvent<'a> {
    MiningStarted { block_data: &'a str },
    MinerJoined { miner_id: &'a str, hashrate: u64 },
    BlockMined { miner_id: &'a str, block_id: &'a str, reward: u64 },
    MiningStopped,
}

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
    chat_messages: Vector<ChatMessage>,
    chat_sequence: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
struct ChatMessage {
    sender: String,
    message: String,
    sequence: u64,
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        env::log("Initializing mining pool with event emissions.");
        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4,
            is_mining_active: false,
            total_hashrate: 0,
            chat_messages: Vector::new(),
            chat_sequence: 0,
        }
    }

    // ================ **Mining Functionality** ================

    /// **Compute SHA-256 hash**
    fn compute_sha256(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// **Start the Mining Process**
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
        app::emit!(MiningEvent::MiningStarted { block_data: &block_data });

        env::log(&format!("Mining started for block: {}", block_data));
        Ok(())
    }

    /// **Join Mining Pool**
    pub fn join_mining(&mut self, miner_id: String, hashrate: u64) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("No active mining session"));
        }

        self.worker_stats.insert(miner_id.clone(), (0, 0, hashrate))?;
        self.active_workers.insert(miner_id.clone(), true)?;
        self.total_hashrate += hashrate;

        // ✅ Emit Event: Miner Joined
        app::emit!(MiningEvent::MinerJoined { miner_id: &miner_id, hashrate });

        env::log(&format!("Miner {} joined with {} H/s", miner_id, hashrate));
        Ok(())
    }

    /// **Execute Mining Task**
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
                app::emit!(MiningEvent::BlockMined {
                    miner_id: &miner_id,
                    block_id: &block_id,
                    reward: miner_reward
                });

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

    /// **Stop Mining**
    pub fn stop_mining(&mut self) -> Result<(), Error> {
        self.is_mining_active = false;

        // ✅ Emit Event: Mining Stopped
        app::emit!(MiningEvent::MiningStopped);

        env::log("Mining stopped");
        Ok(())
    }

    // ================ **Utility Methods** ================

    pub fn get_all_mined_blocks(&self) -> Result<Vec<String>, Error> {
        let mut blocks = Vec::new();
        for (block_id, _) in self.mined_blocks.entries()? {
            blocks.push(block_id);
        }
        Ok(blocks)
    }

    pub fn get_miner_reward(&self, miner_id: String) -> Result<u64, Error> {
        match self.worker_stats.get(&miner_id)? {
            Some((_, reward, _)) => Ok(reward),
            None => Err(Error::msg("Miner not found")),
        }
    }
}
*/



/* 
use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};
use sha2::{Sha256, Digest};

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningPool {
    current_block: Option<String>,
    mined_blocks: UnorderedMap<String, String>,
    worker_stats: UnorderedMap<String, (u64, u64, u64)>,
    active_workers: UnorderedMap<String, bool>,
    difficulty: u32,
    is_mining_active: bool,
    total_hashrate: u64,
    chat_messages: Vector<ChatMessage>,
    chat_sequence: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
struct ChatMessage {
    sender: String,
    message: String,
    sequence: u64,
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        env::log("Initializing mining pool with chat system");
        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4,
            is_mining_active: true,
            total_hashrate: 0,
            chat_messages: Vector::new(),
            chat_sequence: 0,
        }
    }

    // ================ Mining Functionality ================
    
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

        env::log(&format!("Starting mining for block: {}", block_data));
        Ok(())
    }

    pub fn join_mining(&mut self, miner_id: String, hashrate: u64) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("No active mining session"));
        }

        self.worker_stats.insert(miner_id.clone(), (0, 0, hashrate))?;
        self.active_workers.insert(miner_id.clone(), true)?;
        self.total_hashrate += hashrate;

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
        env::log("Mining stopped");
        Ok(())
    }

    // ================ Utility Methods ================

    pub fn get_all_mined_blocks(&self) -> Result<Vec<String>, Error> {
        let mut blocks = Vec::new();
        for (block_id, _) in self.mined_blocks.entries()? { // Using entries instead of keys
            blocks.push(block_id);
        }
        Ok(blocks)
    }

    pub fn get_miner_reward(&self, miner_id: String) -> Result<u64, Error> {
        match self.worker_stats.get(&miner_id)? {
            Some((_, reward, _)) => Ok(reward),
            None => Err(Error::msg("Miner not found")),
        }
    }
}
 */





// ================ Mining Pool OLD VERSION================
/* 
use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};
use sha2::{Sha256, Digest};
use std::convert::TryInto;
use calimero_sdk::serde::{Serialize, Deserialize};

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningPool {
    current_block: Option<String>, 
    mined_blocks: UnorderedMap<String, String>,
    worker_stats: UnorderedMap<String, (u64, u64, u64)>,
    active_workers: UnorderedMap<String, bool>,
    difficulty: u32, 
    is_mining_active: bool, 
    total_hashrate: u64, 
    chat_messages: Vector<ChatMessage>, 
    chat_sequence: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)] // ✅ Correct placement
#[borsh(crate = "calimero_sdk::borsh")] 
struct ChatMessage {
    sender: String,
    message: String,
    sequence: u64,
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        env::log("Initializing mining pool with chat system.");
        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4, 
            is_mining_active: true, 
            total_hashrate: 0,
            chat_messages: Vector::new(),
            chat_sequence: 0,
        }
    }

    fn compute_sha256(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn join_mining(&mut self, miner_id: String, hashrate: u64) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining is not active."));
        }
        self.worker_stats.insert(miner_id.clone(), (0, 0, hashrate))?;
        self.active_workers.insert(miner_id.clone(), true)?;
        self.total_hashrate += hashrate;
        env::log(&format!("Miner {} joined the mining pool with hashrate {}.", miner_id, hashrate));
        Ok(())
    }

    pub fn execute_mining(&mut self, miner_id: String) -> Result<Option<String>, Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining is not active."));
        }
        if !self.active_workers.get(&miner_id)?.unwrap_or(false) {
            return Err(Error::msg("Miner is not active."));
        }
        let block_data = self.current_block.clone().ok_or_else(|| Error::msg("No active block."))?;
        let (hash_count, reward, hashrate) = self.worker_stats.get(&miner_id)?.unwrap_or((0, 0, 0));
        let nonce_range = 1_000_000 * hashrate / self.total_hashrate;
        let start_nonce = 0;
        let end_nonce = start_nonce + nonce_range;
        env::log(&format!("Miner {} mining from {} to {}", miner_id, start_nonce, end_nonce));

        let mut nonce = start_nonce;
        loop {
            if !self.is_mining_active {
                env::log(&format!("Mining stopped, miner {} exiting.", miner_id));
                break;
            }
            let input = format!("{}{}", block_data, nonce);
            let hash = self.compute_sha256(&input);
            if nonce % 100_000 == 0 {
                env::log(&format!("Miner {} is working... Nonce: {}", miner_id, nonce));
            }
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.mined_blocks.insert(block_id.clone(), block_data.clone())?;
                self.is_mining_active = false;
                let miner_reward = (hash_count * hashrate) / self.total_hashrate;
                self.worker_stats.insert(miner_id.clone(), (hash_count, miner_reward, hashrate))?;
                env::log(&format!(
                    "EVENT: block_mined, miner: {}, block: {}, reward: {}",
                    miner_id, block_id, miner_reward
                ));
                return Ok(Some(block_id));
            }
            nonce += 1;
        }
        self.worker_stats.insert(miner_id.clone(), (hash_count, reward, hashrate))?;
        Ok(None)
    }

    pub fn stop_mining(&mut self) -> Result<(), Error> {
        env::log("Stopping mining process...");
        self.is_mining_active = false;
        Ok(())
    }

    pub fn get_all_mined_blocks(&self) -> Result<Vec<String>, Error> {
        env::log("Fetching all mined blocks...");
        let mut blocks = Vec::new();
        for (block_id, _) in self.mined_blocks.entries()? {
            blocks.push(block_id);
        }
        Ok(blocks)
    }
}
 */


/* use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;
use sha2::{Sha256, Digest};

// Custom `now` function
pub fn now() -> f64 {
    // Replace this with a custom implementation if needed
    // For now, we'll return a fixed timestamp (e.g., 0.0)
    0.0
}

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningPool {
    current_block: Option<String>, // The block being mined
    mined_blocks: UnorderedMap<String, String>, // Completed blocks
    worker_stats: UnorderedMap<String, (u64, u64, u64)>, // (Start time, hash count, reward)
    active_workers: UnorderedMap<String, bool>, // Track active miners
    difficulty: u32, // Mining difficulty
    is_mining_active: bool, // Is mining currently active?
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        env::log("Initializing mining pool in Calimero context.");
        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4, // Adjust difficulty here
            is_mining_active: false,
        }
    }

    /// **Generate SHA-256 hash**
    fn compute_sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// **Start mining a new block**
    pub fn start_mining(&mut self, block_data: String) -> Result<(), Error> {
        if self.is_mining_active {
            return Err(Error::msg("Mining already in progress."));
        }

        env::log(&format!("Starting mining for block: {}", block_data));
        self.current_block = Some(block_data.clone());
        self.is_mining_active = true;
        self.worker_stats.clear()?;
        self.active_workers.clear()?;

        Ok(())
    }

    /// **Worker joins mining**
    pub fn join_mining(&mut self, miner_id: String) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("No active mining session."));
        }

        let start_time = (now() / 1000.0) as u64; // Use custom `now` function
        self.worker_stats.insert(miner_id.clone(), (start_time, 0, 0))?;
        self.active_workers.insert(miner_id.clone(), true)?;

        env::log(&format!("Miner {} joined the mining pool.", miner_id));

        Ok(())
    }

    /// **Actual Mining Execution (Each Worker Mines Separately)**
    pub fn execute_mining(&mut self, miner_id: String) -> Result<Option<String>, Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining is not active."));
        }

        if self.active_workers.get(&miner_id)?.unwrap_or(false) == false {
            return Err(Error::msg("Miner is not active."));
        }

        let block_data = self.current_block.clone().ok_or(Error::msg("No active block."))?;
        let (start_time, mut hash_count, reward) = self.worker_stats.get(&miner_id)?.unwrap_or((0, 0, 0));

        let mut nonce = 0;
        let worker_count = self.active_workers.entries()?.count() as u64;
        let nonce_range = 1_000_000 / worker_count; // Dynamic nonce assignment
        let start_nonce = nonce * nonce_range;
        let end_nonce = start_nonce + nonce_range;

        env::log(&format!("Miner {} mining from {} to {}", miner_id, start_nonce, end_nonce));

        loop {
            // Check if another miner has already found a block
            if !self.is_mining_active {
                env::log(&format!("Mining stopped, miner {} exiting.", miner_id));
                break;
            }

            let input = format!("{}{}", block_data, nonce);
            let hash = Self::compute_sha256(&input);
            hash_count += 1;

            if nonce % 100_000 == 0 {
                env::log(&format!("Miner {} is working... Nonce: {}", miner_id, nonce));
            }

            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.mined_blocks.insert(block_id.clone(), block_data.clone())?;
                self.is_mining_active = false; // ❌ Stop all mining

                // Calculate reward
                let total_time = (now() / 1000.0) as u64 - start_time; // Use instant::now() for timestamps
                let miner_reward = (total_time as f64 * 0.5 + hash_count as f64 * 0.5) as u64;
                self.worker_stats.insert(miner_id.clone(), (start_time, hash_count, miner_reward))?;

                env::log(&format!("EVENT: block_mined, miner: {}, block: {}, reward: {}", miner_id, block_id, miner_reward));
                return Ok(Some(block_id));
            }

            nonce += 1;
        }

        self.worker_stats.insert(miner_id.clone(), (start_time, hash_count, reward))?;
        Ok(None)
    }

    /// **Stop mining**
    pub fn stop_mining(&mut self) -> Result<(), Error> {
        env::log("Stopping mining process...");
        self.is_mining_active = false;
        Ok(())
    }

    /// **Retrieve all mined blocks**
    pub fn get_all_mined_blocks(&self) -> Result<Vec<String>, Error> {
        env::log("Fetching all mined blocks...");
        let mut blocks = Vec::new();
        for (block_id, _) in self.mined_blocks.entries()? {
            blocks.push(block_id);
        }
        Ok(blocks)
    }

    /// **Retrieve miner rewards**
    pub fn get_miner_rewards(&self) -> Result<Vec<(String, u64)>, Error> {
        env::log("Fetching all miner rewards...");
        let mut rewards = Vec::new();
        for (miner_id, (_, _, reward)) in self.worker_stats.entries()? {
            rewards.push((miner_id, reward));
        }
        Ok(rewards)
    }
} */