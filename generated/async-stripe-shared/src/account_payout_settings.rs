#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountPayoutSettings {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    /// See [Understanding Connect account balances](/connect/account-balances) for details.
    /// The default value is `false` when [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts, otherwise `true`.
    pub debit_negative_balances: bool,
    pub schedule: stripe_shared::TransferSchedule,
    /// The text that appears on the bank account statement for payouts.
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPayoutSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountPayoutSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountPayoutSettingsBuilder {
    debit_negative_balances: Option<bool>,
    schedule: Option<stripe_shared::TransferSchedule>,
    statement_descriptor: Option<Option<String>>,
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

    impl Deserialize for AccountPayoutSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPayoutSettings>,
        builder: AccountPayoutSettingsBuilder,
    }

    impl Visitor for Place<AccountPayoutSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountPayoutSettingsBuilder {
                    debit_negative_balances: Deserialize::default(),
                    schedule: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "debit_negative_balances" => {
                    Deserialize::begin(&mut self.builder.debit_negative_balances)
                }
                "schedule" => Deserialize::begin(&mut self.builder.schedule),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(debit_negative_balances), Some(schedule), Some(statement_descriptor)) = (
                self.builder.debit_negative_balances,
                self.builder.schedule.take(),
                self.builder.statement_descriptor.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(AccountPayoutSettings {
                debit_negative_balances,
                schedule,
                statement_descriptor,
            });
            Ok(())
        }
    }
};
