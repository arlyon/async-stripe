// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "invoice_payment_method_options_us_bank_account".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<InvoicePaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions:
        Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>>,

    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch>>,
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    Balances,
    PaymentMethod,
    Transactions,
}

impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::Balances => "balances",
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::PaymentMethod => "payment_method",
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    Balances,
    Transactions,
}

impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::Balances => {
                "balances"
            }
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::Transactions => {
                "transactions"
            }
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => "automatic",
            InvoicePaymentMethodOptionsUsBankAccountVerificationMethod::Instant => "instant",
            InvoicePaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}
