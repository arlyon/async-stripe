#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKlarna {
    /// The payer details for this transaction.
    pub payer_details: Option<stripe_shared::KlarnaPayerDetails>,
    /// The Klarna payment method used for this transaction.
    /// Can be one of `pay_later`, `pay_now`, `pay_with_financing`, or `pay_in_installments`
    pub payment_method_category: Option<String>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    /// Can be one of `de-AT`, `en-AT`, `nl-BE`, `fr-BE`, `en-BE`, `de-DE`, `en-DE`, `da-DK`, `en-DK`, `es-ES`, `en-ES`, `fi-FI`, `sv-FI`, `en-FI`, `en-GB`, `en-IE`, `it-IT`, `en-IT`, `nl-NL`, `en-NL`, `nb-NO`, `en-NO`, `sv-SE`, `en-SE`, `en-US`, `es-US`, `fr-FR`, `en-FR`, `cs-CZ`, `en-CZ`, `ro-RO`, `en-RO`, `el-GR`, `en-GR`, `en-AU`, `en-NZ`, `en-CA`, `fr-CA`, `pl-PL`, `en-PL`, `pt-PT`, `en-PT`, `de-CH`, `fr-CH`, `it-CH`, or `en-CH`.
    pub preferred_locale: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsKlarnaBuilder {
    payer_details: Option<Option<stripe_shared::KlarnaPayerDetails>>,
    payment_method_category: Option<Option<String>>,
    preferred_locale: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKlarna>,
        builder: PaymentMethodDetailsKlarnaBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsKlarnaBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsKlarnaBuilder {
        type Out = PaymentMethodDetailsKlarna;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payer_details" => Deserialize::begin(&mut self.payer_details),
                "payment_method_category" => Deserialize::begin(&mut self.payment_method_category),
                "preferred_locale" => Deserialize::begin(&mut self.preferred_locale),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payer_details: Deserialize::default(),
                payment_method_category: Deserialize::default(),
                preferred_locale: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payer_details), Some(payment_method_category), Some(preferred_locale)) = (
                self.payer_details.take(),
                self.payment_method_category.take(),
                self.preferred_locale.take(),
            ) else {
                return None;
            };
            Some(Self::Out { payer_details, payment_method_category, preferred_locale })
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

    impl ObjectDeser for PaymentMethodDetailsKlarna {
        type Builder = PaymentMethodDetailsKlarnaBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsKlarna {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsKlarnaBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payer_details" => b.payer_details = FromValueOpt::from_value(v),
                    "payment_method_category" => {
                        b.payment_method_category = FromValueOpt::from_value(v)
                    }
                    "preferred_locale" => b.preferred_locale = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
