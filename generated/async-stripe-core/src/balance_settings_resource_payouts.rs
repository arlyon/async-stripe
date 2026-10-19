#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourcePayouts {
    /// Configures per-currency rules for automatically transferring funds from the payments balance to a FinancialAccount.
    pub automatic_transfer_rules_by_currency: Option<
        std::collections::HashMap<
            String,
            Vec<stripe_core::BalanceSettingsResourceAutomaticTransferRule>,
        >,
    >,
    /// The minimum balance amount to retain per currency after automatic payouts.
    /// Only funds that exceed these amounts are paid out.
    /// Learn more about the [minimum balances for automatic payouts](/payouts/minimum-balances-for-automatic-payouts).
    pub minimum_balance_by_currency: Option<std::collections::HashMap<String, i64>>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    /// See our [Setting Bank and Debit Card Payouts](https://docs.stripe.com/connect/bank-transfers#payout-information) documentation for details.
    pub schedule: Option<stripe_core::BalanceSettingsResourcePayoutSchedule>,
    /// The text that appears on the bank account statement for payouts.
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    pub statement_descriptor: Option<String>,
    /// Whether the funds in this account can be paid out.
    pub status: stripe_shared::BalanceSettingsResourcePayoutsStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourcePayouts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettingsResourcePayouts").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceSettingsResourcePayoutsBuilder {
    automatic_transfer_rules_by_currency: Option<
        Option<
            std::collections::HashMap<
                String,
                Vec<stripe_core::BalanceSettingsResourceAutomaticTransferRule>,
            >,
        >,
    >,
    minimum_balance_by_currency: Option<Option<std::collections::HashMap<String, i64>>>,
    schedule: Option<Option<stripe_core::BalanceSettingsResourcePayoutSchedule>>,
    statement_descriptor: Option<Option<String>>,
    status: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
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

    impl Deserialize for BalanceSettingsResourcePayouts {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceSettingsResourcePayouts>,
        builder: BalanceSettingsResourcePayoutsBuilder,
    }

    impl Visitor for Place<BalanceSettingsResourcePayouts> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceSettingsResourcePayoutsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceSettingsResourcePayoutsBuilder {
        type Out = BalanceSettingsResourcePayouts;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "automatic_transfer_rules_by_currency" => {
                    Deserialize::begin(&mut self.automatic_transfer_rules_by_currency)
                }
                "minimum_balance_by_currency" => {
                    Deserialize::begin(&mut self.minimum_balance_by_currency)
                }
                "schedule" => Deserialize::begin(&mut self.schedule),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                automatic_transfer_rules_by_currency: Some(None),
                minimum_balance_by_currency: Some(None),
                schedule: Some(None),
                statement_descriptor: Some(None),
                status: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(automatic_transfer_rules_by_currency),
                Some(minimum_balance_by_currency),
                Some(schedule),
                Some(statement_descriptor),
                Some(status),
            ) = (
                self.automatic_transfer_rules_by_currency.take(),
                self.minimum_balance_by_currency.take(),
                self.schedule.take(),
                self.statement_descriptor.take(),
                self.status.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                automatic_transfer_rules_by_currency,
                minimum_balance_by_currency,
                schedule,
                statement_descriptor,
                status,
            })
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

    impl ObjectDeser for BalanceSettingsResourcePayouts {
        type Builder = BalanceSettingsResourcePayoutsBuilder;
    }

    impl FromValueOpt for BalanceSettingsResourcePayouts {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceSettingsResourcePayoutsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "automatic_transfer_rules_by_currency" => {
                        b.automatic_transfer_rules_by_currency = FromValueOpt::from_value(v)
                    }
                    "minimum_balance_by_currency" => {
                        b.minimum_balance_by_currency = FromValueOpt::from_value(v)
                    }
                    "schedule" => b.schedule = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether the funds in this account can be paid out.
pub type BalanceSettingsResourcePayoutsStatus = stripe_shared::BalanceSettingsResourcePayoutsStatus;
