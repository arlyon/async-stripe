#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAcssDebit {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Estimated date to debit the customer's bank account. A date string in YYYY-MM-DD format.
    pub expected_debit_date: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Institution number of the bank account
    pub institution_number: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    pub mandate: Option<String>,
    /// Transit number of the bank account.
    pub transit_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsAcssDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsAcssDebitBuilder {
    bank_name: Option<Option<String>>,
    expected_debit_date: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    institution_number: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAcssDebit>,
        builder: PaymentMethodDetailsAcssDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsAcssDebitBuilder {
        type Out = PaymentMethodDetailsAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "expected_debit_date" => Deserialize::begin(&mut self.expected_debit_date),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "institution_number" => Deserialize::begin(&mut self.institution_number),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "transit_number" => Deserialize::begin(&mut self.transit_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_name: Some(None),
                expected_debit_date: Some(None),
                fingerprint: Some(None),
                institution_number: Some(None),
                last4: Some(None),
                mandate: Some(None),
                transit_number: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                bank_name: self.bank_name.take().flatten(),
                expected_debit_date: self.expected_debit_date.take().flatten(),
                fingerprint: self.fingerprint.take().flatten(),
                institution_number: self.institution_number.take().flatten(),
                last4: self.last4.take().flatten(),
                mandate: self.mandate.take().flatten(),
                transit_number: self.transit_number.take().flatten(),
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

    impl ObjectDeser for PaymentMethodDetailsAcssDebit {
        type Builder = PaymentMethodDetailsAcssDebitBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsAcssDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "expected_debit_date" => b.expected_debit_date = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "institution_number" => b.institution_number = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    "transit_number" => b.transit_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
