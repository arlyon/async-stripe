#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsCardChecksBuilder {
    address_line1_check: Option<Option<String>>,
    address_postal_code_check: Option<Option<String>>,
    cvc_check: Option<Option<String>>,
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
                builder: SetupAttemptPaymentMethodDetailsCardChecksBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupAttemptPaymentMethodDetailsCardChecksBuilder {
        type Out = SetupAttemptPaymentMethodDetailsCardChecks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_postal_code_check" => {
                    Deserialize::begin(&mut self.address_postal_code_check)
                }
                "cvc_check" => Deserialize::begin(&mut self.cvc_check),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address_line1_check: Deserialize::default(),
                address_postal_code_check: Deserialize::default(),
                cvc_check: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address_line1_check), Some(address_postal_code_check), Some(cvc_check)) = (
                self.address_line1_check.take(),
                self.address_postal_code_check.take(),
                self.cvc_check.take(),
            ) else {
                return None;
            };
            Some(Self::Out { address_line1_check, address_postal_code_check, cvc_check })
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

    impl ObjectDeser for SetupAttemptPaymentMethodDetailsCardChecks {
        type Builder = SetupAttemptPaymentMethodDetailsCardChecksBuilder;
    }

    impl FromValueOpt for SetupAttemptPaymentMethodDetailsCardChecks {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupAttemptPaymentMethodDetailsCardChecksBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address_line1_check" => b.address_line1_check = FromValueOpt::from_value(v),
                    "address_postal_code_check" => {
                        b.address_postal_code_check = FromValueOpt::from_value(v)
                    }
                    "cvc_check" => b.cvc_check = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
