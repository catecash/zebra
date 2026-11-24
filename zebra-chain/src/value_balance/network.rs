//! Explicit network selection for chain value-pool accounting.

use crate::{
    amount::{Amount, NegativeAllowed, NonNegative},
    block::Height,
    parameters::{Network, NetworkKind},
};

use super::{
    wide::{self, WideValueBalance},
    ValueBalance, ValueBalanceError,
};

/// Chain pool balances whose representation follows the selected network.
///
/// Per-block changes retain their existing transaction-width representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetworkChainValuePools {
    /// Zcash retains its existing 21-million-coin issued-supply bound.
    Zcash(ValueBalance<NonNegative>),
    /// Catecash uses checked 128-bit aggregate balances.
    Catecash(WideValueBalance),
}

impl Default for NetworkChainValuePools {
    fn default() -> Self {
        Self::Zcash(ValueBalance::zero())
    }
}

impl From<ValueBalance<NonNegative>> for NetworkChainValuePools {
    fn from(value: ValueBalance<NonNegative>) -> Self {
        Self::Zcash(value)
    }
}

impl PartialEq<ValueBalance<NonNegative>> for NetworkChainValuePools {
    fn eq(&self, other: &ValueBalance<NonNegative>) -> bool {
        matches!(self, Self::Zcash(value) if value == other)
    }
}

impl PartialEq<NetworkChainValuePools> for ValueBalance<NonNegative> {
    fn eq(&self, other: &NetworkChainValuePools) -> bool {
        other == self
    }
}

/// A chain pool change that fails the selected network's rules.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// The update failed the existing Zcash pool rules.
    #[error("invalid Zcash chain value pools: {0:?}")]
    Zcash(ValueBalanceError),
    /// The update failed the Catecash wide pool rules.
    #[error("invalid Catecash chain value pools: {0}")]
    Catecash(wide::Error),
    /// Catecash devnet does not activate the NSM reserve.
    #[error("Catecash NSM reserve must remain zero")]
    UnexpectedCatecashNsm,
}

impl NetworkChainValuePools {
    /// Maximum subsidy that could have been issued through `height` on Catecash.
    /// This is a height-dependent consensus check, not a fixed monetary cap.
    pub fn catecash_subsidy_bound(height: Height) -> i128 {
        i128::from(height.0) * i128::from(crate::parameters::catecash::BLOCK_REWARD_ZAT)
    }

    /// Check that circulating pools do not contain more than the subsidy
    /// permitted through this height. Fees and pool transfers do not issue coins.
    pub fn validate_at_height(self, height: Height) -> Result<Self, Error> {
        if let Self::Catecash(pools) = self {
            if pools.issued_supply().map_err(Error::Catecash)?
                > Self::catecash_subsidy_bound(height)
            {
                return Err(Error::Catecash(wide::Error::SupplyLimit));
            }
        }
        Ok(self)
    }

    /// Apply a block change and check the subsidy bound at its new height.
    pub fn apply_change_at_height(
        self,
        change: ValueBalance<NegativeAllowed>,
        height: Height,
    ) -> Result<Self, Error> {
        self.apply_change(change)?.validate_at_height(height)
    }

    /// Return all pool balances in base units, including the NSM reserve last.
    pub fn pools(self) -> [i128; 7] {
        match self {
            Self::Catecash(value) => value.pools(),
            Self::Zcash(value) => [
                value.transparent_amount().zatoshis().into(),
                value.sprout_amount().zatoshis().into(),
                value.sapling_amount().zatoshis().into(),
                value.orchard_amount().zatoshis().into(),
                value.deferred_amount().zatoshis().into(),
                value.ironwood_amount().zatoshis().into(),
                value.nsm_amount().zatoshis().into(),
            ],
        }
    }

    /// Return the parent reserve without narrowing circulating balances.
    pub fn nsm_amount(self) -> Amount<NonNegative> {
        self.nsm_reserve()
            .expect("validated Catecash pools have no NSM reserve")
    }

    /// Select accounting rules from an explicit consensus network.
    pub fn zero(network: &Network) -> Self {
        match network.kind() {
            NetworkKind::CatecashDevnet
            | NetworkKind::CatecashMainnet
            | NetworkKind::CatecashTestnet => Self::Catecash(WideValueBalance::default()),
            NetworkKind::Mainnet | NetworkKind::Testnet | NetworkKind::Regtest => {
                Self::Zcash(ValueBalance::zero())
            }
        }
    }

    /// Convert a pre-migration balance using the selected network's rules.
    pub fn from_narrow(network: &Network, value: ValueBalance<NonNegative>) -> Result<Self, Error> {
        match network.kind() {
            NetworkKind::CatecashDevnet
            | NetworkKind::CatecashMainnet
            | NetworkKind::CatecashTestnet => {
                if value.nsm_amount().zatoshis() != 0 {
                    return Err(Error::UnexpectedCatecashNsm);
                }
                WideValueBalance::from_narrow(value, wide::CATECASH_MAX_SCHEDULED_SUPPLY_ZAT)
                    .map(Self::Catecash)
                    .map_err(Error::Catecash)
            }
            NetworkKind::Mainnet | NetworkKind::Testnet | NetworkKind::Regtest => {
                value
                    .total()
                    .map_err(|error| Error::Zcash(ValueBalanceError::Total(error)))?;
                Ok(Self::Zcash(value))
            }
        }
    }

    /// Return the reserve used by block subsidy and fee accounting.
    /// Catecash devnet keeps the reserve inactive and therefore zero.
    pub fn nsm_reserve(self) -> Result<Amount<NonNegative>, Error> {
        match self {
            Self::Zcash(value) => Ok(value.nsm_amount()),
            Self::Catecash(value) if value.pools()[6] == 0 => Ok(Amount::zero()),
            Self::Catecash(_) => Err(Error::UnexpectedCatecashNsm),
        }
    }

    /// Apply a block's validated pool change with the network's supply rule.
    pub fn apply_change(self, change: ValueBalance<NegativeAllowed>) -> Result<Self, Error> {
        match self {
            Self::Zcash(value) => value
                .add_chain_value_pool_change(change)
                .map(Self::Zcash)
                .map_err(Error::Zcash),
            Self::Catecash(value) => {
                if value.pools()[6] != 0 || change.nsm_amount().zatoshis() != 0 {
                    return Err(Error::UnexpectedCatecashNsm);
                }
                value
                    .apply_change(change, wide::CATECASH_MAX_SCHEDULED_SUPPLY_ZAT)
                    .map(Self::Catecash)
                    .map_err(Error::Catecash)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        amount::{Amount, MAX_MONEY},
        parameters::Network,
    };

    use super::*;

    #[test]
    fn supply_boundary_depends_on_network_identity() {
        const REWARD: i64 = 500_000_000_000;
        let before = ValueBalance::from_transparent_amount(
            Amount::<NonNegative>::try_from(MAX_MONEY).unwrap(),
        );
        let change = ValueBalance::from_transparent_amount(Amount::try_from(REWARD).unwrap());

        let zcash = NetworkChainValuePools::from_narrow(&Network::Mainnet, before).unwrap();
        assert!(matches!(zcash.apply_change(change), Err(Error::Zcash(_))));

        let catecash =
            NetworkChainValuePools::from_narrow(&Network::CatecashDevnet, before).unwrap();
        assert_eq!(
            catecash.nsm_reserve().unwrap(),
            Amount::<NonNegative>::zero()
        );
        let after = catecash.apply_change(change).unwrap();
        assert_eq!(
            after,
            NetworkChainValuePools::Catecash(
                WideValueBalance::new(
                    [i128::from(MAX_MONEY) + i128::from(REWARD), 0, 0, 0, 0, 0, 0],
                    wide::CATECASH_MAX_SCHEDULED_SUPPLY_ZAT,
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn catecash_rejects_nsm_reserve_changes() {
        let mut change = ValueBalance::<NegativeAllowed>::zero();
        change.set_nsm_amount(Amount::try_from(1).unwrap());
        assert_eq!(
            NetworkChainValuePools::zero(&Network::CatecashDevnet).apply_change(change),
            Err(Error::UnexpectedCatecashNsm)
        );
    }

    #[test]
    fn reward_pool_crosses_zcash_cap_after_block_4200() {
        const REWARD: i64 = 500_000_000_000;
        let change = ValueBalance::from_transparent_amount(Amount::try_from(REWARD).unwrap());
        for network in [
            Network::CatecashDevnet,
            Network::CatecashMainnet,
            Network::CatecashTestnet,
        ] {
            let mut pools = NetworkChainValuePools::zero(&network);
            for height in 1..=4_200 {
                pools = pools
                    .apply_change_at_height(change, Height(height))
                    .unwrap();
            }
            assert_eq!(pools.pools()[0], i128::from(MAX_MONEY));
            pools = pools.apply_change_at_height(change, Height(4_201)).unwrap();
            assert_eq!(pools.pools()[0], i128::from(MAX_MONEY) + i128::from(REWARD));
        }
    }

    #[test]
    fn issued_supply_is_bounded_by_the_current_height() {
        const REWARD: i64 = 500_000_000_000;
        let change = ValueBalance::from_transparent_amount(Amount::try_from(REWARD).unwrap());
        let zero = NetworkChainValuePools::zero(&Network::CatecashDevnet);
        assert!(matches!(
            zero.apply_change_at_height(change, Height(0)),
            Err(Error::Catecash(wide::Error::SupplyLimit))
        ));
        let first = zero.apply_change_at_height(change, Height(1)).unwrap();
        assert!(matches!(
            first.apply_change_at_height(change, Height(1)),
            Err(Error::Catecash(wide::Error::SupplyLimit))
        ));
        assert_eq!(
            first
                .apply_change_at_height(change, Height(2))
                .unwrap()
                .pools()[0],
            2 * i128::from(REWARD)
        );
    }
}
