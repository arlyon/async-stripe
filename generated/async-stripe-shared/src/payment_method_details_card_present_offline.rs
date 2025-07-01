#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardPresentOffline {
    /// Time at which the payment was collected while offline
    pub stored_at: Option<stripe_types::Timestamp>,
    /// The method used to process this payment method offline. Only deferred is allowed.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<PaymentMethodDetailsCardPresentOfflineType>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardPresentOfflineBuilder {
    stored_at: Option<Option<stripe_types::Timestamp>>,
    type_: Option<Option<PaymentMethodDetailsCardPresentOfflineType>>,
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
                builder: PaymentMethodDetailsCardPresentOfflineBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardPresentOfflineBuilder {
        type Out = PaymentMethodDetailsCardPresentOffline;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "stored_at" => Deserialize::begin(&mut self.stored_at),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { stored_at: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(stored_at), Some(type_)) = (self.stored_at, self.type_) else {
                return None;
            };
            Some(Self::Out { stored_at, type_ })
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

    impl ObjectDeser for PaymentMethodDetailsCardPresentOffline {
        type Builder = PaymentMethodDetailsCardPresentOfflineBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCardPresentOffline {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCardPresentOfflineBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "stored_at" => b.stored_at = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The method used to process this payment method offline. Only deferred is allowed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardPresentOfflineType {
    Deferred,
}
impl PaymentMethodDetailsCardPresentOfflineType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardPresentOfflineType::*;
        match self {
            Deferred => "deferred",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardPresentOfflineType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardPresentOfflineType::*;
        match s {
            "deferred" => Ok(Deferred),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardPresentOfflineType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardPresentOfflineType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentMethodDetailsCardPresentOfflineType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardPresentOfflineType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsCardPresentOfflineType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsCardPresentOfflineType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardPresentOfflineType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsCardPresentOfflineType")
        })
    }
}
