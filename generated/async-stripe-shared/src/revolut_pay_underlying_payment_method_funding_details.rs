#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RevolutPayUnderlyingPaymentMethodFundingDetails {
    pub card: Option<stripe_shared::PaymentMethodDetailsPassthroughCard>,
    /// funding type of the underlying payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<RevolutPayUnderlyingPaymentMethodFundingDetailsType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RevolutPayUnderlyingPaymentMethodFundingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RevolutPayUnderlyingPaymentMethodFundingDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RevolutPayUnderlyingPaymentMethodFundingDetailsBuilder {
    card: Option<stripe_shared::PaymentMethodDetailsPassthroughCard>,
    type_: Option<RevolutPayUnderlyingPaymentMethodFundingDetailsType>,
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

    impl Deserialize for RevolutPayUnderlyingPaymentMethodFundingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RevolutPayUnderlyingPaymentMethodFundingDetails>,
        builder: RevolutPayUnderlyingPaymentMethodFundingDetailsBuilder,
    }

    impl Visitor for Place<RevolutPayUnderlyingPaymentMethodFundingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RevolutPayUnderlyingPaymentMethodFundingDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RevolutPayUnderlyingPaymentMethodFundingDetailsBuilder {
        type Out = RevolutPayUnderlyingPaymentMethodFundingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "type" => stripe_types::miniserde_helpers::begin_nullable(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (card, type_) = (self.card.take(), self.type_.take()) else {
                return None;
            };
            Some(Self::Out { card, type_ })
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

    impl ObjectDeser for RevolutPayUnderlyingPaymentMethodFundingDetails {
        type Builder = RevolutPayUnderlyingPaymentMethodFundingDetailsBuilder;
    }

    impl FromValueOpt for RevolutPayUnderlyingPaymentMethodFundingDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// funding type of the underlying payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    Card,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    pub fn as_str(&self) -> &str {
        use RevolutPayUnderlyingPaymentMethodFundingDetailsType::*;
        match self {
            Card => "card",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RevolutPayUnderlyingPaymentMethodFundingDetailsType::*;
        match s {
            "card" => Ok(Card),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "RevolutPayUnderlyingPaymentMethodFundingDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(RevolutPayUnderlyingPaymentMethodFundingDetailsType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RevolutPayUnderlyingPaymentMethodFundingDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            RevolutPayUnderlyingPaymentMethodFundingDetailsType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RevolutPayUnderlyingPaymentMethodFundingDetailsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RevolutPayUnderlyingPaymentMethodFundingDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
