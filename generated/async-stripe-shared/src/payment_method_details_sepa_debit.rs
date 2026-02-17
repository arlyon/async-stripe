#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsSepaDebit {
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
#[doc(hidden)]
pub struct PaymentMethodDetailsSepaDebitBuilder {
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

    impl Deserialize for PaymentMethodDetailsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsSepaDebit>,
        builder: PaymentMethodDetailsSepaDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsSepaDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsSepaDebitBuilder {
        type Out = PaymentMethodDetailsSepaDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "branch_code" => Deserialize::begin(&mut self.branch_code),
                "country" => Deserialize::begin(&mut self.country),
                "expected_debit_date" => Deserialize::begin(&mut self.expected_debit_date),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                branch_code: Deserialize::default(),
                country: Deserialize::default(),
                expected_debit_date: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                mandate: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(bank_code),
                Some(branch_code),
                Some(country),
                Some(expected_debit_date),
                Some(fingerprint),
                Some(last4),
                Some(mandate),
            ) = (
                self.bank_code.take(),
                self.branch_code.take(),
                self.country.take(),
                self.expected_debit_date.take(),
                self.fingerprint.take(),
                self.last4.take(),
                self.mandate.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                bank_code,
                branch_code,
                country,
                expected_debit_date,
                fingerprint,
                last4,
                mandate,
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

    impl ObjectDeser for PaymentMethodDetailsSepaDebit {
        type Builder = PaymentMethodDetailsSepaDebitBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsSepaDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsSepaDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_code" => b.bank_code = FromValueOpt::from_value(v),
                    "branch_code" => b.branch_code = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "expected_debit_date" => b.expected_debit_date = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
