use crate::types::allocation::Allocation;
use crate::generated::trusted_relay::auctioneer_apis::v1 as raw;

#[derive(Debug, Clone)]
pub struct AllocationDeliveryRequest {
    allocation: Allocation,
    block_height: u64
}

impl AllocationDeliveryRequest {
    pub fn new(allocation: Allocation, block_height: u64) -> Self {
        Self {
            allocation,
            block_height
        }
    }

    pub fn allocation(&self) -> &Allocation {
        &self.allocation
    }

    pub fn block_height(&self) -> u64 {
        self.block_height
    }

    pub fn into_raw(self) -> raw::DeliverAllocationRequest {
        raw::DeliverAllocationRequest {
            allocation: Some(self.allocation.into_raw()),
            block_height: self.block_height
        }
    }
}
