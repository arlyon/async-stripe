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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodDetailsAcssDebitBuilder {
                    bank_name: Deserialize::default(),
                    expected_debit_date: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    institution_number: Deserialize::default(),
                    last4: Deserialize::default(),
                    mandate: Deserialize::default(),
                    transit_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "expected_debit_date" => Deserialize::begin(&mut self.builder.expected_debit_date),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "institution_number" => Deserialize::begin(&mut self.builder.institution_number),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "mandate" => Deserialize::begin(&mut self.builder.mandate),
                "transit_number" => Deserialize::begin(&mut self.builder.transit_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_name),
                Some(expected_debit_date),
                Some(fingerprint),
                Some(institution_number),
                Some(last4),
                Some(mandate),
                Some(transit_number),
            ) = (
                self.builder.bank_name.take(),
                self.builder.expected_debit_date.take(),
                self.builder.fingerprint.take(),
                self.builder.institution_number.take(),
                self.builder.last4.take(),
                self.builder.mandate.take(),
                self.builder.transit_number.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsAcssDebit {
                bank_name,
                expected_debit_date,
                fingerprint,
                institution_number,
                last4,
                mandate,
                transit_number,
            });
            Ok(())
        }
    }
};
