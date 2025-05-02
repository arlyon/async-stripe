#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionVerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: stripe_types::Timestamp,
    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    /// The type of the microdeposit sent to the customer.
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType>,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionVerifyWithMicrodepositsBuilder {
    arrival_date: Option<stripe_types::Timestamp>,
    hosted_verification_url: Option<String>,
    microdeposit_type:
        Option<Option<PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType>>,
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

    impl Deserialize for PaymentIntentNextActionVerifyWithMicrodeposits {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionVerifyWithMicrodeposits>,
        builder: PaymentIntentNextActionVerifyWithMicrodepositsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionVerifyWithMicrodeposits> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionVerifyWithMicrodepositsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionVerifyWithMicrodepositsBuilder {
        type Out = PaymentIntentNextActionVerifyWithMicrodeposits;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "arrival_date" => Deserialize::begin(&mut self.arrival_date),
                "hosted_verification_url" => Deserialize::begin(&mut self.hosted_verification_url),
                "microdeposit_type" => Deserialize::begin(&mut self.microdeposit_type),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                arrival_date: Deserialize::default(),
                hosted_verification_url: Deserialize::default(),
                microdeposit_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(arrival_date), Some(hosted_verification_url), Some(microdeposit_type)) =
                (self.arrival_date, self.hosted_verification_url.take(), self.microdeposit_type)
            else {
                return None;
            };
            Some(Self::Out { arrival_date, hosted_verification_url, microdeposit_type })
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

    impl ObjectDeser for PaymentIntentNextActionVerifyWithMicrodeposits {
        type Builder = PaymentIntentNextActionVerifyWithMicrodepositsBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionVerifyWithMicrodeposits {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionVerifyWithMicrodepositsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "arrival_date" => b.arrival_date = FromValueOpt::from_value(v),
                    "hosted_verification_url" => {
                        b.hosted_verification_url = FromValueOpt::from_value(v)
                    }
                    "microdeposit_type" => b.microdeposit_type = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the microdeposit sent to the customer.
/// Used to distinguish between different verification methods.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}
impl PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match self {
            Amounts => "amounts",
            DescriptorCode => "descriptor_code",
        }
    }
}

impl std::str::FromStr for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match s {
            "amounts" => Ok(Amounts),
            "descriptor_code" => Ok(DescriptorCode),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType",
            )
        })
    }
}
