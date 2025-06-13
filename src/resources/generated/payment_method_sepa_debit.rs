// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable};
use crate::resources::{Charge, SetupAttempt};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_sepa_debit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodSepaDebit {

    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Information about the object that generated this PaymentMethod.
    pub generated_from: Option<SepaDebitGeneratedFrom>,

    /// Last four characters of the IBAN.
    pub last4: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SepaDebitGeneratedFrom {

    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<Expandable<Charge>>,

    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<Expandable<SetupAttempt>>,
}
