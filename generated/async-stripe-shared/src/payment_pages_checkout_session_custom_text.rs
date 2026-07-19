#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    pub after_submit: Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    pub terms_of_service_acceptance:
        Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionCustomText").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomTextBuilder {
    after_submit: Option<Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>>,
    shipping_address: Option<Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>>,
    submit: Option<Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>>,
    terms_of_service_acceptance:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionCustomTextPosition>>,
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
                builder: PaymentPagesCheckoutSessionCustomTextBuilder {
                    after_submit: Deserialize::default(),
                    shipping_address: Deserialize::default(),
                    submit: Deserialize::default(),
                    terms_of_service_acceptance: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "after_submit" => Deserialize::begin(&mut self.builder.after_submit),
                "shipping_address" => Deserialize::begin(&mut self.builder.shipping_address),
                "submit" => Deserialize::begin(&mut self.builder.submit),
                "terms_of_service_acceptance" => {
                    Deserialize::begin(&mut self.builder.terms_of_service_acceptance)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(after_submit),
                Some(shipping_address),
                Some(submit),
                Some(terms_of_service_acceptance),
            ) = (
                self.builder.after_submit.take(),
                self.builder.shipping_address.take(),
                self.builder.submit.take(),
                self.builder.terms_of_service_acceptance.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionCustomText {
                after_submit,
                shipping_address,
                submit,
                terms_of_service_acceptance,
            });
            Ok(())
        }
    }
};
