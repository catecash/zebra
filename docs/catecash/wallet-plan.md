# Catecash wallet integration plan

**Status:** implementation plan, not a claim of complete wallet compatibility. Audited against the Catecash `zingolib`, `zingo-mobile`, and `zingo-pc` fork checkouts on 2026-11-22. The local Catecash devnet has a distinct genesis, transaction branch ID, and provisional address encodings, but production consensus rules and wallet integration remain unfinished. Keep it isolated from public Zcash networks. The Zingolib integration branch has a Catecash devnet chain type, seed-based keys, separate wallet files, and live Zebra → Zaino → wallet sync. A WIF mining reward was swept into Ironwood and a confirmed Ironwood spend returned 1 CATE to a transparent address on 2026-11-23. See [devnet mining evidence](devnet-mining.md). A separate [Catecash testnet private rehearsal](testnet-private-rehearsal.md) then synced a fresh wallet through Zaino to block 9; public wallet release gates remain open.

## Stack and ownership

Catecash compact gRPC `GetTaddressBalance` and its streamed variant now always
return `Balance.valueZatWide`. Decode the two protobuf `fixed64` fields as
`(high << 64) | low` into `u128`, and require the wide field even for zero.
`Balance.valueZat` is the shared Zcash field and is not authoritative on
Catecash. The wallet interface must use this contract before the hosted
Catecash endpoint is redeployed.

| Component | Catecash responsibility |
| --- | --- |
| Catecash Zebra | Validate the independent chain; expose correct block, transaction, tree, fee, and broadcast RPC data. |
| Zaino fork | Index Catecash Zebra and serve compact blocks, treestates, mempool data, and transaction submission to light clients. |
| Zingolib fork | Own wallet network identity, keys and addresses, scanning, balances, fee selection, transaction construction, signatures, proofs, and wallet storage. |
| Zingo Mobile fork | Display CATE for Catecash networks; use Catecash Zingolib bindings and Catecash server configuration. |
| Zingo PC fork | Reuse the same Catecash Zingolib protocol behavior; adapt desktop packaging and network UI. |

The dependency direction is **node and protocol specification → indexer → wallet engine → apps**. Mobile and desktop must share one protocol implementation and test vectors. Branding can proceed independently, but a branded build must not accept funds until the chain checks below pass.

## Source audit and immediate blockers

| Area | Current code | Required change |
| --- | --- | --- |
| Wallet network model | `zingolib/crates/common/src/chain.rs` now has `CatecashDevnet`, the provisional coin type and addresses through the shared vendor crates, and the Catecash transaction branch ID override. | Add public Catecash networks after their parameters are frozen; validate signed transactions against Zebra. |
| Wallet storage | `zingolib/crates/file-format/src/file.rs` uses chain ID 3 for Catecash; `crates/zingolib/src/config.rs` selects a Catecash data directory and wallet filename. A save/restore test rejects the same file as Zcash Mainnet. | Test upgrades and app bindings against the separate wallet storage. |
| Indexer selection | Catecash has no Zcash default server. The wallet checks Zaino's chain name, branch ID, Sapling activation, and genesis hash before sync. A live Zebra → Zaino → wallet test synced the hosted mined block. | Add a permanent Catecash endpoint after server hardening and keep verifying identity on reconnect. |
| Mobile binding | `zingo-mobile/binding-layer.json` pins upstream Zingolib commit `bb149eca…` and upstream Android coordinates. | Publish and pin Catecash-built Android and iOS binding bundles, with reproducible digests and an FFI version check. |
| Desktop native engine | `zingo-pc/native/Cargo.toml` pins three `zingolabs/zingolib` crates to upstream commit `bb4b42de…`. | Pin all three to one tested Catecash commit and rebuild the native module and lockfile. |
| App network and services | Mobile `app/AppState/enums/ChainNameEnum.ts`, `app/uris/serverUris.ts`, and `app/services/addWalletDraft.ts` use Zcash chains and endpoints. Desktop `src/components/appstate`, `src/utils/pickAutoServer.ts`, and `src/rpc/rpc.ts` use the same Zcash assumptions and ZEC/TAZ display. | Expose Catecash chain identity end to end, remove Zcash defaults, and gate price, swaps, names, explorers, and payment links until Catecash equivalents are specified. |

The local Catecash `zaino` fork now uses the Catecash Zebra crates and matching Zcash library cohort. Its `CatecashDevnet` network variant has separate index storage, and network adoption verifies the validator's chain name, genesis hash, and reported activation heights before indexing. The local development config uses loopback ports. Targeted network and config tests pass.

On 2026-11-22, an isolated local Zebra and Zaino run mined a height-one block with hash `1c29d093b49b2d6ecd2a9e102830ad994e75f41beda9318050803243c6ac041f`. Zaino served that block through JSON-RPC `getblock` and lightwallet gRPC `GetBlock`; both its genesis block and the compact block's parent hash matched `6b9c4d783359202111571c123309b2c0511d1bf9036f74dee4921f04e1f0a9cf`. `GetLightdInfo` reported `catecash-devnet` and branch `ea3beeb4`. Zebra was restarted with mining disabled and retained six mined blocks before Zaino indexed them. This smoke test does not cover a reorganization, wrong-chain validator rejection in a live process, or wallet scanning.

The temporary no-peer mining shortcut used for that first smoke test has been removed. The current [two-node devnet rehearsal](mainnet-runbook.md) connects a miner to one peer before mining; the peer independently validated the new chain through height four.

The machine-readable [development contract](devnet-wallet-contract.json) records the node values already fixed in source. Its address encodings are provisional and `wallet_enabled: false` remains the release gate. The Catecash node profile now exposes RPC on loopback for local Zaino integration.

## 0. Freeze the protocol contract

1. Keep the current Equihash, no-merged-mining direction aligned across this repository and `website/docs/protocol-plan.md`. Record any future consensus change in both plans before building wallet assumptions around it.
2. Specify mainnet, testnet, and devnet genesis hashes, network IDs, launch transaction rules, branch IDs, activation schedule, block interval, fee units, minimum confirmations, and coinbase maturity. Treat the current devnet branch ID as a development value until the launch rules are frozen.
3. Allocate noncolliding transparent P2PKH/P2SH and TEX bytes; Sapling, Orchard, and Unified Address encodings; viewing-key encodings; and the relevant seed/key derivation network domains. State which keys or seed phrases may be portable and never imply that a Zcash address is a Catecash address.
4. Publish shared positive and negative test vectors: address encode/decode, key derivation, signature hash, transaction ID, shielded output scanning, and cross-chain rejection. Confirm that the launch branch ID gives transaction replay separation from Zcash.
5. Resolve the continuing 5,000 CATE reward's amount and pool accounting before wallet balances can be trusted long term. The node plan identifies the 21-million-coin limit and signed 64-bit pool balance boundaries as blockers.

**Gate:** one versioned network-parameters artifact and vectors are accepted by node, indexer, and wallet implementations. No wallet release uses provisional Zcash identifiers.

## 1. Make the node a reliable wallet backend

1. Finish Catecash network selection, address validation, genesis/checkpoints, difficulty, reward, maturity, and shielded validation. Reject Zcash peers, addresses, transactions, and checkpoints on Catecash networks, and the reverse.
2. Audit the RPC methods needed by Zaino: chain tip, block/compact-block source data, tree state and subtree roots, transaction lookup, mempool, fee estimation, and raw transaction submission. Return Catecash network identity and branch ID explicitly where clients need them.
3. Run an isolated local devnet with loopback RPC, then a separate public testnet only after consensus behavior is stable. Do not point a wallet at public Zcash infrastructure.

**Gate:** two fresh nodes sync, restart, reorganize, and reject cross-network data; node RPC returns values matching independent fixtures.

## 2. Adapt Zaino

1. Replace or parameterize Zcash-only consensus constants, address classification, Zebra type conversions, expected genesis, network names, and upstream RPC assumptions. Keep Catecash index/state storage separate from Zcash storage.
2. Verify compact blocks for all enabled shielded pools from height one, including commitment trees, nullifiers, subtree roots, treestates, transparent outputs, and mempool changes. Preserve block and transaction IDs exactly as the Catecash node reports them.
3. Expose an unambiguous Catecash network identifier at the light-wallet endpoint. Refuse requests if the connected validator has the wrong genesis or network. Provide TLS for remote clients and a documented testnet endpoint.
4. Exercise chain reorganizations, index rebuild, restart, pruning, and a 25-second target interval under sustained block production.

**Gate:** Zaino's indexed tip and compact-block fixtures match the node before and after restart and reorganization; a wrong-chain validator connection fails closed.

## 3. Adapt Zingolib

1. Add explicit Catecash mainnet/testnet/devnet network types, genesis checks, and chain-specific server lists. Remove implicit Zcash defaults from wallet creation and restore. Reject Zcash server responses, addresses, and transaction branch IDs.
2. Update address and viewing-key parsing/encoding and seed/key derivation from the frozen vectors. Put chain ID in wallet-file metadata; use distinct file locations and prevent opening a Zcash wallet as Catecash without an explicit, reviewed migration design.
3. Connect the wallet to Catecash Zaino. Validate compact-block continuity, scanning birthdays, tree roots, notes, nullifiers, transparent UTXOs, and reorg rollback for all enabled pools.
4. Construct and broadcast Catecash transactions with the correct branch ID and signature hash. Verify shielded proof generation, fee calculation, change, dust policy, confirmation display, and 240-block coinbase maturity. Avoid assuming Zcash's 21-million supply cap in wallet balance arithmetic.
5. Disable Zcash-specific integrations until each is explicitly adapted: Zcash price feeds, swaps, names, server registries, fiat ramps, voting, telemetry labels, and external links.

**Gate:** CLI-level tests create and restore a wallet, receive and spend transparent and shielded funds, handle a reorg, and reject cross-chain addresses and servers. The wallet's balance and history agree with node fixtures.

## 4. Adapt mobile and desktop apps

1. Pin both apps to the tested Catecash Zingolib build and verify the native bindings are rebuilt from the fork, not fetched from upstream Zingo bundles.
2. Replace display name, ticker, denomination, address examples, QR/payment URI, server defaults, support links, application IDs, icons, build identifiers, and update channels. Keep upstream licenses and attribution.
3. Make network selection and backup/restore copy explicit. Remove unsupported Zcash services from visible flows. Use separate app storage and signing identities from upstream Zingo releases.
4. Test on actual Android, iOS, Windows, macOS, and Linux targets selected for the first release. Confirm send/receive, screen lock, seed backup, restore, offline/reconnect, and error handling with the same Catecash testnet fixtures.

**Gate:** no app build can silently connect to Zcash, import Zcash funds as CATE, or display a balance before its selected Catecash server passes the genesis/network check.

## 5. End-to-end release evidence

- Automated harness: Catecash Zebra → Catecash Zaino → Catecash Zingolib → one mobile app and one desktop app.
- Transactions: transparent-to-transparent, transparent-to-shielded, shielded-to-shielded, shielded-to-transparent, and every shielded pool enabled at launch.
- State changes: fresh sync, seed restore, viewing-key restore, reorg across a spend, server failover, index rebuild, and wallet restart.
- Boundaries: zero balance, fees and change, 240-block coinbase maturity, high issuance and pool balances, malformed addresses, wrong genesis, wrong branch ID, and Zcash server/transaction rejection.
- Security: independent review of chain separation, wallet key handling, signing, proof generation, and server trust. Publish reproducible builds and testnet results before a mainnet wallet release.

## First implementation slice

Start with one isolated Catecash devnet and a CLI wallet. The first reviewable vertical slice has these ordered deliverables:

1. Publish a development-only parameter fixture with the devnet genesis hash, `0xEA3BEEB4` branch ID, activation heights, provisional address encodings, and negative Zcash vectors. Mark every provisional value as such.
2. Make Zebra and Zaino report and verify the same chain identity and genesis. Demonstrate one mined block through the lightwallet endpoint, then reject a Zcash node or indexer.
3. Add the Catecash devnet variant to Zingolib's network and wallet-file formats. Create and restore a CLI wallet in a separate Catecash directory. Reject Zcash wallet files, servers, and addresses.
4. Scan one mined block, receive one transparent output, and submit one signed transparent transaction. Check the branch ID, transaction ID, amount, fee, balance, and restart behavior against Zebra RPC.

Next add shielded scanning and spends with reorg tests. Then rebuild the mobile binding bundles and desktop native module from the tested Catecash engine. Development values may change before public testnet; migration rules must be explicit if test wallets are retained.
