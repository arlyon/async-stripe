#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceCustomerMappingSettings {
    /// The key in the meter event payload to use for mapping the event to a customer.
    pub event_payload_key: String,
    /// The method for mapping a meter event to a customer.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingMeterResourceCustomerMappingSettingsType,
}
#[doc(hidden)]
pub struct BillingMeterResourceCustomerMappingSettingsBuilder {
    event_payload_key: Option<String>,
    type_: Option<BillingMeterResourceCustomerMappingSettingsType>,
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

    impl Deserialize for BillingMeterResourceCustomerMappingSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterResourceCustomerMappingSettings>,
        builder: BillingMeterResourceCustomerMappingSettingsBuilder,
    }

    impl Visitor for Place<BillingMeterResourceCustomerMappingSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterResourceCustomerMappingSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterResourceCustomerMappingSettingsBuilder {
        type Out = BillingMeterResourceCustomerMappingSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "event_payload_key" => Deserialize::begin(&mut self.event_payload_key),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { event_payload_key: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(event_payload_key), Some(type_)) =
                (self.event_payload_key.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { event_payload_key, type_ })
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

    impl ObjectDeser for BillingMeterResourceCustomerMappingSettings {
        type Builder = BillingMeterResourceCustomerMappingSettingsBuilder;
    }

    impl FromValueOpt for BillingMeterResourceCustomerMappingSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterResourceCustomerMappingSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "event_payload_key" => b.event_payload_key = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The method for mapping a meter event to a customer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingMeterResourceCustomerMappingSettingsType {
    ById,
}
impl BillingMeterResourceCustomerMappingSettingsType {
    pub fn as_str(self) -> &'static str {
        use BillingMeterResourceCustomerMappingSettingsType::*;
        match self {
            ById => "by_id",
        }
    }
}

impl std::str::FromStr for BillingMeterResourceCustomerMappingSettingsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterResourceCustomerMappingSettingsType::*;
        match s {
            "by_id" => Ok(ById),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingMeterResourceCustomerMappingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingMeterResourceCustomerMappingSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingMeterResourceCustomerMappingSettingsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingMeterResourceCustomerMappingSettingsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingMeterResourceCustomerMappingSettingsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingMeterResourceCustomerMappingSettingsType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingMeterResourceCustomerMappingSettingsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterResourceCustomerMappingSettingsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BillingMeterResourceCustomerMappingSettingsType",
            )
        })
    }
}
