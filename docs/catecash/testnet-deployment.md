# Public Catecash testnet deployment

**Status (2026-11-24 UTC): HOSTED REHEARSAL SERVICES RUNNING; PUBLIC TESTNET NOT ANNOUNCED.** Two dedicated Hetzner nodes agree on the testnet genesis and mined tip over a private network. The testnet Zaino wallet gateway and separate CateScan explorer serve verified TLS endpoints. Public P2P remains closed. The running `catecash-devnet` remains a separate development chain. This checklist is the release and operations plan for a distinct public testnet.

## Release gates before opening public access

- [x] Record a versioned [testnet parameter contract](testnet-network-v1.json) for private rehearsal, including separate mined genesis bytes, identifiers, address encodings, upgrade schedule, proof-of-work limit, DigiShield rules, 5,000 CATE subsidy, and 240-block coinbase maturity. Independent review remains open before public launch.
- [ ] Implement `CatecashTestnet` in Zebra and protocol/address libraries. Give it its own genesis, checkpoint, network storage, P2P handshake, RPC chain name, mining rules, and cross-network rejection tests.
- [ ] Implement the same network in Zaino and Zingolib. Verify chain name, genesis, branch ID, and activation heights before indexing or wallet sync. Use separate index and wallet storage.
- [ ] Build Catecash mobile and desktop bindings from the tested Zingolib commit. Show the testnet name explicitly, set the testnet gateway, and reject devnet, Zcash, and mainnet servers and addresses.
- [ ] Complete the wide-balance boundary run across heights 4,200/4,201 and compare Zebra, Zaino, and wallet values. Record the deterministic index persistence and reorg fixtures and the live boundary results.
- [ ] Produce a tagged, reproducible testnet release with source revision, binary hashes, public parameter contract, and operator configuration examples that contain no secrets.

## Services and operations beyond the nodes

- [ ] **Bootstrap and mining:** provision at least two independently operated P2P seed nodes, publish seed DNS or fixed peers, and document a testnet miner with a controlled payout key. Keep node RPC authenticated and private.
- [ ] **Wallet gateway:** run a dedicated testnet Zaino indexer with its own state and TLS hostname. Restrict its validator link, validate genesis at startup and after reconnect, monitor index lag, and test gRPC compact blocks, treestates, balance, mempool, and broadcast.
- [ ] **Explorer:** deploy a separate CateScan instance bound to the testnet node or indexer. Set exact expected chain and genesis, label every page as testnet, and serve it through HTTPS. Never repoint `scan-devnet.catecash.com` or `scan.catecash.com` to testnet.
- [ ] **DNS and TLS:** allocate distinct testnet hostnames for seed, wallet gateway, and explorer; verify certificate coverage, gRPC transport, and external P2P reachability. Publish endpoint names only after they resolve and pass identity checks.
- [ ] **Funding:** provide documented test coin access through a faucet or controlled miner distribution. Add abuse limits and make clear that testnet CATE has no monetary value.
- [ ] **Monitoring and recovery:** alert on node and indexer tip divergence, stalled blocks, peer loss, RPC/gRPC errors, certificate expiry, disk pressure, and explorer failures. Back up configs and service state; document restore and restart procedures without reusing devnet state.
- [ ] **Client release:** publish testnet wallet builds and connection instructions, seed backup and restore guidance, and known limitations. Gate wallet balance display on the expected genesis.
- [ ] **End-to-end acceptance:** demonstrate two independent nodes agreeing on tips; mining and 240-block maturity; transparent and enabled shielded sends; wallet restore; restart; deliberate reorganization; wrong-chain rejection; gateway and explorer agreement; and remote TLS access. Publish a dated evidence report.

## Deployment order

1. Freeze and review the parameter contract and fixtures.
2. Build and test the node, indexer, wallet engine, and app artifacts against that contract.
3. Start private testnet seed nodes and miner; verify genesis, blocks, peer propagation, and restarts.
4. Start the dedicated Zaino gateway; compare its tip and balances with the nodes.
5. Start the dedicated explorer; verify exact chain and genesis checks.
6. Enable DNS, TLS, firewall rules, and public endpoints; run the end-to-end acceptance checks.
7. Publish the network parameters, binaries, endpoints, funding instructions, and evidence report.

**Go/no-go:** Do not advertise a public Catecash testnet until the release gates and end-to-end acceptance checks pass. The hosted devnet is available for rehearsals, but its genesis, provisional addresses, wallet data, and rewards are not testnet assets.

The [2026-11-23 private rehearsal](testnet-private-rehearsal.md) completed the first node → indexer → wallet path on loopback and records its exact evidence and open gates.

The [hosted node rehearsal](testnet-hosted-nodes.md) records the allocated servers, private peering, independently built binaries, continuing mining, gateway and explorer checks, and DNS. Public P2P remains firewalled pending the release gates above. The gateway is `https://wallet-testnet.catecash.com:443`, and the explorer is `https://scan-testnet.catecash.com`.
