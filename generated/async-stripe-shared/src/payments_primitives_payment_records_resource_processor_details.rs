/// Processor information associated with this payment.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetails {
pub custom: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails>,
    /// The processor used for this payment attempt.
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType,

}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder {
    custom: Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails>>,
type_: Option<PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType>,

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
                builder:
                    PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder {
        type Out = PaymentsPrimitivesPaymentRecordsResourceProcessorDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom" => Deserialize::begin(&mut self.custom),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { custom: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(custom), Some(type_)) = (self.custom.take(), self.type_) else {
                return None;
            };
            Some(Self::Out { custom, type_ })
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

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourceProcessorDetails {
        type Builder = PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder;
    }

    impl FromValueOpt for PaymentsPrimitivesPaymentRecordsResourceProcessorDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom" => b.custom = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The processor used for this payment attempt.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    Custom,
}
impl PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    pub fn as_str(self) -> &'static str {
        use PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsType",
            )
        })
    }
}
