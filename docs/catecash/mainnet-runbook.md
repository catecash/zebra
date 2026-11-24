# Catecash network launch runbook

Track launch readiness in the [mainnet checklist](mainnet-checklist.md). Its open gates are the current go/no-go record.

**Status:** a reproducible devnet and a local-only Catecash mainnet candidate.
The mainnet magic, ports, and genesis timestamp are allocated; the
[`CatecashMainnet` candidate](mainnet-candidate-rehearsal.md) mined through
block 10 on two loopback nodes. Its proof-of-work target, address and key
encodings, branch ID, and coin type await review. There is no public network
release candidate yet.

| Launch gate | Current evidence | Remaining work |
| --- | --- | --- |
| Consensus | Devnet reward, DigiShield retargeting, and 240-block maturity have focused tests; a fresh two-node chain reached height nine and survived a peer restart. Checked 128-bit pool and address totals now flow through Zebra state and RPC and Zaino JSON, with focused tests. A deterministic address-index persistence and replay check passed above the old limit. | Complete the separate wallet interface work; verify [wide supply](wide-supply-migration.md) on a live chain across block 4,201; freeze public-network rules and test live maturity spends. |
| Identity | Mainnet wire magic, ports, and genesis timestamp are frozen. Zebra has a separate candidate genesis, network identity, and address/key domains in [identity.md](identity.md). | Independently reproduce and review the genesis and remaining candidate parameters; implement the matching identities in Zaino and wallets. |
| Wallet stack | Devnet Zebra → Zaino → Zingolib sync, a confirmed WIF reward sweep into Ironwood, and a confirmed Ironwood-to-transparent spend. | Test independent recipient wallets, reorgs, public-network wallets, and app bindings. |
| Public testnet | A [private version 1 rehearsal](testnet-private-rehearsal.md) ran the node → gateway → wallet path through height 9. Two [dedicated hosted nodes](testnet-hosted-nodes.md) now mine and agree over private P2P. The Zaino wallet gateway and separate CateScan explorer serve verified TLS endpoints and agreed with the nodes at height 106. | Open reviewed public P2P, add controlled-key test coin funding, complete mature spends, wallet recovery, deliberate reorgs, sustained wide-balance tests, and app builds. |
| Release | No mainnet release candidate exists. | Independent review, reproducible builds, bootstrap peers, tagged release, and published testnet evidence. |

## Rehearse a miner with one peer

Run these commands from the root of this Zebra checkout. The two configs use separate state directories under `target/`, bind P2P and RPC to loopback, and disable RPC cookie authentication only for this local rehearsal. The peer listens on `127.0.0.1:34344`; the miner dials that address from `initial_testnet_peers`. Build with `internal-miner`, because a normal `zebrad` build does not start the miner.

```sh
cargo build -p zebrad --features internal-miner
target/debug/zebrad -c configs/catecash-devnet-peer.toml start
```

In a second terminal, from the same checkout:

```sh
target/debug/zebrad -c configs/catecash-devnet-miner.toml start
```

The miner config pays to a valid **test address with no known private key**. Replace `mining.miner_address` with a Catecash devnet address you control before testing spendable rewards. Do not copy this address into a public network configuration.

Check the connection and both chain tips:

```sh
curl -sS -H 'content-type: application/json' \
  --data '{"jsonrpc":"2.0","id":1,"method":"getpeerinfo","params":[]}' \
  http://127.0.0.1:34342
curl -sS -H 'content-type: application/json' \
  --data '{"jsonrpc":"2.0","id":2,"method":"getblockchaininfo","params":[]}' \
  http://127.0.0.1:34342
curl -sS -H 'content-type: application/json' \
  --data '{"jsonrpc":"2.0","id":3,"method":"getblockchaininfo","params":[]}' \
  http://127.0.0.1:34345
curl -sS -H 'content-type: application/json' \
  --data '{"jsonrpc":"2.0","id":4,"method":"getblockhash","params":[0]}' \
  http://127.0.0.1:34342
curl -sS -H 'content-type: application/json' \
  --data '{"jsonrpc":"2.0","id":5,"method":"getblockhash","params":[0]}' \
  http://127.0.0.1:34345
```

`getpeerinfo` on the miner should show `127.0.0.1:34344` as connected. Both nodes should report `catecash-devnet`, the same genesis `6b9c4d783359202111571c123309b2c0511d1bf9036f74dee4921f04e1f0a9cf`, and eventually the same best block hash above height zero. Stop the miner with Ctrl-C when enough blocks have been mined; the peer should retain the accepted blocks. For a fresh rehearsal, use new state directories instead of reusing a previous chain.

This two-node procedure was run on 2026-11-22 with the peer-sync mining check enabled. The miner reported a connected peer at `127.0.0.1:34344` and accepted height-one block `3f194c598f02f9ddde10f9b649542578dc042ccc51e8d9fecac1ffc66e17ea07`. The peer independently returned that hash at height one, the expected Catecash genesis at height zero, and a tip at height four. A single peer demonstrates the P2P path; it is not a resilient public bootstrap network.

After wiring DigiShield retargeting and 240-block coinbase maturity, a fresh
two-node rehearsal ran on 2026-11-23 with separate temporary state and loopback
ports. The miner connected to one peer; both nodes agreed at height eight with
hash `39774d3bd90584f919f3d44af969bfda1a6f25af1f76607791c0436a68e6df28`.
After mining stopped and the peer restarted, it retained height nine with hash
`348685239a77063b8328fe90541e98f483f355af1a0a881ff02866d22247562c`.
Its reported tip difficulty was `2.0370560501990647`, confirming a target
different from the initial limit. This rehearsal did not test a deliberate
reorganization, a 240-block spend boundary on a live chain, or wallet spending.

## Prepare a public Catecash network

Complete these in order. Each step produces an artifact that the next step can verify.

1. **Freeze the consensus specification.** Publish the final Equihash parameters, 25-second target and retarget algorithm, initial difficulty and limit, continuing 5,000 CATE subsidy, fee rules, 240-block coinbase maturity, and long-term amount and pool-balance representation. Resolve the outstanding issues in the [protocol plan](protocol-plan.md) and [chain implementation contract](chain-and-privacy.md). A config file cannot change these consensus rules safely.
2. **Freeze the mainnet identity.** Select the genesis timestamp and message, mine and publish its serialized bytes and proof, require no spendable genesis allocation, and publish its hash and height-zero checkpoint. Finalize P2P magic, ports, transaction branch ID, transparent and shielded address encodings, viewing-key encodings, and derivation domains. The allocations in [identity.md](identity.md) are reserved values until this review is complete. Record the values and negative Zcash vectors in a versioned contract like [devnet-wallet-contract.json](devnet-wallet-contract.json).
3. **Implement dedicated mainnet code.** Add a `CatecashMainnet` variant to Zebra and the vendored protocol/address libraries. Route genesis, checkpoint, PoW, rewards, maturity, upgrades, storage names, peer handshake, RPC network name, and address parsing through it. Keep devnet state and credentials separate. Add the same network to Zaino and make it verify the final genesis before indexing.
4. **Build a public testnet first.** Give it its own genesis, magic, ports, branch ID, addresses, and peer seeds. Run independent nodes that mine, sync, restart, reorganize, and reject Zcash peers and blocks. Verify transparent and shielded transaction creation, wallet restore and scanning, Zaino compact blocks, and wrong-chain rejection. Publish the fixtures and results.
5. **Set up mainnet peers and miner access.** Prepare multiple independently operated bootstrap peers with stable public P2P addresses and DNS seed records. Each must run the same tagged release and agree on the published genesis. Bind P2P on the approved public address and port; keep RPC authenticated and restricted to trusted operators or local services. A miner needs a controlled Catecash address, a mining-enabled build or compatible external miner, and at least one reachable peer before relying on chain-tip readiness.
6. **Release only after the gates pass.** Obtain an independent review of consensus separation, supply accounting, wallet keys and signing, and cross-chain rejection. Publish a tagged source release, reproducible build instructions, genesis bytes and hash, network parameters, seed addresses, and testnet evidence before opening public mining.

Do not rename `CatecashDevnet` to mainnet or reuse its genesis, provisional addresses, test miner address, or loopback peer. Mainnet values must be fixed in code and verified by at least two independent implementations or fixtures before release.
