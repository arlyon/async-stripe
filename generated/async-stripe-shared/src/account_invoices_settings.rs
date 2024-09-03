#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountInvoicesSettings {
    /// The list of default Account Tax IDs to automatically include on invoices.
    /// Account Tax IDs get added when an invoice is finalized.
    pub default_account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
}
#[doc(hidden)]
pub struct AccountInvoicesSettingsBuilder {
    default_account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
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

    impl Deserialize for AccountInvoicesSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountInvoicesSettings>,
        builder: AccountInvoicesSettingsBuilder,
    }

    impl Visitor for Place<AccountInvoicesSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountInvoicesSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountInvoicesSettingsBuilder {
        type Out = AccountInvoicesSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_account_tax_ids" => Deserialize::begin(&mut self.default_account_tax_ids),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { default_account_tax_ids: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(default_account_tax_ids),) = (self.default_account_tax_ids.take(),) else {
                return None;
            };
            Some(Self::Out { default_account_tax_ids })
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

    impl ObjectDeser for AccountInvoicesSettings {
        type Builder = AccountInvoicesSettingsBuilder;
    }

    impl FromValueOpt for AccountInvoicesSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountInvoicesSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "default_account_tax_ids" => {
                        b.default_account_tax_ids = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
