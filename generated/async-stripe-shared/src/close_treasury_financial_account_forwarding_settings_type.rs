/// The type of the bank account provided. This can be either "financial_account" or "payment_method"
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CloseTreasuryFinancialAccountForwardingSettingsType {
    FinancialAccount,
    PaymentMethod,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CloseTreasuryFinancialAccountForwardingSettingsType {
    pub fn as_str(&self) -> &str {
        use CloseTreasuryFinancialAccountForwardingSettingsType::*;
        match self {
            FinancialAccount => "financial_account",
            PaymentMethod => "payment_method",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CloseTreasuryFinancialAccountForwardingSettingsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CloseTreasuryFinancialAccountForwardingSettingsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "payment_method" => Ok(PaymentMethod),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CloseTreasuryFinancialAccountForwardingSettingsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CloseTreasuryFinancialAccountForwardingSettingsType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CloseTreasuryFinancialAccountForwardingSettingsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
