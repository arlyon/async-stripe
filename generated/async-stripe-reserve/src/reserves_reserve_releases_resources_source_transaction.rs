#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReservesReserveReleasesResourcesSourceTransaction {
    /// The ID of the dispute.
    pub dispute: Option<stripe_types::Expandable<stripe_shared::Dispute>>,
    /// The ID of the refund.
    pub refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    /// The type of source transaction.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ReservesReserveReleasesResourcesSourceTransactionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservesReserveReleasesResourcesSourceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReservesReserveReleasesResourcesSourceTransaction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReservesReserveReleasesResourcesSourceTransactionBuilder {
    dispute: Option<Option<stripe_types::Expandable<stripe_shared::Dispute>>>,
    refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    type_: Option<ReservesReserveReleasesResourcesSourceTransactionType>,
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

    impl Deserialize for ReservesReserveReleasesResourcesSourceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReservesReserveReleasesResourcesSourceTransaction>,
        builder: ReservesReserveReleasesResourcesSourceTransactionBuilder,
    }

    impl Visitor for Place<ReservesReserveReleasesResourcesSourceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReservesReserveReleasesResourcesSourceTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ReservesReserveReleasesResourcesSourceTransactionBuilder {
        type Out = ReservesReserveReleasesResourcesSourceTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dispute" => Deserialize::begin(&mut self.dispute),
                "refund" => Deserialize::begin(&mut self.refund),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dispute: Deserialize::default(),
                refund: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(dispute), Some(refund), Some(type_)) =
                (self.dispute.take(), self.refund.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { dispute, refund, type_ })
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

    impl ObjectDeser for ReservesReserveReleasesResourcesSourceTransaction {
        type Builder = ReservesReserveReleasesResourcesSourceTransactionBuilder;
    }

    impl FromValueOpt for ReservesReserveReleasesResourcesSourceTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ReservesReserveReleasesResourcesSourceTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dispute" => b.dispute = FromValueOpt::from_value(v),
                    "refund" => b.refund = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of source transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReservesReserveReleasesResourcesSourceTransactionType {
    Dispute,
    Refund,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReservesReserveReleasesResourcesSourceTransactionType {
    pub fn as_str(&self) -> &str {
        use ReservesReserveReleasesResourcesSourceTransactionType::*;
        match self {
            Dispute => "dispute",
            Refund => "refund",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReservesReserveReleasesResourcesSourceTransactionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReservesReserveReleasesResourcesSourceTransactionType::*;
        match s {
            "dispute" => Ok(Dispute),
            "refund" => Ok(Refund),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ReservesReserveReleasesResourcesSourceTransactionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReservesReserveReleasesResourcesSourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReservesReserveReleasesResourcesSourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservesReserveReleasesResourcesSourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReservesReserveReleasesResourcesSourceTransactionType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReservesReserveReleasesResourcesSourceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ReservesReserveReleasesResourcesSourceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ReservesReserveReleasesResourcesSourceTransactionType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ReservesReserveReleasesResourcesSourceTransactionType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ReservesReserveReleasesResourcesSourceTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReservesReserveReleasesResourcesSourceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
