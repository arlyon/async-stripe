#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsKlarna").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodDetailsKlarnaBuilder {
                    payer_details: Deserialize::default(),
                    payment_method_category: Deserialize::default(),
                    preferred_locale: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payer_details" => Deserialize::begin(&mut self.builder.payer_details),
                "payment_method_category" => {
                    Deserialize::begin(&mut self.builder.payment_method_category)
                }
                "preferred_locale" => Deserialize::begin(&mut self.builder.preferred_locale),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payer_details), Some(payment_method_category), Some(preferred_locale)) = (
                self.builder.payer_details.take(),
                self.builder.payment_method_category.take(),
                self.builder.preferred_locale.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsKlarna {
                payer_details,
                payment_method_category,
                preferred_locale,
            });
            Ok(())
        }
    }
};
