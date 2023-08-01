/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FinancialAccountFeatures {
#[serde(skip_serializing_if = "Option::is_none")]
pub card_issuing: Option<stripe_treasury::treasury::financial_account::toggle_settings::ToggleSettings>,
#[serde(skip_serializing_if = "Option::is_none")]
pub deposit_insurance: Option<stripe_treasury::treasury::financial_account::toggle_settings::ToggleSettings>,
#[serde(skip_serializing_if = "Option::is_none")]
pub financial_addresses: Option<stripe_treasury::treasury::financial_account_features::financial_addresses::FinancialAddresses>,
#[serde(skip_serializing_if = "Option::is_none")]
pub inbound_transfers: Option<stripe_treasury::treasury::financial_account_features::inbound_transfers::InboundTransfers>,
#[serde(skip_serializing_if = "Option::is_none")]
pub intra_stripe_flows: Option<stripe_treasury::treasury::financial_account::toggle_settings::ToggleSettings>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: FinancialAccountFeaturesObject,
#[serde(skip_serializing_if = "Option::is_none")]
pub outbound_payments: Option<stripe_treasury::treasury::financial_account_features::outbound_payments::OutboundPayments>,
#[serde(skip_serializing_if = "Option::is_none")]
pub outbound_transfers: Option<stripe_treasury::treasury::financial_account_features::outbound_transfers::OutboundTransfers>,

}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAccountFeaturesObject {
    TreasuryFinancialAccountFeatures,
}

impl FinancialAccountFeaturesObject {
    pub fn as_str(self) -> &'static str {
        use FinancialAccountFeaturesObject::*;
        match self {
            TreasuryFinancialAccountFeatures => "treasury.financial_account_features",
        }
    }
}

impl std::str::FromStr for FinancialAccountFeaturesObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialAccountFeaturesObject::*;
        match s {
            "treasury.financial_account_features" => Ok(TreasuryFinancialAccountFeatures),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FinancialAccountFeaturesObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountFeaturesObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FinancialAccountFeaturesObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAccountFeaturesObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialAccountFeaturesObject")
        })
    }
}
pub mod financial_addresses;
pub use financial_addresses::FinancialAddresses;
pub mod inbound_transfers;
pub use inbound_transfers::InboundTransfers;
pub mod outbound_payments;
pub use outbound_payments::OutboundPayments;
pub mod outbound_transfers;
pub use outbound_transfers::OutboundTransfers;
pub mod platform_restriction;
pub use platform_restriction::PlatformRestriction;
