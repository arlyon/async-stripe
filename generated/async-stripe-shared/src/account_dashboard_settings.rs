#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct AccountDashboardSettingsBuilder {
    display_name: Option<Option<String>>,
    timezone: Option<Option<String>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: AccountDashboardSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountDashboardSettingsBuilder {
        type Out = AccountDashboardSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.display_name),
                "timezone" => Deserialize::begin(&mut self.timezone),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { display_name: Deserialize::default(), timezone: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(display_name), Some(timezone)) =
                (self.display_name.take(), self.timezone.take())
            else {
                return None;
            };
            Some(Self::Out { display_name, timezone })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for AccountDashboardSettings {
        type Builder = AccountDashboardSettingsBuilder;
    }

    impl FromValueOpt for AccountDashboardSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountDashboardSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "timezone" => b.timezone = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
