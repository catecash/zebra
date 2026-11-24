# Catecash mainnet genesis candidate

This is a **candidate fixture**, not an activated mainnet genesis. It uses the
timestamp frozen in [identity.md](identity.md), the proposed Catecash genesis
message, and an initial target about eight times harder than the development
proof-of-work limit. The initial mainnet target and the rest of the public-network
identity remain candidates, so changing those
inputs requires mining a different block.

| Input or result | Value |
| --- | --- |
| Timestamp | 2026-11-23 04:43:11 UTC (`1795408991`) |
| Header bits | `200fffff` (about 8× the development minimum difficulty; provisional) |
| Coinbase message | `I am the fortunate one. The High G, The One above.` |
| Coinbase output | Zero-value `OP_RETURN`; no spendable premine |
| Block hash | `050bf7476db81684aeb38337fc931bd8bc4480fde43fb0a14fef172f85c5c323` |
| Serialized block | [`block-catecash-mainnet-candidate-0.txt`](../../zebra-chain/src/block/genesis/block-catecash-mainnet-candidate-0.txt) |

The 8× target was chosen as a mineable candidate after the hosted devnet was
observed near difficulty 15 with recent blocks around 20–25 seconds apart.
Alternatives considered were 4× (`201fffff`) and 16× (`2007ffff`). The local
candidate miner accepted block 1 about 24 seconds after receiving its first
template and reached block 10 with one connected peer. This is a calibration
check on one machine, not a security estimate for a public network.

Reproduce it from the checkout with:

```sh
cargo run -q -p zebra-chain --example catecash_genesis --features internal-miner -- mainnet-candidate
cargo test -q -p zebra-chain catecash_mainnet_candidate_genesis_has_valid_proof_and_no_premine --lib
```

The test checks the hash, Equihash solution, difficulty threshold, Merkle root,
zero-value coinbase, and serialized bytes. Independent reproduction and review
are still required before this or a replacement fixture can become the public
mainnet genesis.

The standard-library [`utilities/verify-mainnet-genesis.py`](../../../../utilities/verify-mainnet-genesis.py)
also parses the serialized bytes independently of Zebra and checks the header
hash, target, timestamp, Merkle root, and sole zero-value `OP_RETURN` output.
It does not validate the Equihash proof. The inherited height-one threshold
path yields compact target `200d70a2`; the one-block DigiShield rule begins at
height two. See the [dated review](consensus-review-2026-11-23.md).
