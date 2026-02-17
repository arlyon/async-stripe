#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountUnificationAccountControllerLosses {
    /// A value indicating who is liable when this account can't pay back negative balances from payments.
    pub payments: AccountUnificationAccountControllerLossesPayments,
}
#[doc(hidden)]
pub struct AccountUnificationAccountControllerLossesBuilder {
    payments: Option<AccountUnificationAccountControllerLossesPayments>,
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

    impl Deserialize for AccountUnificationAccountControllerLosses {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountUnificationAccountControllerLosses>,
        builder: AccountUnificationAccountControllerLossesBuilder,
    }

    impl Visitor for Place<AccountUnificationAccountControllerLosses> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountUnificationAccountControllerLossesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountUnificationAccountControllerLossesBuilder {
        type Out = AccountUnificationAccountControllerLosses;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payments" => Deserialize::begin(&mut self.payments),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payments: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payments),) = (self.payments.take(),) else {
                return None;
            };
            Some(Self::Out { payments })
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

    impl ObjectDeser for AccountUnificationAccountControllerLosses {
        type Builder = AccountUnificationAccountControllerLossesBuilder;
    }

    impl FromValueOpt for AccountUnificationAccountControllerLosses {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountUnificationAccountControllerLossesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payments" => b.payments = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A value indicating who is liable when this account can't pay back negative balances from payments.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountUnificationAccountControllerLossesPayments {
    Application,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountUnificationAccountControllerLossesPayments {
    pub fn as_str(&self) -> &str {
        use AccountUnificationAccountControllerLossesPayments::*;
        match self {
            Application => "application",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerLossesPayments {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerLossesPayments::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountUnificationAccountControllerLossesPayments"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountUnificationAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountUnificationAccountControllerLossesPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountUnificationAccountControllerLossesPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountUnificationAccountControllerLossesPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountUnificationAccountControllerLossesPayments::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountUnificationAccountControllerLossesPayments);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerLossesPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
