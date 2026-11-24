# Catecash mainnet launch checklist

**Started:** 2026-11-23. **Decision:** NO-GO. CatecashDevnet and private candidate networks have been rehearsed; no public testnet exists. The boxes below require published evidence before they can be checked. Complete the gates in order; do not launch from the devnet config or treat reserved identifiers as final.

This checklist tracks the [launch runbook](mainnet-runbook.md). The [consensus candidate v1](consensus-candidate-v1.md) records the current implementation for review. The [protocol plan](protocol-plan.md), [chain and privacy contract](chain-and-privacy.md), [wide-supply migration](wide-supply-migration.md), [identity allocation](identity.md), and [wallet plan](wallet-plan.md) define the detailed work. A passing local test is evidence for its narrow claim, not a substitute for public testnet or independent review.

## 1. Freeze consensus and supply rules — BLOCKED

- [ ] Publish one versioned specification for the first mineable block: Equihash parameters, 25-second target, exact DigiShield arithmetic and early-block targets, proof-of-work limit, 5,000 CATE subsidy, fee allocation, 240-block coinbase maturity, minimum-difficulty policy, transaction versions, shielded activation, NSM policy, and upgrade/replay domain. Resolve the open decisions in the protocol plan. **Proof:** reviewed specification and independent test vectors.
- [ ] Integrate checked 128-bit aggregate pool balances through non-finalized state, finalized state, miner templates, disk encoding, restore, and reorganization. Keep Zcash accounting unchanged. **Proof:** persistence and reorg tests at the 4,200/4,201 reward boundary and signed 64-bit boundary.
- [ ] Widen transparent address balance and lifetime received totals; replace saturating accounting in the Catecash path with checked arithmetic. **Proof:** deterministic save/reopen and index replay above the old limit, plus repeated live payouts to one address past the boundary.
- [ ] Specify exact RPC, compact-block, and wallet encodings for totals above JavaScript's safe integer limit. Implement them together in Zebra, Zaino, and Zingolib. **Proof:** cross-component values and wrong-network rejection tests.
- [ ] Test subsidy, fees, PoW retargeting, live 240-block maturity spends, pool transfers, and deliberate reorganizations on a fresh chain. **Proof:** versioned fixtures, commands, logs, and expected hashes.

**Current evidence:** Devnet has focused subsidy, DigiShield, maturity, and wide-type tests. The hosted devnet was reset from genesis with the wide implementation. Three Zebra nodes agreed on block 79, the exact circulating-pool total, and the miner-address balance; Zaino matched the same height and balance. Zebra carries network-selected 128-bit pool totals through finalized and non-finalized state, miner templates, RPC, and versioned disk records. State updates check issued supply against subsidy permitted through the block height. Its transparent address index uses checked 128-bit balance and received totals. Zebra and Zaino always include a two-part 128-bit balance in Catecash compact gRPC responses while retaining the Zcash field for Zcash networks. Zaino parses and serves exact wide JSON totals. Focused disk round-trip, reward-boundary, received-total, RPC serialization, and Zaino parse/wire tests pass. A deterministic address-index test also passed save/reopen and replay above the old limit. The boxes stay open pending a mined chain past block 4,201, live chain agreement, cross-component values above the narrow limit, and the wallet work tracked separately. See the [wide-supply migration](wide-supply-migration.md), [devnet mining log](devnet-mining.md), and [runbook](mainnet-runbook.md).

**Local verification on 2026-11-23:** The DigiShield boundary, height-4,201 pool bound (now exercised for all three Catecash identities), wide disk round-trip, received-total overflow, contextual retarget, subsidy, candidate genesis proof and no-premine, Zebra and Zaino 128-bit gRPC balances, and Zingolib aggregate-balance tests passed. These are component tests, not the live-chain or independent review evidence required above. A Zebra state test fixture was updated to handle the new network variants. See the [candidate rules and vectors](consensus-candidate-v1.md).

The [2026-11-23 consensus review](consensus-review-2026-11-23.md) adds an
independent byte-level genesis check, independent arithmetic vectors, and a
height-one target test. It found and corrected a collision between the
candidate mainnet and private-testnet R2 Unified Address HRPs. A full public
manifest pass also rederived all 202,000 transparent payout addresses under
the unchanged `0684` prefix. These checks do not satisfy external review or
release approval.

An isolated state fixture now passes a deliberate replacement of a height-4,201 block, with wide pool and transparent address totals following the replacement chain. The [devnet mining log](devnet-mining.md) also records a live Zebra/Zaino exact-balance match at height 1,206. Neither result substitutes for a live-chain reorg or a cross-component comparison above the old limit.

## 2. Freeze independent network identities — IN PROGRESS

- [ ] Finalize separate mainnet and public-testnet magic, ports, branch IDs, address and viewing-key encodings, and derivation domains. Reserved values in the identity allocation are provisional. **Proof:** versioned parameter contract, collision tests, and negative Zcash vectors.
- [ ] Mine and independently reproduce a no-premine mainnet genesis from frozen inputs. Publish serialized block bytes, timestamp, message, target, nonce, Equihash proof, hash, and height-zero checkpoint. **Proof:** two independent verifiers agree; no spendable genesis output exists.
- [ ] Create a distinct public-testnet genesis and identity. **Proof:** separate fixtures and cross-network rejection tests.

**Current evidence:** The mainnet magic, ports, and genesis timestamp are frozen. A [mined mainnet genesis candidate](mainnet-genesis-candidate.md) uses a provisional 8× difficulty target and has a matching height-zero checkpoint in Zebra. Its initial target, address/key encodings, branch ID, and coin type remain candidates pending review and independent reproduction. The [testnet version 1 contract](testnet-network-v1.json) now records a separate mined genesis and rehearsal parameters; implementation and independent verification are still required. Zaino and Zingolib now recognize the candidate mainnet identity locally; that does not finalize its parameters.

## 3. Implement dedicated public networks — IN PROGRESS

- [ ] Add explicit `CatecashMainnet` and `CatecashTestnet` identities throughout Zebra and the vendored protocol/address libraries. Route consensus, genesis, checkpoints, storage, P2P, RPC, and address parsing by identity. **Proof:** focused tests and fresh-node checks; no Zcash or devnet state reuse.
- [ ] Add the same identities to Zaino and Zingolib, verify genesis before indexing or scanning, and isolate wallet files and keys. **Proof:** end-to-end tests with rejected wrong-chain endpoints and addresses.
- [ ] Integrate and verify mobile and desktop bindings against each public network. **Proof:** independent recipient, send, receive, restore, and reorg tests.

**Current evidence:** Zebra has a private `CatecashMainnet` candidate with
separate magic, storage, genesis, checkpoint, branch ID, address parsing, and
three hosted peers restricted to their private network. Those peers are at
genesis with mining disabled. An earlier [two-node rehearsal](mainnet-candidate-rehearsal.md) mined
through block 10 with matching tips. Zaino recognizes the candidate genesis and
network name, and Zingolib now has a separate candidate wallet chain, storage,
address parsing, and a [genesis-checked indexer connection](mainnet-wallet-candidate-rehearsal.md). The parameters remain
provisional and no public testnet has run. A distinct `CatecashTestnet`
identity now compiles in Zebra, Zaino, and Zingolib. The [private rehearsal](testnet-private-rehearsal.md)
mined nine blocks on two loopback nodes and synced a local wallet through Zaino.
This is evidence for a private testnet path, while the public-network gate remains open.

## 4. Operate a public testnet — NOT STARTED

- [ ] Run separately operated nodes, seed peers, indexers, wallets, and miners on the final testnet rules. **Proof:** published versions, configs without secrets, peer and tip observations.
- [ ] Demonstrate mining, synchronization, restart, reorg, maturity spends, wallet recovery, transparent and shielded transfers, compact-block scanning, and wrong-chain rejection. **Proof:** reproducible test report and fixtures.
- [ ] Resolve testnet findings and repeat affected checks on a release candidate. **Proof:** issue/fix ledger and passing reruns.

**Current evidence:** The [private testnet rehearsal](testnet-private-rehearsal.md)
verified one genesis, node restart and tip agreement, Zaino serving height 9,
wallet scanning to height 9, and wrong-chain rejection. All nodes and services
were loopback only on one host; no public testnet has launched.

## 5. Review and prepare release — NOT STARTED

- [ ] Obtain independent review of consensus separation, supply accounting, wallet signing and recovery, replay protection, and cross-chain rejection. Resolve critical findings. **Proof:** review report and linked fixes.
- [ ] Produce tagged, reproducible node, indexer, and wallet releases from the reviewed source. **Proof:** build instructions, matching artifact hashes, signed release metadata.
- [ ] Prepare multiple independently operated mainnet bootstrap peers and DNS seeds. Authenticate and restrict RPC; verify each peer serves the published mainnet genesis. **Proof:** operator checks and peer observations.
- [ ] Publish network parameters, genesis fixture, source and builds, seed list, testnet evidence, and operator instructions before public mining. **Proof:** public release bundle.

## 6. Mainnet go/no-go and launch — NOT STARTED

- [ ] Have independent operators verify every gate and sign off on the exact release, genesis hash, launch procedure, and rollback/incident contacts. **Proof:** dated go/no-go record.
- [ ] Start peers and mining only after GO; monitor block production, difficulty, peer diversity, state agreement, RPC exposure, and wallet sync. **Proof:** launch log and matching independently observed tips.

## Next work item

The reset hosted Catecash devnet is mining repeated payouts to one address
toward block 4,201. A read-only watcher will record exact Zebra pool and
address totals at blocks 4,200 and 4,201. Compare those values with Zaino and
the wallet after the boundary. The wallet's inherited scanned-balance cap
also needs a checked Catecash aggregate path. Keep every box open until its
proof exists.
