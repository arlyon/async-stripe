/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>,
}
