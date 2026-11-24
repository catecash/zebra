# Catecash protocol plan

**Status:** proposed consensus blueprint. The isolated local development profile has experimental reward rules and selects NU7 validation from the first mineable block. No Catecash chain or network launch has been verified.

The [identity and network allocation](identity.md) records the CATE ticker,
brand asset, and provisional P2P identifiers. The local devnet profile has
experimental reward validation; it is not a Catecash chain.

## Target parameters

1. Start an independent chain from the Zebra codebase, with its own genesis state, chain identifiers, address/network prefixes, and peer discovery. Apply one launch validation rule set to every mineable block, with supported shielded features available from height one.
2. Keep Equihash proof of work. Replace the inherited issuance rules with a fixed subsidy of 5,000 CATE per publicly mineable block.
3. Continue that subsidy without halving, a scheduled end date, or a maximum cumulative supply. At the 25-second target, this implies approximately 3,456 blocks and 17.28 million new CATE per day, or 6.3072 billion new CATE per 365-day year. Actual issuance varies with block production.
4. Pay transaction fees to miners separately from the subsidy. Set no spendable premine or reserved founder, team, or foundation allocation; 100% of newly issued coins go to miners. Document the genesis and reward rules so the public can verify the launch distribution.
5. Aim for full current Zcash feature parity on the new network. Define the parity checklist and test shielded and transparent transactions, transaction validation, wallet integration, and viewing capabilities before claiming completion. Future incompatible changes still need an explicit consensus process.

The local devnet selects NU7 at height one, so Orchard, Ironwood, and the
current Sapling rules are available from its first mineable block. Genesis
retains the separate height-zero rules needed to validate its special
coinbase. This profile is a configuration experiment: NU7 supplies Catecash's
chosen 25-second target spacing. The node uses Catecash's `0xEA3BEEB4`
transaction branch ID on the devnet, but the devnet does not yet have a
complete independent consensus identity.

## Consensus targets

| Parameter | Catecash target |
| --- | --- |
| Consensus and proof of work | Equihash, inherited from Zebra |
| Target block interval | 25 seconds |
| Difficulty adjustment | Every block, using DigiShield |
| Difficulty retargeting | Enabled |
| Merged mining | None planned |
| Coinbase maturity | 240 blocks (about 100 minutes at the target rate) |
| Mainnet minimum-difficulty exception | Disabled |
| Upgrade method | Hard forks for incompatible changes; miner-signaled soft forks where the rule supports signaling |
| Miner signaling window | 10,080 blocks (about 70 hours at the target rate) |
| Versionbits activation threshold | 9,576 signaling blocks per window (95%) for deployments configured to use this rule |

The issuance target draws on Dogecoin's continuing reward, not its proof of work or chain identity. The devnet now applies one-block DigiShield retargeting from height two and 240-block transparent coinbase maturity. These development rules still need live-chain testing and a final public-network specification. The 10,080-block signaling window applies only to explicitly configured deployments; it does not activate every network upgrade automatically.

Zebra parses block headers with an Equihash solution and checks Equihash proof of work. The Catecash devnet has separate validation for its experimental reward, retargeting, maturity, and NU7-at-height-one schedule, but other Zcash rules remain. A configuration file alone cannot create a launch-ready independent chain. Implement and review dedicated public networks while preserving Zcash validation.

## Decisions still required

- Difficulty adjustment, target block interval, initial difficulty, and proof-of-work limit.
- Final genesis data, launch time, confirmation of proposed network ports and
  identifiers, and peer seeds.
- Fee policy and any reward-activation exceptions.
- Address formats, wallet compatibility, and the exact launch validation rules.
- Which upgrades use versionbits signaling, their bit numbers, start/timeout conditions, and whether the 95% threshold is appropriate for each.
- Public governance and stewardship process for a 100% community-owned project.
- Testnet criteria, independent security review, and launch readiness gates.

Changing Zebra's consensus and proof-of-work rules while preserving Zcash privacy features is substantial engineering work. A source fork and a website do not make a functioning independent chain. The implementation should be reviewed and tested before any testnet or mainnet claim.

## Acceptance evidence for a fair launch

- Published source and reproducible builds before public mining begins.
- Genesis and first blocks show no spendable premine or reserved allocation.
- Consensus tests enforce the published subsidy, fee allocation, proof of work, difficulty targets, 240-block maturity boundary, mainnet minimum-difficulty policy, and signaling windows.
- Public testnet demonstrates mining, synchronization, shielded and transparent transactions, wallet recovery, and chain reorganizations.
- Security review findings and fixes are published before a production launch decision.

The ongoing reward is modeled on [Dogecoin Core's chain parameters](https://github.com/dogecoin/dogecoin/blob/master/src/chainparams.cpp). The privacy model is documented in [Zcash's address guide](https://zcash.readthedocs.io/en/master/rtd_pages/addresses.html). These references do not describe a completed Catecash implementation.
