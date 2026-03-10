#[derive(Clone, Debug, Eq, PartialEq)]
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
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordAffirmBuilder {
    location: Option<Option<String>>,
    reader: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
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
                builder: PaymentMethodDetailsPaymentRecordAffirmBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaymentRecordAffirmBuilder {
        type Out = PaymentMethodDetailsPaymentRecordAffirm;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "location" => Deserialize::begin(&mut self.location),
                "reader" => Deserialize::begin(&mut self.reader),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                location: Deserialize::default(),
                reader: Deserialize::default(),
                transaction_id: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(location), Some(reader), Some(transaction_id)) =
                (self.location.take(), self.reader.take(), self.transaction_id.take())
            else {
                return None;
            };
            Some(Self::Out { location, reader, transaction_id })
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

    impl ObjectDeser for PaymentMethodDetailsPaymentRecordAffirm {
        type Builder = PaymentMethodDetailsPaymentRecordAffirmBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsPaymentRecordAffirm {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsPaymentRecordAffirmBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "location" => b.location = FromValueOpt::from_value(v),
                    "reader" => b.reader = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
