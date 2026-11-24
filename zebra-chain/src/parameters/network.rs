//! Consensus parameters for each Zcash network.

use std::{fmt, str::FromStr, sync::Arc};

use thiserror::Error;

use crate::{
    amount::{Amount, NonNegative},
    block::{self, Height},
    parameters::NetworkUpgrade,
    transparent,
};

pub mod catecash;
pub mod catecash_mainnet;
pub mod catecash_testnet;
mod error;
pub mod magic;
pub mod subsidy;
pub mod testnet;

#[cfg(test)]
mod tests;

// Mainnet temporary Orchard-disabling soft-fork height, shipped publicly in Zebra v4.5.3.
// This is DISTINCT from the NU6.2 *activation* (re-enable) height (3_364_600, see
// `network_upgrade.rs`), which lands 1_174 blocks later. Do NOT change this value: it is
// already deployed, so changing it would fork from live v4.5.3 nodes in the disable window.
const MAINNET_TEMPORARY_ORCHARD_DISABLING_SOFT_FORK_HEIGHT: Height = Height(3_363_426);

// Default Testnet temporary Orchard-disabling soft-fork height. As on Mainnet, this is DISTINCT
// from the NU6.2 *activation* (re-enable) height (4_052_000, see `network_upgrade.rs`), which
// lands 3_500 blocks later.
const TESTNET_TEMPORARY_ORCHARD_DISABLING_SOFT_FORK_HEIGHT: Height = Height(4_048_500);

/// An enum describing the kind of network, whether it's the production mainnet or a testnet.
// Note: The order of these variants is important for correct bincode (de)serialization
//       of history trees in the db format.
// TODO: Replace bincode (de)serialization of `HistoryTreeParts` in a db format upgrade?
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NetworkKind {
    /// The production mainnet.
    #[default]
    Mainnet,

    /// A test network.
    Testnet,

    /// Regtest mode
    Regtest,

    /// The independent Catecash development network.
    CatecashDevnet,
    /// The candidate independent Catecash mainnet.
    CatecashMainnet,
    /// The independent Catecash public test network.
    CatecashTestnet,
}

impl From<Network> for NetworkKind {
    fn from(net: Network) -> Self {
        NetworkKind::from(&net)
    }
}

impl From<&Network> for NetworkKind {
    fn from(net: &Network) -> Self {
        net.kind()
    }
}

/// An enum describing the possible network choices.
#[derive(Clone, Default, Eq, PartialEq, Serialize)]
#[serde(into = "NetworkKind")]
pub enum Network {
    /// The production mainnet.
    #[default]
    Mainnet,

    /// A test network such as the default public testnet,
    /// a configured testnet, or Regtest.
    Testnet(Arc<testnet::Parameters>),

    /// The independent Catecash development network.
    CatecashDevnet,
    /// The candidate independent Catecash mainnet.
    CatecashMainnet,
    /// The independent Catecash public test network.
    CatecashTestnet,
}

impl NetworkKind {
    /// Returns the human-readable prefix for Base58Check-encoded transparent
    /// pay-to-public-key-hash payment addresses for the network.
    pub fn b58_pubkey_address_prefix(self) -> [u8; 2] {
        match self {
            Self::Mainnet => zcash_address::constants::mainnet::B58_PUBKEY_ADDRESS_PREFIX,
            Self::Testnet | Self::Regtest => {
                zcash_address::constants::testnet::B58_PUBKEY_ADDRESS_PREFIX
            }
            Self::CatecashDevnet => {
                zcash_address::constants::catecash_devnet::B58_PUBKEY_ADDRESS_PREFIX
            }
            Self::CatecashMainnet => {
                zcash_address::constants::catecash_mainnet::B58_PUBKEY_ADDRESS_PREFIX
            }
            Self::CatecashTestnet => {
                zcash_address::constants::catecash_testnet::B58_PUBKEY_ADDRESS_PREFIX
            }
        }
    }

    /// Returns the human-readable prefix for Base58Check-encoded transparent pay-to-script-hash
    /// payment addresses for the network.
    pub fn b58_script_address_prefix(self) -> [u8; 2] {
        match self {
            Self::Mainnet => zcash_address::constants::mainnet::B58_SCRIPT_ADDRESS_PREFIX,
            Self::Testnet | Self::Regtest => {
                zcash_address::constants::testnet::B58_SCRIPT_ADDRESS_PREFIX
            }
            Self::CatecashDevnet => {
                zcash_address::constants::catecash_devnet::B58_SCRIPT_ADDRESS_PREFIX
            }
            Self::CatecashMainnet => {
                zcash_address::constants::catecash_mainnet::B58_SCRIPT_ADDRESS_PREFIX
            }
            Self::CatecashTestnet => {
                zcash_address::constants::catecash_testnet::B58_SCRIPT_ADDRESS_PREFIX
            }
        }
    }

    /// Return the network name as defined in
    /// [BIP70](https://github.com/bitcoin/bips/blob/master/bip-0070.mediawiki#paymentdetailspaymentrequest)
    pub fn bip70_network_name(&self) -> String {
        match self {
            Self::Mainnet => "main".to_string(),
            Self::Testnet | Self::Regtest => "test".to_string(),
            Self::CatecashDevnet => "catecash-devnet".to_string(),
            Self::CatecashMainnet => "catecash-mainnet".to_string(),
            Self::CatecashTestnet => "catecash-testnet".to_string(),
        }
    }

    /// Returns the 2 bytes prefix for Bech32m-encoded transparent TEX
    /// payment addresses for the network as defined in [ZIP-320](https://zips.z.cash/zip-0320.html).
    pub fn tex_address_prefix(self) -> [u8; 2] {
        // TODO: Add this bytes to `zcash_protocol::constants`?
        match self {
            Self::Mainnet => [0x1c, 0xb8],
            Self::Testnet | Self::Regtest => [0x1d, 0x25],
            Self::CatecashDevnet => {
                zcash_address::constants::catecash_devnet::B58_PUBKEY_ADDRESS_PREFIX
            }
            Self::CatecashMainnet => {
                zcash_address::constants::catecash_mainnet::B58_PUBKEY_ADDRESS_PREFIX
            }
            Self::CatecashTestnet => {
                zcash_address::constants::catecash_testnet::B58_PUBKEY_ADDRESS_PREFIX
            }
        }
    }
}

impl From<NetworkKind> for &'static str {
    fn from(network: NetworkKind) -> &'static str {
        // These should be different from the `Display` impl for `Network` so that its lowercase form
        // can't be parsed as the default Testnet in the `Network` `FromStr` impl, it's easy to
        // distinguish them in logs, and so it's generally harder to confuse the two.
        match network {
            NetworkKind::Mainnet => "MainnetKind",
            NetworkKind::Testnet => "TestnetKind",
            NetworkKind::Regtest => "RegtestKind",
            NetworkKind::CatecashDevnet => "CatecashDevnetKind",
            NetworkKind::CatecashMainnet => "CatecashMainnetKind",
            NetworkKind::CatecashTestnet => "CatecashTestnetKind",
        }
    }
}

impl fmt::Display for NetworkKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str((*self).into())
    }
}

impl<'a> From<&'a Network> for &'a str {
    fn from(network: &'a Network) -> &'a str {
        match network {
            Network::Mainnet => "Mainnet",
            Network::Testnet(params) => params.network_name(),
            Network::CatecashDevnet => "CatecashDevnet",
            Network::CatecashMainnet => "CatecashMainnet",
            Network::CatecashTestnet => "CatecashTestnet",
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl std::fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mainnet => write!(f, "{self}"),
            Self::Testnet(params) if params.is_regtest() => f
                .debug_struct("Regtest")
                .field("activation_heights", params.activation_heights())
                .field("funding_streams", params.funding_streams())
                .field("lockbox_disbursements", &params.lockbox_disbursements())
                .field("checkpoints", &params.checkpoints())
                .finish(),
            Self::Testnet(params) if params.is_default_testnet() => {
                write!(f, "{self}")
            }
            Self::Testnet(params) => f.debug_tuple("ConfiguredTestnet").field(params).finish(),
            Self::CatecashDevnet => f.write_str("CatecashDevnet"),
            Self::CatecashMainnet => f.write_str("CatecashMainnet"),
            Self::CatecashTestnet => f.write_str("CatecashTestnet"),
        }
    }
}

impl Network {
    /// Creates a new [`Network::Testnet`] with the default Testnet [`testnet::Parameters`].
    pub fn new_default_testnet() -> Self {
        Self::Testnet(Arc::new(testnet::Parameters::default()))
    }

    /// Creates a new configured [`Network::Testnet`] with the provided Testnet [`testnet::Parameters`].
    pub fn new_configured_testnet(params: testnet::Parameters) -> Self {
        Self::Testnet(Arc::new(params))
    }

    /// Creates a new [`Network::Testnet`] with `Regtest` parameters and the provided network upgrade activation heights.
    pub fn new_regtest(params: testnet::RegtestParameters) -> Self {
        Self::new_configured_testnet(
            testnet::Parameters::new_regtest(params)
                .expect("regtest parameters should always be valid"),
        )
    }

    /// Returns true if the network is the default Testnet, or false otherwise.
    pub fn is_default_testnet(&self) -> bool {
        if let Self::Testnet(params) = self {
            params.is_default_testnet()
        } else {
            false
        }
    }

    /// Returns true if the network is Regtest, or false otherwise.
    pub fn is_regtest(&self) -> bool {
        if let Self::Testnet(params) = self {
            params.is_regtest()
        } else {
            false
        }
    }

    /// Returns true for the isolated Catecash development profile.
    pub fn is_catecash_devnet(&self) -> bool {
        matches!(self, Self::CatecashDevnet)
    }

    /// Returns true for either Catecash network.
    pub fn is_catecash(&self) -> bool {
        matches!(
            self,
            Self::CatecashDevnet | Self::CatecashMainnet | Self::CatecashTestnet
        )
    }

    /// Returns the [`NetworkKind`] for this network.
    pub fn kind(&self) -> NetworkKind {
        match self {
            Network::Mainnet => NetworkKind::Mainnet,
            Network::Testnet(params) if params.is_regtest() => NetworkKind::Regtest,
            Network::Testnet(_) => NetworkKind::Testnet,
            Network::CatecashDevnet => NetworkKind::CatecashDevnet,
            Network::CatecashMainnet => NetworkKind::CatecashMainnet,
            Network::CatecashTestnet => NetworkKind::CatecashTestnet,
        }
    }

    /// Returns [`NetworkKind::Testnet`] on Testnet and Regtest, or [`NetworkKind::Mainnet`] on Mainnet.
    ///
    /// This is used for transparent addresses, as the address prefix is the same on Regtest as it is on Testnet.
    pub fn t_addr_kind(&self) -> NetworkKind {
        match self {
            Network::Mainnet => NetworkKind::Mainnet,
            Network::Testnet(_) => NetworkKind::Testnet,
            Network::CatecashDevnet => NetworkKind::CatecashDevnet,
            Network::CatecashMainnet => NetworkKind::CatecashMainnet,
            Network::CatecashTestnet => NetworkKind::CatecashTestnet,
        }
    }

    /// Returns an iterator over [`Network`] variants.
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Mainnet, Self::new_default_testnet()].into_iter()
    }

    /// Returns true if the maximum block time rule is active for `network` and `height`.
    ///
    /// Always returns true if `network` is the Mainnet.
    /// If `network` is the Testnet, the `height` should be at least
    /// TESTNET_MAX_TIME_START_HEIGHT to return true.
    /// Returns false otherwise.
    ///
    /// Part of the consensus rules at <https://zips.z.cash/protocol/protocol.pdf#blockheader>
    pub fn is_max_block_time_enforced(&self, height: block::Height) -> bool {
        match self {
            Network::Mainnet => true,
            // TODO: Move `TESTNET_MAX_TIME_START_HEIGHT` to a field on testnet::Parameters (#8364)
            Network::Testnet(_params) => height >= super::TESTNET_MAX_TIME_START_HEIGHT,
            Network::CatecashDevnet => false,
            Network::CatecashMainnet => true,
            Network::CatecashTestnet => false,
        }
    }

    /// Get the default port associated to this network.
    pub fn default_port(&self) -> u16 {
        match self {
            Network::Mainnet => 8233,
            // TODO: Add a `default_port` field to `testnet::Parameters` to return here. (zcashd uses 18344 for Regtest)
            Network::Testnet(_params) => 18233,
            Network::CatecashDevnet => catecash::P2P_PORT,
            Network::CatecashMainnet => catecash_mainnet::P2P_PORT,
            Network::CatecashTestnet => catecash_testnet::P2P_PORT,
        }
    }

    /// Minimum number of blocks before a transparent coinbase output can be spent.
    pub fn transparent_coinbase_maturity(&self) -> u32 {
        if self.is_catecash() {
            catecash::COINBASE_MATURITY
        } else {
            transparent::MIN_TRANSPARENT_COINBASE_MATURITY
        }
    }

    /// Get the mandatory minimum checkpoint height for this network.
    ///
    /// Mandatory checkpoints are a Zebra-specific feature.
    /// If a Zcash consensus rule only applies before the mandatory checkpoint,
    /// Zebra can skip validation of that rule.
    /// This is necessary because Zebra can't fully validate the blocks prior to Canopy.
    // TODO:
    // - Support constructing pre-Canopy coinbase tx and block templates and return `Height::MAX` instead of panicking
    //   when Canopy activation height is `None` (#8434)
    pub fn mandatory_checkpoint_height(&self) -> Height {
        // Currently this is just before Canopy activation
        NetworkUpgrade::Canopy
            .activation_height(self)
            .expect("Canopy activation height must be present on all networks")
            .previous()
            .expect("Canopy activation height must be above min height")
    }

    /// Return the network name as defined in
    /// [BIP70](https://github.com/bitcoin/bips/blob/master/bip-0070.mediawiki#paymentdetailspaymentrequest)
    pub fn bip70_network_name(&self) -> String {
        self.kind().bip70_network_name()
    }

    /// Return the lowercase network name.
    pub fn lowercase_name(&self) -> String {
        self.to_string().to_ascii_lowercase()
    }

    /// Returns the network's persistent storage namespace.
    ///
    /// Mainnet, public Testnet, and Regtest retain their existing directory names.
    /// Configured Testnets include their wire magic: distinct networks must use distinct magic,
    /// even when they share a human-readable name.
    pub fn directory_name(&self) -> String {
        let mut name = self.lowercase_name();
        if matches!(self, Self::Testnet(params) if !params.is_regtest() && !params.is_default_testnet())
            || self.is_catecash()
        {
            use fmt::Write;
            write!(name, "-{:08x}", u32::from_be_bytes(self.magic().0))
                .expect("writing to a String cannot fail");
        }
        name
    }

    /// Returns `true` if this network is a testing network.
    pub fn is_a_test_network(&self) -> bool {
        !matches!(self, Network::Mainnet | Network::CatecashMainnet)
    }

    /// Returns the Sapling activation height for this network.
    // TODO: Return an `Option` here now that network upgrade activation heights are configurable on Regtest and custom Testnets
    pub fn sapling_activation_height(&self) -> Height {
        super::NetworkUpgrade::Sapling
            .activation_height(self)
            .expect("Sapling activation height needs to be set")
    }

    /// Returns the expected total value of the sum of all NU6.1 one-time lockbox disbursement output values for this network at
    /// the provided height.
    pub fn lockbox_disbursement_total_amount(&self, height: Height) -> Amount<NonNegative> {
        if Some(height) != NetworkUpgrade::Nu6_1.activation_height(self) {
            return Amount::zero();
        };

        match self {
            Self::Mainnet => {
                subsidy::constants::mainnet::EXPECTED_NU6_1_LOCKBOX_DISBURSEMENTS_TOTAL
            }
            Self::Testnet(params) if params.is_default_testnet() => {
                subsidy::constants::testnet::EXPECTED_NU6_1_LOCKBOX_DISBURSEMENTS_TOTAL
            }
            Self::Testnet(params) => params.lockbox_disbursement_total_amount(),
            Self::CatecashDevnet => Amount::zero(),
            Self::CatecashMainnet => Amount::zero(),
            Self::CatecashTestnet => Amount::zero(),
        }
    }

    /// Returns the expected NU6.1 lockbox disbursement outputs for this network at the provided height.
    pub fn lockbox_disbursements(
        &self,
        height: Height,
    ) -> Vec<(transparent::Address, Amount<NonNegative>)> {
        if Some(height) != NetworkUpgrade::Nu6_1.activation_height(self) {
            return Vec::new();
        };

        let expected_lockbox_disbursements = match self {
            Self::Mainnet => subsidy::constants::mainnet::NU6_1_LOCKBOX_DISBURSEMENTS.to_vec(),
            Self::Testnet(params) if params.is_default_testnet() => {
                subsidy::constants::testnet::NU6_1_LOCKBOX_DISBURSEMENTS.to_vec()
            }
            Self::Testnet(params) => return params.lockbox_disbursements(),
            Self::CatecashDevnet => return Vec::new(),
            Self::CatecashMainnet => return Vec::new(),
            Self::CatecashTestnet => return Vec::new(),
        };

        expected_lockbox_disbursements
            .into_iter()
            .map(|(addr, amount)| {
                (
                    addr.parse().expect("hard-coded address must deserialize"),
                    amount,
                )
            })
            .collect()
    }

    /// Returns the height at which the soft fork that temporarily disables Orchard
    /// actions in transactions activates, if it is configured for this network.
    pub fn temporary_orchard_disabling_soft_fork_height(&self) -> Option<Height> {
        match self {
            Network::Mainnet => Some(MAINNET_TEMPORARY_ORCHARD_DISABLING_SOFT_FORK_HEIGHT),
            Network::Testnet(parameters) => {
                parameters.temporary_orchard_disabling_soft_fork_height()
            }
            Network::CatecashDevnet => None,
            Network::CatecashMainnet => None,
            Network::CatecashTestnet => None,
        }
    }

    /// Returns whether Orchard has been temporarily disabled in transactions.
    pub fn temporary_orchard_disabling_soft_fork_active(&self, height: Height) -> bool {
        self.temporary_orchard_disabling_soft_fork_height()
            .is_some_and(|h| height >= h)
    }

    /// Returns whether Orchard is temporarily disabled in transactions at `height`.
    ///
    /// The temporary-disable soft fork is bounded above by NU6.2, which re-enables
    /// Orchard actions: once NU6.2 is active the temporary-disable rule no longer
    /// applies. On networks where NU6.2 is unscheduled this matches
    /// [`Self::temporary_orchard_disabling_soft_fork_active`].
    pub fn is_orchard_temporarily_disabled(&self, height: Height) -> bool {
        self.temporary_orchard_disabling_soft_fork_active(height)
            && NetworkUpgrade::Nu6_2
                .activation_height(self)
                .is_none_or(|nu6_2| height < nu6_2)
    }

    /// Returns whether `height` is the first height at which the soft fork that
    /// temporarily disables Orchard actions applies.
    ///
    /// This is the boundary at which the mempool must revalidate its contents, to drop
    /// any transactions containing Orchard actions that were accepted before the soft
    /// fork activated.
    pub fn is_temporary_orchard_disabling_soft_fork_activation_height(
        &self,
        height: Height,
    ) -> bool {
        self.temporary_orchard_disabling_soft_fork_height() == Some(height)
    }

    /// Returns whether the consensus rule requiring a canonically-sized Orchard proof
    /// is active at `height`.
    ///
    /// This rule activates with the network upgrade that re-enables Orchard actions
    /// (NU6.2). On networks where NU6.2 is unscheduled the rule is always inactive. It is a
    /// constricting rule, so it must stay height-gated, or it would reject historical
    /// Orchard actions mined before the soft fork that temporarily disabled them, and
    /// prevent syncing.
    pub fn orchard_canonical_proof_size_rule_active(&self, height: Height) -> bool {
        NetworkUpgrade::Nu6_2
            .activation_height(self)
            .is_some_and(|h| height >= h)
    }
}

// This is used for parsing a command-line argument for the `TipHeight` command in zebrad.
impl FromStr for Network {
    type Err = InvalidNetworkError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "mainnet" => Ok(Network::Mainnet),
            "testnet" => Ok(Network::new_default_testnet()),
            "catecash-testnet" => Ok(Network::CatecashTestnet),
            _ => Err(InvalidNetworkError(string.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Error)]
#[error("Invalid network: {0}")]
pub struct InvalidNetworkError(String);

impl zcash_protocol::consensus::Parameters for Network {
    fn network_type(&self) -> zcash_protocol::consensus::NetworkType {
        self.kind().into()
    }

    fn activation_height(
        &self,
        nu: zcash_protocol::consensus::NetworkUpgrade,
    ) -> Option<zcash_protocol::consensus::BlockHeight> {
        NetworkUpgrade::from(nu)
            .activation_height(self)
            .map(|Height(h)| zcash_protocol::consensus::BlockHeight::from_u32(h))
    }

    fn branch_id_override(
        &self,
        height: zcash_protocol::consensus::BlockHeight,
    ) -> Option<zcash_protocol::consensus::BranchId> {
        (self.is_catecash_devnet()
            && NetworkUpgrade::current(self, Height(u32::from(height))) == NetworkUpgrade::Nu7)
            .then_some(zcash_protocol::consensus::BranchId::Catecash)
            .or_else(|| {
                (matches!(self, Self::CatecashMainnet)
                    && NetworkUpgrade::current(self, Height(u32::from(height)))
                        == NetworkUpgrade::Nu7)
                    .then_some(zcash_protocol::consensus::BranchId::CatecashMainnet)
            })
            .or_else(|| {
                (matches!(self, Self::CatecashTestnet)
                    && NetworkUpgrade::current(self, Height(u32::from(height)))
                        == NetworkUpgrade::Nu7)
                    .then_some(zcash_protocol::consensus::BranchId::CatecashTestnet)
            })
    }
}
