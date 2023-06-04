/// Stripe Treasury provides users with a container for money called a FinancialAccount that is separate from their Payments balance.
/// FinancialAccounts serve as the source and destination of Treasury’s money movement APIs.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialAccount {
    /// The array of paths to active Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_features: Option<Vec<FinancialAccountActiveFeatures>>,
    pub balance: crate::treasury::financial_account::balance::Balance,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<crate::treasury::financial_account_features::FinancialAccountFeatures>,
    /// The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses:
        Vec<crate::treasury::financial_account::financial_address::FinancialAddress>,
    /// Unique identifier for the object.
    pub id: crate::treasury::financial_account::TreasuryFinancialAccountId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::Metadata>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FinancialAccountObject,
    /// The array of paths to pending Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_features: Option<Vec<FinancialAccountPendingFeatures>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<
        crate::treasury::financial_account_features::platform_restriction::PlatformRestriction,
    >,
    /// The array of paths to restricted Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_features: Option<Vec<FinancialAccountRestrictedFeatures>>,
    /// The enum specifying what state the account is in.
    pub status: FinancialAccountStatus,
    pub status_details: crate::treasury::financial_account::status_details::StatusDetails,
    /// The currencies the FinancialAccount can hold a balance in.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAccount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The array of paths to active Features in the Features hash.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAccountActiveFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl FinancialAccountActiveFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardIssuing => "card_issuing",
            Self::DepositInsurance => "deposit_insurance",
            Self::FinancialAddressesAba => "financial_addresses.aba",
            Self::InboundTransfersAch => "inbound_transfers.ach",
            Self::IntraStripeFlows => "intra_stripe_flows",
            Self::OutboundPaymentsAch => "outbound_payments.ach",
            Self::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            Self::OutboundTransfersAch => "outbound_transfers.ach",
            Self::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            Self::RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl AsRef<str> for FinancialAccountActiveFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountActiveFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAccountObject {
    #[serde(rename = "treasury.financial_account")]
    TreasuryFinancialAccount,
}

impl FinancialAccountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryFinancialAccount => "treasury.financial_account",
        }
    }
}

impl AsRef<str> for FinancialAccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The array of paths to pending Features in the Features hash.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAccountPendingFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl FinancialAccountPendingFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardIssuing => "card_issuing",
            Self::DepositInsurance => "deposit_insurance",
            Self::FinancialAddressesAba => "financial_addresses.aba",
            Self::InboundTransfersAch => "inbound_transfers.ach",
            Self::IntraStripeFlows => "intra_stripe_flows",
            Self::OutboundPaymentsAch => "outbound_payments.ach",
            Self::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            Self::OutboundTransfersAch => "outbound_transfers.ach",
            Self::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            Self::RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl AsRef<str> for FinancialAccountPendingFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountPendingFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The array of paths to restricted Features in the Features hash.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAccountRestrictedFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl FinancialAccountRestrictedFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardIssuing => "card_issuing",
            Self::DepositInsurance => "deposit_insurance",
            Self::FinancialAddressesAba => "financial_addresses.aba",
            Self::InboundTransfersAch => "inbound_transfers.ach",
            Self::IntraStripeFlows => "intra_stripe_flows",
            Self::OutboundPaymentsAch => "outbound_payments.ach",
            Self::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            Self::OutboundTransfersAch => "outbound_transfers.ach",
            Self::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            Self::RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl AsRef<str> for FinancialAccountRestrictedFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountRestrictedFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The enum specifying what state the account is in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAccountStatus {
    Closed,
    Open,
}

impl FinancialAccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
        }
    }
}

impl AsRef<str> for FinancialAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for FinancialAccount {
    type Id = crate::treasury::financial_account::TreasuryFinancialAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TreasuryFinancialAccountId);
pub mod ach_toggle_settings;
pub mod requests;
pub use ach_toggle_settings::AchToggleSettings;
pub mod balance;
pub use balance::Balance;
pub mod financial_address;
pub use financial_address::FinancialAddress;
pub mod status_details;
pub use status_details::StatusDetails;
pub mod toggle_settings;
pub use toggle_settings::ToggleSettings;
