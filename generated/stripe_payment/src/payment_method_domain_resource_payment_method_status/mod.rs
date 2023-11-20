/// Indicates the status of a specific payment method on a payment method domain.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    /// The status of the payment method on the domain.
    pub status: PaymentMethodDomainResourcePaymentMethodStatusStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details:
        Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}
/// The status of the payment method on the domain.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDomainResourcePaymentMethodStatusStatus {
    Active,
    Inactive,
}

impl PaymentMethodDomainResourcePaymentMethodStatusStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDomainResourcePaymentMethodStatusStatus",
            )
        })
    }
}
