/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAccountFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    TreasuryFinancialAccountFeatures,
}

impl TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject::*;
        match self {
            TreasuryFinancialAccountFeatures => "treasury.financial_account_features",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject::*;
        match s {
            "treasury.financial_account_features" => Ok(TreasuryFinancialAccountFeatures),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceFinancialAccountFeaturesObject"))
    }
}
