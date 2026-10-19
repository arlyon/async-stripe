#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountInvoicesSettings {
    /// The list of default Account Tax IDs to automatically include on invoices.
    /// Account Tax IDs get added when an invoice is finalized.
    pub default_account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
    /// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
    pub hosted_payment_method_save: Option<stripe_shared::HostedPaymentMethodSave>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountInvoicesSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountInvoicesSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountInvoicesSettingsBuilder {
    default_account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
    hosted_payment_method_save: Option<Option<stripe_shared::HostedPaymentMethodSave>>,
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
                "hosted_payment_method_save" => {
                    Deserialize::begin(&mut self.hosted_payment_method_save)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { default_account_tax_ids: Some(None), hosted_payment_method_save: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(default_account_tax_ids), Some(hosted_payment_method_save)) =
                (self.default_account_tax_ids.take(), self.hosted_payment_method_save.take())
            else {
                return None;
            };
            Some(Self::Out { default_account_tax_ids, hosted_payment_method_save })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
                    "hosted_payment_method_save" => {
                        b.hosted_payment_method_save = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
pub type AccountInvoicesSettingsHostedPaymentMethodSave = stripe_shared::HostedPaymentMethodSave;
