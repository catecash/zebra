# Catecash static checkpoint promotion

Each Catecash network has its own genesis and block history. A checkpoint is a
height and hash on one network only; never reuse a hash on another network.
Checkpoint lists live under `zebra-chain/src/parameters/network/` and use
400-block intervals, matching Zebra's maximum checkpoint height gap.

## Verified candidates (2026-11-27 UTC)

| Network | Last checkpoint | Hash | Independent agreement |
| --- | ---: | --- | --- |
| Devnet | 10,400 | `037da4e61a511f6799c20170e361675a618cf1057dd6f44bde8fb759f21e520e` | Miner and third hosted verifier |
| Testnet | 10,000 | `0002e294701d6f00864581e4dcbece97c5f901edbd6b64c4344435d28cae61de` | Both hosted testnet nodes |
| Mainnet candidate | 8,000 | `002177e57574550bb0de6afedc895ac89e64da0f1d14f65e14e4edc676820673` | Five hosted RPC nodes; already deployed |

At selection, devnet was above height 11,769 and testnet above height 11,064.
All listed devnet and testnet hashes, including genesis, were compared on the
two corresponding nodes. The mainnet checkpoint was independently verified
and deployed during the 2026-11-26 incident; see the workspace incident record.

## Promotion gates

1. Build one ARM container from the source containing these lists. Keep keys,
   RPC cookies, and node configuration out of the image and build context.
   Run the focused `zebra-chain` checkpoint tests and record the image digest.
2. Back up the live devnet configuration, service units, and state. Move the
   devnet verifier first, then peer, then miner to the container, one at a
   time. Preserve their separate state and RPC cookie directories as mounts.
   Confirm each reports `catecash-devnet`, the expected genesis and height
   10,400 hash, logs checkpoint validation, and resumes sync. Confirm a new
   block and agreement between the miner and verifier before continuing.
3. Back up both testnet configurations, service units, and state. Move the
   first testnet node, then the second, one at a time. Confirm each reports
   `catecash-testnet`, the expected genesis and height 10,000 hash, logs
   checkpoint validation, and resumes sync. Confirm both agree on a new block
   and the node 2 Zaino gateway remains ready before continuing.
4. Confirm the five hosted mainnet RPC nodes still agree on height 8,000 and
   report checkpoint validation. If promoting a new image on mainnet, back up
   each host and roll it out one at a time, checking chain identity, tip
   agreement, wallet gateway, and public seed links after each host.

Do not advance to the next network if a hash differs, a node fails to resume,
or the independently observed tips disagree. Restore the prior image and unit
with the preserved state; do not replace the state with an unrelated snapshot.

## Rollout record (2026-11-27 UTC)

The ARM image `catecash-zebra:dev-test-checkpoints-20261126-arm64` was built
from this source with `internal-miner`. Its image ID is
`34ea5ea5fe570cfd7de2258ca185fa0ef04106872479f88cce9fb09fce6821ef`;
the transferred gzip archive SHA-256 is
`53bd61f6b79f1f2affd69937381486b8041ff531ebdb5b1d36389259f07b4543`.
The archive checksum and image ID were checked on all three destination hosts.

- Devnet: the third verifier, peer, and miner were moved one at a time to
  Podman units in `deploy/catecash-static-checkpoints/`. Each preserved its
  config and state mount. A short initial peer handshake delay and later sync
  lag resolved. All three agreed at height 11,818 on
  `012878b8b172248f25fde548c55c51514942d8ff6ea2e6d0f25feeadb1828109`.
- Testnet: Podman was installed on both hosts. Node 1 moved first and agreed
  with node 2 on block 11,098. Node 2 moved next; its dependent Zaino service
  needed a restart after systemd waited for shutdown. Both nodes then agreed
  at height 11,107 on
  `01d331f23b82cbe6d26526bb1475ff8b91cccbcc4bc46c6ca69f40ffc7d37f20`.
  The public wallet `GetLightdInfo` gRPC call succeeded after the restart.
- Mainnet: no new checkpoint or image was needed. All five hosted RPC nodes
  independently returned the existing height-8,000 hash above and the expected
  mainnet genesis. The deployed checkpoint image was unchanged.

The original units, TOML files, and stopped-state archives are restricted to
`/var/backups/catecash/checkpoint-2026-11-26/` on each devnet and testnet host.
The state archives include RPC cookie material and must stay on those hosts.
