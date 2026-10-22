/// Contains information about the future authorisations related to this authentication
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceFutureUsage {
    pub installment: Option<stripe_payment::ThreeDSecureStandaloneResourceInstallment>,
    pub recurring: Option<stripe_payment::ThreeDSecureStandaloneResourceRecurring>,
    /// The type of future usage declared for this 3DS Authentication.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ThreeDSecureStandaloneResourceFutureUsageType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceFutureUsage").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceFutureUsageBuilder {
    installment: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceInstallment>>,
    recurring: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceRecurring>>,
    type_: Option<ThreeDSecureStandaloneResourceFutureUsageType>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceFutureUsage {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceFutureUsage>,
        builder: ThreeDSecureStandaloneResourceFutureUsageBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceFutureUsage> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceFutureUsageBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceFutureUsageBuilder {
        type Out = ThreeDSecureStandaloneResourceFutureUsage;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "installment" => Deserialize::begin(&mut self.installment),
                "recurring" => Deserialize::begin(&mut self.recurring),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { installment: Some(None), recurring: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(installment), Some(recurring), Some(type_)) =
                (self.installment.take(), self.recurring.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { installment, recurring, type_ })
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceFutureUsage {
        type Builder = ThreeDSecureStandaloneResourceFutureUsageBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceFutureUsage {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceFutureUsageBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "installment" => b.installment = FromValueOpt::from_value(v),
                    "recurring" => b.recurring = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of future usage declared for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceFutureUsageType {
    CardOnFile,
    Installment,
    Recurring,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceFutureUsageType {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceFutureUsageType::*;
        match self {
            CardOnFile => "card_on_file",
            Installment => "installment",
            Recurring => "recurring",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceFutureUsageType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceFutureUsageType::*;
        match s {
            "card_on_file" => Ok(CardOnFile),
            "installment" => Ok(Installment),
            "recurring" => Ok(Recurring),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceFutureUsageType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceFutureUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceFutureUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceFutureUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureStandaloneResourceFutureUsageType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceFutureUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureStandaloneResourceFutureUsageType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureStandaloneResourceFutureUsageType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ThreeDSecureStandaloneResourceFutureUsageType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureStandaloneResourceFutureUsageType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureStandaloneResourceFutureUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
