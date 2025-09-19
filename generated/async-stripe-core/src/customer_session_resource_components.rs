/// Configuration for the components supported by this Customer Session.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponents {
    pub buy_button: stripe_core::CustomerSessionResourceComponentsResourceBuyButton,
    pub payment_element: stripe_core::CustomerSessionResourceComponentsResourcePaymentElement,
    pub pricing_table: stripe_core::CustomerSessionResourceComponentsResourcePricingTable,
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsBuilder {
    buy_button: Option<stripe_core::CustomerSessionResourceComponentsResourceBuyButton>,
    payment_element: Option<stripe_core::CustomerSessionResourceComponentsResourcePaymentElement>,
    pricing_table: Option<stripe_core::CustomerSessionResourceComponentsResourcePricingTable>,
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

    impl Deserialize for CustomerSessionResourceComponents {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponents>,
        builder: CustomerSessionResourceComponentsBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponents> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionResourceComponentsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerSessionResourceComponentsBuilder {
        type Out = CustomerSessionResourceComponents;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buy_button" => Deserialize::begin(&mut self.buy_button),
                "payment_element" => Deserialize::begin(&mut self.payment_element),
                "pricing_table" => Deserialize::begin(&mut self.pricing_table),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                buy_button: Deserialize::default(),
                payment_element: Deserialize::default(),
                pricing_table: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(buy_button), Some(payment_element), Some(pricing_table)) =
                (self.buy_button, self.payment_element.take(), self.pricing_table)
            else {
                return None;
            };
            Some(Self::Out { buy_button, payment_element, pricing_table })
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

    impl ObjectDeser for CustomerSessionResourceComponents {
        type Builder = CustomerSessionResourceComponentsBuilder;
    }

    impl FromValueOpt for CustomerSessionResourceComponents {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "buy_button" => b.buy_button = FromValueOpt::from_value(v),
                    "payment_element" => b.payment_element = FromValueOpt::from_value(v),
                    "pricing_table" => b.pricing_table = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
