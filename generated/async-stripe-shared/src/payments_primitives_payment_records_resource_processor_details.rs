/// Processor information associated with this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetails {
pub custom: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails>,
    /// The processor used for this payment attempt.
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourceProcessorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentsPrimitivesPaymentRecordsResourceProcessorDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder {
    custom: Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails>>,
type_: Option<PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType>,

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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourceProcessorDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentsPrimitivesPaymentRecordsResourceProcessorDetails>,
        builder: PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder,
    }

    impl Visitor for Place<PaymentsPrimitivesPaymentRecordsResourceProcessorDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder {
                    custom: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom" => Deserialize::begin(&mut self.builder.custom),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(custom), Some(type_)) =
                (self.builder.custom.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(PaymentsPrimitivesPaymentRecordsResourceProcessorDetails { custom, type_ });
            Ok(())
        }
    }
};
/// The processor used for this payment attempt.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    Custom,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    pub fn as_str(&self) -> &str {
        use PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType::*;
        match self {
            Custom => "custom",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType::*;
        match s {
            "custom" => Ok(Custom),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
