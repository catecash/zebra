//! Extra per-block info tracked in the state.
use crate::value_balance::network::NetworkChainValuePools;

/// Extra per-block info tracked in the state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BlockInfo {
    /// The pool balances after the block.
    value_pools: NetworkChainValuePools,
    /// The size of the block in bytes.
    size: u32,
}

impl BlockInfo {
    /// Creates a new [`BlockInfo`] with the given value pools.
    pub fn new(value_pools: impl Into<NetworkChainValuePools>, size: u32) -> Self {
        BlockInfo {
            value_pools: value_pools.into(),
            size,
        }
    }

    /// Returns the value pools of this block.
    pub fn value_pools(&self) -> &NetworkChainValuePools {
        &self.value_pools
    }

    /// Returns the size of this block.
    pub fn size(&self) -> u32 {
        self.size
    }
}
