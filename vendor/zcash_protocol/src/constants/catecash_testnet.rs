//! Catecash testnet version 1 network constants.

/// Testnet derivation coin type.
pub const COIN_TYPE: u32 = 34353;
/// Sapling extended spending key HRP.
pub const HRP_SAPLING_EXTENDED_SPENDING_KEY: &str = "secret-extended-key-catet";
/// Sapling extended full viewing key HRP.
pub const HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY: &str = "catetzxviews";
/// Sapling payment address HRP.
pub const HRP_SAPLING_PAYMENT_ADDRESS: &str = "catets";
/// Sprout address prefix, reserved for parser separation.
pub const B58_SPROUT_ADDRESS_PREFIX: [u8; 2] = [0x07, 0x86];
/// Transparent P2PKH address prefix.
pub const B58_PUBKEY_ADDRESS_PREFIX: [u8; 2] = [0x07, 0x84];
/// Transparent P2SH address prefix.
pub const B58_SCRIPT_ADDRESS_PREFIX: [u8; 2] = [0x07, 0x85];
/// Transparent secret key prefix.
pub const B58_SECRET_KEY_PREFIX: [u8; 1] = [0xd3];
/// TEX address HRP.
pub const HRP_TEX_ADDRESS: &str = "catettex";
/// Revision 0 Unified Address HRP.
pub const HRP_UNIFIED_ADDRESS: &str = "catet";
/// Revision 0 Unified FVK HRP.
pub const HRP_UNIFIED_FVK: &str = "catetview";
/// Revision 0 Unified IVK HRP.
pub const HRP_UNIFIED_IVK: &str = "catetivk";
