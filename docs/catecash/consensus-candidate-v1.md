# Catecash consensus candidate v1

**Status:** implementation snapshot for review, 2026-11-23. This is not a final
mainnet specification or an independent review. The mainnet go/no-go decision
remains NO-GO in [the launch checklist](mainnet-checklist.md).

This document records the rules currently selected by the Catecash Zebra
checkout. The [mainnet identity contract](mainnet-network-candidate.json) and
[testnet contract](testnet-network-v1.json) contain the network-specific
identifiers and genesis fixtures. A change to a consensus rule or genesis
input requires a new version of this document and new fixtures.

## Rules from the first mineable block

| Rule | Candidate value | Implementation |
| --- | --- | --- |
| First mineable height | 1; genesis creates no spendable output | `zebra-chain/src/block/genesis.rs`, `parameters/network/subsidy.rs` |
| Validation rules | NU7 from height 1; genesis retains height-zero rules | `parameters/network/catecash_mainnet.rs`, `catecash_testnet.rs` |
| Proof of work | Equihash `n=200`, `k=9`; network-specific target limit | `work/equihash.rs`, `work/difficulty.rs`, network identity modules |
| Target spacing | 25 seconds | `work/digishield.rs` |
| Retarget | One-block DigiShield from height 2; no minimum-difficulty exception | `work/digishield.rs`, `zebra-state/src/service/check/difficulty.rs` |
| Subsidy | 5,000 CATE (500,000,000,000 base units) at every height >= 1; no halving | `parameters/network/subsidy.rs` |
| Reward allocation | No founder or funding-stream allocation; miner may claim subsidy plus transaction fees | `parameters/network/subsidy.rs`, coinbase validation |
| Coinbase maturity | 240 blocks | `parameters/network.rs`, state UTXO checks |
| NSM reserve | Must remain zero on Catecash networks | `value_balance/network.rs` |
| Aggregate balances | Checked 128-bit pool and address totals; issued supply at height h <= h × 500,000,000,000 base units | `value_balance/wide.rs`, `value_balance/network.rs` |
| Transaction replay domain | Network-specific NU7 branch ID; mainnet candidate `5bc68392`, testnet `b80dae22` | `parameters/network_upgrade.rs` |

The DigiShield input at height `h >= 2` is the previous block's compact target
and the previous block timestamp minus its parent's timestamp, in whole
seconds. Let `d = actual_seconds - 25`. Integer division truncates toward
zero. The filtered span is `clamp(25 + trunc(d / 8), 19, 38)`. Decode the
previous compact target, calculate `floor(previous_target × filtered_span /
25)`, cap at the network proof-of-work limit, and encode the result as a
compact target. Invalid previous targets are rejected. Height one follows
the existing genesis-to-first-block threshold path in Zebra. From the
candidate mainnet genesis it yields compact target `200d70a2`, asserted in a
focused state test. A separate implementation should validate this vector
before the candidate is frozen.

Mainnet candidate genesis uses compact target `200fffff` and proof-of-work
limit `0fffff0000000000000000000000000000000000000000000000000000000000`.
Testnet v1 uses `207fffff` and
`7fffff0000000000000000000000000000000000000000000000000000000000`.
These values are network-specific and the mainnet pair remains provisional.

## Reference vectors to reproduce independently

The DigiShield vectors below are asserted in `work/digishield.rs`; they are
implementation vectors, not independent verification. Each uses previous
compact target `1e00ffff` and limit `1f00ffff`.

| Actual seconds | Expected next compact target |
| ---: | --- |
| -60 | `1e00c28e` |
| 0 | `1e00e146` |
| 25 | `1e00ffff` |
| 300 | `1e01851d` |

With every reward paid to one transparent address and no spends or fees,
expected issued supply and that address's lifetime received total are:

| Height | Base units | CATE |
| ---: | ---: | ---: |
| 4,200 | 2,100,000,000,000,000 | 21,000,000 |
| 4,201 | 2,100,500,000,000,000 | 21,005,000 |

The height 4,201 value exceeds Zcash's `MAX_MONEY` while remaining below the
Catecash height-based issuance bound. These arithmetic vectors do not prove
disk persistence, index rebuild, wallet display, or reorganization behavior.

## Exact aggregate interface contract

All base-unit totals are integers; callers must not pass them through a JSON
floating-point number. Zebra's Catecash `getblockchaininfo` pool
`chainValueZat` and address-balance RPC totals use base-10 strings. Its pool
`chainValue` is also an exact decimal CATE string with eight fractional
digits. Zaino preserves the exact base-unit strings in its JSON responses.
Zingolib's Catecash balance JSON uses base-10 base-unit strings for each
present `confirmed_*_balance`, `unconfirmed_*_balance`, and
`total_*_balance` field; an unavailable pool is `null`.

For compact gRPC `GetTaddressBalance` and the streamed address method,
Catecash requires `Balance.valueZatWide` even when the total is zero. Its
`fixed64 low` and `fixed64 high` fields encode `high × 2^64 + low`.
`Balance.valueZat` is not authoritative on Catecash; Zaino writes zero there.
Zcash retains its existing narrow field and JSON behavior. This is the
implemented interface contract, pending cross-component live-chain tests.

## Reproducible local checks

Run from this Zebra checkout with the adjacent Zaino and Zingolib checkouts
at the revisions under review. On 2026-11-23 these focused tests passed:

```sh
cargo test -p zebra-chain catecash_digishield_one_block_boundaries --lib --offline
cargo test -p zebra-chain reward_pool_crosses_zcash_cap_after_block_4200 --lib --offline
cargo test -p zebra-chain catecash_devnet_reward_has_no_halving_or_allocations --lib --offline
cargo test -p zebra-chain catecash_mainnet_candidate_genesis_has_valid_proof_and_no_premine --lib --offline
cargo test -p zebra-state catecash_wide_pool_and_address_records_roundtrip --lib --offline
cargo test -p zebra-state received_remains_exact_past_u64_max --lib --offline
cargo test -p zebra-state catecash_devnet_retargets_from_the_previous_two_blocks --lib --offline
cargo test -p zebra-rpc catecash_balance_round_trips_all_128_bits --lib --offline
```

From `../zaino` and `../zingolib`, respectively:

```sh
cargo test -p zaino-state catecash_balance_round_trips_all_128_bits --lib --offline
cargo test -p zingolib tests_that_catecash_aggregate_crosses_legacy_money_cap --lib --offline
```

The hosted devnet's read-only boundary watcher and three-node comparison are
documented in [the mining log](devnet-mining.md). They have not reached the
4,201 boundary. These checks are scoped evidence and
do not constitute independent consensus review.

## Review and evidence still required

1. Independently reproduce both genesis blocks and the height-one and
   DigiShield target vectors from serialized blocks. Confirm Equihash
   parameters, transaction versions, fee and shielded validation, and the
   network-specific signing domain against a second implementation.
2. Mine a fresh chain through height 4,201 with repeated payouts and compare
   Zebra, Zaino, and wallet exact totals. The deterministic address-index
   save/reopen and replay test already passed, as did an isolated state reorg
   fixture. Repeat with deterministic fixtures at signed 64-bit and JSON
   safe-integer boundaries.
3. Demonstrate mature coinbase spending, transparent and shielded pool
   transfers, fee collection, wrong-network rejection, and wallet recovery
   on the release candidate. Publish commands, hashes, and logs.
4. Decide whether the mainnet initial target, proof-of-work limit, branch ID,
   transaction and address domains, and NU7-at-height-one rule are final.
   Review the policy for future upgrades; no versionbits deployment is
   activated by this candidate.

No checkbox in the launch checklist is satisfied solely by this document.
