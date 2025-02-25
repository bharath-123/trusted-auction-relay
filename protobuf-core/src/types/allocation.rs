use crate::generated::astria::auction::v1alpha1 as raw;
use crate::types::bid::Bid;
use crate::types::crypto::{Signature, VerificationKey};
use crate::types::sequencer_key::SequencerKey;
use bytes::Bytes;
use prost::{Message, Name};

#[derive(Debug, Clone)]
pub struct Allocation {
    signature: Signature,
    verification_key: VerificationKey,
    bid_bytes: pbjson_types::Any,
}

impl Allocation {
    pub fn new(bid: Bid, sequencer_key: &SequencerKey) -> Self {
        let bid_bytes = pbjson_types::Any {
            type_url: raw::Bid::type_url(),
            value: bid.into_raw().encode_to_vec().into(),
        };
        let signature = sequencer_key.signing_key().sign(&bid_bytes.value);
        let verification_key = sequencer_key.signing_key().verification_key();
        Self {
            signature,
            verification_key,
            bid_bytes,
        }
    }

    pub fn into_raw(self) -> raw::Allocation {
        let Self {
            signature,
            verification_key,
            bid_bytes,
        } = self;

        raw::Allocation {
            signature: Bytes::copy_from_slice(&signature.to_bytes()),
            public_key: Bytes::copy_from_slice(&verification_key.to_bytes()),
            bid: Some(bid_bytes),
        }
    }
}
