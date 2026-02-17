/// ABA Records contain U.S. bank account details per the ABA format.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The account number.
    pub account_number: Option<String>,
    /// The last four characters of the account number.
    pub account_number_last4: String,
    /// Name of the bank.
    pub bank_name: String,
    /// Routing number for the account.
    pub routing_number: String,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceAbaRecordBuilder {
    account_holder_name: Option<String>,
    account_number: Option<Option<String>>,
    account_number_last4: Option<String>,
    bank_name: Option<String>,
    routing_number: Option<String>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceAbaRecord {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceAbaRecord>,
        builder: TreasuryFinancialAccountsResourceAbaRecordBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceAbaRecord> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceAbaRecordBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceAbaRecordBuilder {
        type Out = TreasuryFinancialAccountsResourceAbaRecord;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_name" => Deserialize::begin(&mut self.account_holder_name),
                "account_number" => Deserialize::begin(&mut self.account_number),
                "account_number_last4" => Deserialize::begin(&mut self.account_number_last4),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_name: Deserialize::default(),
                account_number: Deserialize::default(),
                account_number_last4: Deserialize::default(),
                bank_name: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_name),
                Some(account_number),
                Some(account_number_last4),
                Some(bank_name),
                Some(routing_number),
            ) = (
                self.account_holder_name.take(),
                self.account_number.take(),
                self.account_number_last4.take(),
                self.bank_name.take(),
                self.routing_number.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_name,
                account_number,
                account_number_last4,
                bank_name,
                routing_number,
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceAbaRecord {
        type Builder = TreasuryFinancialAccountsResourceAbaRecordBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceAbaRecord {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceAbaRecordBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_name" => b.account_holder_name = FromValueOpt::from_value(v),
                    "account_number" => b.account_number = FromValueOpt::from_value(v),
                    "account_number_last4" => b.account_number_last4 = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
