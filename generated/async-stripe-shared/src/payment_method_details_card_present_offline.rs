#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardPresentOffline {
    /// Time at which the payment was collected while offline
    pub stored_at: Option<stripe_types::Timestamp>,
    /// The method used to process this payment method offline. Only deferred is allowed.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<PaymentMethodDetailsCardPresentOfflineType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCardPresentOffline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsCardPresentOffline").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardPresentOfflineBuilder {
    stored_at: Option<Option<stripe_types::Timestamp>>,
    type_: Option<Option<PaymentMethodDetailsCardPresentOfflineType>>,
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

    impl Deserialize for PaymentMethodDetailsCardPresentOffline {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardPresentOffline>,
        builder: PaymentMethodDetailsCardPresentOfflineBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardPresentOffline> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCardPresentOfflineBuilder {
                    stored_at: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "stored_at" => Deserialize::begin(&mut self.builder.stored_at),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(stored_at), Some(type_)) =
                (self.builder.stored_at, self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsCardPresentOffline { stored_at, type_ });
            Ok(())
        }
    }
};
/// The method used to process this payment method offline. Only deferred is allowed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsCardPresentOfflineType {
    Deferred,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodDetailsCardPresentOfflineType {
    pub fn as_str(&self) -> &str {
        use PaymentMethodDetailsCardPresentOfflineType::*;
        match self {
            Deferred => "deferred",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardPresentOfflineType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardPresentOfflineType::*;
        match s {
            "deferred" => Ok(Deferred),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodDetailsCardPresentOfflineType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardPresentOfflineType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodDetailsCardPresentOfflineType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCardPresentOfflineType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodDetailsCardPresentOfflineType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCardPresentOfflineType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodDetailsCardPresentOfflineType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardPresentOfflineType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodDetailsCardPresentOfflineType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardPresentOfflineType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
