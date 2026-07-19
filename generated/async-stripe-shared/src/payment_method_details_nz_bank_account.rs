#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// Estimated date to debit the customer's bank account. A date string in YYYY-MM-DD format.
    pub expected_debit_date: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: String,
    /// The suffix of the bank account number.
    pub suffix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsNzBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsNzBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsNzBankAccountBuilder {
    account_holder_name: Option<Option<String>>,
    bank_code: Option<String>,
    bank_name: Option<String>,
    branch_code: Option<String>,
    expected_debit_date: Option<Option<String>>,
    last4: Option<String>,
    suffix: Option<Option<String>>,
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
                builder: PaymentMethodDetailsNzBankAccountBuilder {
                    account_holder_name: Deserialize::default(),
                    bank_code: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    branch_code: Deserialize::default(),
                    expected_debit_date: Deserialize::default(),
                    last4: Deserialize::default(),
                    suffix: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_name" => Deserialize::begin(&mut self.builder.account_holder_name),
                "bank_code" => Deserialize::begin(&mut self.builder.bank_code),
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "branch_code" => Deserialize::begin(&mut self.builder.branch_code),
                "expected_debit_date" => Deserialize::begin(&mut self.builder.expected_debit_date),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "suffix" => Deserialize::begin(&mut self.builder.suffix),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_holder_name),
                Some(bank_code),
                Some(bank_name),
                Some(branch_code),
                Some(expected_debit_date),
                Some(last4),
                Some(suffix),
            ) = (
                self.builder.account_holder_name.take(),
                self.builder.bank_code.take(),
                self.builder.bank_name.take(),
                self.builder.branch_code.take(),
                self.builder.expected_debit_date.take(),
                self.builder.last4.take(),
                self.builder.suffix.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsNzBankAccount {
                account_holder_name,
                bank_code,
                bank_name,
                branch_code,
                expected_debit_date,
                last4,
                suffix,
            });
            Ok(())
        }
    }
};
