# Local Catecash mainnet candidate rehearsal

On 2026-11-23, two Zebra processes used the loopback-only
[`candidate miner`](../../configs/catecash-mainnet-candidate.toml) and
[`candidate peer`](../../configs/catecash-mainnet-candidate-peer.toml) configs.
Both started from separate state directories, committed the candidate genesis,
connected through `550c6ba9` wire magic, and reached height 10. Each RPC
reported `catecash-mainnet`, genesis
`050bf7476db81684aeb38337fc931bd8bc4480fde43fb0a14fef172f85c5c323`,
and block 10
`0ef2977f1cde5edb3ea1cfc3ebe5fdbfaac3c81472d950e5994ee2de0148001b`.
Each reported one peer. Both processes were stopped after the check.

The candidate genesis target is `200fffff`, about eight times harder than the
devnet minimum. The test miner on this Mac accepted block 1 and continued
mining through block 10. The payout address in the config is a test vector with
no known private key, so these rewards were not spendable by the operator.
This describes the historical rehearsal config. The current local candidate
config has mining disabled pending payout schedule review.

This verifies a narrow local node path. It does not establish a launch-ready
mainnet, a spend test, independent mining, wallet indexing, seed peers, or a
public testnet. The [candidate parameters](mainnet-network-candidate.json) and
genesis still require independent review.
