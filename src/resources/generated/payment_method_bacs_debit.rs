// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_bacs_debit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodBacsDebit {

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    pub sort_code: Option<String>,
}
