# Mainnet candidate wallet and indexer rehearsal

**Date:** 2026-11-23. **Scope:** one host, loopback only, candidate chain at genesis. This is preparation evidence, not a public mainnet launch or a final wallet release.

## Identity checked

- Zebra candidate node reported `chain = catecash-mainnet`, height 0, genesis `050bf7476db81684aeb38337fc931bd8bc4480fde43fb0a14fef172f85c5c323`, and next-block branch `5bc68392` through its local RPC.
- Zaino served the same candidate identity on `127.0.0.1:34337`. At height zero, its `GetLightdInfo` chain-tip branch is `00000000`; Zingolib accepts that value only at height zero and still checks the genesis hash. At later heights, it requires the launch branch.
- Zingolib's focused live test accepted the candidate endpoint and rejected the same endpoint as `CatecashTestnet`.
- An offline candidate CLI wallet used `catecash-mainnet-wallet.dat` and produced a `Ca` transparent address. Opening a copied candidate wallet as Zcash mainnet was rejected.
- The candidate wallet has no default public indexer; the local test passed an explicit loopback server.

## Checks run

`cargo check -p zl-common -p zl-file-format -p zingolib -p zingolib-cli -p zingolib-ffi --offline` passed in Zingolib. Focused chain, file-format, wallet-config, Zaino network, address, and genesis tests passed. `cargo build -p zingolib-cli --offline -j 2` passed with incremental builds disabled. The ignored live `catecash_mainnet_candidate_indexer_matches_node_contract` test passed against the local node and Zaino.

A connected CLI session passed the identity check and then stopped at its separate mixnet requirement because this local build has no bundled `nym-proxy`. No send, receive, restore, mobile, or public-network claim follows from this rehearsal.

## Remaining gates

The [mainnet launch checklist](mainnet-checklist.md) stays **NO-GO**. Freeze and independently reproduce the candidate consensus and genesis; complete wide-supply and reward-boundary testing; operate a public testnet; review signing and recovery; build and verify release artifacts; and obtain a dated launch decision before public mining.
