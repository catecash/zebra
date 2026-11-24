# Catecash devnet payout keys and mining

The external payout rotation rehearsal and its repeatable promotion steps are
recorded in [`utilities/PROMOTION.md`](../../../../utilities/PROMOTION.md).
On 2026-11-24 UTC, two scheduled devnet addresses received live rewards, and
a daily external rotation timer was enabled on the hosted devnet miner. This
uses public keys from the separate testnet rehearsal pool and no mainnet key.

The local Catecash devnet uses provisional address encodings. The node accepts
transparent P2PKH payout addresses. A private key is needed later to spend a
mining reward; the node itself needs only the public address.

Generate a new development key on the machine where it will be backed up:

```sh
mkdir -m 700 -p "$HOME/.catecash/devnet"
cargo run -p zebra-chain --example catecash_miner_key -- \
  --network devnet \
  "$HOME/.catecash/devnet/miner-key.json"
```

The command prints the Catecash payout address and creates the destination
file with owner-only permissions. The file contains a provisional compressed
WIF private key. The command refuses to overwrite an existing file. Keep this
file out of the repository and off the node server, and back it up before
mining any blocks. The provisional WIF prefix and address prefix may change
before public testnet.

Set `mining.miner_address` to the printed address and connect the miner node
to a separate Catecash devnet peer. The [two-node rehearsal](mainnet-runbook.md)
shows the peer and miner configuration. RPC listens on loopback and uses cookie
authentication on the hosted node. An external Equihash miner can request
`getblocktemplate`, solve the header, and call `submitblock`. The internal
miner requires a feature-enabled build. A bounded development run mined a
height-one block to a controlled key; continuous mining remains off.

On 2026-11-22, the isolated Hetzner miner accepted block
`238004e8e789f863e124d57686940b18020784886ef77492f3c1243a6e575ade`
at height one. Its coinbase has one P2PKH output of 5,000 devnet CATE to
`yPcjsh1MxoRFtkaye1hjrZqi2ZGsB7jby1H`. A second loopback-only node
independently synced that block and both nodes reported the same tip. The miner
was switched off after the block; the connected miner node then served a
height-two template with the normal peer-sync check. The private key stayed on
the operator's local machine.

The Catecash Zingolib integration branch creates and restores seed-based
Catecash devnet wallets, derives Catecash addresses, checks the indexer's
genesis and branch ID, and syncs the mined chain through Zaino. Its
`catecash_mining` module imports the standalone compressed WIF, derives the
same payout address, and queries the mining reward through Zaino. The WIF
stays separate from the seed-based wallet's normal account list.

The current Catecash node source sets transparent coinbase maturity to 240
blocks. Under that rule, the height-one reward can first appear as an input in
block 241, after the tip reaches height 240. Both hosted nodes ran the matching
240-block rule for the live spend. The coinbase sweep has no transparent
outputs and uses an Ironwood receiver. The devnet retargets from height two.

On 2026-11-23, the hosted miner reached height 240 and stopped. Both nodes
agreed on the tip. A locally held WIF signed the height-one 5,000 CATE reward
to the backed-up seed wallet's Ironwood receiver. Both nodes accepted
transaction `369be6048735d661b1f16b8e332ae157ee1346dcd8d0a980469faa2f4643d10f`
and confirmed it in block 241. Zingolib synced the block and recorded
4,999.99985 CATE in Ironwood after a 0.00015 CATE fee. At height 243, the
wallet reported the entire note as spendable.

The wallet then sent 1 CATE from Ironwood to its own transparent address
`yPrdWV2arCwuDTSTYSBvbxshfKEt4H5Hp3Q`. Transaction
`629003a078d3e2ef86994ba37733eac8e8e9ae68aacfd41b432c7c3faa873c1c`
was accepted into the mempool and confirmed in block 244. Both nodes agreed
on that block. The wallet resynced and recorded a confirmed, unspent 1 CATE
transparent output, 4,998.99982 CATE in Ironwood change, and a 0.00003 CATE
fee. Mining was disabled after each bounded run. These transfers validate one
local devnet path. Wide supply accounting, reorgs, and public-network wallet
support remain unfinished.

On 2026-11-23, the hosted devnet was reset from genesis to deploy the wide
state and balance encoding. The previous node, peer, and Zaino binaries and
both previous chain-state directories were retained under
`/opt/catecash/backups/2026-11-23-wide-balance/` on the host. The miner node
was rebuilt with `--features internal-miner`; a bounded run reached height 21,
and the second node independently verified block 21. Mining was then disabled
in the hosted miner config. The public `wallet-devnet.catecash.com:443` Zaino
endpoint reported `catecash-devnet` at height 21 and returned the wide balance
field, including for a zero balance. Earlier transaction IDs and wallet
balances in this document describe the superseded chain; new wallet tests must
start from the reset chain.

Mining was resumed on the same reset chain for the wide-supply boundary run.
At height 34, Zebra and Zaino both reported the miner address balance as
17,000,000,000,000 catemoto. At height 46, Zebra reported 23,000,000,000,000
catemoto in the circulating pools and the same amount received by the miner
address, matching 46 full subsidies. The unprivileged
`catecash-wide-supply-watch.service` runs
[`scripts/catecash-wide-supply-watch.py`](../../scripts/catecash-wide-supply-watch.py)
on the host and writes exact pool and address snapshots at heights 4,200 and
4,201 to `/var/lib/catecash/wide-supply-run.jsonl`. It does not stop mining.
Zaino and wallet comparisons at the boundary remain to be recorded.

A third hosted Zebra verifier was then started with separate state, RPC, and
P2P ports. The three loopback nodes allow two connections per IP so the new
node can connect to both existing nodes. At height 79, all three reported
block `02a962a1179e3dad93bf29365f1f36666abd407b7b3d27953925046cd73931fe`,
39,500,000,000,000 catemoto in the circulating pools, and the same exact
miner-address balance. The miner and first peer reported three peer
connections each; the third node reported two. Zaino also reported height 79
and the same wide miner balance. The read-only
[`three-node check`](../../scripts/catecash-three-node-check.py) reproduces
the node comparison. All three nodes share one host, so this checks protocol
agreement and P2P propagation rather than independent infrastructure.

At 2026-11-24 02:14 UTC, a read-only hosted check found the boundary watcher
and miner services active. The watcher reported height 1,130, circulating
pool total `565000000000000` catemoto, and the same scheduled issuance. A
three-node check immediately after found all nodes at height 1,131 and hash
`094095ac455bf7ee01d8e1d03afbd4aea1f0cba17b02667bbb06b7de2f94a876`.
Each reported `565500000000000` catemoto in the pools, matching 1,131
subsidies. The boundary log had no recorded 4,200/4,201 snapshot yet. This
check does not establish restart, index rebuild, or reorganization behavior
at the boundary.

At 2026-11-24 02:44 UTC, a read-only Zebra/Zaino comparison captured both
services at height 1,206. Zebra's tip was
`020c35be5a5af8605431b26fcf1af940d6cabfe91894e99745ae55306c157b3a`;
Zaino's `GetLatestBlock` hash bytes were the reverse-order encoding of that
same hash. Zebra `getaddressbalance` and Zaino `GetTaddressBalance.valueZatWide`
both reported `602500000020000` catemoto for the miner address. Zaino's legacy
`valueZat` field was zero, as specified for Catecash. This is a cross-service
wire check below the old monetary and JSON precision boundaries, not proof of
large-value agreement at height 4,201 or later.
