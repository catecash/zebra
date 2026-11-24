//! Types used in `getblockchaininfo` RPC method.

use derive_getters::Getters;
use derive_new::new;
use zebra_chain::{
    amount::{Amount, NegativeAllowed, NonNegative},
    value_balance::{network::NetworkChainValuePools, ValueBalance},
};

use zec::Zec;

use super::*;

/// Exact base-unit RPC balance. Catecash totals are decimal strings so JSON clients
/// cannot lose integer precision; Zcash retains its existing numeric response.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RpcPoolAmount {
    Zcash(Amount<NonNegative>),
    Catecash(String),
}

impl RpcPoolAmount {
    /// Return an exact aggregate in base units.
    pub fn zatoshis(&self) -> i128 {
        match self {
            Self::Zcash(amount) => i128::from(amount.zatoshis()),
            Self::Catecash(decimal) => decimal.parse().expect("validated Catecash totals fit i128"),
        }
    }
}

impl PartialEq<Amount<NonNegative>> for RpcPoolAmount {
    fn eq(&self, other: &Amount<NonNegative>) -> bool {
        matches!(self, Self::Zcash(value) if value == other)
    }
}

impl PartialEq<Amount<NonNegative>> for &RpcPoolAmount {
    fn eq(&self, other: &Amount<NonNegative>) -> bool {
        *self == other
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RpcCoinBalance {
    Zcash(Zec<NonNegative>),
    Catecash(String),
}

fn catecash_coins(zat: i128) -> String {
    format!("{}.{:08}", zat / 100_000_000, zat % 100_000_000)
}

/// A value pool's balance in Zec and Zatoshis
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, Getters, new)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockchainInfoBalance {
    /// Name of the pool
    #[serde(skip_serializing_if = "String::is_empty", default)]
    id: String,
    /// Total amount in the pool, in ZEC
    chain_value: RpcCoinBalance,
    /// Total amount in the pool, in zatoshis
    chain_value_zat: RpcPoolAmount,
    /// Whether the value pool balance is being monitored.
    monitored: bool,
    /// Change to the amount in the pool produced by this block, in ZEC
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[getter(copy)]
    value_delta: Option<Zec<NegativeAllowed>>,
    /// Change to the amount in the pool produced by this block, in zatoshis
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[getter(copy)]
    value_delta_zat: Option<Amount>,
}

impl GetBlockchainInfoBalance {
    /// Returns a list of [`GetBlockchainInfoBalance`]s converted from the default [`ValueBalance`].
    pub fn zero_pools() -> [Self; 6] {
        Self::value_pools(ValueBalance::<NonNegative>::zero(), None)
    }

    /// Creates a new [`GetBlockchainInfoBalance`] from a pool name and its value balance
    /// and optionally with a delta value.
    pub(crate) fn new_internal(
        id: impl ToString,
        amount: Amount<NonNegative>,
        delta_amount: Option<Amount<NegativeAllowed>>,
    ) -> Self {
        Self {
            id: id.to_string(),
            chain_value: RpcCoinBalance::Zcash(Zec::from(amount)),
            chain_value_zat: RpcPoolAmount::Zcash(amount),
            monitored: amount.zatoshis() != 0,
            value_delta: delta_amount.map(Zec::from),
            value_delta_zat: delta_amount,
        }
    }

    /// Construct an exact Catecash aggregate for a pool or chain supply.
    pub fn new_catecash(id: &str, amount: i128, delta: Option<Amount<NegativeAllowed>>) -> Self {
        Self {
            id: id.to_string(),
            chain_value: RpcCoinBalance::Catecash(catecash_coins(amount)),
            chain_value_zat: RpcPoolAmount::Catecash(amount.to_string()),
            monitored: amount != 0,
            value_delta: delta.map(Zec::from),
            value_delta_zat: delta,
        }
    }

    /// Creates a [`GetBlockchainInfoBalance`] for the transparent pool.
    pub fn transparent(
        amount: Amount<NonNegative>,
        delta: Option<Amount<NegativeAllowed>>,
    ) -> Self {
        Self::new_internal("transparent", amount, delta)
    }

    /// Creates a [`GetBlockchainInfoBalance`] for the Sprout pool.
    pub fn sprout(amount: Amount<NonNegative>, delta: Option<Amount<NegativeAllowed>>) -> Self {
        Self::new_internal("sprout", amount, delta)
    }

    /// Creates a [`GetBlockchainInfoBalance`] for the Sapling pool.
    pub fn sapling(amount: Amount<NonNegative>, delta: Option<Amount<NegativeAllowed>>) -> Self {
        Self::new_internal("sapling", amount, delta)
    }

    /// Creates a [`GetBlockchainInfoBalance`] for the Orchard pool.
    pub fn orchard(amount: Amount<NonNegative>, delta: Option<Amount<NegativeAllowed>>) -> Self {
        Self::new_internal("orchard", amount, delta)
    }

    /// Creates a [`GetBlockchainInfoBalance`] for the Lockbox pool.
    pub fn deferred(amount: Amount<NonNegative>, delta: Option<Amount<NegativeAllowed>>) -> Self {
        Self::new_internal("lockbox", amount, delta)
    }

    /// Creates a [`GetBlockchainInfoBalance`] for the Ironwood pool (NU6.3).
    pub fn ironwood(amount: Amount<NonNegative>, delta: Option<Amount<NegativeAllowed>>) -> Self {
        Self::new_internal("ironwood", amount, delta)
    }

    /// Creates a [`GetBlockchainInfoBalance`] for the Network Sustainability Mechanism (NSM)
    /// value balance (NU7) after the block at `height` on `network`. `amount` and `delta` are
    /// [ZIP 237]'s values. It is `monitored` from NU7 - 1, also when the balance is zero.
    ///
    /// The NSM value balance is not part of the issued supply, so it is not a value pool and
    /// is not counted in [`Self::chain_supply`].
    ///
    /// [ZIP 237]: https://zips.z.cash/zip-0237
    pub fn nsm(
        network: &zebra_chain::parameters::Network,
        height: zebra_chain::block::Height,
        amount: Amount<NonNegative>,
        delta: Option<Amount<NegativeAllowed>>,
    ) -> Self {
        Self {
            // The next block is at or after NU7 activation; `Height::MAX` has no next block.
            monitored: zebra_chain::parameters::NetworkUpgrade::Nu7
                .activation_height(network)
                .is_some_and(|activation_height| {
                    height
                        .next()
                        .map_or(true, |next_height| next_height >= activation_height)
                }),
            ..Self::new_internal("nsm", amount, delta)
        }
    }

    /// Converts a [`ValueBalance`] to a list of [`GetBlockchainInfoBalance`]s.
    pub fn value_pools(
        value_balance: impl Into<NetworkChainValuePools>,
        delta_balance: Option<ValueBalance<NegativeAllowed>>,
    ) -> [Self; 6] {
        let value_balance = value_balance.into();
        if let NetworkChainValuePools::Catecash(value) = value_balance {
            let pools = value.pools();
            let delta = delta_balance;
            return [
                Self::new_catecash(
                    "transparent",
                    pools[0],
                    delta.map(|b| b.transparent_amount()),
                ),
                Self::new_catecash("sprout", pools[1], delta.map(|b| b.sprout_amount())),
                Self::new_catecash("sapling", pools[2], delta.map(|b| b.sapling_amount())),
                Self::new_catecash("orchard", pools[3], delta.map(|b| b.orchard_amount())),
                Self::new_catecash("lockbox", pools[4], delta.map(|b| b.deferred_amount())),
                Self::new_catecash("ironwood", pools[5], delta.map(|b| b.ironwood_amount())),
            ];
        }
        let NetworkChainValuePools::Zcash(value_balance) = value_balance else {
            unreachable!()
        };
        [
            Self::transparent(
                value_balance.transparent_amount(),
                delta_balance.map(|b| b.transparent_amount()),
            ),
            Self::sprout(
                value_balance.sprout_amount(),
                delta_balance.map(|b| b.sprout_amount()),
            ),
            Self::sapling(
                value_balance.sapling_amount(),
                delta_balance.map(|b| b.sapling_amount()),
            ),
            Self::orchard(
                value_balance.orchard_amount(),
                delta_balance.map(|b| b.orchard_amount()),
            ),
            Self::deferred(
                value_balance.deferred_amount(),
                delta_balance.map(|b| b.deferred_amount()),
            ),
            // The Ironwood chain value pool balance is zero before NU6.3 activates.
            Self::ironwood(
                value_balance.ironwood_amount(),
                delta_balance.map(|b| b.ironwood_amount()),
            ),
        ]
    }

    // The NU7 NSM value balance is deliberately not a pool here: it is not part of the Issued
    // Supply, so `chain_supply()` correctly falls as fees are burned. `nsm()` reports it.

    /// Converts a [`ValueBalance`] to a [`GetBlockchainInfoBalance`] representing the total chain supply.
    pub fn chain_supply(value_balance: impl Into<NetworkChainValuePools>) -> Self {
        let value_balance = value_balance.into();
        if let NetworkChainValuePools::Catecash(value) = value_balance {
            return Self::new_catecash(
                "",
                value
                    .issued_supply()
                    .expect("validated pools have a representable supply"),
                None,
            );
        }
        let NetworkChainValuePools::Zcash(value_balance) = value_balance else {
            unreachable!()
        };
        Self::value_pools(value_balance, None)
            .into_iter()
            .reduce(|a, b| {
                let (RpcPoolAmount::Zcash(a_value), RpcPoolAmount::Zcash(b_value)) =
                    (a.chain_value_zat, b.chain_value_zat)
                else {
                    unreachable!()
                };
                GetBlockchainInfoBalance::new_internal(
                    "",
                    (a_value + b_value).expect("sum of value balances should not overflow"),
                    None,
                )
            })
            .expect("at least one pool")
    }
}
