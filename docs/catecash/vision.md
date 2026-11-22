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
| Proof of work | Scrypt |
| Target block interval | 60 seconds |
| Block subsidy | 10,000 Catecash units per block, starting with publicly mineable blocks |
| Issuance end date | None planned; continuing fixed block subsidy |
| Premine / reserved allocation | None planned |

These are target parameters for a new chain, not an assertion that Zebra currently implements them for Catecash. Difficulty adjustment, genesis configuration, address formats, network identifiers, merged-mining policy, activation schedule, and wallet support require a published protocol specification and tests before launch.

## What “privacy” means here

The ambition is to preserve Zcash capabilities, including shielded payments that protect transaction details and transparent payments when chosen. “Full feature set” is a parity goal to be defined feature by feature and validated in a testnet; it should not be read as a claim of current compatibility or security.

## Source references

- [Dogecoin Core FAQ](https://github.com/dogecoin/dogecoin/blob/master/doc/FAQ.md): Scrypt proof of work, one-minute target, and the ongoing 10,000 DOGE subsidy used as the economic reference.
- [Dogecoin supply explanation](https://dogecoin.com/dogepedia/faq/putting-a-cap-on-dogecoin/): continuing issuance without a terminal supply cap.
- [Zebra](https://github.com/ZcashFoundation/zebra): maintained Rust Zcash node selected as the technical starting point.
- [Zcash address and value-pool documentation](https://zcash.readthedocs.io/en/master/rtd_pages/addresses.html): shielded and transparent transaction model.

Catecash is independent and is not affiliated with or endorsed by Dogecoin, the Zcash Foundation, or Electric Coin Company. Applicable upstream licenses and attribution must be preserved.
