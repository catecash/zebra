# Catecash wide-supply migration

**Status:** Zebra state, its address index, RPC, and Zaino now carry wide
aggregate totals on the Catecash devnet. The wallet interface is being handled
separately. Integration and live-chain boundary evidence remain open; no public
network can rely on continuing 5,000 CATE rewards yet.

## Boundaries the implementation must cross

At eight decimal places, each reward is 500,000,000,000 base units. With the
reward left in tracked pools, the current `MAX_MONEY` cap is reached at block
4,200 and exceeded by block 4,201. A signed 64-bit balance can hold 18,446,744
rewards and overflows on the next. An unsigned 64-bit received total overflows
after 36,893,488 rewards. JavaScript `Number` consumers of JSON stop
representing every base unit exactly after 18,014 rewards, before either
integer overflows.

Catecash has no fixed monetary cap or scheduled halving: each valid mineable
block may issue its specified 5,000 CATE subsidy. The amount legitimately
issued by height `h` is therefore bounded by the subsidies permitted through
`h`, rather than by a lifetime coin cap.

Zebra's current `Height::MAX` is 2,147,483,647. A 5,000 CATE reward at every
mineable height schedules 1,073,741,823,500,000,000,000 base units through
that height. This is a representation bound for the current block-height type,
not a monetary cap or a scheduled halving. A future height extension would
require a reviewed network upgrade.

## Type and consensus rules

- Keep transaction outputs, input values, fees, signature hashing, and proof
  value representations at their existing wire widths. A single output remains
  bounded by its transaction format. Wider *aggregate* balances do not change
  transaction serialization.
- Store each chain pool and transparent address balance as a nonnegative
  128-bit base-unit integer. Apply signed block and transaction deltas with
  checked arithmetic. Reject a negative pool, arithmetic overflow, or issued
  supply above the subsidy permitted through the block's height. The maximum
  scheduled amount for the current height range is only a technical bound.
- Exclude NSM from issued supply, while validating its own nonnegative range.
  The Catecash devnet selector rejects a nonzero NSM reserve or change. Public
  Catecash rules must make this choice explicitly.
- Preserve Zcash's `MAX_MONEY` limit and existing state/RPC behavior on Zcash
  networks. A Catecash rule must be selected by an explicit network identity.

`zebra-chain/src/value_balance/wide.rs` implements checked aggregate pool
values. Its 112-byte encoding stores seven little-endian signed 128-bit
integers in the existing pool order. The state database now uses a tagged wide
record for Catecash and reads legacy narrow records for Zcash. The transparent
address index writes 40-byte wide records and reads legacy 16- and 24-byte
records; database format version 29.1 marks this change. `value_balance/network.rs`
selects wide pools for Catecash devnet and retains the `MAX_MONEY` check for
Zcash. Adding public Catecash variants requires an explicit match arm.
Block pool-change calculation takes the parent's narrow NSM reserve without
narrowing circulating pools.

## Integration order

1. Route Catecash chain-pool updates in both non-finalized and finalized state
   through the wide type. `ValueBalance<NegativeAllowed>` remains the narrow
   per-block delta. Carry the wide balance through `BlockInfo`, state requests,
   chain-tip responses, and miner templates. Apply the same checks when
   restoring state and reorganizing a chain.
2. Widen `balance_by_transparent_addr` balance and lifetime `received` values.
   The index now keeps its first-output location and uses checked arithmetic
   for balances and received totals. Exercise a miner that reuses one address
   beyond block 4,200. Test address-index save/reopen and replay separately
   with a deterministic fixture above the old limit.
3. Version the on-disk format. Catecash pool and `BlockInfo` records use a
   tagged wide schema; the reader accepts old narrow Zcash pool records.
   Transparent address records use a 16-byte balance and 16-byte received
   total on both networks, with legacy readers for existing Zcash records.
   Development state may be rebuilt from genesis.
4. Zebra and Zaino JSON endpoints emit Catecash aggregate base-unit totals as
   exact decimal strings. The existing compact gRPC balance method has an
   128-bit response field described below. Transaction amounts retain
   their existing wire widths. Coordinate client support with the separate
   wallet work.
5. Test 4,200/4,201 rewards, the JSON precision boundary, signed and unsigned
   64-bit boundaries, the maximum supported height, pool transfers, repeated
   payouts to one address, deterministic index persistence and replay, and a reorganization across
   those boundaries. Publish independent expected values.

## Compact gRPC balance encoding

Catecash starts with a new client deployment, so its `GetTaddressBalance`
response always includes `Balance.valueZatWide` at protobuf field 2. Standard
protobuf `fixed64 low` and `fixed64 high` halves encode the exact nonnegative
total as `(high << 64) | low`. The same encoding is used for the streamed
address method. No opt-in metadata or new RPC method is required. Catecash
clients must require field 2 even when the balance is zero. Field 1 is the
existing Zcash `int64` field and is not authoritative on Catecash.

The shared implementation retains field 1 for Zcash clients, preserving their
existing wire response. Test both Catecash servers at zero, `i64::MAX`,
`i64::MAX + 1`, `u64::MAX`, and the maximum supported height's scheduled
issuance, including restart and reorg. Use deterministic state fixtures above
the signed 64-bit boundary;
mining enough 5,000 CATE rewards would require 18,446,745 payouts to one
address. A shorter live-chain test covers the 4,200/4,201 boundary.

Mainnet and public testnet genesis, addresses, and wallet releases remain gated
on this end-to-end accounting path.
