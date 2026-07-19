#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodBacsDebit {
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    pub sort_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodBacsDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodBacsDebitBuilder {
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodBacsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodBacsDebit>,
        builder: PaymentMethodBacsDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodBacsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodBacsDebitBuilder {
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                    sort_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "sort_code" => Deserialize::begin(&mut self.builder.sort_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(fingerprint), Some(last4), Some(sort_code)) = (
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
                self.builder.sort_code.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodBacsDebit { fingerprint, last4, sort_code });
            Ok(())
        }
    }
};
