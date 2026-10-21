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
                builder: AccountPayoutSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountPayoutSettingsBuilder {
        type Out = AccountPayoutSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "debit_negative_balances" => Deserialize::begin(&mut self.debit_negative_balances),
                "schedule" => Deserialize::begin(&mut self.schedule),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { debit_negative_balances: None, schedule: None, statement_descriptor: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    debit_negative_balances: Some(debit_negative_balances),
                    schedule: Some(schedule),
                    statement_descriptor: Some(statement_descriptor),
                } => Some(Self::Out { debit_negative_balances, schedule, statement_descriptor }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for AccountPayoutSettings {
        type Builder = AccountPayoutSettingsBuilder;
    }

    impl FromValueOpt for AccountPayoutSettings {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
