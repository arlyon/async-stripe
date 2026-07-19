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
    location: Option<Option<stripe_misc::ClimateRemovalsLocation>>,
    metric_tons: Option<String>,
    registry_url: Option<Option<String>>,
    supplier: Option<stripe_misc::ClimateSupplier>,
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
                builder: ClimateRemovalsOrderDeliveriesBuilder {
                    delivered_at: Deserialize::default(),
                    location: Deserialize::default(),
                    metric_tons: Deserialize::default(),
                    registry_url: Deserialize::default(),
                    supplier: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "delivered_at" => Deserialize::begin(&mut self.builder.delivered_at),
                "location" => Deserialize::begin(&mut self.builder.location),
                "metric_tons" => Deserialize::begin(&mut self.builder.metric_tons),
                "registry_url" => Deserialize::begin(&mut self.builder.registry_url),
                "supplier" => Deserialize::begin(&mut self.builder.supplier),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(delivered_at),
                Some(location),
                Some(metric_tons),
                Some(registry_url),
                Some(supplier),
            ) = (
                self.builder.delivered_at,
                self.builder.location.take(),
                self.builder.metric_tons.take(),
                self.builder.registry_url.take(),
                self.builder.supplier.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ClimateRemovalsOrderDeliveries {
                delivered_at,
                location,
                metric_tons,
                registry_url,
                supplier,
            });
            Ok(())
        }
    }
};
