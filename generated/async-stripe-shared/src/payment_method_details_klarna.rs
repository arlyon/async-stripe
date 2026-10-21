#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKlarna {
    /// ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to.
    pub location: Option<String>,
    /// The payer details for this transaction.
    pub payer_details: Option<stripe_shared::KlarnaPayerDetails>,
    /// The Klarna payment method used for this transaction.
    /// Can be one of `pay_later`, `pay_now`, `pay_with_financing`, or `pay_in_installments`
    pub payment_method_category: Option<String>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    /// Can be one of `de-AT`, `en-AT`, `nl-BE`, `fr-BE`, `en-BE`, `de-DE`, `en-DE`, `da-DK`, `en-DK`, `es-ES`, `en-ES`, `fi-FI`, `sv-FI`, `en-FI`, `en-GB`, `en-IE`, `it-IT`, `en-IT`, `nl-NL`, `en-NL`, `nb-NO`, `en-NO`, `sv-SE`, `en-SE`, `en-US`, `es-US`, `fr-FR`, `en-FR`, `cs-CZ`, `en-CZ`, `ro-RO`, `en-RO`, `el-GR`, `en-GR`, `en-AU`, `en-NZ`, `en-CA`, `fr-CA`, `pl-PL`, `en-PL`, `pt-PT`, `en-PT`, `de-CH`, `fr-CH`, `it-CH`, or `en-CH`.
    pub preferred_locale: Option<String>,
    /// ID of the [reader](https://docs.stripe.com/api/terminal/readers) this transaction was made on.
    pub reader: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsKlarna").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsKlarnaBuilder {
    location: Option<Option<String>>,
    payer_details: Option<stripe_shared::KlarnaPayerDetails>,
    payment_method_category: Option<Option<String>>,
    preferred_locale: Option<Option<String>>,
    reader: Option<Option<String>>,
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
                "location" => Deserialize::begin(&mut self.location),
                "payer_details" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payer_details)
                }
                "payment_method_category" => Deserialize::begin(&mut self.payment_method_category),
                "preferred_locale" => Deserialize::begin(&mut self.preferred_locale),
                "reader" => Deserialize::begin(&mut self.reader),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                location: Some(None),
                payer_details: None,
                payment_method_category: Some(None),
                preferred_locale: Some(None),
                reader: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    location: Some(location),
                    payer_details,
                    payment_method_category: Some(payment_method_category),
                    preferred_locale: Some(preferred_locale),
                    reader: Some(reader),
                } => Some(Self::Out {
                    location,
                    payer_details,
                    payment_method_category,
                    preferred_locale,
                    reader,
                }),
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

    impl ObjectDeser for PaymentMethodDetailsKlarna {
        type Builder = PaymentMethodDetailsKlarnaBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsKlarna {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
