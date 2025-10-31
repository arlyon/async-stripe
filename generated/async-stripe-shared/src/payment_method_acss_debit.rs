#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodAcssDebit {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Institution number of the bank account.
    pub institution_number: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Transit number of the bank account.
    pub transit_number: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodAcssDebitBuilder {
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    institution_number: Option<Option<String>>,
    last4: Option<Option<String>>,
    transit_number: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodAcssDebit>,
        builder: PaymentMethodAcssDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodAcssDebitBuilder {
        type Out = PaymentMethodAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "institution_number" => Deserialize::begin(&mut self.institution_number),
                "last4" => Deserialize::begin(&mut self.last4),
                "transit_number" => Deserialize::begin(&mut self.transit_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_name: Deserialize::default(),
                fingerprint: Deserialize::default(),
                institution_number: Deserialize::default(),
                last4: Deserialize::default(),
                transit_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(bank_name),
                Some(fingerprint),
                Some(institution_number),
                Some(last4),
                Some(transit_number),
            ) = (
                self.bank_name.take(),
                self.fingerprint.take(),
                self.institution_number.take(),
                self.last4.take(),
                self.transit_number.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { bank_name, fingerprint, institution_number, last4, transit_number })
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

    impl ObjectDeser for PaymentMethodAcssDebit {
        type Builder = PaymentMethodAcssDebitBuilder;
    }

    impl FromValueOpt for PaymentMethodAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodAcssDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "institution_number" => b.institution_number = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "transit_number" => b.transit_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
