# Catecash hosted testnet node rehearsal

**Date:** 2026-11-23–24 UTC. **Status:** two private hosted nodes mining and agreeing, with public TLS wallet gateway and explorer previews. Public P2P remains closed.

| Role | Hetzner server | Public IPv4 | Private IPv4 | P2P |
| --- | --- | --- | --- | --- |
| Seed 1 and rehearsal miner | `catecash-testnet-1` (`177391612`) | `91.99.104.49` | `10.57.1.1` | Private `34353` |
| Seed 2 | `catecash-testnet-2` (`177391614`) | `91.99.162.121` | `10.57.1.2` | Private `34353` |

Both are Ubuntu 24.04 ARM `cax21` servers in `nbg1`, on the dedicated `catecash-testnet` network (`10.57.0.0/16`). The node firewall allows inbound SSH with a key and blocks public P2P and RPC. A separate firewall allows HTTP certificate renewal and TLS wallet traffic on node 2. The private Zebra listeners use [node 1](../../configs/catecash-testnet-hosted-1.toml) and [node 2](../../configs/catecash-testnet-hosted-2.toml) configurations. RPC stays on `127.0.0.1:34352`. The [systemd unit](../../configs/catecash-testnet-hosted.service) runs as the dedicated `catecash` account and starts at boot.

The `testnet.catecash.com` A record resolves to `91.99.104.49`. Public P2P is still blocked by the firewall. `91.99.162.121` is the second node's public IP and the DNS-only wallet gateway origin. The explorer runs on a separate VM at `2.28.235.78`, behind a proxied Cloudflare A record.

## Verification

- [x] Both independently built ARM release binaries have SHA-256 `d078024a3e6721c6f5f119c9ad5c8ef4a6d26af74a9dee9007bd7f407628a06e`. Both systemd services start and survive a restart.
- [x] Both report `catecash-testnet` and genesis `15999c77e978a180c2f3fe2ec8478f8123e5c923f20bedaaf1b156fc38c4dda2` over loopback RPC.
- [x] Both connect over private P2P and agreed at height 21 on tip `0be624f8903271e45f032230f1aebdd96ed3821d0b8891d11758e362499483ea`. Each was restarted with its state preserved; node 2 had one peer after restart.
- [x] External TCP probes found SSH port 22 open and P2P `34353` and RPC `34352` closed or filtered on both public IPs.

The internal miner produced its first blocks using an unspendable public test-vector address. On 2026-11-24 UTC, its payout was changed in `/etc/catecash/testnet.toml` to the controlled address `EMDhJhRcLFL7ccnj53pbSnyB3eQRyW9dcH4`, and only `catecash-testnet.service` was restarted. The state directory and genesis were retained. The local key and an owner-only copy are under `~/.catecash/testnet/`; neither is on the miner. The explorer confirmed a 5,000 CATE coinbase output at height 351 to the new address and the unchanged genesis `15999c77e978a180c2f3fe2ec8478f8123e5c923f20bedaaf1b156fc38c4dda2`. The first controlled reward can be spent in block 591 or later. A seed wallet at `~/.catecash/testnet/transfer-staging/` has the Ironwood receiver `catet1xh2lc70dr5dumddc40rpdlfz2p5s5raw874ptlkak8xcjd9shmzs6hdmz4u0e7yjq66dgcfu8ul4095jwg8ayyw3sxv98cefscpdvzl9` for sweeping. An independent offline backup, mature spend, and public release gates remain open in the [deployment plan](testnet-deployment.md).

On 2026-11-24 UTC, the external payout controller in
[`utilities/rotate-miner.py`](../../../../utilities/rotate-miner.py) made two
subsequent live assignments from a new 2,000-key testnet pool. The first
address `EMN7pgS8qXR55YkXZ4ZU15zq2xBSfcYXRUx` had received
`1000000000000` base units by height 677. The second
`EMaRvLWfsPCiL2jtUySSMqSYY6xhAff7u2J` had received
`1500000000000` by height 681. Both testnet nodes agreed on block 681 hash
`03b8c70c59d43da43a49afd032397a18893a65e3175184d085050b35f0b5a369`.
Node 1 now has a daily external rotation timer. Node 2 was subsequently enabled
as a second miner with a disjoint schedule slot and still hosts the wallet
gateway. Its timer is disabled so wallet gateway restarts can be coordinated.
See the [promotion record](../../../../utilities/PROMOTION.md)
for the reproducible procedure and the separate mainnet staging status.
The installed timer's service was also run once manually: it selected a third
address, `EMBoBHriu5TF2LoSAFt54VtF1WDXb4JJCsT`, which received
`1500000000000` base units by height 702. Both nodes agreed on block 700 hash
`0795751267769838838764fa420d500b605942bf63fdaf4def143f22ac0f1737`.
Node 2 paid `EMPki5eK59iG4TBJYTnA53xJAH6UvD6mJFE` at block 731 and
later paid `EMCVq9p34coXbjXD42Zzoonxb21XE141qqX` at block 736; node 1
accepted both block hashes. A restart of node 2's Zebra service stopped its
dependent Zaino service. Zaino was restarted and reported gRPC ready. Future
manual rotations on node 2 must verify that gateway after Zebra restarts.

## Gateway and explorer checks

- Zaino's ARM release binary has SHA-256 `bfb55b157b4f622681672d344cd863b9c0f42a329f58b193314a0d9909a33e19`. Its systemd service runs on node 2 with a valid certificate for `wallet-testnet.catecash.com`. A remote Zingolib client reported `catecash-testnet`, branch `b80dae22`, and height 103, then saved a wallet at height 105. After a Zaino restart, the client reported height 106.
- CateScan runs on dedicated server `catecash-testnet-explorer` (`177395906`) through a restricted SSH tunnel to node 2's loopback RPC. Its public HTTPS page, `/api/status`, and `/api/blocks?size=10` each returned HTTP 200 with valid TLS. The status API reported the exact genesis above.
- At height 106, both Zebra nodes and CateScan reported tip `03e9a0eb1f2d83a358df12f2d58c0c0db62bf22316f32f8351654ad636a5b8a4`.
- A later check at height 116 found both nodes, the public CateScan API, and the public Zaino gateway at the same height. The nodes and explorer reported tip `0183bfa64976f33e2a0abeccd74b67e45d74b1bab992412285a47a7f7717a584`. The miner remained enabled.

The [wallet gateway notes](../../../zaino/docs/example_configs/catecash-testnet-hetzner.md) and [explorer deployment notes](../../../catescan/deploy/testnet.md) hold service configuration and recovery commands. Wallet app builds, controlled-key funding, mature spends, reorganization, and public P2P remain release work.
