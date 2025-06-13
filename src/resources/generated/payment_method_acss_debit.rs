// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_acss_debit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodAcssDebit {

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Institution number of the bank account.
    pub institution_number: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Transit number of the bank account.
    pub transit_number: Option<String>,
}
