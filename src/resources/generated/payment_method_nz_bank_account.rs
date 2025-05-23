// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_nz_bank_account".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodNzBankAccount {

    /// The name on the bank account.
    ///
    /// Only present if the account holder name is different from the name of the authorized signatory collected in the PaymentMethod’s billing details.
    pub account_holder_name: Option<String>,

    /// The numeric code for the bank account's bank.
    pub bank_code: String,

    /// The name of the bank.
    pub bank_name: String,

    /// The numeric code for the bank account's bank branch.
    pub branch_code: String,

    /// Last four digits of the bank account number.
    pub last4: String,

    /// The suffix of the bank account number.
    pub suffix: Option<String>,
}
