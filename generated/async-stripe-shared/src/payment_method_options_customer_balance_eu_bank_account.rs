#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCustomerBalanceEuBankAccount {
    /// The desired country code of the bank account information.
    /// Permitted values include: `DE`, `FR`, `IE`, or `NL`.
    pub country: stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccountCountry,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceEuBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsCustomerBalanceEuBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder {
    country: Option<stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccountCountry>,
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

    impl Deserialize for PaymentMethodOptionsCustomerBalanceEuBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
        builder: PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCustomerBalanceEuBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder {
        type Out = PaymentMethodOptionsCustomerBalanceEuBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { country: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(country),) = (self.country.take(),) else {
                return None;
            };
            Some(Self::Out { country })
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

    impl ObjectDeser for PaymentMethodOptionsCustomerBalanceEuBankAccount {
        type Builder = PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCustomerBalanceEuBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The desired country code of the bank account information.
/// Permitted values include: `DE`, `FR`, `IE`, or `NL`.
pub type PaymentMethodOptionsCustomerBalanceEuBankAccountCountry =
    stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccountCountry;
