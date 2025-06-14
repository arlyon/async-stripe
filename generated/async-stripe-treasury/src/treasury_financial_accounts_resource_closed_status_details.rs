#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {
    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder {
    reasons: Option<Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceClosedStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceClosedStatusDetails>,
        builder: TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceClosedStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder {
        type Out = TreasuryFinancialAccountsResourceClosedStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reasons" => Deserialize::begin(&mut self.reasons),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reasons: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(reasons),) = (self.reasons.take(),) else {
                return None;
            };
            Some(Self::Out { reasons })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceClosedStatusDetails {
        type Builder = TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceClosedStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TreasuryFinancialAccountsResourceClosedStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reasons" => b.reasons = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The array that contains reasons for a FinancialAccount closure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}
impl TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::*;
        match self {
            AccountRejected => "account_rejected",
            ClosedByPlatform => "closed_by_platform",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::*;
        match s {
            "account_rejected" => Ok(AccountRejected),
            "closed_by_platform" => Ok(ClosedByPlatform),
            "other" => Ok(Other),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceClosedStatusDetailsReasons::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourceClosedStatusDetailsReasons
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons",
            )
        })
    }
}
