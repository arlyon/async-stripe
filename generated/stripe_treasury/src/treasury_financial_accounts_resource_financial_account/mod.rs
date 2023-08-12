/// Stripe Treasury provides users with a container for money called a FinancialAccount that is separate from their Payments balance.
/// FinancialAccounts serve as the source and destination of Treasury’s money movement APIs.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAccount {
    /// The array of paths to active Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_features: Option<Vec<Array>>,
    pub balance: stripe_treasury::TreasuryFinancialAccountsResourceBalance,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAccountFeatures>,
    /// The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses: Vec<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddress>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury_financial_accounts_resource_financial_account::TreasuryFinancialAccountId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The array of paths to pending Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_features: Option<Vec<Array>>,
    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<stripe_treasury::TreasuryFinancialAccountsResourcePlatformRestrictions>,
    /// The array of paths to restricted Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_features: Option<Vec<Array>>,
    /// The enum specifying what state the account is in.
    pub status: TreasuryFinancialAccountsResourceFinancialAccountStatus,
    pub status_details: stripe_treasury::TreasuryFinancialAccountsResourceStatusDetails,
    /// The currencies the FinancialAccount can hold a balance in.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}
/// The enum specifying what state the account is in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAccountStatus {
    Closed,
    Open,
}

impl TreasuryFinancialAccountsResourceFinancialAccountStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceFinancialAccountStatus::*;
        match self {
            Closed => "closed",
            Open => "open",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceFinancialAccountStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceFinancialAccountStatus::*;
        match s {
            "closed" => Ok(Closed),
            "open" => Ok(Open),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceFinancialAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceFinancialAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceFinancialAccountStatus"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use Array::*;
        match self {
            CardIssuing => "card_issuing",
            DepositInsurance => "deposit_insurance",
            FinancialAddressesAba => "financial_addresses.aba",
            InboundTransfersAch => "inbound_transfers.ach",
            IntraStripeFlows => "intra_stripe_flows",
            OutboundPaymentsAch => "outbound_payments.ach",
            OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            OutboundTransfersAch => "outbound_transfers.ach",
            OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl std::str::FromStr for Array {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Array::*;
        match s {
            "card_issuing" => Ok(CardIssuing),
            "deposit_insurance" => Ok(DepositInsurance),
            "financial_addresses.aba" => Ok(FinancialAddressesAba),
            "inbound_transfers.ach" => Ok(InboundTransfersAch),
            "intra_stripe_flows" => Ok(IntraStripeFlows),
            "outbound_payments.ach" => Ok(OutboundPaymentsAch),
            "outbound_payments.us_domestic_wire" => Ok(OutboundPaymentsUsDomesticWire),
            "outbound_transfers.ach" => Ok(OutboundTransfersAch),
            "outbound_transfers.us_domestic_wire" => Ok(OutboundTransfersUsDomesticWire),
            "remote_deposit_capture" => Ok(RemoteDepositCapture),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for Array"))
    }
}
impl stripe_types::Object for TreasuryFinancialAccountsResourceFinancialAccount {
    type Id = stripe_treasury::treasury_financial_accounts_resource_financial_account::TreasuryFinancialAccountId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TreasuryFinancialAccountId);
#[cfg(feature = "treasury_financial_accounts_resource_financial_account")]
mod requests;
#[cfg(feature = "treasury_financial_accounts_resource_financial_account")]
pub use requests::*;
