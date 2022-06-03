// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "invoice_payment_method_options_customer_balance".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<InvoicePaymentMethodOptionsCustomerBalanceFundingType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `jp_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsCustomerBalance`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsCustomerBalanceFundingType::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}
