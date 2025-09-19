/// A Climate product represents a type of carbon removal unit available for reservation.
/// You can retrieve it to see the current price and availability.
#[derive(Clone, Debug)]
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
                builder: ClimateProductBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ClimateProductBuilder {
        type Out = ClimateProduct;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "current_prices_per_metric_ton" => {
                    Deserialize::begin(&mut self.current_prices_per_metric_ton)
                }
                "delivery_year" => Deserialize::begin(&mut self.delivery_year),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metric_tons_available" => Deserialize::begin(&mut self.metric_tons_available),
                "name" => Deserialize::begin(&mut self.name),
                "suppliers" => Deserialize::begin(&mut self.suppliers),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                current_prices_per_metric_ton: Deserialize::default(),
                delivery_year: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metric_tons_available: Deserialize::default(),
                name: Deserialize::default(),
                suppliers: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.created,
                self.current_prices_per_metric_ton.take(),
                self.delivery_year,
                self.id.take(),
                self.livemode,
                self.metric_tons_available.take(),
                self.name.take(),
                self.suppliers.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                created,
                current_prices_per_metric_ton,
                delivery_year,
                id,
                livemode,
                metric_tons_available,
                name,
                suppliers,
            })
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

    impl ObjectDeser for ClimateProduct {
        type Builder = ClimateProductBuilder;
    }

    impl FromValueOpt for ClimateProduct {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ClimateProductBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "current_prices_per_metric_ton" => {
                        b.current_prices_per_metric_ton = FromValueOpt::from_value(v)
                    }
                    "delivery_year" => b.delivery_year = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metric_tons_available" => {
                        b.metric_tons_available = FromValueOpt::from_value(v)
                    }
                    "name" => b.name = FromValueOpt::from_value(v),
                    "suppliers" => b.suppliers = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
