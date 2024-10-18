// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object};
use crate::resources::{Address};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceTaxSettings".
///
/// For more details see <https://stripe.com/docs/api/tax/settings/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxSettings {

    pub defaults: TaxProductResourceTaxSettingsDefaults,

    /// The place where your business is located.
    pub head_office: Option<TaxProductResourceTaxSettingsHeadOffice>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The status of the Tax `Settings`.
    pub status: TaxSettingsStatus,

    pub status_details: TaxProductResourceTaxSettingsStatusDetails,
}

impl Object for TaxSettings {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "tax.settings"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxSettingsDefaults {

    /// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
    pub tax_behavior: Option<TaxProductResourceTaxSettingsDefaultsTaxBehavior>,

    /// Default [tax code](https://stripe.com/docs/tax/tax-categories) used to classify your products and prices.
    pub tax_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxSettingsHeadOffice {

    pub address: Address,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxSettingsStatusDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<TaxProductResourceTaxSettingsStatusDetailsResourceActive>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourceActive {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePending {

    /// The list of missing fields that are required to perform calculations.
    ///
    /// It includes the entry `head_office` when the status is `pending`.
    /// It is recommended to set the optional values even if they aren't listed as required for calculating taxes.
    /// Calculations can fail if missing fields aren't explicitly provided on every call.
    pub missing_fields: Option<Vec<String>>,
}

/// An enum representing the possible values of an `TaxProductResourceTaxSettingsDefaults`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
}

impl TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::Exclusive => "exclusive",
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::Inclusive => "inclusive",
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::InferredByCurrency => "inferred_by_currency",
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `TaxSettings`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxSettingsStatus {
    Active,
    Pending,
}

impl TaxSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxSettingsStatus::Active => "active",
            TaxSettingsStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for TaxSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxSettingsStatus {
    fn default() -> Self {
        Self::Active
    }
}
