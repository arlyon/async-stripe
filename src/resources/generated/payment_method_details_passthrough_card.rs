// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_details_passthrough_card".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsPassthroughCard {

    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `link`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: Option<i64>,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: Option<i64>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,
}
