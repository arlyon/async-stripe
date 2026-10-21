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
                "adjustable_quantity" => Deserialize::begin(&mut self.adjustable_quantity),
                "prices" => Deserialize::begin(&mut self.prices),
                "product" => Deserialize::begin(&mut self.product),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { adjustable_quantity: None, prices: None, product: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    adjustable_quantity: Some(adjustable_quantity),
                    prices: Some(prices),
                    product: Some(product),
                } => Some(Self::Out { adjustable_quantity, prices, product }),
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

    impl ObjectDeser for PortalSubscriptionUpdateProduct {
        type Builder = PortalSubscriptionUpdateProductBuilder;
    }

    impl FromValueOpt for PortalSubscriptionUpdateProduct {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
