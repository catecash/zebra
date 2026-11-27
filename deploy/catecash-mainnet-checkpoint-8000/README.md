# Hosted Catecash mainnet checkpoint 8000

`docker.service` and `podman.service` are the deployed systemd units for the
five hosted RPC nodes. Install the appropriate file as
`/etc/systemd/system/catecash-mainnet-candidate.service` after loading the
matching architecture image. The units mount the existing TOML read-only and
chain state read-write; the image and build context contain no node secrets.

The Docker unit uses the `checkpoint-8000-20261126-amd64` image tag on miners
3 and 4. The Podman unit uses `checkpoint-8000-20261126` on miners 1 and 2 and
the wallet gateway. The seed image and Compose file are in
`../catecash-mainnet-seed/`.

The incident evidence, post-event state backups, image checksums, and rollout
verification are in the workspace's `artifacts/mainnet-incident-2026-11-26/`
directory. The height-8000 checkpoint is below Zebra's 1,000-block maximum
reorganization window at the deployment tip. It does not fix miner
concentration or propagation delays at later heights.
