#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountInvoicesSettings {
    /// The list of default Account Tax IDs to automatically include on invoices.
    /// Account Tax IDs get added when an invoice is finalized.
    pub default_account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
    /// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
    pub hosted_payment_method_save: Option<AccountInvoicesSettingsHostedPaymentMethodSave>,
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
    hosted_payment_method_save: Option<Option<AccountInvoicesSettingsHostedPaymentMethodSave>>,
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
                builder: AccountInvoicesSettingsBuilder {
                    default_account_tax_ids: Deserialize::default(),
                    hosted_payment_method_save: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_account_tax_ids" => {
                    Deserialize::begin(&mut self.builder.default_account_tax_ids)
                }
                "hosted_payment_method_save" => {
                    Deserialize::begin(&mut self.builder.hosted_payment_method_save)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(default_account_tax_ids), Some(hosted_payment_method_save)) = (
                self.builder.default_account_tax_ids.take(),
                self.builder.hosted_payment_method_save.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(AccountInvoicesSettings {
                default_account_tax_ids,
                hosted_payment_method_save,
            });
            Ok(())
        }
    }
};
/// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountInvoicesSettingsHostedPaymentMethodSave {
    Always,
    Never,
    Offer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountInvoicesSettingsHostedPaymentMethodSave {
    pub fn as_str(&self) -> &str {
        use AccountInvoicesSettingsHostedPaymentMethodSave::*;
        match self {
            Always => "always",
            Never => "never",
            Offer => "offer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountInvoicesSettingsHostedPaymentMethodSave {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountInvoicesSettingsHostedPaymentMethodSave::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            "offer" => Ok(Offer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountInvoicesSettingsHostedPaymentMethodSave"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountInvoicesSettingsHostedPaymentMethodSave))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<AccountInvoicesSettingsHostedPaymentMethodSave> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(AccountInvoicesSettingsHostedPaymentMethodSave::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountInvoicesSettingsHostedPaymentMethodSave {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
