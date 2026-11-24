//! Candidate Catecash mainnet network constants; subject to public review.

/// Candidate derivation coin type. Registration with SLIP-44 is pending.
pub const COIN_TYPE: u32 = 34333;
/// Sapling extended spending key HRP.
pub const HRP_SAPLING_EXTENDED_SPENDING_KEY: &str = "secret-extended-key-cate";
/// Sapling extended full viewing key HRP.
pub const HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY: &str = "catezxviews";
/// Sapling payment address HRP.
pub const HRP_SAPLING_PAYMENT_ADDRESS: &str = "cates";
/// Sprout address prefix, reserved for parser separation.
pub const B58_SPROUT_ADDRESS_PREFIX: [u8; 2] = [0x06, 0x86];
/// Transparent P2PKH address prefix.
pub const B58_PUBKEY_ADDRESS_PREFIX: [u8; 2] = [0x06, 0x84];
/// Transparent P2SH address prefix.
pub const B58_SCRIPT_ADDRESS_PREFIX: [u8; 2] = [0x06, 0x85];
/// Transparent secret key prefix.
pub const B58_SECRET_KEY_PREFIX: [u8; 1] = [0xd2];
/// TEX address HRP.
pub const HRP_TEX_ADDRESS: &str = "catetex";
/// Revision 0 Unified Address HRP.
pub const HRP_UNIFIED_ADDRESS: &str = "cate";
/// Revision 0 Unified FVK HRP.
pub const HRP_UNIFIED_FVK: &str = "cateview";
/// Revision 0 Unified IVK HRP.
pub const HRP_UNIFIED_IVK: &str = "cateivk";
