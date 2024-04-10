#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionUpdateProduct {
    /// The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,
    /// The product ID.
    pub product: String,
}
#[doc(hidden)]
pub struct PortalSubscriptionUpdateProductBuilder {
    prices: Option<Vec<String>>,
    product: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalSubscriptionUpdateProduct {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionUpdateProduct>,
        builder: PortalSubscriptionUpdateProductBuilder,
    }

    impl Visitor for Place<PortalSubscriptionUpdateProduct> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionUpdateProductBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionUpdateProductBuilder {
        type Out = PortalSubscriptionUpdateProduct;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "prices" => Deserialize::begin(&mut self.prices),
                "product" => Deserialize::begin(&mut self.product),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { prices: Deserialize::default(), product: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { prices: self.prices.take()?, product: self.product.take()? })
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

    impl ObjectDeser for PortalSubscriptionUpdateProduct {
        type Builder = PortalSubscriptionUpdateProductBuilder;
    }

    impl FromValueOpt for PortalSubscriptionUpdateProduct {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionUpdateProductBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "prices" => b.prices = Some(FromValueOpt::from_value(v)?),
                    "product" => b.product = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
