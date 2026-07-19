#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    /// Indicates whether or not multiple captures are supported.
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder {
    status: Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus>,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>,
        builder: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder {
                        status: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(status),) = (self.builder.status.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
                status,
            });
            Ok(())
        }
    }
};
/// Indicates whether or not multiple captures are supported.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    Available,
    Unavailable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus::*;
        match self {
            Available => "available",
            Unavailable => "unavailable",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus::*;
        match s {
            "available" => Ok(Available),
            "unavailable" => Ok(Unavailable),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
