# Catecash protocol plan

**Status:** proposal. No Catecash consensus changes or network launch have been verified yet.

## Target parameters

1. Start an independent chain from the Zebra codebase, with its own genesis state, chain identifiers, address/network prefixes, peer discovery, and upgrade schedule.
2. Replace the inherited proof-of-work and issuance rules with Scrypt mining, a 60-second target block interval, and a fixed subsidy of 10,000 Catecash units per publicly mineable block.
3. Continue that subsidy without a scheduled end date or maximum cumulative supply, following the ongoing-issuance shape of Dogecoin.
4. Set no spendable premine or reserved founder, team, or foundation allocation. Document the genesis and reward rules so the public can verify the launch distribution.
5. Aim for full Zcash feature parity on the new network. Define the parity checklist and test shielded and transparent transactions, transaction validation, wallet integration, viewing capabilities, and network upgrades before claiming completion.

## Decisions still required

- Difficulty-adjustment algorithm and whether merged mining is supported.
- Final genesis data, launch time, network ports, peer seeds, and chain identifiers.
- Exact coinbase maturity, fee policy, and any reward-activation exceptions.
- Address formats, wallet compatibility, and upgrade activation rules.
- Public governance and stewardship process for a 100% community-owned project.
- Testnet criteria, independent security review, and launch readiness gates.

Changing Zebra's consensus and proof-of-work rules while preserving Zcash privacy features is substantial engineering work. A source fork and a website do not make a functioning independent chain. The implementation should be reviewed and tested before any testnet or mainnet claim.

## Acceptance evidence for a fair launch

- Published source and reproducible builds before public mining begins.
- Genesis and first blocks show no spendable premine or reserved allocation.
- Consensus tests enforce the published subsidy and timing targets.
- Public testnet demonstrates mining, synchronization, shielded and transparent transactions, wallet recovery, and chain reorganizations.
- Security review findings and fixes are published before a production launch decision.

Reference parameters are documented in the [Dogecoin Core FAQ](https://github.com/dogecoin/dogecoin/blob/master/doc/FAQ.md); the privacy model is documented in [Zcash's address guide](https://zcash.readthedocs.io/en/master/rtd_pages/addresses.html). These references describe upstream systems, not a completed Catecash implementation.
