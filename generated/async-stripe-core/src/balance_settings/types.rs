/// Options for customizing account balances and payout settings for a Stripe platform’s connected accounts.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettings {
    pub payments: stripe_core::BalanceSettingsResourcePayments,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceSettingsBuilder {
    payments: Option<stripe_core::BalanceSettingsResourcePayments>,
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

    impl Deserialize for BalanceSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceSettings>,
        builder: BalanceSettingsBuilder,
    }

    impl Visitor for Place<BalanceSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceSettingsBuilder { payments: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payments" => Deserialize::begin(&mut self.builder.payments),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payments),) = (self.builder.payments.take(),) else {
                return Ok(());
            };
            *self.out = Some(BalanceSettings { payments });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BalanceSettings {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BalanceSettings", 2)?;
        s.serialize_field("payments", &self.payments)?;

        s.serialize_field("object", "balance_settings")?;
        s.end()
    }
}
