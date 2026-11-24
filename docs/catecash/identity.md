# Catecash identity and network allocation

**Status:** development implementation. There is no public Catecash coin or chain.

| Item | Choice |
| --- | --- |
| Coin and project | Catecash |
| Ticker | CATE |
| Unit label | Catecash (CATE) |
| Source node | Zebra fork, with upstream attribution retained |
| Brand mark | Gold cat coin with a white C, in [`logo.png`](brand/logo.png) |
| Brand colors | Gold, cream, and white |

The name and ticker are display choices. They do not change consensus rules,
wallet address formats, or the network a node joins. The existing Zebra binary
and upstream defaults still connect to Zcash unless a custom configuration is
selected. CATE is a proposed ticker and has not been checked for global
uniqueness across exchanges or other chains.

## P2P identifiers and ports

These values are allocated for Catecash development. Each network needs its own
genesis block, consensus parameters, and wallet address format before use as a
real cryptocurrency network.

| Network | P2P magic (hex) | P2P TCP port | RPC TCP port | State |
| --- | --- | ---: | ---: | --- |
| Mainnet | `550c6ba9` | 34333 | 34332 | Frozen allocation; local-only candidate implementation |
| Testnet | `e79c55e7` | 34353 | 34352 | [Version 1 private-rehearsal contract](testnet-network-v1.json); implementation in progress |
| Local devnet | `6f8305bc` | 34343 | 34342 | Dedicated `CatecashDevnet` network type in [`catecash-devnet.toml`](../../configs/catecash-devnet.toml) |

The four-byte magic values are the first four bytes of SHA-256 of
`catecash:<network>:p2p:v1`. They differ from Zcash's mainnet, testnet, and
regtest magic values. The ports differ from Zcash's 8233 and 18233 P2P ports.
An operating system may still report a port conflict if another local process
uses a chosen port; these numbers are project allocations, not global
reservations.

The mainnet genesis timestamp is **2026-11-23 04:43:11 UTC**, frozen for the
candidate by the project owner. It is not a network activation time or public
launch date. A mined [candidate genesis fixture](mainnet-genesis-candidate.md)
uses a provisional initial target of `200fffff`. The candidate network's
[parameter contract](mainnet-network-candidate.json) records separate address
and key encodings, derivation coin type, and transaction branch ID. Those
remaining values need review before selection as final public consensus.

The local rehearsal config binds P2P to `127.0.0.1`, explicitly sets an empty
initial peer list, and disables the peer cache. A default `CatecashDevnet`
config instead resolves `devnet.catecash.com:34343` as its initial P2P peer.
Use a DNS-only A record to the devnet node; a web proxy cannot carry
Zebra's raw TCP peer protocol. The hosted P2P listener binds
`0.0.0.0:34343`, while RPC remains on loopback. The Hetzner Cloud firewall
allows inbound TCP 34343; a remote TCP check reached the seed on 2026-11-23.
Zebra's configured-network storage name includes
the magic, so it does not reuse public Zcash Testnet state. Its P2P decoder
rejects messages with a different network magic. These measures prevent this
profile from discovering or handshaking with public Zcash peers.

## Mainnet candidate identifiers

The [candidate contract](mainnet-network-candidate.json) uses a separate
transaction branch ID `5bc68392`, the first four SHA-256 bytes of
`catecash:mainnet:nu7:v1`. It differs from Zcash's NU7 branch ID and
Catecash devnet's `ea3beeb4`. A distinct branch ID gives transactions a
separate signing domain; the public value still needs review against the
[ZIP 200 globally unique identifier rule](https://zips.z.cash/zip-0200).

Candidate transparent address version bytes are `0684` for P2PKH and `0685`
for P2SH; a `0x42` repeated hash encodes as
`CaGoTLXeYqiyzhQVT2kgy1TwpTqP2okCHZX`. The remaining HRPs are under the
`cate` namespace and differ from Zcash and Catecash devnet encodings. The
candidate derivation coin type is `34333`, distinct from devnet's `3239`.
It is **not registered** in the
[SLIP-44 registry](https://github.com/satoshilabs/slips/blob/master/slip-0044.md)
and may need to change before wallet release. The local candidate configs
bind both P2P and RPC to loopback; Zebra rejects a public P2P or RPC bind for
this candidate.

## Remaining chain work

The local profile has dedicated `Network`, `NetworkKind`, and address-library
network variants, a distinct mined genesis block, a height-zero checkpoint,
fixed P2P magic and proof-of-work limit, and provisional devnet address
encodings. It no longer uses configured Testnet parameters. RPC binds only to
loopback. It still inherits Equihash and most Zcash validation rules.
It selects NU7 validation and Catecash's distinct `0xEA3BEEB4` transaction
branch ID from height one.
It has experimental Catecash reward rules but is not a complete independent
chain. Do not distribute devnet rewards as production CATE.

The devnet now applies one-block DigiShield retargeting from height two and
240-block transparent coinbase maturity. A two-node mining and restart
rehearsal passed; deliberate reorganization and live maturity-spend tests
remain. Before a Catecash network can launch, finish the eight-decimal
continuing reward's wide supply accounting, mainnet difficulty and maturity
rules, mainnet minimum-difficulty policy,
miner-signaling deployments if later needed, a fixed launch rule set, final
mainnet/testnet wallet address prefixes, and separate mainnet/testnet peer seeds. Then replace this profile with
separately tested mainnet and testnet configurations.

The provisional devnet encodings are recorded in
[`devnet-wallet-contract.json`](devnet-wallet-contract.json). They may change
before a public testnet and must not be used for real funds. The separate
[`CatecashTestnet` version 1 contract](testnet-network-v1.json) has its own
genesis, branch ID, address encodings, and loopback-only
[private rehearsal](testnet-private-rehearsal.md). It is not yet a public network.

See the [Catecash protocol plan](protocol-plan.md) for the broader design and
release criteria.
The [chain and privacy implementation contract](chain-and-privacy.md) defines
the order and acceptance checks for a distinct genesis and address families.
The [network launch runbook](mainnet-runbook.md) gives a tested two-node devnet
rehearsal and the remaining mainnet preparation steps.
