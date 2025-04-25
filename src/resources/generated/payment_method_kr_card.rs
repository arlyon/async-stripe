// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_kr_card".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodKrCard {

    /// The local credit or debit card brand.
    pub brand: Option<PaymentMethodKrCardBrand>,

    /// The last four digits of the card.
    ///
    /// This may not be present for American Express cards.
    pub last4: Option<String>,
}

/// An enum representing the possible values of an `PaymentMethodKrCard`'s `brand` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodKrCardBrand {
    Bc,
    Citi,
    Hana,
    Hyundai,
    Jeju,
    Jeonbuk,
    Kakaobank,
    Kbank,
    Kdbbank,
    Kookmin,
    Kwangju,
    Lotte,
    Mg,
    Nh,
    Post,
    Samsung,
    Savingsbank,
    Shinhan,
    Shinhyup,
    Suhyup,
    Tossbank,
    Woori,
}

impl PaymentMethodKrCardBrand {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodKrCardBrand::Bc => "bc",
            PaymentMethodKrCardBrand::Citi => "citi",
            PaymentMethodKrCardBrand::Hana => "hana",
            PaymentMethodKrCardBrand::Hyundai => "hyundai",
            PaymentMethodKrCardBrand::Jeju => "jeju",
            PaymentMethodKrCardBrand::Jeonbuk => "jeonbuk",
            PaymentMethodKrCardBrand::Kakaobank => "kakaobank",
            PaymentMethodKrCardBrand::Kbank => "kbank",
            PaymentMethodKrCardBrand::Kdbbank => "kdbbank",
            PaymentMethodKrCardBrand::Kookmin => "kookmin",
            PaymentMethodKrCardBrand::Kwangju => "kwangju",
            PaymentMethodKrCardBrand::Lotte => "lotte",
            PaymentMethodKrCardBrand::Mg => "mg",
            PaymentMethodKrCardBrand::Nh => "nh",
            PaymentMethodKrCardBrand::Post => "post",
            PaymentMethodKrCardBrand::Samsung => "samsung",
            PaymentMethodKrCardBrand::Savingsbank => "savingsbank",
            PaymentMethodKrCardBrand::Shinhan => "shinhan",
            PaymentMethodKrCardBrand::Shinhyup => "shinhyup",
            PaymentMethodKrCardBrand::Suhyup => "suhyup",
            PaymentMethodKrCardBrand::Tossbank => "tossbank",
            PaymentMethodKrCardBrand::Woori => "woori",
        }
    }
}

impl AsRef<str> for PaymentMethodKrCardBrand {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodKrCardBrand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodKrCardBrand {
    fn default() -> Self {
        Self::Bc
    }
}
