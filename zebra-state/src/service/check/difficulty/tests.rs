//! Regression tests for difficulty averaging across NU7.

use hex::FromHex as _;
use zebra_chain::parameters::testnet::{ConfiguredActivationHeights, Parameters};

use super::*;

const NU7_HEIGHT: u32 = 400_000;
const PREVIOUS_TIME: i64 = 1_600_000_000;

fn testnet(nu7_height: u32) -> Network {
    Parameters::build()
        .with_slow_start_interval(zebra_chain::block::Height::MIN)
        .with_activation_heights(ConfiguredActivationHeights {
            blossom: Some(1),
            nu7: Some(nu7_height),
            ..Default::default()
        })
        .expect("activation heights are valid")
        .with_funding_streams(Vec::new())
        .to_network()
        .expect("configured Testnet parameters are valid")
}

fn recent_block_data(
    threshold: CompactDifficulty,
    spacing: i64,
) -> impl Iterator<Item = (CompactDifficulty, DateTime<Utc>)> {
    (0..MAX_POW_ADJUSTMENT_BLOCK_SPAN).map(move |index| {
        let seconds = PREVIOUS_TIME - i64::try_from(index).unwrap() * spacing;
        (threshold, DateTime::from_timestamp(seconds, 0).unwrap())
    })
}

#[test]
fn catecash_devnet_retargets_from_the_previous_two_blocks() {
    let _init_guard = zebra_test::init();
    let network = Network::CatecashDevnet;
    let limit = network.target_difficulty_limit();
    let genesis_time = DateTime::from_timestamp(PREVIOUS_TIME - 25, 0).unwrap();
    let first_time = DateTime::from_timestamp(PREVIOUS_TIME, 0).unwrap();
    let candidate_time = DateTime::from_timestamp(PREVIOUS_TIME + 25, 0).unwrap();

    assert!(pow_adjustment_block_span(&network, block::Height(1)) > 2);
    assert_eq!(pow_adjustment_block_span(&network, block::Height(2)), 2);
    let historical_genesis_time = DateTime::from_timestamp(1_795_395_600, 0).unwrap();
    let historical_first_time = DateTime::from_timestamp(1_795_406_383, 0).unwrap();
    let first = AdjustedDifficulty::new_from_header_time(
        historical_first_time,
        block::Height::MIN,
        &network,
        [(limit.to_compact(), historical_genesis_time)],
    );
    assert_eq!(
        first.expected_difficulty_threshold(),
        CompactDifficulty::from_hex("206b851d").unwrap()
    );

    let second = AdjustedDifficulty::new_from_header_time(
        candidate_time,
        block::Height(1),
        &network,
        [
            (limit.to_compact(), first_time),
            (limit.to_compact(), genesis_time),
        ],
    );
    assert_eq!(second.expected_difficulty_threshold(), limit.to_compact());

    let fast_first_time = genesis_time + Duration::seconds(1);
    let second = AdjustedDifficulty::new_from_header_time(
        candidate_time,
        block::Height(1),
        &network,
        [
            (limit.to_compact(), fast_first_time),
            (limit.to_compact(), genesis_time),
        ],
    );
    let expected =
        zebra_chain::work::digishield::next_target(limit.to_compact(), 1, limit).unwrap();
    assert_eq!(second.expected_difficulty_threshold(), expected);
    assert_ne!(expected, limit.to_compact());
}

#[test]
fn catecash_mainnet_first_mineable_target_is_fixed_from_genesis() {
    let _init_guard = zebra_test::init();
    let network = Network::CatecashMainnet;
    let genesis_time = DateTime::from_timestamp(1_795_408_991, 0).unwrap();
    let candidate_time = genesis_time + Duration::seconds(25);
    let genesis_bits = CompactDifficulty::from_hex("200fffff").unwrap();
    let adjustment = AdjustedDifficulty::new_from_header_time(
        candidate_time,
        block::Height::MIN,
        &network,
        [(genesis_bits, genesis_time)],
    );

    // Height one uses the inherited early-chain threshold path, then height
    // two and later use one-block DigiShield.
    assert_eq!(pow_adjustment_block_span(&network, block::Height(2)), 2);
    assert_eq!(
        adjustment.expected_difficulty_threshold(),
        CompactDifficulty::from_hex("200d70a2").unwrap()
    );
}

#[test]
fn nu7_testnet_pow_limit_mean_does_not_overflow() {
    let _init_guard = zebra_test::init();
    let network = testnet(NU7_HEIGHT);
    let limit = network.target_difficulty_limit();
    assert!(!network.is_regtest());
    assert_eq!(U256::from(limit), U256::from(0x07ffff_u32) << 232);

    for candidate_height in [NU7_HEIGHT - 1, NU7_HEIGHT] {
        // All 113 ancestors use Testnet's real powLimit. The 451-second gaps
        // permit those easy targets before NU7, but the candidate's 25-second
        // gap must exercise ordinary retargeting rather than minimum difficulty.
        let adjustment = AdjustedDifficulty::new_from_header_time(
            DateTime::from_timestamp(PREVIOUS_TIME + 25, 0).unwrap(),
            block::Height(candidate_height - 1),
            &network,
            recent_block_data(limit.to_compact(), 451),
        );

        assert_eq!(adjustment.mean_target_difficulty(), limit);
        assert_eq!(
            adjustment.expected_difficulty_threshold(),
            limit.to_compact()
        );
        assert_eq!(
            adjustment.median_time_past(),
            DateTime::from_timestamp(PREVIOUS_TIME - 5 * 451, 0).unwrap()
        );
        let window = if candidate_height < NU7_HEIGHT {
            17
        } else {
            102
        };
        assert_eq!(
            adjustment.median_timespan(),
            Duration::seconds(window * 451)
        );
        let bounded_timespan = if candidate_height < NU7_HEIGHT {
            1_683
        } else {
            3_366
        };
        assert_eq!(
            adjustment.median_timespan_bounded(),
            Duration::seconds(bounded_timespan)
        );
    }
}

#[test]
fn nu7_testnet_mean_preserves_nondivisible_remainders() {
    let _init_guard = zebra_test::init();
    let network = testnet(NU7_HEIGHT);
    let limit = network.target_difficulty_limit();
    let lower = (limit / 2_u64).to_compact();
    let mut context: Vec<_> = recent_block_data(limit.to_compact(), 25).collect();
    context[101].0 = lower;

    let adjustment = AdjustedDifficulty::new_from_header_time(
        DateTime::from_timestamp(PREVIOUS_TIME + 25, 0).unwrap(),
        block::Height(NU7_HEIGHT - 1),
        &network,
        context,
    );

    // floor((101 * limit + lower) / 102) = limit - ceil((limit - lower) / 102).
    // This independent expression never constructs the overflowing target sum.
    let difference = U256::from(limit) - U256::from(lower.to_expanded().unwrap());
    let window = U256::from(102_u32);
    assert_ne!(difference % window, U256::zero());
    let expected = U256::from(limit) - (difference + window - U256::one()) / window;
    assert_eq!(
        adjustment.mean_target_difficulty(),
        ExpandedDifficulty::from(expected)
    );
}

#[test]
fn mean_target_uses_pow_limit_through_exact_window_height() {
    let _init_guard = zebra_test::init();
    let network = testnet(100);
    let limit = network.target_difficulty_limit();
    let threshold = (limit / 4_u64).to_compact();
    let target = threshold.to_expanded().unwrap();

    // Exercise both the original and NU7 windows, including their genesis
    // ancestor. The first height allowed to average targets is window + 1.
    for window in [17_u32, 102] {
        for candidate_height in [window - 1, window, window + 1] {
            let adjustment = AdjustedDifficulty::new_from_header_time(
                DateTime::from_timestamp(PREVIOUS_TIME + 25, 0).unwrap(),
                block::Height(candidate_height - 1),
                &network,
                recent_block_data(threshold, 25).take(usize::try_from(candidate_height).unwrap()),
            );
            assert_eq!(
                adjustment.mean_target_difficulty(),
                if candidate_height <= window {
                    limit
                } else {
                    target
                },
                "candidate height {candidate_height} with window {window}"
            );
        }
    }
}
