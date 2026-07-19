#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// Estimated date to debit the customer's bank account. A date string in YYYY-MM-DD format.
    pub expected_debit_date: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four characters of the IBAN.
    pub last4: Option<String>,
    /// Find the ID of the mandate used for this payment under the [payment_method_details.sepa_debit.mandate](https://docs.stripe.com/api/charges/object#charge_object-payment_method_details-sepa_debit-mandate) property on the Charge.
    /// Use this mandate ID to [retrieve the Mandate](https://docs.stripe.com/api/mandates/retrieve).
    pub mandate: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordSepaDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordSepaDebitBuilder {
    bank_code: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    country: Option<Option<String>>,
    expected_debit_date: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordSepaDebit>,
        builder: PaymentMethodDetailsPaymentRecordSepaDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordSepaDebitBuilder {
                    bank_code: Deserialize::default(),
                    branch_code: Deserialize::default(),
                    country: Deserialize::default(),
                    expected_debit_date: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                    mandate: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.builder.bank_code),
                "branch_code" => Deserialize::begin(&mut self.builder.branch_code),
                "country" => Deserialize::begin(&mut self.builder.country),
                "expected_debit_date" => Deserialize::begin(&mut self.builder.expected_debit_date),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "mandate" => Deserialize::begin(&mut self.builder.mandate),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_code),
                Some(branch_code),
                Some(country),
                Some(expected_debit_date),
                Some(fingerprint),
                Some(last4),
                Some(mandate),
            ) = (
                self.builder.bank_code.take(),
                self.builder.branch_code.take(),
                self.builder.country.take(),
                self.builder.expected_debit_date.take(),
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
                self.builder.mandate.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsPaymentRecordSepaDebit {
                bank_code,
                branch_code,
                country,
                expected_debit_date,
                fingerprint,
                last4,
                mandate,
            });
            Ok(())
        }
    }
};
