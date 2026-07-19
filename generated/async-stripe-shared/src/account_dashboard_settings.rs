#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountDashboardSettings {
    /// The display name for this account.
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    pub display_name: Option<String>,
    /// The timezone used in the Stripe Dashboard for this account.
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    pub timezone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountDashboardSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountDashboardSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountDashboardSettingsBuilder {
    display_name: Option<Option<String>>,
    timezone: Option<Option<String>>,
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

    impl Deserialize for AccountDashboardSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountDashboardSettings>,
        builder: AccountDashboardSettingsBuilder,
    }

    impl Visitor for Place<AccountDashboardSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountDashboardSettingsBuilder {
                    display_name: Deserialize::default(),
                    timezone: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "timezone" => Deserialize::begin(&mut self.builder.timezone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(display_name), Some(timezone)) =
                (self.builder.display_name.take(), self.builder.timezone.take())
            else {
                return Ok(());
            };
            *self.out = Some(AccountDashboardSettings { display_name, timezone });
            Ok(())
        }
    }
};
