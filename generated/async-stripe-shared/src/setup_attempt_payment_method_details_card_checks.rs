#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetailsCardChecks {
    /// If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    /// If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check: Option<String>,
    /// If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsCardChecks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupAttemptPaymentMethodDetailsCardChecks").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsCardChecksBuilder {
    address_line1_check: Option<Option<String>>,
    address_postal_code_check: Option<Option<String>>,
    cvc_check: Option<Option<String>>,
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

    impl Deserialize for SetupAttemptPaymentMethodDetailsCardChecks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetailsCardChecks>,
        builder: SetupAttemptPaymentMethodDetailsCardChecksBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetailsCardChecks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupAttemptPaymentMethodDetailsCardChecksBuilder {
                    address_line1_check: Deserialize::default(),
                    address_postal_code_check: Deserialize::default(),
                    cvc_check: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.builder.address_line1_check),
                "address_postal_code_check" => {
                    Deserialize::begin(&mut self.builder.address_postal_code_check)
                }
                "cvc_check" => Deserialize::begin(&mut self.builder.cvc_check),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(address_line1_check), Some(address_postal_code_check), Some(cvc_check)) = (
                self.builder.address_line1_check.take(),
                self.builder.address_postal_code_check.take(),
                self.builder.cvc_check.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SetupAttemptPaymentMethodDetailsCardChecks {
                address_line1_check,
                address_postal_code_check,
                cvc_check,
            });
            Ok(())
        }
    }
};
