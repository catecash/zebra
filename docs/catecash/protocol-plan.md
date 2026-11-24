# Catecash protocol plan

**Status:** proposal. No Catecash consensus changes or network launch have been verified yet.

## Target parameters

1. Start an independent chain from the Zebra codebase, with its own genesis state, chain identifiers, address/network prefixes, peer discovery, and upgrade schedule.
2. Replace the inherited proof-of-work and issuance rules with Scrypt mining and merged mining (AuxPoW), a 60-second target block interval, and a fixed subsidy of 10,000 CATE per publicly mineable block.
3. Continue that subsidy without halving, a scheduled end date, or a maximum cumulative supply. At the target block rate, this implies approximately 1,440 blocks and 14.4 million new CATE per day, or 5.256 billion new CATE per 365-day year. Actual issuance varies with block production.
4. Pay transaction fees to miners separately from the subsidy. Set no spendable premine or reserved founder, team, or foundation allocation; 100% of newly issued coins go to miners. Document the genesis and reward rules so the public can verify the launch distribution.
5. Aim for full Zcash feature parity on the new network. Define the parity checklist and test shielded and transparent transactions, transaction validation, wallet integration, viewing capabilities, and network upgrades before claiming completion.

## Decisions still required

- Difficulty-adjustment algorithm and exact AuxPoW rules, including parent-chain compatibility and activation.
- Final genesis data, launch time, network ports, peer seeds, and chain identifiers.
- Exact coinbase maturity, fee policy, and any reward-activation exceptions.
- Address formats, wallet compatibility, and upgrade activation rules.
- Public governance and stewardship process for a 100% community-owned project.
- Testnet criteria, independent security review, and launch readiness gates.

Changing Zebra's consensus and proof-of-work rules while preserving Zcash privacy features is substantial engineering work. A source fork and a website do not make a functioning independent chain. The implementation should be reviewed and tested before any testnet or mainnet claim.

## Acceptance evidence for a fair launch

- Published source and reproducible builds before public mining begins.
- Genesis and first blocks show no spendable premine or reserved allocation.
- Consensus tests enforce the published subsidy, fee allocation, timing targets, and AuxPoW rules.
- Public testnet demonstrates mining, synchronization, shielded and transparent transactions, wallet recovery, and chain reorganizations.
- Security review findings and fixes are published before a production launch decision.

Reference parameters are documented in the [Dogecoin Core FAQ](https://github.com/dogecoin/dogecoin/blob/master/doc/FAQ.md); the privacy model is documented in [Zcash's address guide](https://zcash.readthedocs.io/en/master/rtd_pages/addresses.html). These references describe upstream systems, not a completed Catecash implementation.
