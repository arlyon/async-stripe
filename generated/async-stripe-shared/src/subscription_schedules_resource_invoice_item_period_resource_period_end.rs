#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd {
    /// A precise Unix timestamp for the end of the invoice item period.
    /// Must be greater than or equal to `period.start`.
    pub timestamp: Option<stripe_types::Timestamp>,
    /// Select how to calculate the end of the invoice item period.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndBuilder {
    timestamp: Option<Option<stripe_types::Timestamp>>,
    type_: Option<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType>,
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

    impl Deserialize for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd>,
        builder: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndBuilder {
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
            *self.out = Some(SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd {
                timestamp,
                type_,
            });
            Ok(())
        }
    }
};
/// Select how to calculate the end of the invoice item period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType {
    MinItemPeriodEnd,
    PhaseEnd,
    Timestamp,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType::*;
        match self {
            MinItemPeriodEnd => "min_item_period_end",
            PhaseEnd => "phase_end",
            Timestamp => "timestamp",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType::*;
        match s {
            "min_item_period_end" => Ok(MinItemPeriodEnd),
            "phase_end" => Ok(PhaseEnd),
            "timestamp" => Ok(Timestamp),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
