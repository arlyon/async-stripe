#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsBacsDebit {
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    pub mandate: Option<String>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    pub sort_code: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsBacsDebitBuilder {
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<String>>,
    sort_code: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsBacsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsBacsDebit>,
        builder: PaymentMethodDetailsBacsDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsBacsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsBacsDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsBacsDebitBuilder {
        type Out = PaymentMethodDetailsBacsDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "sort_code" => Deserialize::begin(&mut self.sort_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                mandate: Deserialize::default(),
                sort_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                fingerprint: self.fingerprint.take()?,
                last4: self.last4.take()?,
                mandate: self.mandate.take()?,
                sort_code: self.sort_code.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodDetailsBacsDebit {
        type Builder = PaymentMethodDetailsBacsDebitBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsBacsDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsBacsDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "mandate" => b.mandate = Some(FromValueOpt::from_value(v)?),
                    "sort_code" => b.sort_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
