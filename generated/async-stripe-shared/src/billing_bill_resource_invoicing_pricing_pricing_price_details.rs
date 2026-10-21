#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingPricingPricingPriceDetails {
    /// The ID of the price this item is associated with.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
    /// The ID of the product this item is associated with.
    pub product: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingPricingPricingPriceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingPricingPricingPriceDetails")
            .finish_non_exhaustive()
    }
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
            Self { price: None, product: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { price: Some(price), product: Some(product) } => {
                    Some(Self::Out { price, product })
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

    impl ObjectDeser for BillingBillResourceInvoicingPricingPricingPriceDetails {
        type Builder = BillingBillResourceInvoicingPricingPricingPriceDetailsBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingPricingPricingPriceDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
