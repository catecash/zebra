# Catecash candidate consensus and genesis review — 2026-11-23 PST

**Disposition: candidate remains NO-GO for mining and public launch.** This
review checked the current implementation and added independent byte-level
genesis verification. It is not a second-implementation Equihash validation,
an external security review, or approval of a release.

## Verified in this pass

- The standalone standard-library verifier
  [`utilities/verify-mainnet-genesis.py`](../../../../utilities/verify-mainnet-genesis.py)
  parses the 1,607 serialized genesis bytes without Zebra. It recomputes the
  header hash `050bf7476db81684aeb38337fc931bd8bc4480fde43fb0a14fef172f85c5c323`,
  checks compact target `200fffff`, timestamp `1795408991`, the single
  coinbase Merkle root, and its only output: zero-value `OP_RETURN`. Zebra's
  focused genesis test also validates the Equihash solution.
- The first mineable block uses the inherited early-chain threshold path,
  giving compact target `200d70a2` from the mainnet candidate genesis;
  DigiShield begins at height 2. A focused state test asserts this exact
  height-one target. A separate standard-library
  [`utilities/verify-mainnet-consensus-vectors.py`](../../../../utilities/verify-mainnet-consensus-vectors.py)
  recomputes it, the four DigiShield examples, and issuance at heights 4,200
  and 4,201. Another reviewer should validate the rule choices before freeze.
- The candidate public pool has 202,000 distinct transparent addresses. The
  standalone [`utilities/verify-mainnet-address-pool.py`](../../../../utilities/verify-mainnet-address-pool.py)
  recomputed every Base58Check address from its compressed public key and
  prefix `0684`, checked each compressed key lies on secp256k1, then checked
  that the shuffled schedule is an exact
  permutation and includes both staged miner addresses. Schedule SHA-256:
  `0b9faae612cf94630c9170ce2547a2e05f3e6094442aa2a1457eb5e02182cf72`.
  Both hosted miners carry that same schedule hash and retain their distinct
  first two destinations with `internal_miner = false`.
  This is public-key/address verification; private-key import from the offline
  vaults remains a separate wallet recovery test.
- The focused `catecash` tests in `zebra-chain` (17) and `zebra-state` (7)
  passed after fixing the issue below. The network-specific branch-ID test in
  `zebra-consensus` passed after replacing a stale configured-testnet fixture
  with the actual Catecash network variants.

## Defect found and corrected

The candidate mainnet transparent-including R2 Unified Address HRP was
`catetu`, identical to the private testnet shielded-only R2 HRP. A mainnet
R2 address therefore decoded as CatecashTestnet. The still-provisional
mainnet transparent-including R2 HRP is now `cateutr`; the mainnet contract and
vendored address library agree, and a regression test checks uniqueness of
all nine Catecash Unified Address HRPs and mainnet round trips. Existing
transparent payout prefixes and all 202,000 payout addresses are unchanged.
The ARM node and gateway binaries were rebuilt from the corrected source and
deployed to the hosted candidate while mining remained disabled. Their hashes
and source archives are in the
[candidate artifact manifest](../../../../artifacts/mainnet-candidate-2026-11-23/README.md).

## Remaining decisions and release gates

1. Have a separate reviewer validate Equihash proof, genesis input choices,
   height-one target, DigiShield arithmetic, branch IDs, address/key domains,
   subsidy/fee rules, coinbase maturity, and cross-network rejection against
   the published specification. The initial target and proof-of-work limit are
   still described as provisional; this review does not silently freeze them.
2. Complete fresh-chain tests for fees, maturity spends, pool transfers,
   reorganization, and exact Zebra/Zaino/wallet totals above the narrow
   accounting limits. The private hosted candidate remains at genesis.
3. Tag an exact reviewed source tree and build matching ARM node and indexer
   artifacts with a recorded toolchain, lockfiles, hashes, and reviewer
   approval. The current checkouts have uncommitted changes, so a binary hash
   alone is not a reproducible release identity.
4. The two encrypted vault batches have matching readbacks on two separate
   USBs. The recorded backup procedure says their decryption secrets are in
   the operator Keychain and still require a separate offline backup before
   funding; no secret was read during this review.

Mining remains disabled on both hosted miners. See the
[launch checklist](mainnet-checklist.md) for the full go/no-go record.
