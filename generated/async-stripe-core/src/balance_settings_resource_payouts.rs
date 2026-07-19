#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourcePayouts {
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
    pub status: BalanceSettingsResourcePayoutsStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourcePayouts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettingsResourcePayouts").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BalanceSettingsResourcePayoutsBuilder {
                    minimum_balance_by_currency: Deserialize::default(),
                    schedule: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "minimum_balance_by_currency" => {
                    Deserialize::begin(&mut self.builder.minimum_balance_by_currency)
                }
                "schedule" => Deserialize::begin(&mut self.builder.schedule),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(minimum_balance_by_currency),
                Some(schedule),
                Some(statement_descriptor),
                Some(status),
            ) = (
                self.builder.minimum_balance_by_currency.take(),
                self.builder.schedule.take(),
                self.builder.statement_descriptor.take(),
                self.builder.status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(BalanceSettingsResourcePayouts {
                minimum_balance_by_currency,
                schedule,
                statement_descriptor,
                status,
            });
            Ok(())
        }
    }
};
/// Whether the funds in this account can be paid out.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BalanceSettingsResourcePayoutsStatus {
    Disabled,
    Enabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BalanceSettingsResourcePayoutsStatus {
    pub fn as_str(&self) -> &str {
        use BalanceSettingsResourcePayoutsStatus::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BalanceSettingsResourcePayoutsStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceSettingsResourcePayoutsStatus::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BalanceSettingsResourcePayoutsStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BalanceSettingsResourcePayoutsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BalanceSettingsResourcePayoutsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourcePayoutsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BalanceSettingsResourcePayoutsStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BalanceSettingsResourcePayoutsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BalanceSettingsResourcePayoutsStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BalanceSettingsResourcePayoutsStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BalanceSettingsResourcePayoutsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
