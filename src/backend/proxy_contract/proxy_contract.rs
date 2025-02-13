use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::env::ext::{AccountId, ProposalId};
use calimero_sdk::serde::{Deserialize, Serialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};

#[app::state(emits = Event)]
#[derive(Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct RandomProxyState {
    quantum_seeds: UnorderedMap<ProposalId, Vector<QuantumSeed>>,
    processed_values: UnorderedMap<ProposalId, Vector<ProcessedValue>>,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct QuantumSeed {
    id: String,
    proposal_id: String,
    seed_value: Vec<u8>,
    created_at: String,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct ProcessedValue {
    node: String,
    value: Vec<u8>,
    sequence: u64,
}

#[app::event]
pub enum Event {
    ProposalCreated { id: ProposalId },
    SeedProcessed { id: ProposalId, node: String },
    ApprovedProposal { id: ProposalId },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct CreateRandomRequest {
    pub action_type: String,
    pub params: serde_json::Value,
}

#[app::logic]
impl RandomProxyState {
    #[app::init]
    pub fn init() -> RandomProxyState {
        RandomProxyState {
            quantum_seeds: UnorderedMap::new(),
            processed_values: UnorderedMap::new(),
        }
    }

    pub fn create_new_proposal(
        &mut self,
        request: CreateRandomRequest,
    ) -> Result<ProposalId, Error> {
        env::log("Starting create_new_random_proposal");
        env::log(&format!("Request type: {}", request.action_type));

        let proposal_id = match request.action_type.as_str() {
            "SetQuantumSeed" => {
                env::log("Processing SetQuantumSeed");
                let seed_value = request.params["seed_value"]
                    .as_str()
                    .ok_or_else(|| Error::msg("seed_value is required"))?;

                Self::external()
                    .propose()
                    .external_function_call(
                        "random_chain".to_string(),
                        "set_seed".to_string(),
                        seed_value.to_string(),
                        0,
                    )
                    .send()
            }
            "ProcessValue" => {
                env::log("Processing ProcessValue");
                let node = request.params["node"]
                    .as_str()
                    .ok_or_else(|| Error::msg("node is required"))?;

                Self::external()
                    .propose()
                    .external_function_call(
                        "random_chain".to_string(),
                        "process_value".to_string(),
                        node.to_string(),
                        0,
                    )
                    .send()
            }
            _ => return Err(Error::msg("Invalid action type")),
        };

        env::emit(&Event::ProposalCreated { id: proposal_id });

        // Initialize vectors for the new proposal
        let old_seeds = self.quantum_seeds.insert(proposal_id, Vector::new())?;
        let old_values = self.processed_values.insert(proposal_id, Vector::new())?;
        
        if old_seeds.is_some() || old_values.is_some() {
            return Err(Error::msg("proposal already exists"));
        }

        Ok(proposal_id)
    }

    pub fn approve_proposal(&self, proposal_id: ProposalId) -> Result<(), Error> {
        Self::external().approve(proposal_id);
        env::emit(&Event::ApprovedProposal { id: proposal_id });
        Ok(())
    }

    pub fn get_quantum_seeds(&self, proposal_id: ProposalId) -> Result<Vec<QuantumSeed>, Error> {
        let Some(seeds) = self.quantum_seeds.get(&proposal_id)? else {
            return Ok(vec![]);
        };
        Ok(seeds.entries()?.collect())
    }

    pub fn get_processed_values(&self, proposal_id: ProposalId) -> Result<Vec<ProcessedValue>, Error> {
        let Some(values) = self.processed_values.get(&proposal_id)? else {
            return Ok(vec![]);
        };
        Ok(values.entries()?.collect())
    }

    pub fn store_quantum_seed(
        &mut self,
        proposal_id: ProposalId,
        seed: QuantumSeed,
    ) -> Result<(), Error> {
        let mut seeds = self.quantum_seeds.get(&proposal_id)?.unwrap_or_default();
        seeds.push(seed)?;
        self.quantum_seeds.insert(proposal_id, seeds)?;
        Ok(())
    }

    pub fn store_processed_value(
        &mut self,
        proposal_id: ProposalId,
        value: ProcessedValue,
    ) -> Result<(), Error> {
        let mut values = self.processed_values.get(&proposal_id)?.unwrap_or_default();
        values.push(value)?;
        self.processed_values.insert(proposal_id, values)?;
        env::emit(&Event::SeedProcessed { 
            id: proposal_id,
            node: value.node,
        });
        Ok(())
    }
} 