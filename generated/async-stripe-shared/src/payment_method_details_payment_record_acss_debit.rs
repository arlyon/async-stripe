#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordAcssDebit {
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
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordAcssDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordAcssDebitBuilder {
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordAcssDebit>,
        builder: PaymentMethodDetailsPaymentRecordAcssDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaymentRecordAcssDebitBuilder {
        type Out = PaymentMethodDetailsPaymentRecordAcssDebit;
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
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    bank_name: Some(bank_name),
                    expected_debit_date: Some(expected_debit_date),
                    fingerprint: Some(fingerprint),
                    institution_number: Some(institution_number),
                    last4: Some(last4),
                    mandate: Some(mandate),
                    transit_number: Some(transit_number),
                } => Some(Self::Out {
                    bank_name,
                    expected_debit_date,
                    fingerprint,
                    institution_number,
                    last4,
                    mandate,
                    transit_number,
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

    impl ObjectDeser for PaymentMethodDetailsPaymentRecordAcssDebit {
        type Builder = PaymentMethodDetailsPaymentRecordAcssDebitBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsPaymentRecordAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
