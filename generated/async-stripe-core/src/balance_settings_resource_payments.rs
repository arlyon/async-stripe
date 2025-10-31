#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct BalanceSettingsResourcePaymentsBuilder {
    debit_negative_balances: Option<Option<bool>>,
    payouts: Option<Option<stripe_core::BalanceSettingsResourcePayouts>>,
    settlement_timing: Option<stripe_core::BalanceSettingsResourceSettlementTiming>,
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
                builder: BalanceSettingsResourcePaymentsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceSettingsResourcePaymentsBuilder {
        type Out = BalanceSettingsResourcePayments;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "debit_negative_balances" => Deserialize::begin(&mut self.debit_negative_balances),
                "payouts" => Deserialize::begin(&mut self.payouts),
                "settlement_timing" => Deserialize::begin(&mut self.settlement_timing),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                debit_negative_balances: Deserialize::default(),
                payouts: Deserialize::default(),
                settlement_timing: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(debit_negative_balances), Some(payouts), Some(settlement_timing)) =
                (self.debit_negative_balances, self.payouts.take(), self.settlement_timing)
            else {
                return None;
            };
            Some(Self::Out { debit_negative_balances, payouts, settlement_timing })
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

    impl ObjectDeser for BalanceSettingsResourcePayments {
        type Builder = BalanceSettingsResourcePaymentsBuilder;
    }

    impl FromValueOpt for BalanceSettingsResourcePayments {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceSettingsResourcePaymentsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "debit_negative_balances" => {
                        b.debit_negative_balances = FromValueOpt::from_value(v)
                    }
                    "payouts" => b.payouts = FromValueOpt::from_value(v),
                    "settlement_timing" => b.settlement_timing = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
