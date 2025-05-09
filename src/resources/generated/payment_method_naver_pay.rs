// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_naver_pay".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodNaverPay {

    /// Uniquely identifies this particular Naver Pay account.
    ///
    /// You can use this attribute to check whether two Naver Pay accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,

    /// Whether to fund this transaction with Naver Pay points or a card.
    pub funding: PaymentMethodNaverPayFunding,
}

/// An enum representing the possible values of an `PaymentMethodNaverPay`'s `funding` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodNaverPayFunding {
    Card,
    Points,
}

impl PaymentMethodNaverPayFunding {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodNaverPayFunding::Card => "card",
            PaymentMethodNaverPayFunding::Points => "points",
        }
    }
}

impl AsRef<str> for PaymentMethodNaverPayFunding {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodNaverPayFunding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodNaverPayFunding {
    fn default() -> Self {
        Self::Card
    }
}
