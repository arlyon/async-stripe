/// The type of automatic transfer rule.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BalanceSettingsResourceAutomaticTransferRuleType {
    TransferAll,
    TransferUpToAmount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BalanceSettingsResourceAutomaticTransferRuleType {
    pub fn as_str(&self) -> &str {
        use BalanceSettingsResourceAutomaticTransferRuleType::*;
        match self {
            TransferAll => "transfer_all",
            TransferUpToAmount => "transfer_up_to_amount",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BalanceSettingsResourceAutomaticTransferRuleType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceSettingsResourceAutomaticTransferRuleType::*;
        match s {
            "transfer_all" => Ok(TransferAll),
            "transfer_up_to_amount" => Ok(TransferUpToAmount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BalanceSettingsResourceAutomaticTransferRuleType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BalanceSettingsResourceAutomaticTransferRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BalanceSettingsResourceAutomaticTransferRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourceAutomaticTransferRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BalanceSettingsResourceAutomaticTransferRuleType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for BalanceSettingsResourceAutomaticTransferRuleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BalanceSettingsResourceAutomaticTransferRuleType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BalanceSettingsResourceAutomaticTransferRuleType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BalanceSettingsResourceAutomaticTransferRuleType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BalanceSettingsResourceAutomaticTransferRuleType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BalanceSettingsResourceAutomaticTransferRuleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
