//! Reproduce Catecash development genesis or mine public-network genesis candidates.
//!
//! Run with `cargo run -p zebra-chain --example catecash_genesis --features internal-miner -- [devnet|mainnet-candidate|testnet]`.

use std::{error::Error, sync::Arc};

use chrono::DateTime;
use hex::FromHex;
use zebra_chain::{
    block::{Block, Hash},
    serialization::{ZcashDeserializeInto, ZcashSerialize},
    transaction::Transaction,
    transparent::CATECASH_GENESIS_COINBASE_SCRIPT_SIG,
    work::{difficulty::CompactDifficulty, equihash::Solution},
};

const DEVNET_GENESIS_TIME: i64 = 1_795_395_600; // 2026-11-23 01:00:00 UTC
const MAINNET_CANDIDATE_TIME: i64 = 1_795_408_991; // 2026-11-23 04:43:11 UTC
const TESTNET_GENESIS_TIME: i64 = 1_795_473_000; // 2026-11-23 22:30:00 UTC
const DEVNET_GENESIS_BITS: u32 = 0x207f_ffff;
const MAINNET_CANDIDATE_BITS: u32 = 0x200f_ffff;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let network = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "devnet".to_string());
    let (genesis_time, genesis_bits) = match network.as_str() {
        "devnet" => (DEVNET_GENESIS_TIME, DEVNET_GENESIS_BITS),
        "mainnet-candidate" => (MAINNET_CANDIDATE_TIME, MAINNET_CANDIDATE_BITS),
        "testnet" => (TESTNET_GENESIS_TIME, DEVNET_GENESIS_BITS),
        _ => return Err("expected devnet, mainnet-candidate, or testnet".into()),
    };
    let mut transaction_bytes = Vec::new();
    transaction_bytes.extend_from_slice(&1u32.to_le_bytes());
    transaction_bytes.push(1);
    transaction_bytes.extend_from_slice(&[0; 32]);
    transaction_bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    transaction_bytes.push(
        u8::try_from(CATECASH_GENESIS_COINBASE_SCRIPT_SIG.len())
            .expect("genesis script fits one-byte length"),
    );
    transaction_bytes.extend_from_slice(&CATECASH_GENESIS_COINBASE_SCRIPT_SIG);
    transaction_bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    transaction_bytes.push(1);
    transaction_bytes.extend_from_slice(&0u64.to_le_bytes());
    transaction_bytes.extend_from_slice(&[1, 0x6a]); // OP_RETURN, zero value.
    transaction_bytes.extend_from_slice(&0u32.to_le_bytes());

    let transaction: Transaction = transaction_bytes.as_slice().zcash_deserialize_into()?;
    let template_bytes =
        Vec::from_hex(include_str!("../src/block/genesis/block-regtest-0-000-000.txt").trim())?;
    let mut block: Block = template_bytes.as_slice().zcash_deserialize_into()?;
    block.transactions = vec![Arc::new(transaction)];

    let header = Arc::make_mut(&mut block.header);
    header.version = 4;
    header.previous_block_hash = Hash([0; 32]);
    header.merkle_root = block.transactions.iter().map(|tx| tx.hash()).collect();
    header.commitment_bytes = [0; 32].into();
    header.time = DateTime::from_timestamp(genesis_time, 0).expect("valid genesis time");
    header.difficulty_threshold = CompactDifficulty::from_hex(format!("{genesis_bits:08x}"))?;
    header.nonce = [0; 32].into();
    header.solution = Solution::for_proposal();

    let mut solved = Solution::solve(*header, || Ok(()))?.into_iter();
    block.header = Arc::new(solved.next().expect("solver returns a header"));
    block.header.solution.check(&block.header)?;

    println!("network={network}");
    println!("hash={}", block.hash());
    println!("time={genesis_time}");
    println!("bits={genesis_bits:08x}");
    println!("block={}", hex::encode(block.zcash_serialize_to_vec()?));
    Ok(())
}
