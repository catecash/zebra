# Catecash static checkpoint promotion

Each Catecash network has its own genesis and block history. A checkpoint is a
height and hash on one network only; never reuse a hash on another network.
Checkpoint lists live under `zebra-chain/src/parameters/network/` and use
400-block intervals, matching Zebra's maximum checkpoint height gap.

## Verified candidates (2026-11-27 UTC)

| Network | Last checkpoint | Hash | Independent agreement |
| --- | ---: | --- | --- |
| Devnet | 12,400 | `062a494402a94a9ff3654b5599149230df2bbc929f29213402e8a1505b3b83b2` | Miner, peer, and third hosted verifier |
| Testnet | 11,600 | `01742b902bdfe63d4dd5cf79611d1895315287c493abffc3446120d7bc103ff6` | Both hosted testnet nodes |
| Mainnet candidate | 10,000 | `0036fdb6089d913ec44451ee9316bd5e12e6f20ebad29e6a8c300ee0d30732fd` | All five hosted RPC nodes |

At selection, devnet was at height 13,577, testnet at 12,925, and mainnet at
11,358. Every added 400-block hash and each network genesis agreed across the
nodes listed above. The new endpoints are more than 1,000 blocks behind the
observed tips. The earlier mainnet height-8,000 deployment is described in the
workspace incident record.

## Promotion gates

1. Build one ARM container from the source containing these lists. Keep keys,
   RPC cookies, and node configuration out of the image and build context.
   Run the focused `zebra-chain` checkpoint tests and record the image digest.
2. Back up the live devnet configuration, service units, and state. Move the
   devnet verifier first, then peer, then miner to the container, one at a
   time. Preserve their separate state and RPC cookie directories as mounts.
   Confirm each reports `catecash-devnet`, the expected genesis and height
   12,400 hash, logs checkpoint validation, and resumes sync. Confirm a new
   block and agreement between the miner and verifier before continuing.
3. Back up both testnet configurations, service units, and state. Move the
   first testnet node, then the second, one at a time. Confirm each reports
   `catecash-testnet`, the expected genesis and height 11,600 hash, logs
   checkpoint validation, and resumes sync. Confirm both agree on a new block
   and the node 2 Zaino gateway remains ready before continuing.
4. Confirm the five hosted mainnet RPC nodes still agree on height 10,000 and
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

## Rollout record (2026-11-27 UTC)

The `checkpoints-20261127` images were built from the updated checkpoint lists
with Rust 1.91.0 and `internal-miner`. The ARM image ID is
`sha256:43b6be2da11d124dc539a612f79c970ae33504c1e67a731bf40a578f69ad1fc5`;
its transferred gzip archive SHA-256 is
`219304d5ba11daa5fb28e366d861fc51c1de70e9fe1e2bd5a551a4f18f76c138`.
The x86 image ID is
`sha256:a71b0b84f8f184239bc8d28f3840b8cbb6ae6cef828597ad2e6a35017bc52b96`;
its archive SHA-256 is
`4f0c1226a3496482d256d53129ded5bd29eb0f7c8e44a7a648ba220eb302ebf7`.
Archive checksums and image architectures were verified on each destination.
The two focused `zebra-chain` checkpoint tests passed before deployment.

- Devnet: verifier, peer, then miner were moved to the new ARM image with
  separate stopped-state backups. All three logged checkpoint validation at
  12,400 and agreed at height 13,601 on
  `053ca5fb4f5f780c8dd4a2a627188727457101dc8c2300025d9d1b18787bc08a`.
- Testnet: node 1, then node 2 were moved to the ARM image. Both logged
  checkpoint validation at 11,600 and agreed at height 12,954 on
  `00d6f19f076ba1399760af8cb20695ad73dd422aca4f09eaa4b903e929126873`.
  Node 2's dependent Zaino service was restarted and reported
  `chain_state: Ready, grpc: Ready`. Public `GetLightdInfo` returned the
  `catecash-testnet` identity.
- Mainnet: miners 1 and 2, miners 3 and 4, then the wallet gateway were
  moved one at a time. The three public seeds followed one at a time. All
  eight containers logged checkpoint validation at 10,000. The five RPC
  nodes agreed at height 11,376 on
  `000ddf972bd403e0ed3c46d7c6b7718b37e4b85ba70c9180481760e3527e9f16`;
  each reached all three public seeds over TCP. The gateway's dependent Zaino
  service returned to `chain_state: Ready, grpc: Ready`, and public
  `GetLightdInfo` returned the `catecash-mainnet` identity.

Each host retains its previous unit or Compose file, live TOML, and stopped
state in the restricted `/var/backups/catecash/checkpoints-2026-11-27/`
directory. These archives include RPC cookie material and must remain on
their hosts. The prior container images remain available for rollback.
