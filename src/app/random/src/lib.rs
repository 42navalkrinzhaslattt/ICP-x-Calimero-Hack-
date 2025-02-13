use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::Vector;
use calimero_sdk::serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[app::event]
pub enum RandomEvent<'a> {
    ValueProcessed { 
        node: &'a str, 
        sequence: u64, 
        value_hash: &'a [u8] 
    },
}

#[app::state(emits = for<'a> RandomEvent<'a>)]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct RandomChain {
    processed_values: Vector<ProcessedValue>,
    processing_sequence: u64,
    current_seed: Option<Vec<u8>>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Serialize, Deserialize)]
struct ProcessedValue {
    node: String,
    value: Vec<u8>,
    sequence: u64,
}

#[app::logic]
impl RandomChain {
    #[app::init]
    pub fn init() -> RandomChain {
        env::log("Initializing random chain application");
        RandomChain {
            processed_values: Vector::new(),
            processing_sequence: 0,
            current_seed: None,
        }
    }

    /// Set initial quantum seed
    pub fn set_seed(&mut self, quantum_seed: Vec<u8>) -> Result<(), Error> {
        self.validate_input(&quantum_seed)?;
        self.current_seed = Some(quantum_seed);
        env::log("Quantum seed set");
        Ok(())
    }

    /// Process value for current node
    pub fn process_value(&mut self, node: String) -> Result<u64, Error> {
        let input = if self.processed_values.len()? == 0 {
            self.current_seed.clone().ok_or(Error::msg("No seed value set"))?
        } else {
            let last_index = self.processed_values.len()? - 1;
            self.processed_values.get(last_index)?
                .ok_or(Error::msg("No previous value found"))?
                .value
        };

        let processed = self.transform(input)?;
        let sequence = self.processing_sequence;

        let value = ProcessedValue {
            node: node.clone(),
            value: processed.clone(),
            sequence,
        };
        
        self.processed_values.push(value)?;
        self.processing_sequence += 1;

        app::emit!(RandomEvent::ValueProcessed {
            node: &node,
            sequence,
            value_hash: &processed,
        });

        env::log(&format!(
            "EVENT: value_processed,node:{},sequence:{}",
            node, sequence
        ));
    
        Ok(sequence)
    }

    /// Get all processed values
    pub fn get_all_values(&self) -> Result<Vec<ProcessedValue>, Error> {
        let mut result = Vec::new();
        let len = self.processed_values.len()?;

        for i in 0..len {
            if let Ok(Some(val)) = self.processed_values.get(i) {
                result.push(val);
            }
        }
        Ok(result)
    }

    pub fn get_last_value(&self) -> Result<Option<ProcessedValue>, Error> {
        let len = self.processed_values.len()?;
        if len == 0 {
            return Ok(None);
        }
        let last_value = self.processed_values.get(len - 1)
            .map_err(|e| Error::msg(format!("{:?}", e).as_str()))?; // Convert StoreError to Error
    
        Ok(last_value)
    }

    // ========== Helper Methods ==========
    
    fn validate_input(&self, input: &[u8]) -> Result<(), Error> {
        if input.is_empty() {
            return Err(Error::msg("Empty input"));
        }
        if input.len() > 1024 {
            return Err(Error::msg("Input too large"));
        }
        Ok(())
    }

    fn transform(&self, input: Vec<u8>) -> Result<Vec<u8>, Error> {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(&input)
            .map_err(|_| Error::msg("Invalid key length"))?;
        mac.update(&self.processing_sequence.to_be_bytes());
        Ok(mac.finalize().into_bytes().to_vec())
    }
}