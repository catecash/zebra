# Catecash checkpoint rollout, 2026-11-27

This directory contains the systemd units for the verified devnet height
12,400, testnet height 11,600, and mainnet height 10,000 checkpoint image.
The old units remain in their original directories for rollback. The public
mainnet seed Compose file is in `../catecash-mainnet-seed/compose.yaml`.

Use the ARM image on the three devnet services, both testnet hosts, mainnet
miners 1 and 2, and the wallet gateway. Use the x86 image on mainnet miners
3 and 4 and the three public seed containers. Preserve each host's live TOML,
state, RPC cookie, and payout rotation data. Install only the matching unit
after loading the image of the matching architecture.

For each instance, stop it, back up its live unit, config, and stopped chain
state to a restricted host directory, install the new unit or Compose file,
and start it. Confirm the chain and genesis, the final checkpoint hash, logs
showing checkpoint validation, sync progress, peer links, and dependent wallet
readiness before moving to the next instance. Restore the old unit and state
if validation fails.

The selected hashes and rollout gates are recorded in
`../../docs/catecash/checkpoint-promotion.md`.
