/// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum HostedPaymentMethodSave {
    Always,
    Never,
    Offer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl HostedPaymentMethodSave {
    pub fn as_str(&self) -> &str {
        use HostedPaymentMethodSave::*;
        match self {
            Always => "always",
            Never => "never",
            Offer => "offer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for HostedPaymentMethodSave {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HostedPaymentMethodSave::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            "offer" => Ok(Offer),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "HostedPaymentMethodSave");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for HostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for HostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for HostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(HostedPaymentMethodSave)).finish_non_exhaustive()
    }
}
impl serde::Serialize for HostedPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for HostedPaymentMethodSave {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<HostedPaymentMethodSave> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(HostedPaymentMethodSave::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(HostedPaymentMethodSave);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for HostedPaymentMethodSave {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
