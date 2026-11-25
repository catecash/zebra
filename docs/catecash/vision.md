# Catecash vision

Catecash is an independent cryptocurrency: **Doge to Cate, now with privacy**. The idea combines Dogecoin's continuing miner-funded issuance with Zebra's Equihash proof of work and a goal of retaining the full Zcash feature set, especially shielded transactions. The cat identity is its own; Catecash is not a Dogecoin chain fork or an existing Zcash network.

Catecash mainnet mining is live. This document records the original design commitments and targets; verify current network parameters against the running network and source code.

## Design commitments

- **Fair launch:** no spendable premine, no founder or team allocation, and no reserved insider supply. All circulating coins are intended to enter through public mining rewards.
- **Community ownership:** open source code, open participation, and no reserved supply controlled by a founding group. Governance and stewardship processes still need to be specified in public.
- **Privacy as a core capability:** target the full Zcash feature set on an independent network, including shielded and transparent transaction support, subject to implementation and security testing.
- **Miner-funded security:** use Equihash proof of work and continuing block rewards, with no fixed end date for issuance.

## Monetary and mining design targets

| Parameter | Catecash target |
| --- | --- |
| Maximum supply | Unlimited |
| Block reward | 5,000 CATE per publicly mineable block |
| Target block time | 25 seconds |
| Expected blocks per day | ~3,456 |
| Expected new coins per day | ~17.28 million CATE |
| Expected new coins per year | ~6.3072 billion CATE (365 days) |
| Halving | None |
| Transaction fees | Paid to miners, separate from the block reward |
| Premine | None at launch |
| Founder allocation | 0% |
| Team allocation | 0% |
| Mining allocation | 100% of newly issued coins |
| Consensus | Equihash proof of work; no merged mining planned |
| Difficulty adjustment | DigiShield, every block; retargeting enabled |
| Mainnet minimum difficulty | Disabled |
| Coinbase maturity | 240 blocks (~100 minutes at the target rate) |
| Miner signaling | 10,080-block window; 95% threshold for configured versionbits deployments |

The daily and yearly issuance figures assume blocks arrive exactly on the 25-second target; actual issuance varies with block production. These figures were design targets; verify current parameters against the running network. The local devnet has a distinct development genesis, network type, provisional address encodings, experimental reward rules, and NU7 validation from height one, but still inherits other Zcash consensus rules. Incompatible future upgrades require hard forks; miner signaling applies only to soft-fork deployments designed for it.

## What “privacy” means here

The ambition is to preserve Zcash capabilities, including shielded payments that protect transaction details and transparent payments when chosen. “Full feature set” is a parity goal to be defined feature by feature and validated in a testnet; it should not be read as a claim of current compatibility or security.

## Source references

- [Dogecoin Core FAQ](https://github.com/dogecoin/dogecoin/blob/master/doc/FAQ.md): the ongoing 10,000 DOGE subsidy used as the economic reference.
- [Dogecoin Core chain parameters](https://github.com/dogecoin/dogecoin/blob/master/src/chainparams.cpp): difficulty, maturity, and miner-signaling reference.
- [Dogecoin supply explanation](https://dogecoin.com/dogepedia/faq/putting-a-cap-on-dogecoin/): continuing issuance without a terminal supply cap.
- [Zebra](https://github.com/ZcashFoundation/zebra): maintained Rust Zcash node selected as the technical starting point.
- [Zcash address and value-pool documentation](https://zcash.readthedocs.io/en/master/rtd_pages/addresses.html): shielded and transparent transaction model.

Catecash is independent and is not affiliated with or endorsed by Dogecoin, the Zcash Foundation, or Electric Coin Company. Applicable upstream licenses and attribution must be preserved.
