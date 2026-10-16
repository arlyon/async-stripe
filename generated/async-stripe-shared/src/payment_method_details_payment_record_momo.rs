#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordMomo {
    /// Uniquely identifies this particular MoMo account.
    /// You can use this attribute to check whether two MoMo accounts are the same.
    pub fingerprint: Option<String>,
    /// ID of the multi-use Mandate created by, or used to make, this MoMo payment.
    pub mandate: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordMomo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordMomo").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordMomoBuilder {
    fingerprint: Option<Option<String>>,
    mandate: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordMomo {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordMomo>,
        builder: PaymentMethodDetailsPaymentRecordMomoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordMomo> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordMomoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaymentRecordMomoBuilder {
        type Out = PaymentMethodDetailsPaymentRecordMomo;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "mandate" => Deserialize::begin(&mut self.mandate),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { fingerprint: Some(None), mandate: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fingerprint), Some(mandate)) = (self.fingerprint.take(), self.mandate.take())
            else {
                return None;
            };
            Some(Self::Out { fingerprint, mandate })
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

    impl ObjectDeser for PaymentMethodDetailsPaymentRecordMomo {
        type Builder = PaymentMethodDetailsPaymentRecordMomoBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsPaymentRecordMomo {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsPaymentRecordMomoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
