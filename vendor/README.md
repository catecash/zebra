# Local librustzcash patches

`zcash_protocol` 0.11.0 and `zcash_primitives` 0.31.0 are copies of the
published crates, retained under their MIT/Apache-2.0 licenses. Cargo's
`[patch.crates-io]` uses these copies for this Zebra fork.

The patches add Catecash's launch branch ID `0xEA3BEEB4` as a distinct
transaction signing domain with NU7 transaction formats, shielded pool
capabilities, and Orchard circuit revision. The ID is the first four bytes
of SHA-256 of `Catecash mainnet launch consensus branch v1`. Upstream Zcash
branch IDs and behavior are retained.

Wallet and lightwallet clients need corresponding support before they can
create Catecash transactions. When updating either upstream crate, reapply
the Catecash variant and run the node's cross-network branch ID tests.
