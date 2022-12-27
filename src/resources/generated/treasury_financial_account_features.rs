// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object};
use crate::resources::{TreasuryFinancialAccountsResourceAchToggleSettings, TreasuryFinancialAccountsResourceToggleSettings};

use serde::{Deserialize, Serialize};


/// The resource representing a Stripe "TreasuryFinancialAccountsResourceFinancialAccountFeatures".
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TreasuryFinancialAccountFeatures {
#[serde(skip_serializing_if = "Option::is_none")]
pub card_issuing: Option<TreasuryFinancialAccountsResourceToggleSettings>,

#[serde(skip_serializing_if = "Option::is_none")]
pub deposit_insurance: Option<TreasuryFinancialAccountsResourceToggleSettings>,

#[serde(skip_serializing_if = "Option::is_none")]
pub financial_addresses: Option<TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,

#[serde(skip_serializing_if = "Option::is_none")]
pub inbound_transfers: Option<TreasuryFinancialAccountsResourceInboundTransfers>,

#[serde(skip_serializing_if = "Option::is_none")]
pub intra_stripe_flows: Option<TreasuryFinancialAccountsResourceToggleSettings>,

#[serde(skip_serializing_if = "Option::is_none")]
pub outbound_payments: Option<TreasuryFinancialAccountsResourceOutboundPayments>,

#[serde(skip_serializing_if = "Option::is_none")]
pub outbound_transfers: Option<TreasuryFinancialAccountsResourceOutboundTransfers>,
}


impl Object for TreasuryFinancialAccountFeatures {
    type Id = ();
    fn id(&self) -> Self::Id {
        
    }
    fn object(&self) -> &'static str {
        "treasury.financial_account_features"
    }
}

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
#[serde(skip_serializing_if = "Option::is_none")]
pub aba: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
#[serde(skip_serializing_if = "Option::is_none")]
pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
}

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
#[serde(skip_serializing_if = "Option::is_none")]
pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,

#[serde(skip_serializing_if = "Option::is_none")]
pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
#[serde(skip_serializing_if = "Option::is_none")]
pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,

#[serde(skip_serializing_if = "Option::is_none")]
pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}






