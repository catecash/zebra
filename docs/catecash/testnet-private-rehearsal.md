# Catecash testnet version 1 private rehearsal

**Date:** 2026-11-23 UTC. **Scope:** one Mac, two loopback Zebra processes, one loopback Zaino process, and one local Zingolib CLI wallet. No public testnet was opened.

The [version 1 contract](testnet-network-v1.json) fixes genesis `15999c77e978a180c2f3fe2ec8478f8123e5c923f20bedaaf1b156fc38c4dda2`, P2P magic `e79c55e7`, transaction branch ID `b80dae22`, and a zero-value OP_RETURN genesis coinbase. The testnet genesis miner reproduced the serialized fixture, and the focused Zebra genesis test verified its Equihash solution, target, hash, timestamp, merkle root, and absence of spendable genesis output.

## Procedure and observations

1. Built Zebra with `cargo build -p zebrad --features internal-miner`. Started [`catecash-testnet-peer.toml`](../../configs/catecash-testnet-peer.toml), then [`catecash-testnet-miner.toml`](../../configs/catecash-testnet-miner.toml). Both bound P2P and RPC to loopback and used separate state directories. The miner address is a public test vector without a known private key, so its rewards are not spendable.
2. The miner accepted nine blocks, then was stopped. It was restarted without mining via [`catecash-testnet-verifier.toml`](../../configs/catecash-testnet-verifier.toml). Both nodes were restarted once to clear a stale local connection. Each then reported chain `catecash-testnet`, height 9, tip `08fa40999604ccdc77d3b951578fda5fdbafae2983454c5e8654394bfeed5069`, genesis `15999c77e978a180c2f3fe2ec8478f8123e5c923f20bedaaf1b156fc38c4dda2`, and one peer. The active NU7 branch ID in `getblockchaininfo.upgrades` was `b80dae22` at height 1.
3. Built Zaino from the adjacent fork and started it with the loopback [`catecash-testnet.toml`](../../../zaino/docs/example_configs/catecash-testnet.toml) config. The Zingolib CLI's `info` response from `127.0.0.1:34357` reported `catecash-testnet`, Sapling activation height 1, branch `b80dae22`, and latest block height 9. A fresh testnet wallet scanned the local chain and its saved file reported height 9. It had zero balance because the mined rewards used an unrelated, unspendable test address. The CLI's `sync run` command continued printing `Syncing` after the saved wallet had reached height 9 and was interrupted; its completion behavior needs investigation.
4. A Catecash devnet wallet refused this testnet gateway with an identity mismatch. Zaino configured as Catecash devnet also refused the testnet validator before serving. The wallet's address test accepted the testnet address vector and rejected that vector as devnet or Zcash Testnet.

The CLI test used the explicit `nakednet-test-mode` build against loopback. That build is for this controlled rehearsal and is not a public wallet release.

Run the same loopback wallet check from the Zingolib checkout after starting the two nodes and Zaino:

```sh
cargo build -p zingolib-cli --no-default-features --features nakednet-test-mode
target/debug/zingolib-cli --chain catecash-testnet --server http://127.0.0.1:34357 --data-dir /private/tmp/catecash-testnet-wallet-rehearsal info
target/debug/zingolib-cli --chain catecash-testnet --server http://127.0.0.1:34357 --data-dir /private/tmp/catecash-testnet-wallet-rehearsal sync run
target/debug/zingolib-cli --chain catecash-testnet --offline --data-dir /private/tmp/catecash-testnet-wallet-rehearsal height
```

## Open gates

- The two nodes share one machine and one implementation. Independent operators and independently reproduced genesis verification remain open.
- The miner's rewards cannot be spent. Test controlled mining keys, 240-block maturity, transparent and shielded transfers, restore, and a deliberate reorganization.
- Test Zaino restart and index rebuild, sustained block production, wide balances beyond the narrow limit, and wallet failover.
- Investigate why `zingolib-cli sync run` did not finish after the saved wallet had scanned to height 9.
- Publish reviewed source, build digests, public seed peers, DNS/TLS gateway and explorer, monitoring, funding, and release instructions before calling this a public testnet.

The initial attempt used a genesis timestamp too far behind the local clock. Zebra correctly withheld mining templates while the node was not close to the estimated tip. The final fixture uses `2026-11-23T22:30:00Z`; the earlier genesis attempts and their state directories were kept only as local backups, not published as testnet parameters.
