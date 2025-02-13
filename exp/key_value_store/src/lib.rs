use std::collections::BTreeMap;

use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;

/*
    #[app::state] macro marks the struct as the application state,
 permitting its use by Calimero SDK

    #[app::logic] macro marks the implementation block as the application logic,
 allowing to define the methods that interact with the application state.

    #[app::init] macro is called when the application
 is executed against a freshly created context.
*/

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct KvStore {
    entries: UnorderedMap<String, String>,
}

#[app::logic]
impl KvStore {
    #[app::init]
    pub fn init() -> Self {
        // Changed back to init() as required by the macro
        Self {
            entries: UnorderedMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        env::log(&format!("Setting key: {} to value: {}", key, value));
        self.entries.insert(key, value)?;
        Ok(())
    }

    pub fn entries(&self) -> Result<BTreeMap<String, String>, Error> {
        env::log("Getting all entries");
        let entries: BTreeMap<String, String> = self.entries.entries()?.collect();
        Ok(entries)
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Error> {
        env::log(&format!("Getting key: {}", key));
        self.entries.get(key).map_err(Into::into)
    }

    pub fn remove(&mut self, key: &str) -> Result<Option<String>, Error> {
        env::log(&format!("Removing key: {}", key));
        self.entries.remove(key).map_err(Into::into)
    }
}
