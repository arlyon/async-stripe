// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_options_customer_balance_eu_bank_account".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsCustomerBalanceEuBankAccount {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: PaymentMethodOptionsCustomerBalanceEuBankAccountCountry,
}

/// An enum representing the possible values of an `PaymentMethodOptionsCustomerBalanceEuBankAccount`'s `country` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "NL")]
    Nl,
}

impl PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Be => "BE",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::De => "DE",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Es => "ES",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Fr => "FR",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Ie => "IE",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Nl => "NL",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn default() -> Self {
        Self::Be
    }
}
