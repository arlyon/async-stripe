#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Information about the object that generated this PaymentMethod.
    pub generated_from: Option<stripe_shared::SepaDebitGeneratedFrom>,
    /// Last four characters of the IBAN.
    pub last4: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodSepaDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodSepaDebitBuilder {
    bank_code: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    generated_from: Option<Option<stripe_shared::SepaDebitGeneratedFrom>>,
    last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodSepaDebit>,
        builder: PaymentMethodSepaDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodSepaDebitBuilder {
                    bank_code: Deserialize::default(),
                    branch_code: Deserialize::default(),
                    country: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    generated_from: Deserialize::default(),
                    last4: Deserialize::default(),
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
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "generated_from" => Deserialize::begin(&mut self.builder.generated_from),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_code),
                Some(branch_code),
                Some(country),
                Some(fingerprint),
                Some(generated_from),
                Some(last4),
            ) = (
                self.builder.bank_code.take(),
                self.builder.branch_code.take(),
                self.builder.country.take(),
                self.builder.fingerprint.take(),
                self.builder.generated_from.take(),
                self.builder.last4.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodSepaDebit {
                bank_code,
                branch_code,
                country,
                fingerprint,
                generated_from,
                last4,
            });
            Ok(())
        }
    }
};
