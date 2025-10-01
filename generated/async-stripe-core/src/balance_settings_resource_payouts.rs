#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourcePayouts {
    /// The minimum balance amount to retain per currency after automatic payouts.
    /// Only funds that exceed these amounts are paid out.
    /// Learn more about the [minimum balances for automatic payouts](/payouts/minimum-balances-for-automatic-payouts).
    pub minimum_balance_by_currency: Option<std::collections::HashMap<String, i64>>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    /// See our [Setting Bank and Debit Card Payouts](https://stripe.com/docs/connect/bank-transfers#payout-information) documentation for details.
    pub schedule: Option<stripe_core::BalanceSettingsResourcePayoutSchedule>,
    /// The text that appears on the bank account statement for payouts.
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    pub statement_descriptor: Option<String>,
    /// Whether the funds in this account can be paid out.
    pub status: BalanceSettingsResourcePayoutsStatus,
}
#[doc(hidden)]
pub struct BalanceSettingsResourcePayoutsBuilder {
    minimum_balance_by_currency: Option<Option<std::collections::HashMap<String, i64>>>,
    schedule: Option<Option<stripe_core::BalanceSettingsResourcePayoutSchedule>>,
    statement_descriptor: Option<Option<String>>,
    status: Option<BalanceSettingsResourcePayoutsStatus>,
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
                minimum_balance_by_currency: Deserialize::default(),
                schedule: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(minimum_balance_by_currency),
                Some(schedule),
                Some(statement_descriptor),
                Some(status),
            ) = (
                self.minimum_balance_by_currency.take(),
                self.schedule.take(),
                self.statement_descriptor.take(),
                self.status,
            )
            else {
                return None;
            };
            Some(Self::Out { minimum_balance_by_currency, schedule, statement_descriptor, status })
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BalanceSettingsResourcePayoutsStatus {
    Disabled,
    Enabled,
}
impl BalanceSettingsResourcePayoutsStatus {
    pub fn as_str(self) -> &'static str {
        use BalanceSettingsResourcePayoutsStatus::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for BalanceSettingsResourcePayoutsStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceSettingsResourcePayoutsStatus::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BalanceSettingsResourcePayoutsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BalanceSettingsResourcePayoutsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BalanceSettingsResourcePayoutsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BalanceSettingsResourcePayoutsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BalanceSettingsResourcePayoutsStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BalanceSettingsResourcePayoutsStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BalanceSettingsResourcePayoutsStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BalanceSettingsResourcePayoutsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BalanceSettingsResourcePayoutsStatus")
        })
    }
}
