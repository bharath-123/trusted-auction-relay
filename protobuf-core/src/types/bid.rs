use crate::generated::astria::auction::v1alpha1 as raw;
use eyre::WrapErr as _;
use prost::bytes::Bytes;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RollupBlockHash(Bytes);

impl RollupBlockHash {
    #[must_use]
    pub(crate) fn new(inner: Bytes) -> Self {
        Self(inner)
    }

    #[must_use]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Bytes> for RollupBlockHash {
    fn from(value: Bytes) -> Self {
        Self::new(value)
    }
}

impl Display for RollupBlockHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::{display::Base64Display, engine::general_purpose::STANDARD};

        if f.alternate() {
            Base64Display::new(&self.0, &STANDARD).fmt(f)?;
        } else {
            for byte in &self.0 {
                write!(f, "{byte:02x}")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    #[must_use]
    pub const fn new(inner: [u8; 32]) -> Self {
        Self(inner)
    }

    #[must_use]
    pub const fn get(self) -> [u8; 32] {
        self.0
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
#[error("block hash requires 32 bytes, but slice contained `{actual}`")]
pub struct HashFromSliceError {
    actual: usize,
    source: std::array::TryFromSliceError,
}

impl<'a> TryFrom<&'a [u8]> for Hash {
    type Error = HashFromSliceError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let inner = value.try_into().map_err(|source| Self::Error {
            actual: value.len(),
            source,
        })?;
        Ok(Self(inner))
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base64::{display::Base64Display, engine::general_purpose::STANDARD};

        if f.alternate() {
            Base64Display::new(&self.0, &STANDARD).fmt(f)?;
        } else {
            for byte in self.0 {
                write!(f, "{byte:02x}")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Bid {
    /// The fee that will be charged for this bid.
    fee: u64,
    /// The byte list of transactions fto be included.
    transactions: Vec<Bytes>,
    /// The hash of the rollup block that this bid is based on.
    rollup_parent_block_hash: RollupBlockHash,
    /// The hash of the sequencer block used to derive the rollup block that this bid is based
    /// on.
    sequencer_parent_block_hash: Hash,
}

impl Bid {
    pub(crate) fn try_from_raw(raw: raw::Bid) -> eyre::Result<Self> {
        let raw::Bid {
            fee,
            transactions,
            sequencer_parent_block_hash,
            rollup_parent_block_hash,
        } = raw;
        Ok(Self {
            fee,
            transactions,
            rollup_parent_block_hash: rollup_parent_block_hash.into(),
            sequencer_parent_block_hash: sequencer_parent_block_hash
                .as_ref()
                .try_into()
                .wrap_err("invalid field .sequencer_parent_block_hash")?,
        })
    }

    pub fn into_raw(self) -> raw::Bid {
        raw::Bid {
            fee: self.fee,
            transactions: self.transactions,
            sequencer_parent_block_hash: Bytes::copy_from_slice(
                self.sequencer_parent_block_hash.as_bytes(),
            ),
            rollup_parent_block_hash: Bytes::copy_from_slice(
                self.rollup_parent_block_hash.as_bytes(),
            ),
        }
    }

    pub(crate) fn bid(&self) -> u64 {
        self.fee
    }

    pub(crate) fn rollup_parent_block_hash(&self) -> &RollupBlockHash {
        &self.rollup_parent_block_hash
    }

    pub(crate) fn sequencer_parent_block_hash(&self) -> &Hash {
        &self.sequencer_parent_block_hash
    }
}
