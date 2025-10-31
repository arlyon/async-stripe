#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodAuBecsDebit {
    /// Six-digit number identifying bank and branch associated with this bank account.
    pub bsb_number: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodAuBecsDebitBuilder {
    bsb_number: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodAuBecsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodAuBecsDebit>,
        builder: PaymentMethodAuBecsDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodAuBecsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodAuBecsDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodAuBecsDebitBuilder {
        type Out = PaymentMethodAuBecsDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bsb_number" => Deserialize::begin(&mut self.bsb_number),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bsb_number: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bsb_number), Some(fingerprint), Some(last4)) =
                (self.bsb_number.take(), self.fingerprint.take(), self.last4.take())
            else {
                return None;
            };
            Some(Self::Out { bsb_number, fingerprint, last4 })
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

    impl ObjectDeser for PaymentMethodAuBecsDebit {
        type Builder = PaymentMethodAuBecsDebitBuilder;
    }

    impl FromValueOpt for PaymentMethodAuBecsDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodAuBecsDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bsb_number" => b.bsb_number = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
