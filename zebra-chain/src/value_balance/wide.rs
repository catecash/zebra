//! Checked chain value pools for networks whose issued supply exceeds `MAX_MONEY`.

use crate::{
    amount::{NegativeAllowed, NonNegative},
    block::Height,
};

use super::ValueBalance;

/// Seven signed 128-bit balances, in the same order as [`ValueBalance::to_bytes`].
pub const SERIALIZED_SIZE: usize = 7 * 16;

/// Maximum scheduled Catecash subsidy through Zebra's maximum supported height.
/// Individual transaction and output amounts retain their existing 64-bit encoding.
// A u32 height and an i64 block reward both fit exactly in i128.
pub const CATECASH_MAX_SCHEDULED_SUPPLY_ZAT: i128 =
    (Height::MAX_AS_U32 as i128) * (crate::parameters::catecash::BLOCK_REWARD_ZAT as i128);

/// A chain balance for each value pool, in base units.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WideValueBalance {
    pools: [i128; 7],
}

/// A chain-balance validation or decoding error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// The record is not exactly seven 16-byte integers.
    #[error("wide value-pool record has an invalid length")]
    InvalidLength,
    /// An addition exceeded the signed 128-bit range.
    #[error("value-pool balance overflow")]
    Overflow,
    /// A pool was debited below zero.
    #[error("value-pool balance became negative")]
    NegativePool,
    /// The issued supply exceeded the selected network limit.
    #[error("issued supply exceeds the network limit")]
    SupplyLimit,
    /// The selected network limit is negative.
    #[error("network supply limit is negative")]
    InvalidSupplyLimit,
}

impl WideValueBalance {
    /// Convert an existing narrow chain balance without changing its value.
    pub fn from_narrow(
        value: ValueBalance<NonNegative>,
        max_issued_supply: i128,
    ) -> Result<Self, Error> {
        Self::new(
            [
                value.transparent_amount().zatoshis().into(),
                value.sprout_amount().zatoshis().into(),
                value.sapling_amount().zatoshis().into(),
                value.orchard_amount().zatoshis().into(),
                value.deferred_amount().zatoshis().into(),
                value.ironwood_amount().zatoshis().into(),
                value.nsm_amount().zatoshis().into(),
            ],
            max_issued_supply,
        )
    }

    /// Construct and validate chain balances against a network supply limit.
    pub fn new(pools: [i128; 7], max_issued_supply: i128) -> Result<Self, Error> {
        if max_issued_supply < 0 {
            return Err(Error::InvalidSupplyLimit);
        }
        if pools.iter().any(|amount| *amount < 0) {
            return Err(Error::NegativePool);
        }
        if pools.iter().any(|amount| *amount > max_issued_supply) {
            return Err(Error::SupplyLimit);
        }
        let balance = Self { pools };
        if balance.issued_supply()? > max_issued_supply {
            return Err(Error::SupplyLimit);
        }
        Ok(balance)
    }

    /// Return the value-pool balances in serialization order.
    pub fn pools(self) -> [i128; 7] {
        self.pools
    }

    /// Sum circulating pools. The final NSM balance is excluded from issued supply.
    pub fn issued_supply(self) -> Result<i128, Error> {
        self.pools[..6].iter().try_fold(0_i128, |sum, amount| {
            sum.checked_add(*amount).ok_or(Error::Overflow)
        })
    }

    /// Apply one block's validated narrow value-pool change.
    pub fn apply_change(
        self,
        change: ValueBalance<NegativeAllowed>,
        max_issued_supply: i128,
    ) -> Result<Self, Error> {
        let changes = [
            change.transparent_amount().zatoshis(),
            change.sprout_amount().zatoshis(),
            change.sapling_amount().zatoshis(),
            change.orchard_amount().zatoshis(),
            change.deferred_amount().zatoshis(),
            change.ironwood_amount().zatoshis(),
            change.nsm_amount().zatoshis(),
        ];
        let mut pools = self.pools;
        for (pool, change) in pools.iter_mut().zip(changes) {
            *pool = pool
                .checked_add(i128::from(change))
                .ok_or(Error::Overflow)?;
        }
        Self::new(pools, max_issued_supply)
    }

    /// Encode each pool as a 16-byte little-endian signed integer.
    pub fn to_bytes(self) -> [u8; SERIALIZED_SIZE] {
        let mut bytes = [0; SERIALIZED_SIZE];
        for (index, amount) in self.pools.iter().enumerate() {
            bytes[index * 16..(index + 1) * 16].copy_from_slice(&amount.to_le_bytes());
        }
        bytes
    }

    /// Decode and validate the fixed-width chain-balance record.
    pub fn from_bytes(bytes: &[u8], max_issued_supply: i128) -> Result<Self, Error> {
        if bytes.len() != SERIALIZED_SIZE {
            return Err(Error::InvalidLength);
        }
        let mut pools = [0; 7];
        for (index, chunk) in bytes.chunks_exact(16).enumerate() {
            let amount = i128::from_le_bytes(
                chunk
                    .try_into()
                    .expect("fixed-size chunks contain 16 bytes"),
            );
            pools[index] = amount;
        }
        Self::new(pools, max_issued_supply)
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::{Amount, MAX_MONEY};

    use super::*;

    const REWARD: i128 = 500_000_000_000;

    fn reward_change() -> ValueBalance<NegativeAllowed> {
        ValueBalance::from_transparent_amount(Amount::try_from(REWARD).unwrap())
    }

    #[test]
    fn catecash_crosses_the_zcash_supply_boundary() {
        let before = WideValueBalance::new(
            [4_200 * REWARD, 0, 0, 0, 0, 0, 0],
            CATECASH_MAX_SCHEDULED_SUPPLY_ZAT,
        )
        .unwrap();
        assert_eq!(before.issued_supply().unwrap(), i128::from(MAX_MONEY));
        let after = before
            .apply_change(reward_change(), CATECASH_MAX_SCHEDULED_SUPPLY_ZAT)
            .unwrap();
        assert_eq!(
            after.issued_supply().unwrap(),
            i128::from(MAX_MONEY) + REWARD
        );
        assert_eq!(
            before.apply_change(reward_change(), i128::from(MAX_MONEY)),
            Err(Error::SupplyLimit)
        );
    }

    #[test]
    fn pool_transfer_preserves_supply_and_nsm_stays_excluded() {
        let limit = CATECASH_MAX_SCHEDULED_SUPPLY_ZAT;
        let before = WideValueBalance::new([4_201 * REWARD, 0, 0, 0, 0, 0, REWARD], limit).unwrap();
        let mut transfer =
            ValueBalance::from_transparent_amount(Amount::try_from(-REWARD).unwrap());
        transfer.set_sapling_value_balance(ValueBalance::from_sapling_amount(
            Amount::try_from(REWARD).unwrap(),
        ));
        let after = before.apply_change(transfer, limit).unwrap();
        assert_eq!(after.issued_supply(), before.issued_supply());
        assert_eq!(after.pools()[0], 4_200 * REWARD);
        assert_eq!(after.pools()[2], REWARD);
        assert_eq!(after.pools()[6], REWARD);
    }

    #[test]
    fn narrow_conversion_checks_the_selected_network_limit() {
        let narrow = ValueBalance::from_transparent_amount(
            Amount::<NonNegative>::try_from(MAX_MONEY).unwrap(),
        );
        let wide = WideValueBalance::from_narrow(narrow, i128::from(MAX_MONEY)).unwrap();
        assert_eq!(wide.issued_supply(), Ok(i128::from(MAX_MONEY)));
        assert_eq!(
            WideValueBalance::from_narrow(narrow, i128::from(MAX_MONEY) - 1),
            Err(Error::SupplyLimit)
        );
    }

    #[test]
    fn wide_balances_round_trip_past_i64_and_at_maximum_height() {
        let beyond_i64 = WideValueBalance::new(
            [i128::from(i64::MAX) + 1, 0, 0, 0, 0, 0, 0],
            CATECASH_MAX_SCHEDULED_SUPPLY_ZAT,
        )
        .unwrap();
        assert_eq!(
            WideValueBalance::from_bytes(&beyond_i64.to_bytes(), CATECASH_MAX_SCHEDULED_SUPPLY_ZAT),
            Ok(beyond_i64)
        );
        let max = WideValueBalance::new(
            [CATECASH_MAX_SCHEDULED_SUPPLY_ZAT, 0, 0, 0, 0, 0, 0],
            CATECASH_MAX_SCHEDULED_SUPPLY_ZAT,
        )
        .unwrap();
        assert_eq!(
            WideValueBalance::from_bytes(&max.to_bytes(), CATECASH_MAX_SCHEDULED_SUPPLY_ZAT),
            Ok(max)
        );
        assert_eq!(
            max.apply_change(reward_change(), CATECASH_MAX_SCHEDULED_SUPPLY_ZAT),
            Err(Error::SupplyLimit)
        );
    }

    #[test]
    fn rejects_negative_overflow_and_invalid_records() {
        let limit = CATECASH_MAX_SCHEDULED_SUPPLY_ZAT;
        assert_eq!(
            WideValueBalance::new([-1, 0, 0, 0, 0, 0, 0], limit),
            Err(Error::NegativePool)
        );
        assert_eq!(
            WideValueBalance::from_bytes(&[0; 111], limit),
            Err(Error::InvalidLength)
        );
        assert_eq!(
            WideValueBalance::new([0; 7], -1),
            Err(Error::InvalidSupplyLimit)
        );
        let near_limit = WideValueBalance::new([i128::MAX, 0, 0, 0, 0, 0, 0], i128::MAX).unwrap();
        assert_eq!(
            near_limit.apply_change(reward_change(), i128::MAX),
            Err(Error::Overflow)
        );
    }
}
