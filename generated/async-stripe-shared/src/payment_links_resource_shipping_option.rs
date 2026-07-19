#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    /// The ID of the Shipping Rate to use for this shipping option.
    pub shipping_rate: stripe_types::Expandable<stripe_shared::ShippingRate>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceShippingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceShippingOption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceShippingOptionBuilder {
    shipping_amount: Option<i64>,
    shipping_rate: Option<stripe_types::Expandable<stripe_shared::ShippingRate>>,
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

    impl Deserialize for PaymentLinksResourceShippingOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceShippingOption>,
        builder: PaymentLinksResourceShippingOptionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceShippingOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceShippingOptionBuilder {
                    shipping_amount: Deserialize::default(),
                    shipping_rate: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "shipping_amount" => Deserialize::begin(&mut self.builder.shipping_amount),
                "shipping_rate" => Deserialize::begin(&mut self.builder.shipping_rate),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(shipping_amount), Some(shipping_rate)) =
                (self.builder.shipping_amount, self.builder.shipping_rate.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceShippingOption { shipping_amount, shipping_rate });
            Ok(())
        }
    }
};
