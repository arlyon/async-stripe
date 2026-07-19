#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordAffirm {
    /// ID of the location that this reader is assigned to.
    pub location: Option<String>,
    /// ID of the reader this transaction was made on.
    pub reader: Option<String>,
    /// The Affirm transaction ID associated with this payment.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordAffirm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordAffirm").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordAffirmBuilder {
    location: Option<Option<String>>,
    reader: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordAffirm {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordAffirm>,
        builder: PaymentMethodDetailsPaymentRecordAffirmBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordAffirm> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordAffirmBuilder {
                    location: Deserialize::default(),
                    reader: Deserialize::default(),
                    transaction_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "location" => Deserialize::begin(&mut self.builder.location),
                "reader" => Deserialize::begin(&mut self.builder.reader),
                "transaction_id" => Deserialize::begin(&mut self.builder.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(location), Some(reader), Some(transaction_id)) = (
                self.builder.location.take(),
                self.builder.reader.take(),
                self.builder.transaction_id.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentMethodDetailsPaymentRecordAffirm { location, reader, transaction_id });
            Ok(())
        }
    }
};
