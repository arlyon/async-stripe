#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture {
    /// The maximum amount that can be captured.
pub maximum_amount_capturable: i64,
    /// Indicates whether or not the authorized amount can be over-captured.
pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus,

}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureBuilder {
    maximum_amount_capturable: Option<i64>,
status: Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus>,

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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>,
    builder: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureBuilder,
}

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureBuilder::deser_default(),
        }))
    }
}

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureBuilder {
    type Out = PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "maximum_amount_capturable" => Deserialize::begin(&mut self.maximum_amount_capturable),
            "status" => Deserialize::begin(&mut self.status),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { maximum_amount_capturable: Deserialize::default(),
status: Deserialize::default(),
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(maximum_amount_capturable),
Some(status),
) = (self.maximum_amount_capturable,
self.status.take(),
) else {
            return None;
        };
        Some(Self::Out { maximum_amount_capturable,status })
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture {
    type Builder = PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureBuilder;
}

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
            "maximum_amount_capturable" => b.maximum_amount_capturable = FromValueOpt::from_value(v),
            "status" => b.status = FromValueOpt::from_value(v),
                _ => {}
            }
        }
        b.take_out()
    }
}
};
/// Indicates whether or not the authorized amount can be over-captured.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus
{
    Available,
    Unavailable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus::*;
        match self {
Available => "available",
Unavailable => "unavailable",
Unknown(v) => v,

        }
    }
}

impl std::str::FromStr for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus::*;
        match s {
    "available" => Ok(Available),
"unavailable" => Ok(Unavailable),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
