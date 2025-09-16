#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountInvoicesSettings {
    /// The list of default Account Tax IDs to automatically include on invoices.
    /// Account Tax IDs get added when an invoice is finalized.
    pub default_account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
    /// Whether payment methods should be saved when a payment is completed for a one-time invoices on a hosted invoice page.
    pub hosted_payment_method_save: Option<AccountInvoicesSettingsHostedPaymentMethodSave>,
}
#[doc(hidden)]
pub struct AccountInvoicesSettingsBuilder {
    default_account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
    hosted_payment_method_save: Option<Option<AccountInvoicesSettingsHostedPaymentMethodSave>>,
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
                "hosted_payment_method_save" => {
                    Deserialize::begin(&mut self.hosted_payment_method_save)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                default_account_tax_ids: Deserialize::default(),
                hosted_payment_method_save: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(default_account_tax_ids), Some(hosted_payment_method_save)) =
                (self.default_account_tax_ids.take(), self.hosted_payment_method_save)
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
/// Whether payment methods should be saved when a payment is completed for a one-time invoices on a hosted invoice page.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountInvoicesSettingsHostedPaymentMethodSave {
    Always,
    Never,
    Offer,
}
impl AccountInvoicesSettingsHostedPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use AccountInvoicesSettingsHostedPaymentMethodSave::*;
        match self {
            Always => "always",
            Never => "never",
            Offer => "offer",
        }
    }
}

impl std::str::FromStr for AccountInvoicesSettingsHostedPaymentMethodSave {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountInvoicesSettingsHostedPaymentMethodSave::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            "offer" => Ok(Offer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountInvoicesSettingsHostedPaymentMethodSave> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountInvoicesSettingsHostedPaymentMethodSave::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountInvoicesSettingsHostedPaymentMethodSave);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AccountInvoicesSettingsHostedPaymentMethodSave",
            )
        })
    }
}
