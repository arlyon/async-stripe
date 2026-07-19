#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    pub after_submit: Option<stripe_shared::PaymentLinksResourceCustomTextPosition>,
    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<stripe_shared::PaymentLinksResourceCustomTextPosition>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<stripe_shared::PaymentLinksResourceCustomTextPosition>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    pub terms_of_service_acceptance: Option<stripe_shared::PaymentLinksResourceCustomTextPosition>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCustomText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCustomText").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomTextBuilder {
    after_submit: Option<Option<stripe_shared::PaymentLinksResourceCustomTextPosition>>,
    shipping_address: Option<Option<stripe_shared::PaymentLinksResourceCustomTextPosition>>,
    submit: Option<Option<stripe_shared::PaymentLinksResourceCustomTextPosition>>,
    terms_of_service_acceptance:
        Option<Option<stripe_shared::PaymentLinksResourceCustomTextPosition>>,
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

    impl Deserialize for PaymentLinksResourceCustomText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomText>,
        builder: PaymentLinksResourceCustomTextBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomTextBuilder {
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
            *self.out = Some(PaymentLinksResourceCustomText {
                after_submit,
                shipping_address,
                submit,
                terms_of_service_acceptance,
            });
            Ok(())
        }
    }
};
