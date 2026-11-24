//! Provisional Catecash devnet network constants.

/// Development-only derivation coin type. This is not a registered SLIP-44 value.
pub const COIN_TYPE: u32 = 3239;
/// Sapling extended spending key HRP.
pub const HRP_SAPLING_EXTENDED_SPENDING_KEY: &str = "secret-extended-key-cashdev";
/// Sapling extended full viewing key HRP.
pub const HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY: &str = "cashdevzxviews";
/// Sapling payment address HRP.
pub const HRP_SAPLING_PAYMENT_ADDRESS: &str = "cashdevsapling";
/// Sprout address prefix, reserved for parser separation.
pub const B58_SPROUT_ADDRESS_PREFIX: [u8; 2] = [0x1f, 0xc2];
/// Transparent P2PKH address prefix.
pub const B58_PUBKEY_ADDRESS_PREFIX: [u8; 2] = [0x1f, 0xc0];
/// Transparent P2SH address prefix.
pub const B58_SCRIPT_ADDRESS_PREFIX: [u8; 2] = [0x1f, 0xc1];
/// Transparent secret key prefix.
pub const B58_SECRET_KEY_PREFIX: [u8; 1] = [0xf0];
/// TEX address HRP.
pub const HRP_TEX_ADDRESS: &str = "cashdevtex";
/// Revision 0 Unified Address HRP.
pub const HRP_UNIFIED_ADDRESS: &str = "cashdev";
/// Revision 0 Unified FVK HRP.
pub const HRP_UNIFIED_FVK: &str = "cashdevview";
/// Revision 0 Unified IVK HRP.
pub const HRP_UNIFIED_IVK: &str = "cashdevivk";
