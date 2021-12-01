// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "invoice_payment_method_options_acss_debit".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsAcssDebit {
    pub mandate_options: Box<Option<InvoicePaymentMethodOptionsAcssDebitMandateOptions>>,

    /// Bank account verification method.
    pub verification_method: Box<Option<InvoicePaymentMethodOptionsAcssDebitVerificationMethod>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    pub transaction_type:
        Box<Option<InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType>>,
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => {
                "business"
            }
            InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => {
                "personal"
            }
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            InvoicePaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            InvoicePaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
