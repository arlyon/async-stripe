/// The delivery of a specified quantity of carbon for an order.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ClimateRemovalsOrderDeliveries {
    /// Time at which the delivery occurred. Measured in seconds since the Unix epoch.
    pub delivered_at: stripe_types::Timestamp,
    /// Specific location of this delivery.
    pub location: Option<stripe_misc::ClimateRemovalsLocation>,
    /// Quantity of carbon removal supplied by this delivery.
    pub metric_tons: String,
    /// Once retired, a URL to the registry entry for the tons from this delivery.
    pub registry_url: Option<String>,
    pub supplier: stripe_misc::ClimateSupplier,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ClimateRemovalsOrderDeliveries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClimateRemovalsOrderDeliveries").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ClimateRemovalsOrderDeliveriesBuilder {
    delivered_at: Option<stripe_types::Timestamp>,
    location: Option<stripe_misc::ClimateRemovalsLocation>,
    metric_tons: Option<String>,
    registry_url: Option<Option<String>>,
    supplier: Option<stripe_misc::ClimateSupplier>,
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

    impl Deserialize for ClimateRemovalsOrderDeliveries {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ClimateRemovalsOrderDeliveries>,
        builder: ClimateRemovalsOrderDeliveriesBuilder,
    }

    impl Visitor for Place<ClimateRemovalsOrderDeliveries> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ClimateRemovalsOrderDeliveriesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ClimateRemovalsOrderDeliveriesBuilder {
        type Out = ClimateRemovalsOrderDeliveries;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "delivered_at" => Deserialize::begin(&mut self.delivered_at),
                "location" => stripe_types::miniserde_helpers::begin_nullable(&mut self.location),
                "metric_tons" => Deserialize::begin(&mut self.metric_tons),
                "registry_url" => Deserialize::begin(&mut self.registry_url),
                "supplier" => Deserialize::begin(&mut self.supplier),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                delivered_at: None,
                location: None,
                metric_tons: None,
                registry_url: Some(None),
                supplier: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    delivered_at: Some(delivered_at),
                    location,
                    metric_tons: Some(metric_tons),
                    registry_url: Some(registry_url),
                    supplier: Some(supplier),
                } => {
                    Some(Self::Out { delivered_at, location, metric_tons, registry_url, supplier })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for ClimateRemovalsOrderDeliveries {
        type Builder = ClimateRemovalsOrderDeliveriesBuilder;
    }

    impl FromValueOpt for ClimateRemovalsOrderDeliveries {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
