#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountSepaDebitPaymentsSettings {
    /// SEPA creditor identifier that identifies the company making the payment.
    pub creditor_id: Option<String>,
}
#[doc(hidden)]
pub struct AccountSepaDebitPaymentsSettingsBuilder {
    creditor_id: Option<Option<String>>,
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

    impl Deserialize for AccountSepaDebitPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountSepaDebitPaymentsSettings>,
        builder: AccountSepaDebitPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountSepaDebitPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountSepaDebitPaymentsSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountSepaDebitPaymentsSettingsBuilder {
        type Out = AccountSepaDebitPaymentsSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "creditor_id" => Deserialize::begin(&mut self.creditor_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { creditor_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(creditor_id),) = (self.creditor_id.take(),) else {
                return None;
            };
            Some(Self::Out { creditor_id })
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

    impl ObjectDeser for AccountSepaDebitPaymentsSettings {
        type Builder = AccountSepaDebitPaymentsSettingsBuilder;
    }

    impl FromValueOpt for AccountSepaDebitPaymentsSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountSepaDebitPaymentsSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "creditor_id" => b.creditor_id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
