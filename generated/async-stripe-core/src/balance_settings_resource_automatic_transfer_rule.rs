#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourceAutomaticTransferRule {
    /// The ID of the FinancialAccount that funds will be transferred to during automatic transfers.
    pub payout_method: String,
    /// The maximum amount in minor units to transfer to the FinancialAccount.
    /// Only applicable when `type` is `transfer_up_to_amount`.
    pub transfer_up_to_amount: Option<i64>,
    /// The type of automatic transfer rule.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BalanceSettingsResourceAutomaticTransferRuleType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourceAutomaticTransferRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettingsResourceAutomaticTransferRule").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceSettingsResourceAutomaticTransferRuleBuilder {
    payout_method: Option<String>,
    transfer_up_to_amount: Option<Option<i64>>,
    type_: Option<BalanceSettingsResourceAutomaticTransferRuleType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BalanceSettingsResourceAutomaticTransferRule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceSettingsResourceAutomaticTransferRule>,
        builder: BalanceSettingsResourceAutomaticTransferRuleBuilder,
    }

    impl Visitor for Place<BalanceSettingsResourceAutomaticTransferRule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceSettingsResourceAutomaticTransferRuleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceSettingsResourceAutomaticTransferRuleBuilder {
        type Out = BalanceSettingsResourceAutomaticTransferRule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payout_method" => Deserialize::begin(&mut self.payout_method),
                "transfer_up_to_amount" => Deserialize::begin(&mut self.transfer_up_to_amount),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payout_method: None, transfer_up_to_amount: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payout_method), Some(transfer_up_to_amount), Some(type_)) =
                (self.payout_method.take(), self.transfer_up_to_amount, self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { payout_method, transfer_up_to_amount, type_ })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BalanceSettingsResourceAutomaticTransferRule {
        type Builder = BalanceSettingsResourceAutomaticTransferRuleBuilder;
    }

    impl FromValueOpt for BalanceSettingsResourceAutomaticTransferRule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceSettingsResourceAutomaticTransferRuleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payout_method" => b.payout_method = FromValueOpt::from_value(v),
                    "transfer_up_to_amount" => {
                        b.transfer_up_to_amount = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
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
#[cfg(feature = "serialize")]
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
