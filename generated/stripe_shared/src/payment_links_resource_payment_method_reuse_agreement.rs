#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourcePaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    /// When set to `auto`, Stripe's defaults will be used.
    ///
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: PaymentLinksResourcePaymentMethodReuseAgreementPosition,
}
/// Determines the position and visibility of the payment method reuse agreement in the UI.
/// When set to `auto`, Stripe's defaults will be used.
///
/// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}
impl PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourcePaymentMethodReuseAgreementPosition::*;
        match self {
            Auto => "auto",
            Hidden => "hidden",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentMethodReuseAgreementPosition::*;
        match s {
            "auto" => Ok(Auto),
            "hidden" => Ok(Hidden),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentLinksResourcePaymentMethodReuseAgreementPosition",
            )
        })
    }
}
