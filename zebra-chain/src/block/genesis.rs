//! Fixed genesis blocks for locally bootstrapped networks.

use std::sync::Arc;

use hex::FromHex;

use crate::{block::Block, serialization::ZcashDeserializeInto};

/// Genesis block for Regtest, copied from zcashd via `getblock 0 0` RPC method
pub fn regtest_genesis_block() -> Arc<Block> {
    let regtest_genesis_block_bytes =
        <Vec<u8>>::from_hex(include_str!("genesis/block-regtest-0-000-000.txt").trim())
            .expect("Block bytes are in valid hex representation");

    regtest_genesis_block_bytes
        .zcash_deserialize_into()
        .map(Arc::new)
        .expect("hard-coded Regtest genesis block data must deserialize successfully")
}

/// Catecash development genesis with a zero-value OP_RETURN coinbase output.
pub fn catecash_devnet_genesis_block() -> Arc<Block> {
    let bytes = <Vec<u8>>::from_hex(include_str!("genesis/block-catecash-devnet-0.txt").trim())
        .expect("Catecash genesis bytes are valid hex");
    bytes
        .zcash_deserialize_into()
        .map(Arc::new)
        .expect("Catecash genesis block deserializes")
}

/// Candidate Catecash mainnet genesis with a zero-value OP_RETURN coinbase.
pub fn catecash_mainnet_genesis_block() -> Arc<Block> {
    let bytes =
        <Vec<u8>>::from_hex(include_str!("genesis/block-catecash-mainnet-candidate-0.txt").trim())
            .expect("Catecash mainnet candidate genesis bytes are valid hex");
    bytes
        .zcash_deserialize_into()
        .map(Arc::new)
        .expect("Catecash mainnet candidate genesis block deserializes")
}

/// Catecash testnet genesis with a zero-value OP_RETURN coinbase output.
pub fn catecash_testnet_genesis_block() -> Arc<Block> {
    let bytes = <Vec<u8>>::from_hex(include_str!("genesis/block-catecash-testnet-0.txt").trim())
        .expect("Catecash testnet genesis bytes are valid hex");
    bytes
        .zcash_deserialize_into()
        .map(Arc::new)
        .expect("Catecash testnet genesis block deserializes")
}

#[cfg(test)]
mod tests {
    use hex::FromHex;

    use crate::{
        block::{Hash, Height},
        serialization::ZcashSerialize,
        transparent::CATECASH_GENESIS_COINBASE_SCRIPT_SIG,
    };

    use super::{
        catecash_devnet_genesis_block, catecash_mainnet_genesis_block,
        catecash_testnet_genesis_block,
    };

    #[test]
    fn catecash_testnet_genesis_has_valid_proof_and_no_premine() {
        let block = catecash_testnet_genesis_block();
        assert_eq!(block.coinbase_height(), Some(Height::MIN));
        assert_eq!(block.header.previous_block_hash, Hash([0; 32]));
        assert_eq!(
            block.hash().to_string(),
            crate::parameters::catecash_testnet::GENESIS_HASH
        );
        assert_eq!(block.header.time.timestamp(), 1_795_473_000);
        assert_eq!(block.header.difficulty_threshold.to_string(), "207fffff");
        assert_ne!(block.hash(), catecash_devnet_genesis_block().hash());
        assert_ne!(block.hash(), catecash_mainnet_genesis_block().hash());
        assert!(block.header.solution.check(&block.header).is_ok());
        assert!(
            block.hash()
                <= block
                    .header
                    .difficulty_threshold
                    .to_expanded()
                    .expect("genesis target is valid")
        );
        assert_eq!(
            block.header.merkle_root,
            block.transactions.iter().map(|tx| tx.hash()).collect()
        );
        assert_eq!(
            block.transactions[0].inputs()[0].coinbase_script(),
            Some(CATECASH_GENESIS_COINBASE_SCRIPT_SIG.to_vec())
        );
        assert!(block.transactions[0]
            .outputs()
            .iter()
            .all(|output| output.value.is_zero()));
        assert_eq!(
            hex::encode(block.zcash_serialize_to_vec().expect("genesis serializes")),
            include_str!("genesis/block-catecash-testnet-0.txt").trim()
        );
    }

    #[test]
    fn catecash_devnet_genesis_has_valid_proof_and_no_premine() {
        let block = catecash_devnet_genesis_block();
        assert_eq!(block.coinbase_height(), Some(Height::MIN));
        assert_eq!(block.header.previous_block_hash, Hash([0; 32]));
        assert_eq!(
            block.hash().to_string(),
            "6b9c4d783359202111571c123309b2c0511d1bf9036f74dee4921f04e1f0a9cf"
        );
        assert!(block.header.solution.check(&block.header).is_ok());
        assert!(
            block.hash()
                <= block
                    .header
                    .difficulty_threshold
                    .to_expanded()
                    .expect("genesis target is valid")
        );
        assert_eq!(
            block.header.merkle_root,
            block.transactions.iter().map(|tx| tx.hash()).collect()
        );
        let coinbase = &block.transactions[0];
        assert_eq!(
            coinbase.inputs()[0].coinbase_script(),
            Some(CATECASH_GENESIS_COINBASE_SCRIPT_SIG.to_vec())
        );
        assert!(coinbase
            .outputs()
            .iter()
            .all(|output| output.value.is_zero()));

        let bytes = block.zcash_serialize_to_vec().expect("genesis serializes");
        assert_eq!(
            hex::encode(bytes),
            include_str!("genesis/block-catecash-devnet-0.txt").trim()
        );
    }

    #[test]
    fn catecash_mainnet_candidate_genesis_has_valid_proof_and_no_premine() {
        let hex = include_str!("genesis/block-catecash-mainnet-candidate-0.txt").trim();
        let bytes = Vec::from_hex(hex).expect("candidate genesis bytes are valid hex");
        let block = catecash_mainnet_genesis_block();

        assert_eq!(block.coinbase_height(), Some(Height::MIN));
        assert_eq!(block.header.previous_block_hash, Hash([0; 32]));
        assert_eq!(
            block.hash().to_string(),
            "050bf7476db81684aeb38337fc931bd8bc4480fde43fb0a14fef172f85c5c323"
        );
        assert_eq!(block.header.time.timestamp(), 1_795_408_991);
        assert_eq!(block.header.difficulty_threshold.to_string(), "200fffff");
        assert_ne!(block.hash(), catecash_devnet_genesis_block().hash());
        assert!(block.header.solution.check(&block.header).is_ok());
        assert!(
            block.hash()
                <= block
                    .header
                    .difficulty_threshold
                    .to_expanded()
                    .expect("candidate genesis target is valid")
        );
        assert_eq!(
            block.header.merkle_root,
            block.transactions.iter().map(|tx| tx.hash()).collect()
        );
        assert_eq!(
            block.transactions[0].inputs()[0].coinbase_script(),
            Some(CATECASH_GENESIS_COINBASE_SCRIPT_SIG.to_vec())
        );
        assert!(block.transactions[0]
            .outputs()
            .iter()
            .all(|output| output.value.is_zero()));
        assert_eq!(block.zcash_serialize_to_vec().unwrap(), bytes);
    }
}
