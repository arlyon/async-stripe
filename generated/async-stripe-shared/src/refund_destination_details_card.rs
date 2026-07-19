#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundDestinationDetailsCard {
    /// Value of the reference number assigned to the refund.
    pub reference: Option<String>,
    /// Status of the reference number on the refund. This can be `pending`, `available` or `unavailable`.
    pub reference_status: Option<String>,
    /// Type of the reference number assigned to the refund.
    pub reference_type: Option<String>,
    /// The type of refund. This can be `refund`, `reversal`, or `pending`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: RefundDestinationDetailsCardType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundDestinationDetailsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundDestinationDetailsCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RefundDestinationDetailsCardBuilder {
    reference: Option<Option<String>>,
    reference_status: Option<Option<String>>,
    reference_type: Option<Option<String>>,
    type_: Option<RefundDestinationDetailsCardType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for RefundDestinationDetailsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundDestinationDetailsCard>,
        builder: RefundDestinationDetailsCardBuilder,
    }

    impl Visitor for Place<RefundDestinationDetailsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RefundDestinationDetailsCardBuilder {
                    reference: Deserialize::default(),
                    reference_status: Deserialize::default(),
                    reference_type: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "reference_status" => Deserialize::begin(&mut self.builder.reference_status),
                "reference_type" => Deserialize::begin(&mut self.builder.reference_type),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(reference), Some(reference_status), Some(reference_type), Some(type_)) = (
                self.builder.reference.take(),
                self.builder.reference_status.take(),
                self.builder.reference_type.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(RefundDestinationDetailsCard {
                reference,
                reference_status,
                reference_type,
                type_,
            });
            Ok(())
        }
    }
};
/// The type of refund. This can be `refund`, `reversal`, or `pending`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RefundDestinationDetailsCardType {
    Pending,
    Refund,
    Reversal,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RefundDestinationDetailsCardType {
    pub fn as_str(&self) -> &str {
        use RefundDestinationDetailsCardType::*;
        match self {
            Pending => "pending",
            Refund => "refund",
            Reversal => "reversal",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RefundDestinationDetailsCardType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefundDestinationDetailsCardType::*;
        match s {
            "pending" => Ok(Pending),
            "refund" => Ok(Refund),
            "reversal" => Ok(Reversal),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "RefundDestinationDetailsCardType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(RefundDestinationDetailsCardType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RefundDestinationDetailsCardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for RefundDestinationDetailsCardType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<RefundDestinationDetailsCardType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RefundDestinationDetailsCardType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RefundDestinationDetailsCardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
