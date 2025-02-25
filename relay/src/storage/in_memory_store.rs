use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::{Arc};
use protobuf_core::types::allocation::Allocation;
use crate::storage_trait::RelayStore;

struct InMemoryStore {
    // TODO - this should actually be per auctioneer address
    delivered_allocation_store: Arc<Mutex<HashMap<u64, Allocation>>>
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            delivered_allocation_store: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn store_delivered_allocation(&self, allocation: Allocation, block_height: u64) -> eyre::Result<()> {
        // TODO - we store only the latest allocation for a given block height.
        let mut store = self.delivered_allocation_store.lock().await;
        store.insert(block_height, allocation);
        Ok(())
    }
}

impl RelayStore for InMemoryStore {
    async fn store_delivered_allocation(&self, allocation: Allocation, block_height: u64) -> eyre::Result<()> {
        self.store_delivered_allocation(allocation, block_height).await
    }
}