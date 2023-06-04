/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialAccountFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<crate::treasury::financial_account::toggle_settings::ToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance:
        Option<crate::treasury::financial_account::toggle_settings::ToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<
        crate::treasury::financial_account_features::financial_addresses::FinancialAddresses,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers:
        Option<crate::treasury::financial_account_features::inbound_transfers::InboundTransfers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows:
        Option<crate::treasury::financial_account::toggle_settings::ToggleSettings>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FinancialAccountFeaturesObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments:
        Option<crate::treasury::financial_account_features::outbound_payments::OutboundPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers:
        Option<crate::treasury::financial_account_features::outbound_transfers::OutboundTransfers>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAccountFeatures {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAccountFeaturesObject {
    #[serde(rename = "treasury.financial_account_features")]
    TreasuryFinancialAccountFeatures,
}

impl FinancialAccountFeaturesObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryFinancialAccountFeatures => "treasury.financial_account_features",
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
