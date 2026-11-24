//! Catecash's one-block DigiShield target calculation.
//!
//! Catecash keeps Zebra's Equihash proof of work and uses this calculation
//! to retarget the development chain after its first mineable block.

use super::difficulty::{CompactDifficulty, ExpandedDifficulty, U256};

/// The intended Catecash spacing and DigiShield retarget period, in seconds.
pub const TARGET_SPACING: i64 = 25;

/// Calculate the next compact target from the previous target and block times.
///
/// `actual_timespan` is the previous block time minus its parent's block time.
/// The amplitude filter divides its deviation from 25 seconds by eight, with
/// division rounded toward zero, as in Dogecoin's DigiShield calculation.
/// The filtered interval is bounded to 19..=38 seconds. This function has no
/// minimum-difficulty exception.
///
/// Returns `None` for an invalid previous target.
pub fn next_target(
    previous: CompactDifficulty,
    actual_timespan: i64,
    pow_limit: ExpandedDifficulty,
) -> Option<CompactDifficulty> {
    let previous = previous.to_expanded()?;
    let limit = U256::from(pow_limit);
    let deviation = actual_timespan.saturating_sub(TARGET_SPACING);
    let filtered = TARGET_SPACING.saturating_add(deviation / 8).clamp(19, 38);
    let spacing = U256::from(TARGET_SPACING);
    let filtered = U256::from(filtered);
    let (quotient, remainder) = U256::from(previous).div_mod(spacing);

    // Compute floor(previous * filtered / spacing) without overflowing U256.
    // If the quotient alone exceeds the limit, the final target is the limit.
    if quotient > limit / filtered {
        return Some(pow_limit.to_compact());
    }
    let target = (quotient * filtered)
        .checked_add(remainder * filtered / spacing)
        .unwrap_or(limit);
    let target = target.min(limit);
    Some(ExpandedDifficulty::from(target).to_compact())
}

#[cfg(test)]
mod tests {
    use hex::FromHex;

    use super::*;

    fn bits(value: &str) -> CompactDifficulty {
        CompactDifficulty::from_hex(value).expect("test compact target is valid")
    }

    #[test]
    fn catecash_digishield_one_block_boundaries() {
        let previous = bits("1e00ffff");
        let limit = bits("1f00ffff").to_expanded().unwrap();

        assert_eq!(next_target(previous, 25, limit), Some(previous));
        assert_eq!(next_target(previous, 0, limit), Some(bits("1e00e146")));
        assert_eq!(next_target(previous, -60, limit), Some(bits("1e00c28e")));
        assert_eq!(next_target(previous, 300, limit), Some(bits("1e01851d")));
        assert_eq!(
            next_target(previous, -10_000, limit),
            next_target(previous, -60, limit)
        );
        assert_eq!(
            next_target(previous, 10_000, limit),
            next_target(previous, 300, limit)
        );
    }

    #[test]
    fn retarget_never_exceeds_pow_limit() {
        let limit = bits("1e00ffff").to_expanded().unwrap();
        assert_eq!(
            next_target(bits("1e00ffff"), 300, limit),
            Some(bits("1e00ffff"))
        );
    }

    #[test]
    fn invalid_previous_target_is_rejected() {
        let limit = bits("1f00ffff").to_expanded().unwrap();
        assert_eq!(next_target(CompactDifficulty(u32::MAX), 25, limit), None);
    }

    #[test]
    fn retarget_supports_the_development_pow_limit() {
        let limit = bits("207fffff").to_expanded().unwrap();
        assert_eq!(
            next_target(bits("207fffff"), 25, limit),
            Some(bits("207fffff"))
        );
        assert_eq!(
            next_target(bits("207fffff"), 300, limit),
            Some(bits("207fffff"))
        );
        assert!(
            next_target(bits("207fffff"), 0, limit)
                .unwrap()
                .to_expanded()
                .unwrap()
                < limit
        );
    }
}
