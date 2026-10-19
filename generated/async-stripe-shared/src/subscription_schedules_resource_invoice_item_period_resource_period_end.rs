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
    pub type_: stripe_shared::CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType,
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
    type_: Option<stripe_shared::CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType>,
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
            builder: SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndBuilder {
        type Out = SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "timestamp" => Deserialize::begin(&mut self.timestamp),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { timestamp: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(timestamp), Some(type_)) = (self.timestamp, self.type_.take()) else {
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd {
        type Builder = SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndBuilder::deser_default();
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
/// Select how to calculate the end of the invoice item period.
pub type SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType =
    stripe_shared::CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType;
