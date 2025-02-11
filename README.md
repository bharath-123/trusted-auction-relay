# Trusted Auction Relay Network

This is a simple implementation of a trusted auction relay network. 

The trusted relay network is a network of grpc servers to which auctioneer submit their allocations to. The proposing validator for a given block will query the relay for the
allocations for the given block. The relay does a 2-phase commit scheme with the validator, where the validator signs a hash of the auctioneer allocations 
after which the auctioneer returns the allocations to the validator. The validator then is expected to include the allocation in the block.

Other validator validating the proposed block can query the relay for the allocation to which the proposing validator has commited to and if the 
allocation has not been added to the block, the block is rejected in the process_proposal phase.

Relays within the network are expected to gossip the allocations to each other. The relays can trust each other to not lie about the allocations they have received from the auctioneers.

### TODO

- [] Implement the auctioneer apis 
- [] Implement the sequencer validator apis
- [] Implement p2p gossip between the relays 