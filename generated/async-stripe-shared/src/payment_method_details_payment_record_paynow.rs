#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordPaynow {
    /// ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to.
    pub location: Option<String>,
    /// ID of the [reader](https://docs.stripe.com/api/terminal/readers) this transaction was made on.
    pub reader: Option<String>,
    /// Reference number associated with this PayNow payment
    pub reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordPaynow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordPaynow").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordPaynowBuilder {
    location: Option<Option<String>>,
    reader: Option<Option<String>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordPaynow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordPaynow>,
        builder: PaymentMethodDetailsPaymentRecordPaynowBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordPaynow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordPaynowBuilder {
                    location: Deserialize::default(),
                    reader: Deserialize::default(),
                    reference: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "location" => Deserialize::begin(&mut self.builder.location),
                "reader" => Deserialize::begin(&mut self.builder.reader),
                "reference" => Deserialize::begin(&mut self.builder.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(location), Some(reader), Some(reference)) = (
                self.builder.location.take(),
                self.builder.reader.take(),
                self.builder.reference.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentMethodDetailsPaymentRecordPaynow { location, reader, reference });
            Ok(())
        }
    }
};
