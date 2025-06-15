#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThresholdsResourceUsageAlertFilter {
    /// Limit the scope of the alert to this customer ID
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ThresholdsResourceUsageAlertFilterType,
}
#[doc(hidden)]
pub struct ThresholdsResourceUsageAlertFilterBuilder {
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    type_: Option<ThresholdsResourceUsageAlertFilterType>,
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

    impl Deserialize for ThresholdsResourceUsageAlertFilter {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThresholdsResourceUsageAlertFilter>,
        builder: ThresholdsResourceUsageAlertFilterBuilder,
    }

    impl Visitor for Place<ThresholdsResourceUsageAlertFilter> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThresholdsResourceUsageAlertFilterBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThresholdsResourceUsageAlertFilterBuilder {
        type Out = ThresholdsResourceUsageAlertFilter;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer" => Deserialize::begin(&mut self.customer),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { customer: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer), Some(type_)) = (self.customer.take(), self.type_) else {
                return None;
            };
            Some(Self::Out { customer, type_ })
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

    impl ObjectDeser for ThresholdsResourceUsageAlertFilter {
        type Builder = ThresholdsResourceUsageAlertFilterBuilder;
    }

    impl FromValueOpt for ThresholdsResourceUsageAlertFilter {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThresholdsResourceUsageAlertFilterBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThresholdsResourceUsageAlertFilterType {
    Customer,
}
impl ThresholdsResourceUsageAlertFilterType {
    pub fn as_str(self) -> &'static str {
        use ThresholdsResourceUsageAlertFilterType::*;
        match self {
            Customer => "customer",
        }
    }
}

impl std::str::FromStr for ThresholdsResourceUsageAlertFilterType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThresholdsResourceUsageAlertFilterType::*;
        match s {
            "customer" => Ok(Customer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ThresholdsResourceUsageAlertFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThresholdsResourceUsageAlertFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThresholdsResourceUsageAlertFilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThresholdsResourceUsageAlertFilterType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThresholdsResourceUsageAlertFilterType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThresholdsResourceUsageAlertFilterType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThresholdsResourceUsageAlertFilterType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThresholdsResourceUsageAlertFilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ThresholdsResourceUsageAlertFilterType")
        })
    }
}
