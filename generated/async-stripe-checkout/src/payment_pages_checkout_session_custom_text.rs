#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    pub after_submit: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    pub terms_of_service_acceptance:
        Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomTextBuilder {
    after_submit: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>>,
    shipping_address:
        Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>>,
    submit: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>>,
    terms_of_service_acceptance:
        Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomTextPosition>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionCustomText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomText>,
        builder: PaymentPagesCheckoutSessionCustomTextBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomTextBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomTextBuilder {
        type Out = PaymentPagesCheckoutSessionCustomText;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "after_submit" => Deserialize::begin(&mut self.after_submit),
                "shipping_address" => Deserialize::begin(&mut self.shipping_address),
                "submit" => Deserialize::begin(&mut self.submit),
                "terms_of_service_acceptance" => {
                    Deserialize::begin(&mut self.terms_of_service_acceptance)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                after_submit: Deserialize::default(),
                shipping_address: Deserialize::default(),
                submit: Deserialize::default(),
                terms_of_service_acceptance: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(after_submit),
                Some(shipping_address),
                Some(submit),
                Some(terms_of_service_acceptance),
            ) = (
                self.after_submit.take(),
                self.shipping_address.take(),
                self.submit.take(),
                self.terms_of_service_acceptance.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { after_submit, shipping_address, submit, terms_of_service_acceptance })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomText {
        type Builder = PaymentPagesCheckoutSessionCustomTextBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCustomText {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCustomTextBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "after_submit" => b.after_submit = FromValueOpt::from_value(v),
                    "shipping_address" => b.shipping_address = FromValueOpt::from_value(v),
                    "submit" => b.submit = FromValueOpt::from_value(v),
                    "terms_of_service_acceptance" => {
                        b.terms_of_service_acceptance = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
