use std::collections::BTreeMap;
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct KvStore {
    entries: UnorderedMap<String, Vec<String>>, // ✅ Store multiple values per key
}

#[app::logic]
impl KvStore {
    #[app::init]
    pub fn init() -> KvStore {
        env::log("Initializing KvStore application in Calimero context.");
        KvStore {
            entries: UnorderedMap::new(), // ✅ Fixed storage initialization
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        env::log(&format!("Setting key: {:?} to value: {:?}", key, value));

        // ✅ Corrected unwrap logic
        let mut values = self.entries.get(&key).unwrap_or_else(|_| Some(Vec::new())).unwrap_or(Vec::new());
        values.push(value.clone()); // Append value

        self.entries.insert(key.clone(), values)?; // Store updated list

        env::log(&format!("EVENT: kv_store:set, key: {}, value: {}", key, value));

        Ok(())
    }

    pub fn entries(&self) -> Result<BTreeMap<String, Vec<String>>, Error> { // ✅ Fixed return type
        env::log("Fetching all stored key-value pairs.");
        
        let mut result = BTreeMap::new();
        for (key, value) in self.entries.entries()? { // ✅ Corrected iteration
            result.insert(key, value);
        }

        Ok(result)
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<String>>, Error> { // ✅ Returns all stored values
        env::log(&format!("Fetching key: {:?}", key));

        match self.entries.get(key) {
            Ok(Some(values)) => {
                env::log(&format!("Found key: {} -> {:?}", key, values));
                Ok(Some(values)) // ✅ Return all stored values for this key
            }
            Ok(None) => {
                env::log(&format!("Key '{}' not found.", key));
                Ok(None)
            }
            Err(err) => {
                env::log(&format!("Error retrieving key: {}", err));
                Err(Error::from(err))
            }
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<Option<Vec<String>>, Error> { // ✅ Return Vec<String>
        env::log(&format!("Removing key: {:?}", key));

        let removed_value = self.entries.remove(key)?;
        if removed_value.is_some() {
            env::log(&format!("EVENT: kv_store:remove, key: {}", key));
        }

        Ok(removed_value)
    }
}




/*
use std::collections::BTreeMap;
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct KvStore {
    entries: UnorderedMap<String, String>,
}

#[app::logic]
impl KvStore {
    #[app::init]
    pub fn init() -> KvStore {
        env::log("Initializing KvStore application in Calimero context.");
        KvStore {
            entries: UnorderedMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        env::log(&format!("Setting key: {:?} to value: {:?}", key, value));

        self.entries.insert(key.clone(), value.clone())?;

        // **Log an event so other nodes in the context are notified**
        env::log(&format!("EVENT: kv_store:set, key: {}, value: {}", key, value));

        Ok(())
    }

    pub fn entries(&self) -> Result<BTreeMap<String, String>, Error> {
        env::log("Fetching all stored key-value pairs.");
        Ok(self.entries.entries()?.collect())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Error> {
        env::log(&format!("Fetching key: {:?}", key));
    
        // Handle the Result<Option<String>, StoreError>
        match self.entries.get(key) {
            Ok(Some(value)) => {
                env::log(&format!("Found key: {} -> {}", key, value));
                Ok(Some(value))
            }
            Ok(None) => {
                env::log(&format!("Key '{}' not found.", key));
                Ok(None)
            }
            Err(err) => {
                env::log(&format!("Error retrieving key: {}", err));
                Err(Error::from(err)) // Convert StoreError into a generic Error
            }
        }
    }
    

    pub fn remove(&mut self, key: &str) -> Result<Option<String>, Error> {
        env::log(&format!("Removing key: {:?}", key));

        let removed_value = self.entries.remove(key)?;
        if removed_value.is_some() {
            // **Log an event so other nodes know the key was deleted**
            env::log(&format!("EVENT: kv_store:remove, key: {}", key));
        }

        Ok(removed_value)
    }
}
*/