// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_details_card_installments_plan".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PaymentMethodDetailsCardInstallmentsPlanInterval>,

    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: PaymentMethodDetailsCardInstallmentsPlanType,
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardInstallmentsPlan`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardInstallmentsPlanInterval {
    Month,
}

impl PaymentMethodDetailsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardInstallmentsPlanInterval::Month => "month",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardInstallmentsPlan`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardInstallmentsPlanType {
    FixedCount,
}

impl PaymentMethodDetailsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardInstallmentsPlanType::FixedCount => "fixed_count",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
