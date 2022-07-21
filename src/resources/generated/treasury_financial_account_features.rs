// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::params::Object;
use crate::resources::AccountServiceResourceTreasuryToggleSettings;

/// The resource representing a Stripe "AccountServiceResourceTreasuryFinancialAccountFeatures".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<AccountServiceResourceTreasuryToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<AccountServiceResourceTreasuryToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<AccountServiceResourceTreasuryFinancialAddressesFeatures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<AccountServiceResourceTreasuryInboundTransfers>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<AccountServiceResourceTreasuryToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<AccountServiceResourceTreasuryOutboundPayments>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<AccountServiceResourceTreasuryOutboundTransfers>,
}

impl Object for TreasuryFinancialAccountFeatures {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "treasury.financial_account_features"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryFinancialAddressesFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<AccountServiceResourceTreasuryToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryInboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AccountServiceResourceTreasuryToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryOutboundPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AccountServiceResourceTreasuryToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<AccountServiceResourceTreasuryToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryOutboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AccountServiceResourceTreasuryToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<AccountServiceResourceTreasuryToggleSettings>,
}
