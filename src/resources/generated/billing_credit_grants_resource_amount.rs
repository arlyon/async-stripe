// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingCreditGrantsResourceAmount".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceAmount {

    /// The monetary amount.
    pub monetary: Option<BillingCreditGrantsResourceMonetaryAmount>,

    /// The type of this amount.
    ///
    /// We currently only support `monetary` credits.
    #[serde(rename = "type")]
    pub type_: BillingCreditGrantsResourceAmountType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceMonetaryAmount {

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// A positive integer representing the amount.
    pub value: i64,
}

/// An enum representing the possible values of an `BillingCreditGrantsResourceAmount`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingCreditGrantsResourceAmountType {
    Monetary,
}

impl BillingCreditGrantsResourceAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingCreditGrantsResourceAmountType::Monetary => "monetary",
        }
    }
}

impl AsRef<str> for BillingCreditGrantsResourceAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingCreditGrantsResourceAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingCreditGrantsResourceAmountType {
    fn default() -> Self {
        Self::Monetary
    }
}
