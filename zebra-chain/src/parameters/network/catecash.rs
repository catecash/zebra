//! Fixed consensus identity for the local Catecash development network.

use std::{collections::BTreeMap, sync::Arc};

use crate::{
    amount::COIN,
    block::{self, Height},
    parameters::{checkpoint::list::CheckpointList, NetworkUpgrade},
    work::difficulty::{ExpandedDifficulty, U256},
};

use super::magic::Magic;

/// Catecash devnet P2P message magic.
pub const MAGIC: Magic = Magic([111, 131, 5, 188]);
/// Catecash devnet P2P TCP port.
pub const P2P_PORT: u16 = 34343;
/// Number of blocks before a development coinbase output can be spent.
pub const COINBASE_MATURITY: u32 = 240;
/// Fixed reward for every mineable development block, in base units.
pub const BLOCK_REWARD_ZAT: i64 = 5_000 * COIN;
/// Hash of the fixed, mined development genesis block.
pub const GENESIS_HASH: &str = "6b9c4d783359202111571c123309b2c0511d1bf9036f74dee4921f04e1f0a9cf";
/// Easiest permitted development proof-of-work target.
pub const POW_LIMIT: &str = "7fffff0000000000000000000000000000000000000000000000000000000000";

/// Returns the fixed development genesis hash.
pub fn genesis_hash() -> block::Hash {
    GENESIS_HASH
        .parse()
        .expect("fixed Catecash genesis hash parses")
}

/// Returns checkpoints for the settled development chain.
pub fn checkpoint_list() -> Arc<CheckpointList> {
    Arc::new(
        include_str!("catecash-devnet-checkpoints.txt")
            .parse()
            .expect("fixed Catecash devnet checkpoints parse"),
    )
}

/// Returns the launch validation schedule for the development network.
pub fn activation_heights() -> BTreeMap<Height, NetworkUpgrade> {
    [
        (Height(0), NetworkUpgrade::Genesis),
        (Height(1), NetworkUpgrade::Nu7),
    ]
    .into_iter()
    .collect()
}

/// Returns the maximum proof-of-work target for the development network.
pub fn pow_limit() -> ExpandedDifficulty {
    ExpandedDifficulty::from(
        POW_LIMIT
            .parse::<U256>()
            .expect("fixed Catecash proof-of-work limit parses"),
    )
}
