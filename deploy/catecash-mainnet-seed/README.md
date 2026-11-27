# Catecash mainnet public bootstrap nodes

`compose.yaml` runs a non-mining Zebra container with host networking. It
publishes P2P on TCP 34333, keeps RPC disabled, reads its TOML from
`/etc/catecash/mainnet-seed.toml`, and stores chain state and recently useful
peer addresses in `/var/lib/catecash-mainnet-seed`. The peer cache is a list of
candidates for later review; an address appearing there is not evidence that
the node is honest.

Use `mainnet-seed.toml` on the dedicated seed VM at `2.31.179.185`. Use
`mainnet-seed-hosted-3.toml` and `mainnet-seed-hosted-4.toml` for the separate
seed containers beside miners 3 and 4. Those miners keep their own loopback
listeners and payout rotation. The seed containers have no mining section.

The deployed image is built from the checkpoint source using the pinned
base images in `docker/Dockerfile`:

```sh
docker buildx build --progress=plain --load --target runtime \
  --build-arg FEATURES=internal-miner \
  -f docker/Dockerfile -t catecash-zebra:checkpoints-20261127-amd64 .
```

Copy only the image and the matching nonsecret TOML and Compose files to each
host. Do not include local `.env` files, wallet files, or keys in the build
context or image. The Hetzner API key is stored on the operator machine in the
workspace root `.env` as `HETZNER_API_KEY`; it is not needed by a seed host.

On each host, give `/var/lib/catecash-mainnet-seed` owner UID/GID `10001`,
mount the TOML at the path used by Compose, and run:

```sh
docker compose -f /opt/catecash/seed/compose.yaml up -d
```

Allow public TCP 34333 in the cloud and host firewalls, while keeping Zebra
RPC closed. Verify each public IP with a fresh Catecash mainnet P2P handshake
and matching chain identity before including it in the default bootstrap list.
The peer cache is under
`/var/lib/catecash-mainnet-seed/zebra/network/*.peers` on the host.
