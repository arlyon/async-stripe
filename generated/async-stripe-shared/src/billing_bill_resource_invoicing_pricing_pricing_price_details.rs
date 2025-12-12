#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingPricingPricingPriceDetails {
    /// The ID of the price this item is associated with.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
    /// The ID of the product this item is associated with.
    pub product: String,
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingPricingPricingPriceDetailsBuilder {
    price: Option<stripe_types::Expandable<stripe_shared::Price>>,
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

    impl Deserialize for BillingBillResourceInvoicingPricingPricingPriceDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingPricingPricingPriceDetails>,
        builder: BillingBillResourceInvoicingPricingPricingPriceDetailsBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingPricingPricingPriceDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    BillingBillResourceInvoicingPricingPricingPriceDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingPricingPricingPriceDetailsBuilder {
        type Out = BillingBillResourceInvoicingPricingPricingPriceDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "price" => Deserialize::begin(&mut self.price),
                "product" => Deserialize::begin(&mut self.product),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { price: Deserialize::default(), product: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(price), Some(product)) = (self.price.take(), self.product.take()) else {
                return None;
            };
            Some(Self::Out { price, product })
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

    impl ObjectDeser for BillingBillResourceInvoicingPricingPricingPriceDetails {
        type Builder = BillingBillResourceInvoicingPricingPricingPriceDetailsBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingPricingPricingPriceDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BillingBillResourceInvoicingPricingPricingPriceDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "price" => b.price = FromValueOpt::from_value(v),
                    "product" => b.product = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
