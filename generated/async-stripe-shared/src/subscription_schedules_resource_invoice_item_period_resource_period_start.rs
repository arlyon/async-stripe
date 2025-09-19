#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart {
    /// A precise Unix timestamp for the start of the invoice item period.
    /// Must be less than or equal to `period.end`.
    pub timestamp: Option<stripe_types::Timestamp>,
    /// Select how to calculate the start of the invoice item period.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType,
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder {
    timestamp: Option<Option<stripe_types::Timestamp>>,
    type_: Option<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType>,
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

    impl Deserialize for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart>,
        builder: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder {
        type Out = SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "timestamp" => Deserialize::begin(&mut self.timestamp),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { timestamp: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(timestamp), Some(type_)) = (self.timestamp, self.type_) else {
                return None;
            };
            Some(Self::Out { timestamp, type_ })
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

    impl ObjectDeser for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart {
        type Builder = SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "timestamp" => b.timestamp = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Select how to calculate the start of the invoice item period.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    MaxItemPeriodStart,
    PhaseStart,
    Timestamp,
}
impl SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    pub fn as_str(self) -> &'static str {
        use SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType::*;
        match self {
            MaxItemPeriodStart => "max_item_period_start",
            PhaseStart => "phase_start",
            Timestamp => "timestamp",
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType::*;
        match s {
            "max_item_period_start" => Ok(MaxItemPeriodStart),
            "phase_start" => Ok(PhaseStart),
            "timestamp" => Ok(Timestamp),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType"))
    }
}
