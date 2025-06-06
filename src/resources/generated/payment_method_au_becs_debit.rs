// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_au_becs_debit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodAuBecsDebit {

    /// Six-digit number identifying bank and branch associated with this bank account.
    pub bsb_number: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,
}
