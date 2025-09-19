#[derive(Clone, Debug)]
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
                builder: RefundDestinationDetailsCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RefundDestinationDetailsCardBuilder {
        type Out = RefundDestinationDetailsCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.reference),
                "reference_status" => Deserialize::begin(&mut self.reference_status),
                "reference_type" => Deserialize::begin(&mut self.reference_type),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                reference: Deserialize::default(),
                reference_status: Deserialize::default(),
                reference_type: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(reference), Some(reference_status), Some(reference_type), Some(type_)) = (
                self.reference.take(),
                self.reference_status.take(),
                self.reference_type.take(),
                self.type_,
            ) else {
                return None;
            };
            Some(Self::Out { reference, reference_status, reference_type, type_ })
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

    impl ObjectDeser for RefundDestinationDetailsCard {
        type Builder = RefundDestinationDetailsCardBuilder;
    }

    impl FromValueOpt for RefundDestinationDetailsCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RefundDestinationDetailsCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "reference_status" => b.reference_status = FromValueOpt::from_value(v),
                    "reference_type" => b.reference_type = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of refund. This can be `refund`, `reversal`, or `pending`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RefundDestinationDetailsCardType {
    Pending,
    Refund,
    Reversal,
}
impl RefundDestinationDetailsCardType {
    pub fn as_str(self) -> &'static str {
        use RefundDestinationDetailsCardType::*;
        match self {
            Pending => "pending",
            Refund => "refund",
            Reversal => "reversal",
        }
    }
}

impl std::str::FromStr for RefundDestinationDetailsCardType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefundDestinationDetailsCardType::*;
        match s {
            "pending" => Ok(Pending),
            "refund" => Ok(Refund),
            "reversal" => Ok(Reversal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for RefundDestinationDetailsCardType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RefundDestinationDetailsCardType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(RefundDestinationDetailsCardType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RefundDestinationDetailsCardType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RefundDestinationDetailsCardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for RefundDestinationDetailsCardType")
        })
    }
}
