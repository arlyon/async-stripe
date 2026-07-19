#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourcePayments {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    /// See [Understanding Connect account balances](/connect/account-balances) for details.
    /// The default value is `false` when [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts, otherwise `true`.
    pub debit_negative_balances: Option<bool>,
    /// Settings specific to the account's payouts.
    pub payouts: Option<stripe_core::BalanceSettingsResourcePayouts>,
    pub settlement_timing: stripe_core::BalanceSettingsResourceSettlementTiming,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourcePayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettingsResourcePayments").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceSettingsResourcePaymentsBuilder {
    debit_negative_balances: Option<Option<bool>>,
    payouts: Option<Option<stripe_core::BalanceSettingsResourcePayouts>>,
    settlement_timing: Option<stripe_core::BalanceSettingsResourceSettlementTiming>,
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

    impl Deserialize for BalanceSettingsResourcePayments {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceSettingsResourcePayments>,
        builder: BalanceSettingsResourcePaymentsBuilder,
    }

    impl Visitor for Place<BalanceSettingsResourcePayments> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceSettingsResourcePaymentsBuilder {
                    debit_negative_balances: Deserialize::default(),
                    payouts: Deserialize::default(),
                    settlement_timing: Deserialize::default(),
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
                "payouts" => Deserialize::begin(&mut self.builder.payouts),
                "settlement_timing" => Deserialize::begin(&mut self.builder.settlement_timing),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(debit_negative_balances), Some(payouts), Some(settlement_timing)) = (
                self.builder.debit_negative_balances,
                self.builder.payouts.take(),
                self.builder.settlement_timing,
            ) else {
                return Ok(());
            };
            *self.out = Some(BalanceSettingsResourcePayments {
                debit_negative_balances,
                payouts,
                settlement_timing,
            });
            Ok(())
        }
    }
};
