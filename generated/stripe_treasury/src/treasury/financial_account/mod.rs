/// Stripe Treasury provides users with a container for money called a FinancialAccount that is separate from their Payments balance.
/// FinancialAccounts serve as the source and destination of Treasury’s money movement APIs.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FinancialAccount {
    /// The array of paths to active Features in the Features hash.
#[serde(skip_serializing_if = "Option::is_none")]
pub active_features: Option<Vec<Array>>,
pub balance: stripe_treasury::treasury::financial_account::balance::Balance,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
pub country: String,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
pub created: stripe_types::Timestamp,
#[serde(skip_serializing_if = "Option::is_none")]
pub features: Option<stripe_treasury::treasury::financial_account_features::FinancialAccountFeatures>,
    /// The set of credentials that resolve to a FinancialAccount.
pub financial_addresses: Vec<stripe_treasury::treasury::financial_account::financial_address::FinancialAddress>,
    /// Unique identifier for the object.
pub id: stripe_treasury::treasury::financial_account::TreasuryFinancialAccountId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: FinancialAccountObject,
    /// The array of paths to pending Features in the Features hash.
#[serde(skip_serializing_if = "Option::is_none")]
pub pending_features: Option<Vec<Array>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
#[serde(skip_serializing_if = "Option::is_none")]
pub platform_restrictions: Option<stripe_treasury::treasury::financial_account_features::platform_restriction::PlatformRestriction>,
    /// The array of paths to restricted Features in the Features hash.
#[serde(skip_serializing_if = "Option::is_none")]
pub restricted_features: Option<Vec<Array>>,
    /// The enum specifying what state the account is in.
pub status: FinancialAccountStatus,
pub status_details: stripe_treasury::treasury::financial_account::status_details::StatusDetails,
    /// The currencies the FinancialAccount can hold a balance in.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
pub supported_currencies: Vec<String>,

}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAccountObject {
    TreasuryFinancialAccount,
}

impl FinancialAccountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryFinancialAccount => "treasury.financial_account",
        }
    }
}

impl std::str::FromStr for FinancialAccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "treasury.financial_account" => Ok(Self::TreasuryFinancialAccount),

            _ => Err(()),
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
impl serde::Serialize for FinancialAccountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAccountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FinancialAccountObject"))
    }
}
/// The enum specifying what state the account is in.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for FinancialAccountStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),

            _ => Err(()),
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
impl serde::Serialize for FinancialAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FinancialAccountStatus"))
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Array {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl Array {
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

impl std::str::FromStr for Array {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "card_issuing" => Ok(Self::CardIssuing),
            "deposit_insurance" => Ok(Self::DepositInsurance),
            "financial_addresses.aba" => Ok(Self::FinancialAddressesAba),
            "inbound_transfers.ach" => Ok(Self::InboundTransfersAch),
            "intra_stripe_flows" => Ok(Self::IntraStripeFlows),
            "outbound_payments.ach" => Ok(Self::OutboundPaymentsAch),
            "outbound_payments.us_domestic_wire" => Ok(Self::OutboundPaymentsUsDomesticWire),
            "outbound_transfers.ach" => Ok(Self::OutboundTransfersAch),
            "outbound_transfers.us_domestic_wire" => Ok(Self::OutboundTransfersUsDomesticWire),
            "remote_deposit_capture" => Ok(Self::RemoteDepositCapture),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Array {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Array {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for Array"))
    }
}
impl stripe_types::Object for FinancialAccount {
    type Id = stripe_treasury::treasury::financial_account::TreasuryFinancialAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryFinancialAccountId);
pub mod ach_toggle_settings;
pub use ach_toggle_settings::AchToggleSettings;
pub mod balance;
pub use balance::Balance;
pub mod financial_address;
pub use financial_address::FinancialAddress;
pub mod status_details;
pub use status_details::StatusDetails;
pub mod toggle_settings;
pub use toggle_settings::ToggleSettings;
