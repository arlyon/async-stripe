#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    /// The ID of the Shipping Rate to use for this shipping option.
    pub shipping_rate: stripe_types::Expandable<stripe_shared::ShippingRate>,
}
#[doc(hidden)]
pub struct PaymentLinksResourceShippingOptionBuilder {
    shipping_amount: Option<i64>,
    shipping_rate: Option<stripe_types::Expandable<stripe_shared::ShippingRate>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentLinksResourceShippingOptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceShippingOptionBuilder {
        type Out = PaymentLinksResourceShippingOption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "shipping_amount" => Deserialize::begin(&mut self.shipping_amount),
                "shipping_rate" => Deserialize::begin(&mut self.shipping_rate),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { shipping_amount: Deserialize::default(), shipping_rate: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(shipping_amount), Some(shipping_rate)) =
                (self.shipping_amount, self.shipping_rate.take())
            else {
                return None;
            };
            Some(Self::Out { shipping_amount, shipping_rate })
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

    impl ObjectDeser for PaymentLinksResourceShippingOption {
        type Builder = PaymentLinksResourceShippingOptionBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceShippingOption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceShippingOptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "shipping_amount" => b.shipping_amount = FromValueOpt::from_value(v),
                    "shipping_rate" => b.shipping_rate = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
