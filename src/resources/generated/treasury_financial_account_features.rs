// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::params::Object;
use crate::resources::AccountServiceResourceToggleSettings;

/// The resource representing a Stripe "AccountServiceResourceTreasuryFinancialAccountFeatures".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<AccountServiceResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<AccountServiceResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<AccountServiceResourceFinancialAddressesFeatures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<AccountServiceResourceInboundTransfers>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<AccountServiceResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<AccountServiceResourceOutboundPayments>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<AccountServiceResourceOutboundTransfers>,
}

impl Object for TreasuryFinancialAccountFeatures {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "treasury.financial_account_features"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceFinancialAddressesFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<AccountServiceResourceToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceInboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AccountServiceResourceToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceOutboundPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AccountServiceResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<AccountServiceResourceToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceOutboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AccountServiceResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<AccountServiceResourceToggleSettings>,
}
