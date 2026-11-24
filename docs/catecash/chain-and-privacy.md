# Independent chain and privacy implementation contract

**Status:** implementation specification, not an active Catecash chain. The
`catecash-devnet.toml` profile has a distinct development genesis but still
inherits most Zcash consensus rules. Its experimental reward rules do not make
it a launch-ready Catecash chain.

The one-block DigiShield arithmetic is implemented and unit-tested in
`zebra-chain/src/work/digishield.rs`. The separate Catecash devnet network type
uses it for contextual block validation and mining templates from height two;
height one uses the devnet proof-of-work limit. The devnet also enforces
240-block transparent coinbase maturity. A fresh two-node mining and restart
rehearsal passed; deliberate reorganization and live 240-block spend tests
remain. See the [network launch runbook](mainnet-runbook.md).
The chosen genesis coinbase message is encoded in a mined development genesis
block. A [mainnet candidate](mainnet-genesis-candidate.md) has also been mined
from provisional parameters; the public mainnet and testnet genesis blocks are
still undefined.

## Order of work

1. The local devnet has dedicated network types in the node and vendored
   address libraries, with provisional development encodings. Finalize these
   encodings and allocate distinct mainnet and testnet encodings. Verify
   cross-network rejection in mining, RPC, and wallet paths.
2. Keep Equihash proof of work and specify Catecash's difficulty schedule.
   Implement the chosen rules in miner templates, block verification, and the
   peer protocol. Make
   the subsidy, maturity, and amount limits network-specific at the same time.
3. Freeze the first-block consensus rules, then
   construct and publish the genesis block bytes, hash, timestamp, coinbase
   message, target, nonce, and a deterministic reproduction procedure.
4. Bind that exact genesis hash to each network's height-zero checkpoint,
   state database namespace, peer handshake, and block locator. Start a node
   with no public Zcash seed peers or checkpoint data.
5. Enable and test the chosen shielded pools and transaction versions from
   the first mineable Catecash block. Update the wallet and lightwallet
   protocol together with the node. Test proofs, signatures, nullifiers, note commitments, scanning,
   recovery, and reorganizations before calling privacy support complete.

Address formats are needed before publicly paying mining rewards or accepting
wallet transactions. **Genesis does not need a payout address.** Catecash's
fair-launch rule should give the height-zero coinbase no spendable outputs.
Zebra currently excludes genesis outputs from issued supply; the Catecash
genesis fixture must explicitly demonstrate zero spendable supply. The first
mineable block is height one, subject to the 5,000 CATE subsidy and 240-block
maturity rules in the development profile. Public-network rules are still open.

## Genesis acceptance checks

- The serialized block parses, has coinbase height zero, a null previous hash,
  a valid Merkle root, and no spendable outputs.
- Its hash equals the compiled network genesis hash and height-zero checkpoint.
  Mainnet, testnet, and development hashes are distinct from one another and
  from every Zcash genesis hash.
- Recomputing the hash and Equihash proof from the published bytes yields the
  published values; the target is within the Catecash proof-of-work limit.
- A fresh node commits only that genesis, rejects a Zcash genesis and a wrong
  Catecash genesis, and never imports Zcash checkpoints or chain state.
- Two fresh nodes connect through Catecash wire magic, agree on genesis, mine
  and validate blocks one and two, then synchronize and survive a restart.
- A node on each Zcash network rejects Catecash peers and blocks, and a
  Catecash node rejects Zcash peers and blocks.

Setting only `genesis_hash` in configured Testnet parameters fails these
checks. It does not supply serialized genesis bytes, a distinct consensus
identity, or a miner. Changing the coinbase or header also invalidates the
old Merkle root, block hash, and proof of work. A renamed Testnet with a new
magic value remains a Zcash consensus network.

The continuing 5,000 CATE subsidy still conflicts with Zebra's 21 million
coin `MAX_MONEY` balance constraint. The configured-network cumulative
issuance check is bypassed for this local profile so its reward rules can be
tested, but its long-term chain balances still cannot represent the issuance.
Changing the subsidy formula alone is insufficient: individual amounts, pool
balances, RPC amounts, storage encodings, and eventual integer overflow need a
specified long-term representation and boundary tests. Do not launch with an
issuance rule that nodes will reject after a finite number of blocks.

At eight decimal places, each 5,000 CATE reward is 500,000,000,000 base units.
The existing 21-million-coin cap is reached after 4,200 rewarded blocks; block
4,201 would exceed it if rewards remain in the tracked value pools. A signed
64-bit chain balance would itself overflow after 18,446,744 rewarded blocks.
At Zebra's maximum supported block height (`2,147,483,647`), the scheduled
supply is `1,073,741,823,500,000,000,000` base units. The chain value pools
and their disk format therefore need at least
128-bit accounting, with network-specific bounds that retain Zcash's current
rules. Test the 4,200/4,201 boundary, the signed 64-bit boundary, shielded and
transparent pool transfers, restart persistence, and the maximum block height
before removing this development-only limitation.
The [wide-supply migration contract](wide-supply-migration.md) maps the state,
address-index, RPC, indexer, and wallet changes required to remove this gate.

## Privacy compatibility contract

- Keep the existing Sapling and Orchard proof systems and their verifying
  parameters only where the *same* circuit and transaction rules apply.
  Changing an address prefix does not require a new zero-knowledge circuit;
  changing circuit statements or consensus semantics does. No privacy claim
  follows from reusing the libraries alone.
- Allocate independent encodings for transparent P2PKH/P2SH and TEX, Sapling
  addresses and viewing keys, Orchard receivers, and unified addresses and
  viewing keys. Define all mainnet, testnet, and development human-readable
  prefixes and Base58 version bytes together, and test collisions and
  round-trips in every parser, RPC, and wallet component.
- Use one explicit Catecash launch rule set for transaction versions and
  shielded pools from height one. The local devnet currently selects NU7 at
  height one with the distinct `0xEA3BEEB4` branch ID. The node parses this
  ID, uses it for signature hashing, and rejects Zcash's NU7 ID on the devnet.
  Wallet and shielded proof compatibility remain to be verified.
- Audit transaction signature hashing and consensus branch IDs before
  accepting cross-network transactions. Prefix changes alone do not provide
  replay protection because addresses are not the transaction signature
  domain. Define a chain-specific replay boundary compatible with the chosen
  transaction format and wallet libraries.
- Check shielded coinbase rules, note commitment trees, anchors, nullifier
  uniqueness, value balance, proof verification, viewing-key scanning, and
  wallet restore on the Catecash chain. Test transparent-to-shielded,
  shielded-to-shielded, and shielded-to-transparent transfers across an upgrade
  boundary and a reorganization.

`NetworkKind` and the vendored `zcash_address`, `zcash_protocol`, and
`zcash_keys` libraries distinguish Catecash devnet. Provisional address
encodings permit development-only payout addresses and loopback RPC. Public
RPC, wallet integration, and final mainnet/testnet formats remain future work.

## Decisions to freeze before generating genesis

- Final Equihash parameters, proof-of-work limit, and block timing.
- Exact DigiShield arithmetic and early-block targets, with test vectors.
- The exact shielded transaction rules at height one and their chain-specific
  replay domain.
- Final address version bytes and human-readable prefixes for every network.
- Launch timestamp. Record the UTC timestamp in the genesis reproduction
  procedure rather than using a build-time clock.

Zcash's genesis coinbase embeds the literal text
`Zcash0b9c4eef8b7cc417ee5001e3500984b6fea35683a7cac141a043c42064835d34`
in `source/zcash/src/chainparams.cpp`. The user chose Catecash's exact
50-byte ASCII message:

> I am the fortunate one. The High G, The One above.

`CATECASH_GENESIS_COINBASE_SCRIPT_SIG` encodes these bytes in the same
height-zero coinbase script shape that Zcash uses. A candidate version-one
coinbase transaction with a zero-value `OP_RETURN` output is mined into the
development genesis fixture at `zebra-chain/src/block/genesis/block-catecash-devnet-0.txt`.
Its timestamp is 2026-11-23 01:00:00 UTC, its compact target is `207fffff`,
and its hash is
`6b9c4d783359202111571c123309b2c0511d1bf9036f74dee4921f04e1f0a9cf`.
Run `cargo run -p zebra-chain --example catecash_genesis --features internal-miner`
to reproduce its serialized bytes and hash. This fixture is only for the local
development profile; no mainnet genesis has been selected.
Changing one character of the message changes the coinbase transaction,
Merkle root, genesis hash, and proof of work.

The [protocol plan](protocol-plan.md) records the economic targets. These
decisions are consensus inputs, not values that can be safely inferred from a
logo, ticker, or peer port.
