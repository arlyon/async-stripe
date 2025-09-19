/// The billing mode of the subscription.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingMode {
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionsResourceBillingModeType,
    /// Details on when the current billing_mode was adopted.
    pub updated_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingModeBuilder {
    type_: Option<SubscriptionsResourceBillingModeType>,
    updated_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for SubscriptionsResourceBillingMode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingMode>,
        builder: SubscriptionsResourceBillingModeBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingMode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingModeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceBillingModeBuilder {
        type Out = SubscriptionsResourceBillingMode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                "updated_at" => Deserialize::begin(&mut self.updated_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default(), updated_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(type_), Some(updated_at)) = (self.type_, self.updated_at) else {
                return None;
            };
            Some(Self::Out { type_, updated_at })
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

    impl ObjectDeser for SubscriptionsResourceBillingMode {
        type Builder = SubscriptionsResourceBillingModeBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingMode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceBillingModeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "updated_at" => b.updated_at = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsResourceBillingModeType {
    Classic,
    Flexible,
}
impl SubscriptionsResourceBillingModeType {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsResourceBillingModeType::*;
        match self {
            Classic => "classic",
            Flexible => "flexible",
        }
    }
}

impl std::str::FromStr for SubscriptionsResourceBillingModeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourceBillingModeType::*;
        match s {
            "classic" => Ok(Classic),
            "flexible" => Ok(Flexible),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionsResourceBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourceBillingModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourceBillingModeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourceBillingModeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionsResourceBillingModeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(SubscriptionsResourceBillingModeType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionsResourceBillingModeType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourceBillingModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SubscriptionsResourceBillingModeType")
        })
    }
}
