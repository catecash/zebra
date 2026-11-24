# Private mainnet candidate hosts

**Status:** private candidate hosts provisioned. The consensus contract remains
`candidate-not-for-public-launch`. The hosts use a separate private network,
state directory, SSH key, and firewall. Private mining began on 2026-11-24 UTC
by direct operator instruction. Public P2P, DNS seeds, and public announcements
stay disabled. A candidate-only wallet gRPC endpoint
is available over HTTPS on the separate gateway host.

The intended mining payouts will use only the 202,000 public addresses in
[`wallet-vault`](../../../../wallet-vault/README.md), with addresses selected from
a shuffled schedule and rotated periodically across miners. The encrypted spending-key vaults were
previously copied and read back from two separate USBs, as recorded in
[`BACKUP-VERIFICATION.md`](../../../../wallet-vault/BACKUP-VERIFICATION.md).
The USBs remain offline; the public manifests suffice for payout selection.
The two vault encryption keys are present in the operator's login Keychain.
The Keychain secrets themselves still need an independent offline backup.
The separately generated address `CaEHKEWhoxfay18C4RvRjLsGnSEaM6Wg5eN`
is not in that pool and is not a mining destination. Its private WIF remains
only in `~/.catecash/mainnet-candidate/miner-key.json` on the operator machine.
No private mining key has been copied to either server. A public
[`payout-schedule.txt`](../../../../wallet-vault/payout-schedule.txt) gives
the 202,000 addresses a cryptographically shuffled, non-repeating order.
The external script in `utilities/` stops Zebra and changes `miner_address`
from this list before restarting. Each host has a separate schedule slot;
several blocks may pay one address before the next daily rotation. The public
schedule and script were staged on both candidate hosts: node 1 used
`CaHeyDdzk5RkupVx8eY3oCHYebeDxEGEjWs` and node 2 used
`CaLk8SyBocSrPNhwQz1oXnscQ2xPP1STP1T`. Both initially restarted at genesis
with `internal_miner = false`. On 2026-11-24 UTC, their separate shuffled
slots moved to positions 3 and 4, `internal_miner = true`, and both daily
external rotation timers became active. Both miners produced accepted blocks;
all three hosts agreed at height 17. Independent review, a reviewed release,
and separate offline backup of the Keychain encryption secrets remain open.
The live rotation rehearsal used a separate 2,000-key testnet pool. Both testnet miners paid distinct scheduled addresses. Node 1 has a daily
external rotation timer; node 2 rotates manually while it hosts testnet Zaino. The promotion record is in
[`utilities/PROMOTION.md`](../../../../utilities/PROMOTION.md).

| Role | Host | Private address | Public P2P | RPC |
| --- | --- | --- | --- | --- |
| Candidate node 1 | `catecash-mainnet-candidate-1` (`2.31.189.104`) | `10.58.1.1` | Closed | `127.0.0.1:34332` with cookie auth |
| Candidate node 2 | `catecash-mainnet-candidate-2` (`178.104.149.171`) | `10.58.1.2` | Closed | `127.0.0.1:34332` with cookie auth |
| Wallet gateway | `catecash-mainnet-candidate-gateway` (`178.105.239.117`) | `10.58.1.3` | Closed | Local Zebra at `127.0.0.1:34332`; Zaino at `127.0.0.1:34337` |

All three hosts use Ubuntu 24.04 ARM in Hetzner `nbg1`. The private network is
`catecash-mainnet-candidate` (`10.58.0.0/16`, subnet `10.58.1.0/24`).
The miners retain the SSH-only public cloud firewall. The gateway has a
separate cloud firewall admitting TCP 22 and 443. Hetzner cloud firewalls do
not filter private Cloud Network traffic, so UFW on each host admits private
P2P port `34333` only from the other two named hosts. The earlier private-RPC
UFW allowances were removed. Zebra RPC remains cookie-authenticated on
`127.0.0.1:34332` and is never proxied to the public. Mining runs on both
miner hosts; the gateway has no mining address or rotation timer.

`mainnet.catecash.com` now resolves to miner node 1, but public P2P remains
closed. `wallet.catecash.com` is a DNS-only A record to the gateway. Caddy
serves its valid TLS certificate on TCP 443 and proxies wallet gRPC `Get*`
reads and `SendTransaction` to local Zaino. Other paths, including
`SubmitBlock`, return HTTP 404. Public probes found miner TCP 443, P2P and
RPC closed; gateway TCP 80, P2P, RPC and raw Zaino closed, with only SSH and
HTTPS open. The iOS app is not yet released for this endpoint. `scan.catecash.com`
now routes to CateScan on the separate gateway. Details and repeatable checks are in
[`utilities/MAINNET-PRIVATE-NETWORK.md`](../../../../utilities/MAINNET-PRIVATE-NETWORK.md).

The node configs are
[`hosted-1`](../../configs/catecash-mainnet-candidate-hosted-1.toml),
[`hosted-2`](../../configs/catecash-mainnet-candidate-hosted-2.toml), and the
non-mining [`gateway`](../../configs/catecash-mainnet-candidate-gateway.toml). The
[`systemd unit`](../../configs/catecash-mainnet-candidate-hosted.service)
runs a dedicated `catecash` account. Deploy only a build whose source and
genesis hash match the reviewed candidate, then verify both nodes report
`catecash-mainnet`, genesis
`050bf7476db81684aeb38337fc931bd8bc4480fde43fb0a14fef172f85c5c323`.
The deployed Zebra ARM binary is the same on all three hosts, SHA-256
`742d7f0a484dd0514bec52fd7970da82592d6e1642395fcbc88a32d971e80f6e`.
It is a candidate build, not a reviewed release. The candidate guard accepts
P2P listeners only at the three named private IPs on port `34333` or loopback;
it rejects public advertising and non-loopback RPC. On 2026-11-24 UTC, all
three services were active. Each local RPC reported `catecash-mainnet`, height
0, the genesis hash above, and connections to both other private peers. After
mining was enabled, all three agreed at height 17, with both miner payout
addresses present in accepted coinbase blocks.

A private CateScan instance was staged on node 1 at `127.0.0.1:3001`;
its service remains inactive. Its
[`systemd unit`](../../../catescan/deploy/catescan-mainnet-candidate.service)
requires the exact chain and genesis before serving chain data. Its local
`/api/status` returned height 0 and the expected genesis on 2026-11-24 UTC.
It has no public proxy or DNS record. The iOS app is not connected to the
candidate.

CateScan now runs on the separate gateway as `catescan-mainnet-gateway.service`
at `127.0.0.1:3001`, using that host's loopback Zebra RPC and cookie. Its
status and block-list APIs verified the expected genesis and live blocks at
height 40. Caddy has a separate `scan.catecash.com` HTTPS route. The origin
certificate was issued on 2026-11-24 UTC; after the Cloudflare proxy was
restored, the public page showed a connected node and live blocks through
height 57. Renewal steps are in the [explorer deployment](../../../catescan/deploy/mainnet.md).

A private Zaino instance now runs on the dedicated gateway host at
`127.0.0.1:34337` with [candidate config](../../../zaino/docs/example_configs/catecash-mainnet-candidate.toml)
and a [systemd unit](../../../zaino/docs/example_configs/catecash-mainnet-candidate-hosted.service).
It uses the local Zebra RPC cookie, selected `CatecashMainnet`, indexed height
0, and reported chain and gRPC ready on 2026-11-24 UTC. Its ARM binary SHA-256
is `9660bcb2e3c5aac7c1fbea2e222a25322c088a9f676615cb75b1ece142e54d6a`;
it is a rehearsal build, not a reviewed release. The earlier direct gRPC
`GetLightdInfo` probe on node 1 returned `chainName=catecash-mainnet` and
`blockHeight=0`; that tunnel was closed. Node 1 Zaino has since been stopped
and masked to prevent accidental restart on a miner. On the separate gateway,
Zebra reported `catecash-mainnet`, the expected genesis at height 0, and both
private peers. The new gateway passed a direct public TLS `GetLightdInfo` probe with
`chainName=catecash-mainnet` at height 0. A live zero-balance call initially
found that Zaino omitted the required `valueZatWide` field for CatecashMainnet.
After the corrected Zaino build was deployed, the same public HTTPS call
returned the explicit wide-zero field. No funded wallet transfer has been tested. A Zingolib CLI wallet has not scanned it: the local CLI
build requires its missing Nym proxy before opening a connected session. The
iOS app is not connected to this candidate. Wallet and iOS each need
wrong-chain rejection, functional checks, and reviewed releases before public
use.

The gateway host was provisioned on 2026-11-23 PST. Zebra's replacement ARM
binary is deployed on all three hosts, and the gateway can follow blocks from
both miner peers over the private network. The public wallet endpoint served
the candidate chain at height 0 before mining began and now follows live blocks.
The corrected Zaino balance response is
deployed and checked; no funded transfer has been tested.

The 2026-11-23 PST [consensus review](consensus-review-2026-11-23.md) found an
R2 Unified Address HRP collision with private testnet and changed the
candidate mainnet transparent-including R2 HRP to `cateutr`. Both ARM binaries
were rebuilt and deployed with that correction. All 202,000 transparent
payout addresses were rederived from their compressed public keys under the
unchanged `0684` prefix. The build source snapshot and binary hashes are
in [`artifacts/mainnet-candidate-2026-11-23`](../../../../artifacts/mainnet-candidate-2026-11-23/README.md).
This remains a candidate, pending independent approval and launch gates.
