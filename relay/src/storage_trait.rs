use async_trait::async_trait;
use protobuf_core::types::allocation::Allocation;

#[async_trait]
pub trait RelayStore {
    // TODO - we need to store the following types of data
    // 1. Auctioneer<>Relay data. This involves
    //      a. Auctioneer sends a signed allocation
    // 2. Relay<>Sequencer data. This involves
    //      a. Sequencer queries for a signed allocation for the given block
    //      b. Sequencer sends back a signed allocation
    // 3. Relay<>Sequencer N/w data. This involves
    //      a. Validators query allocation for a given block to validate whether the correct allocation was included in a block.
    async fn store_delivered_allocation(&self, allocation: Allocation, block_height: u64) -> eyre::Result<()>;

    // TODO: define Relay<>Sequencer methods
}