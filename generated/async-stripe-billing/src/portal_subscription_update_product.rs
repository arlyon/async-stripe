#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionUpdateProduct {
    pub adjustable_quantity: stripe_billing::PortalSubscriptionUpdateProductAdjustableQuantity,
    /// The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,
    /// The product ID.
    pub product: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdateProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionUpdateProduct").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionUpdateProductBuilder {
    adjustable_quantity: Option<stripe_billing::PortalSubscriptionUpdateProductAdjustableQuantity>,
    prices: Option<Vec<String>>,
    product: Option<String>,
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
                builder: PortalSubscriptionUpdateProductBuilder {
                    adjustable_quantity: Deserialize::default(),
                    prices: Deserialize::default(),
                    product: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "adjustable_quantity" => Deserialize::begin(&mut self.builder.adjustable_quantity),
                "prices" => Deserialize::begin(&mut self.builder.prices),
                "product" => Deserialize::begin(&mut self.builder.product),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(adjustable_quantity), Some(prices), Some(product)) = (
                self.builder.adjustable_quantity,
                self.builder.prices.take(),
                self.builder.product.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PortalSubscriptionUpdateProduct { adjustable_quantity, prices, product });
            Ok(())
        }
    }
};
