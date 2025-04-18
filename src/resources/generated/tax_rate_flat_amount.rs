// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxRateFlatAmount".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxRateFlatAmount {

    /// Amount of the tax when the `rate_type` is `flat_amount`.
    ///
    /// This positive integer represents how much to charge in the smallest currency unit (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,

    /// Three-letter ISO currency code, in lowercase.
    pub currency: Currency,
}
