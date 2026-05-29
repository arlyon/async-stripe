#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordKlarna {
        /// ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to.
pub location: Option<String>,
    /// The payer details for this transaction.
pub payer_details: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKlarnaDetailsResourcePayerDetails>,
        /// The Klarna payment method used for this transaction.
    /// Can be one of `pay_later`, `pay_now`, `pay_with_financing`, or `pay_in_installments`.
pub payment_method_category: Option<String>,
        /// Preferred language of the Klarna authorization page that the customer is redirected to.
    /// Can be one of `de-AT`, `en-AT`, `nl-BE`, `fr-BE`, `en-BE`, `de-DE`, `en-DE`, `da-DK`, `en-DK`, `es-ES`, `en-ES`, `fi-FI`, `sv-FI`, `en-FI`, `en-GB`, `en-IE`, `it-IT`, `en-IT`, `nl-NL`, `en-NL`, `nb-NO`, `en-NO`, `sv-SE`, `en-SE`, `en-US`, `es-US`, `fr-FR`, `en-FR`, `cs-CZ`, `en-CZ`, `ro-RO`, `en-RO`, `el-GR`, `en-GR`, `en-AU`, `en-NZ`, `en-CA`, `fr-CA`, `pl-PL`, `en-PL`, `pt-PT`, `en-PT`, `de-CH`, `fr-CH`, `it-CH`, or `en-CH`.
pub preferred_locale: Option<String>,
    /// ID of the [reader](https://docs.stripe.com/api/terminal/readers) this transaction was made on.
pub reader: Option<String>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordKlarna").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordKlarnaBuilder {
    location: Option<Option<String>>,
payer_details: Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKlarnaDetailsResourcePayerDetails>>,
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordKlarna>,
        builder: PaymentMethodDetailsPaymentRecordKlarnaBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordKlarnaBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaymentRecordKlarnaBuilder {
        type Out = PaymentMethodDetailsPaymentRecordKlarna;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "location" => Deserialize::begin(&mut self.location),
                "payer_details" => Deserialize::begin(&mut self.payer_details),
                "payment_method_category" => Deserialize::begin(&mut self.payment_method_category),
                "preferred_locale" => Deserialize::begin(&mut self.preferred_locale),
                "reader" => Deserialize::begin(&mut self.reader),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                location: Some(None),
                payer_details: Some(None),
                payment_method_category: Some(None),
                preferred_locale: Some(None),
                reader: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(location),
                Some(payer_details),
                Some(payment_method_category),
                Some(preferred_locale),
                Some(reader),
            ) = (
                self.location.take(),
                self.payer_details.take(),
                self.payment_method_category.take(),
                self.preferred_locale.take(),
                self.reader.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                location,
                payer_details,
                payment_method_category,
                preferred_locale,
                reader,
            })
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

    impl ObjectDeser for PaymentMethodDetailsPaymentRecordKlarna {
        type Builder = PaymentMethodDetailsPaymentRecordKlarnaBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsPaymentRecordKlarna {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsPaymentRecordKlarnaBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "location" => b.location = FromValueOpt::from_value(v),
                    "payer_details" => b.payer_details = FromValueOpt::from_value(v),
                    "payment_method_category" => {
                        b.payment_method_category = FromValueOpt::from_value(v)
                    }
                    "preferred_locale" => b.preferred_locale = FromValueOpt::from_value(v),
                    "reader" => b.reader = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
