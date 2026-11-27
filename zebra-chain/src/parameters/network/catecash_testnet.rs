//! Fixed consensus identity for the Catecash testnet private rehearsal.

use std::{collections::BTreeMap, sync::Arc};

use crate::{
    block::{self, Height},
    parameters::{checkpoint::list::CheckpointList, NetworkUpgrade},
    work::difficulty::{ExpandedDifficulty, U256},
};

use super::magic::Magic;

/// Catecash testnet P2P magic.
pub const MAGIC: Magic = Magic([0xe7, 0x9c, 0x55, 0xe7]);
/// Catecash testnet P2P port.
pub const P2P_PORT: u16 = 34353;
/// Catecash testnet RPC port.
pub const RPC_PORT: u16 = 34352;
/// Mined, zero-premine genesis hash.
pub const GENESIS_HASH: &str = "15999c77e978a180c2f3fe2ec8478f8123e5c923f20bedaaf1b156fc38c4dda2";
/// Easiest permitted testnet proof-of-work target.
pub const POW_LIMIT: &str = "7fffff0000000000000000000000000000000000000000000000000000000000";

/// Returns the fixed genesis hash.
pub fn genesis_hash() -> block::Hash {
    GENESIS_HASH
        .parse()
        .expect("fixed Catecash testnet genesis hash parses")
}

/// Returns checkpoints for the settled testnet chain.
pub fn checkpoint_list() -> Arc<CheckpointList> {
    Arc::new(
        include_str!("catecash-testnet-checkpoints.txt")
            .parse()
            .expect("fixed Catecash testnet checkpoints parse"),
    )
}

/// NU7 validates every mineable block.
pub fn activation_heights() -> BTreeMap<Height, NetworkUpgrade> {
    [
        (Height(0), NetworkUpgrade::Genesis),
        (Height(1), NetworkUpgrade::Nu7),
    ]
    .into_iter()
    .collect()
}

/// Returns the maximum proof-of-work target.
pub fn pow_limit() -> ExpandedDifficulty {
    ExpandedDifficulty::from(
        POW_LIMIT
            .parse::<U256>()
            .expect("fixed Catecash testnet proof-of-work limit parses"),
    )
}
