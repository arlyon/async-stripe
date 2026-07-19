#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder {
    timestamp: Option<Option<stripe_types::Timestamp>>,
    type_: Option<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType>,
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
                builder: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartBuilder {
                    timestamp: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "timestamp" => Deserialize::begin(&mut self.builder.timestamp),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(timestamp), Some(type_)) =
                (self.builder.timestamp, self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart {
                timestamp,
                type_,
            });
            Ok(())
        }
    }
};
/// Select how to calculate the start of the invoice item period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    MaxItemPeriodStart,
    PhaseStart,
    Timestamp,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType::*;
        match self {
            MaxItemPeriodStart => "max_item_period_start",
            PhaseStart => "phase_start",
            Timestamp => "timestamp",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType::*;
        match s {
            "max_item_period_start" => Ok(MaxItemPeriodStart),
            "phase_start" => Ok(PhaseStart),
            "timestamp" => Ok(Timestamp),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType
        ))
        .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize
    for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
