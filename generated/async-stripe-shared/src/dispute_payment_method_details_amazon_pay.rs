#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetailsAmazonPay {
    /// The AmazonPay dispute type, chargeback or claim
    pub dispute_type: Option<DisputePaymentMethodDetailsAmazonPayDisputeType>,
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsAmazonPayBuilder {
    dispute_type: Option<Option<DisputePaymentMethodDetailsAmazonPayDisputeType>>,
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

    impl Deserialize for DisputePaymentMethodDetailsAmazonPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetailsAmazonPay>,
        builder: DisputePaymentMethodDetailsAmazonPayBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetailsAmazonPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsAmazonPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputePaymentMethodDetailsAmazonPayBuilder {
        type Out = DisputePaymentMethodDetailsAmazonPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dispute_type" => Deserialize::begin(&mut self.dispute_type),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { dispute_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(dispute_type),) = (self.dispute_type,) else {
                return None;
            };
            Some(Self::Out { dispute_type })
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

    impl ObjectDeser for DisputePaymentMethodDetailsAmazonPay {
        type Builder = DisputePaymentMethodDetailsAmazonPayBuilder;
    }

    impl FromValueOpt for DisputePaymentMethodDetailsAmazonPay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputePaymentMethodDetailsAmazonPayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dispute_type" => b.dispute_type = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The AmazonPay dispute type, chargeback or claim
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DisputePaymentMethodDetailsAmazonPayDisputeType {
    Chargeback,
    Claim,
}
impl DisputePaymentMethodDetailsAmazonPayDisputeType {
    pub fn as_str(self) -> &'static str {
        use DisputePaymentMethodDetailsAmazonPayDisputeType::*;
        match self {
            Chargeback => "chargeback",
            Claim => "claim",
        }
    }
}

impl std::str::FromStr for DisputePaymentMethodDetailsAmazonPayDisputeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputePaymentMethodDetailsAmazonPayDisputeType::*;
        match s {
            "chargeback" => Ok(Chargeback),
            "claim" => Ok(Claim),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DisputePaymentMethodDetailsAmazonPayDisputeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            DisputePaymentMethodDetailsAmazonPayDisputeType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisputePaymentMethodDetailsAmazonPayDisputeType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for DisputePaymentMethodDetailsAmazonPayDisputeType",
            )
        })
    }
}
