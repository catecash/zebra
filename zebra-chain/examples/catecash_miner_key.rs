//! Generate a Catecash transparent miner key for a selected network.
//!
//! Usage: `cargo run -p zebra-chain --example catecash_miner_key -- --network testnet /private/path/miner-key.json`
//! The destination must not exist. Only the public payout address is printed.

use std::{env, error::Error, fs::OpenOptions, io::Write, path::PathBuf};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

use secp256k1::{PublicKey, SecretKey};
use zcash_transparent::address::TransparentAddress;
use zebra_chain::{parameters::NetworkKind, transparent::Address};

fn address_and_wif(secret: &SecretKey, network: NetworkKind) -> (String, String) {
    let public = PublicKey::from_secret_key(secret);
    let TransparentAddress::PublicKeyHash(hash) = TransparentAddress::from_pubkey(&public) else {
        unreachable!("a public key always derives a P2PKH address")
    };
    let address = Address::from_pub_key_hash(network, hash).to_string();

    let mut wif_bytes = Vec::with_capacity(34);
    let prefix = match network {
        NetworkKind::CatecashDevnet => {
            zcash_protocol::constants::catecash_devnet::B58_SECRET_KEY_PREFIX
        }
        NetworkKind::CatecashTestnet => {
            zcash_protocol::constants::catecash_testnet::B58_SECRET_KEY_PREFIX
        }
        NetworkKind::CatecashMainnet => {
            zcash_protocol::constants::catecash_mainnet::B58_SECRET_KEY_PREFIX
        }
        _ => unreachable!("only Catecash networks are accepted"),
    };
    wif_bytes.extend_from_slice(&prefix);
    wif_bytes.extend_from_slice(&secret.to_secret_bytes());
    wif_bytes.push(1); // Compressed public key.
    let wif = bs58::encode(wif_bytes).with_check().into_string();

    (address, wif)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args_os().skip(1);
    let network = if arguments.next().as_deref() == Some(std::ffi::OsStr::new("--network")) {
        match arguments.next().as_deref() {
            Some(value) if value == "devnet" => NetworkKind::CatecashDevnet,
            Some(value) if value == "testnet" => NetworkKind::CatecashTestnet,
            Some(value) if value == "mainnet" => NetworkKind::CatecashMainnet,
            _ => return Err("--network must be devnet, testnet, or mainnet".into()),
        }
    } else {
        return Err("provide --network devnet, testnet, or mainnet".into());
    };
    let path = PathBuf::from(
        arguments
            .next()
            .ok_or("provide a private output file path")?,
    );
    if arguments.next().is_some() {
        return Err("provide exactly one output file path".into());
    }

    let secret = loop {
        if let Ok(secret) = SecretKey::from_secret_bytes(rand::random()) {
            break secret;
        }
    };
    let (address, wif) = address_and_wif(&secret, network);

    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(&path)?;
    serde_json::to_writer_pretty(
        &mut file,
        &serde_json::json!({
            "network": format!("{network:?}"),
            "key_type": "transparent-p2pkh",
            "derivation": "random-single-key",
            "address": address,
            "private_key_wif": wif,
        }),
    )?;
    writeln!(file)?;
    file.sync_all()?;

    println!("{address}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_address_and_wif_recover_the_same_secret() {
        let secret = SecretKey::from_secret_bytes([0x42; 32]).unwrap();
        for (network, prefix) in [
            (NetworkKind::CatecashDevnet, 0xf0),
            (NetworkKind::CatecashTestnet, 0xd3),
            (NetworkKind::CatecashMainnet, 0xd2),
        ] {
            let (address, wif) = address_and_wif(&secret, network);
            let decoded = bs58::decode(wif).with_check(None).into_vec().unwrap();
            assert_eq!(decoded[0], prefix);
            assert_eq!(&decoded[1..33], &secret.to_secret_bytes());
            assert_eq!(decoded[33], 1);
            assert_eq!(address.parse::<Address>().unwrap().network_kind(), network);
        }
    }
}
