/// A Climate product represents a type of carbon removal unit available for reservation.
/// You can retrieve it to see the current price and availability.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ClimateProduct {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Current prices for a metric ton of carbon removal in a currency's smallest unit.
    pub current_prices_per_metric_ton:
        std::collections::HashMap<String, stripe_misc::ClimateRemovalsProductsPrice>,
    /// The year in which the carbon removal is expected to be delivered.
    pub delivery_year: Option<i64>,
    /// Unique identifier for the object. For convenience, Climate product IDs are human-readable strings
    /// that start with `climsku_`.
    /// See [carbon removal inventory](https://stripe.com/docs/climate/orders/carbon-removal-inventory).
    /// for a list of available carbon removal products.
    pub id: stripe_misc::ClimateProductId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The quantity of metric tons available for reservation.
    pub metric_tons_available: String,
    /// The Climate product's name.
    pub name: String,
    /// The carbon removal suppliers that fulfill orders for this Climate product.
    pub suppliers: Vec<stripe_misc::ClimateSupplier>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ClimateProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClimateProduct").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ClimateProductBuilder {
    created: Option<stripe_types::Timestamp>,
    current_prices_per_metric_ton:
        Option<std::collections::HashMap<String, stripe_misc::ClimateRemovalsProductsPrice>>,
    delivery_year: Option<Option<i64>>,
    id: Option<stripe_misc::ClimateProductId>,
    livemode: Option<bool>,
    metric_tons_available: Option<String>,
    name: Option<String>,
    suppliers: Option<Vec<stripe_misc::ClimateSupplier>>,
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

    impl Deserialize for ClimateProduct {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ClimateProduct>,
        builder: ClimateProductBuilder,
    }

    impl Visitor for Place<ClimateProduct> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ClimateProductBuilder {
                    created: Deserialize::default(),
                    current_prices_per_metric_ton: Deserialize::default(),
                    delivery_year: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metric_tons_available: Deserialize::default(),
                    name: Deserialize::default(),
                    suppliers: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "current_prices_per_metric_ton" => {
                    Deserialize::begin(&mut self.builder.current_prices_per_metric_ton)
                }
                "delivery_year" => Deserialize::begin(&mut self.builder.delivery_year),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metric_tons_available" => {
                    Deserialize::begin(&mut self.builder.metric_tons_available)
                }
                "name" => Deserialize::begin(&mut self.builder.name),
                "suppliers" => Deserialize::begin(&mut self.builder.suppliers),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(current_prices_per_metric_ton),
                Some(delivery_year),
                Some(id),
                Some(livemode),
                Some(metric_tons_available),
                Some(name),
                Some(suppliers),
            ) = (
                self.builder.created,
                self.builder.current_prices_per_metric_ton.take(),
                self.builder.delivery_year,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metric_tons_available.take(),
                self.builder.name.take(),
                self.builder.suppliers.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ClimateProduct {
                created,
                current_prices_per_metric_ton,
                delivery_year,
                id,
                livemode,
                metric_tons_available,
                name,
                suppliers,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ClimateProduct {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ClimateProduct", 9)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("current_prices_per_metric_ton", &self.current_prices_per_metric_ton)?;
        s.serialize_field("delivery_year", &self.delivery_year)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metric_tons_available", &self.metric_tons_available)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("suppliers", &self.suppliers)?;

        s.serialize_field("object", "climate.product")?;
        s.end()
    }
}
impl stripe_types::Object for ClimateProduct {
    type Id = stripe_misc::ClimateProductId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ClimateProductId);
