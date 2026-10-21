#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardPresent {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentMethodOptionsCardPresentCaptureMethod>,
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://docs.stripe.com/terminal/features/extended-authorizations#authorization-validity).
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://docs.stripe.com/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    /// Check [incremental_authorization_supported](https://docs.stripe.com/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://docs.stripe.com/api/payment_intents/confirm) response to verify support.
    pub request_incremental_authorization_support: Option<bool>,
    pub routing: Option<stripe_shared::PaymentMethodOptionsCardPresentRouting>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsCardPresent").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCardPresentBuilder {
    capture_method: Option<PaymentMethodOptionsCardPresentCaptureMethod>,
    request_extended_authorization: Option<Option<bool>>,
    request_incremental_authorization_support: Option<Option<bool>>,
    routing: Option<stripe_shared::PaymentMethodOptionsCardPresentRouting>,
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

    impl Deserialize for PaymentMethodOptionsCardPresent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCardPresent>,
        builder: PaymentMethodOptionsCardPresentBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCardPresent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCardPresentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCardPresentBuilder {
        type Out = PaymentMethodOptionsCardPresent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.capture_method)
                }
                "request_extended_authorization" => {
                    Deserialize::begin(&mut self.request_extended_authorization)
                }
                "request_incremental_authorization_support" => {
                    Deserialize::begin(&mut self.request_incremental_authorization_support)
                }
                "routing" => stripe_types::miniserde_helpers::begin_nullable(&mut self.routing),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_method: None,
                request_extended_authorization: Some(None),
                request_incremental_authorization_support: Some(None),
                routing: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    capture_method,
                    request_extended_authorization: Some(request_extended_authorization),
                    request_incremental_authorization_support:
                        Some(request_incremental_authorization_support),
                    routing,
                } => Some(Self::Out {
                    capture_method,
                    request_extended_authorization,
                    request_incremental_authorization_support,
                    routing,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodOptionsCardPresent {
        type Builder = PaymentMethodOptionsCardPresentBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCardPresent {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Controls when the funds will be captured from the customer's account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsCardPresentCaptureMethod {
    Manual,
    ManualPreferred,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsCardPresentCaptureMethod {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsCardPresentCaptureMethod::*;
        match self {
            Manual => "manual",
            ManualPreferred => "manual_preferred",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardPresentCaptureMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardPresentCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            "manual_preferred" => Ok(ManualPreferred),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsCardPresentCaptureMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsCardPresentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodOptionsCardPresentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsCardPresentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodOptionsCardPresentCaptureMethod))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsCardPresentCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsCardPresentCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsCardPresentCaptureMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodOptionsCardPresentCaptureMethod::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsCardPresentCaptureMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardPresentCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
