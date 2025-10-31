/// Options for customizing account balances and payout settings for a Stripe platform’s connected accounts.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettings {
    pub payments: stripe_core::BalanceSettingsResourcePayments,
}
#[doc(hidden)]
pub struct BalanceSettingsBuilder {
    payments: Option<stripe_core::BalanceSettingsResourcePayments>,
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
                builder: BalanceSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceSettingsBuilder {
        type Out = BalanceSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payments" => Deserialize::begin(&mut self.payments),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payments: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payments),) = (self.payments.take(),) else {
                return None;
            };
            Some(Self::Out { payments })
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

    impl ObjectDeser for BalanceSettings {
        type Builder = BalanceSettingsBuilder;
    }

    impl FromValueOpt for BalanceSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payments" => b.payments = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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
