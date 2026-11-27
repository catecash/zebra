//! Candidate consensus identity for Catecash mainnet.
//!
//! These values require public testnet evidence and independent review before launch.

use std::{collections::BTreeMap, sync::Arc};

use crate::{
    block::{self, Height},
    parameters::{checkpoint::list::CheckpointList, NetworkUpgrade},
    work::difficulty::{ExpandedDifficulty, U256},
};

use super::magic::Magic;

/// Candidate Catecash mainnet P2P magic.
pub const MAGIC: Magic = Magic([0x55, 0x0c, 0x6b, 0xa9]);
/// Candidate Catecash mainnet P2P port.
pub const P2P_PORT: u16 = 34333;
/// Candidate Catecash mainnet RPC port.
pub const RPC_PORT: u16 = 34332;
/// Candidate genesis hash, mined from the frozen timestamp and provisional target.
pub const GENESIS_HASH: &str = "050bf7476db81684aeb38337fc931bd8bc4480fde43fb0a14fef172f85c5c323";
/// Candidate proof-of-work limit, approximately eight times harder than devnet's.
pub const POW_LIMIT: &str = "0fffff0000000000000000000000000000000000000000000000000000000000";

/// Returns the candidate genesis hash.
pub fn genesis_hash() -> block::Hash {
    GENESIS_HASH
        .parse()
        .expect("fixed Catecash mainnet candidate genesis hash parses")
}

/// Returns checkpoints for the settled candidate mainnet chain.
pub fn checkpoint_list() -> Arc<CheckpointList> {
    Arc::new(
        include_str!("catecash-mainnet-checkpoints.txt")
            .parse()
            .expect("fixed Catecash mainnet candidate checkpoint parses"),
    )
}

/// Returns the proposed activation schedule, with NU7 rules from height one.
pub fn activation_heights() -> BTreeMap<Height, NetworkUpgrade> {
    [
        (Height(0), NetworkUpgrade::Genesis),
        (Height(1), NetworkUpgrade::Nu7),
    ]
    .into_iter()
    .collect()
}

/// Returns the candidate maximum proof-of-work target.
pub fn pow_limit() -> ExpandedDifficulty {
    ExpandedDifficulty::from(
        POW_LIMIT
            .parse::<U256>()
            .expect("fixed Catecash mainnet candidate proof-of-work limit parses"),
    )
}
