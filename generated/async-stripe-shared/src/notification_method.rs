/// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
/// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum NotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl NotificationMethod {
    pub fn as_str(&self) -> &str {
        use NotificationMethod::*;
        match self {
            DeprecatedNone => "deprecated_none",
            Email => "email",
            Manual => "manual",
            None => "none",
            StripeEmail => "stripe_email",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for NotificationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NotificationMethod::*;
        match s {
            "deprecated_none" => Ok(DeprecatedNone),
            "email" => Ok(Email),
            "manual" => Ok(Manual),
            "none" => Ok(None),
            "stripe_email" => Ok(StripeEmail),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "NotificationMethod");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for NotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for NotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(NotificationMethod)).finish_non_exhaustive()
    }
}
impl serde::Serialize for NotificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for NotificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
