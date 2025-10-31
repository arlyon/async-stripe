#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsNzBankAccount {
    /// The name on the bank account.
    /// Only present if the account holder name is different from the name of the authorized signatory collected in the PaymentMethod’s billing details.
    pub account_holder_name: Option<String>,
    /// The numeric code for the bank account's bank.
    pub bank_code: String,
    /// The name of the bank.
    pub bank_name: String,
    /// The numeric code for the bank account's bank branch.
    pub branch_code: String,
    /// Last four digits of the bank account number.
    pub last4: String,
    /// The suffix of the bank account number.
    pub suffix: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsNzBankAccountBuilder {
    account_holder_name: Option<Option<String>>,
    bank_code: Option<String>,
    bank_name: Option<String>,
    branch_code: Option<String>,
    last4: Option<String>,
    suffix: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsNzBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsNzBankAccount>,
        builder: PaymentMethodDetailsNzBankAccountBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsNzBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsNzBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsNzBankAccountBuilder {
        type Out = PaymentMethodDetailsNzBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "branch_code" => Deserialize::begin(&mut self.branch_code),
                "last4" => Deserialize::begin(&mut self.last4),
                "suffix" => Deserialize::begin(&mut self.suffix),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_name: Deserialize::default(),
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                branch_code: Deserialize::default(),
                last4: Deserialize::default(),
                suffix: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_name),
                Some(bank_code),
                Some(bank_name),
                Some(branch_code),
                Some(last4),
                Some(suffix),
            ) = (
                self.account_holder_name.take(),
                self.bank_code.take(),
                self.bank_name.take(),
                self.branch_code.take(),
                self.last4.take(),
                self.suffix.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_name,
                bank_code,
                bank_name,
                branch_code,
                last4,
                suffix,
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

    impl ObjectDeser for PaymentMethodDetailsNzBankAccount {
        type Builder = PaymentMethodDetailsNzBankAccountBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsNzBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsNzBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_name" => b.account_holder_name = FromValueOpt::from_value(v),
                    "bank_code" => b.bank_code = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "branch_code" => b.branch_code = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "suffix" => b.suffix = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
