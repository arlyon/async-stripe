/// The delivery of a specified quantity of carbon for an order.
#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct ClimateRemovalsOrderDeliveriesBuilder {
    delivered_at: Option<stripe_types::Timestamp>,
    location: Option<Option<stripe_misc::ClimateRemovalsLocation>>,
    metric_tons: Option<String>,
    registry_url: Option<Option<String>>,
    supplier: Option<stripe_misc::ClimateSupplier>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "location" => Deserialize::begin(&mut self.location),
                "metric_tons" => Deserialize::begin(&mut self.metric_tons),
                "registry_url" => Deserialize::begin(&mut self.registry_url),
                "supplier" => Deserialize::begin(&mut self.supplier),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                delivered_at: Deserialize::default(),
                location: Deserialize::default(),
                metric_tons: Deserialize::default(),
                registry_url: Deserialize::default(),
                supplier: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                delivered_at: self.delivered_at?,
                location: self.location.take()?,
                metric_tons: self.metric_tons.take()?,
                registry_url: self.registry_url.take()?,
                supplier: self.supplier.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for ClimateRemovalsOrderDeliveries {
        type Builder = ClimateRemovalsOrderDeliveriesBuilder;
    }

    impl FromValueOpt for ClimateRemovalsOrderDeliveries {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ClimateRemovalsOrderDeliveriesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "delivered_at" => b.delivered_at = Some(FromValueOpt::from_value(v)?),
                    "location" => b.location = Some(FromValueOpt::from_value(v)?),
                    "metric_tons" => b.metric_tons = Some(FromValueOpt::from_value(v)?),
                    "registry_url" => b.registry_url = Some(FromValueOpt::from_value(v)?),
                    "supplier" => b.supplier = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
