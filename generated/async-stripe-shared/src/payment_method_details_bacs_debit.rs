#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsBacsDebit {
    /// Estimated date to debit the customer's bank account. A date string in YYYY-MM-DD format.
    pub expected_debit_date: Option<String>,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsBacsDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsBacsDebitBuilder {
    expected_debit_date: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<String>>,
    sort_code: Option<Option<String>>,
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
                builder: PaymentMethodDetailsBacsDebitBuilder {
                    expected_debit_date: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                    mandate: Deserialize::default(),
                    sort_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expected_debit_date" => Deserialize::begin(&mut self.builder.expected_debit_date),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "mandate" => Deserialize::begin(&mut self.builder.mandate),
                "sort_code" => Deserialize::begin(&mut self.builder.sort_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(expected_debit_date),
                Some(fingerprint),
                Some(last4),
                Some(mandate),
                Some(sort_code),
            ) = (
                self.builder.expected_debit_date.take(),
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
                self.builder.mandate.take(),
                self.builder.sort_code.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsBacsDebit {
                expected_debit_date,
                fingerprint,
                last4,
                mandate,
                sort_code,
            });
            Ok(())
        }
    }
};
