# Catecash vision

Catecash is a proposed independent cryptocurrency: **Doge to Cate, now with privacy**. The idea combines Dogecoin's continuing miner-funded issuance and Scrypt proof of work with a goal of retaining the full Zcash feature set, especially shielded transactions. The cat identity is its own; Catecash is not a Dogecoin chain fork or an existing Zcash network.

The project is in development. There is no live Catecash network or token. The statements below are design commitments and targets, not claims that the protocol has been implemented or audited.

## Proposed launch commitments

- **Fair launch:** no spendable premine, no founder or team allocation, and no reserved insider supply. All circulating coins are intended to enter through public mining rewards.
- **Community ownership:** open source code, open participation, and no reserved supply controlled by a founding group. Governance and stewardship processes still need to be specified in public.
- **Privacy as a core capability:** target the full Zcash feature set on an independent network, including shielded and transparent transaction support, subject to implementation and security testing.
- **Miner-funded security:** use Scrypt proof of work and continuing block rewards, with no fixed end date for issuance.

## Proposed monetary and mining model

| Parameter | Catecash target |
| --- | --- |
| Maximum supply | Unlimited |
| Block reward | 10,000 CATE per publicly mineable block |
| Target block time | 1 minute |
| Expected blocks per day | ~1,440 |
| Expected new coins per day | ~14.4 million CATE |
| Expected new coins per year | ~5.256 billion CATE (365 days) |
| Halving | None |
| Transaction fees | Paid to miners, separate from the block reward |
| Premine | None at launch |
| Founder allocation | 0% |
| Team allocation | 0% |
| Mining allocation | 100% of newly issued coins |
| Consensus | Scrypt proof of work with merged mining (AuxPoW) |

The daily and yearly issuance figures assume blocks arrive exactly on the one-minute target; actual issuance varies with block production. These are target parameters for a new chain, not an assertion that Zebra currently implements them for Catecash. Difficulty adjustment, genesis configuration, address formats, network identifiers, AuxPoW validation rules, activation schedule, and wallet support require a published protocol specification and tests before launch.

## What “privacy” means here

The ambition is to preserve Zcash capabilities, including shielded payments that protect transaction details and transparent payments when chosen. “Full feature set” is a parity goal to be defined feature by feature and validated in a testnet; it should not be read as a claim of current compatibility or security.

## Source references

- [Dogecoin Core FAQ](https://github.com/dogecoin/dogecoin/blob/master/doc/FAQ.md): Scrypt proof of work, one-minute target, and the ongoing 10,000 DOGE subsidy used as the economic reference.
- [Dogecoin supply explanation](https://dogecoin.com/dogepedia/faq/putting-a-cap-on-dogecoin/): continuing issuance without a terminal supply cap.
- [Zebra](https://github.com/ZcashFoundation/zebra): maintained Rust Zcash node selected as the technical starting point.
- [Zcash address and value-pool documentation](https://zcash.readthedocs.io/en/master/rtd_pages/addresses.html): shielded and transparent transaction model.

Catecash is independent and is not affiliated with or endorsed by Dogecoin, the Zcash Foundation, or Electric Coin Company. Applicable upstream licenses and attribution must be preserved.
