#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountSettings {
    pub bacs_debit_payments: Option<stripe_shared::AccountBacsDebitPaymentsSettings>,
    pub branding: stripe_shared::AccountBrandingSettings,
    pub card_issuing: Option<stripe_shared::AccountCardIssuingSettings>,
    pub card_payments: stripe_shared::AccountCardPaymentsSettings,
    pub dashboard: stripe_shared::AccountDashboardSettings,
    pub invoices: Option<stripe_shared::AccountInvoicesSettings>,
    pub payments: stripe_shared::AccountPaymentsSettings,
    pub payouts: Option<stripe_shared::AccountPayoutSettings>,
    pub sepa_debit_payments: Option<stripe_shared::AccountSepaDebitPaymentsSettings>,
    pub treasury: Option<stripe_shared::AccountTreasurySettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountSettingsBuilder {
    bacs_debit_payments: Option<stripe_shared::AccountBacsDebitPaymentsSettings>,
    branding: Option<stripe_shared::AccountBrandingSettings>,
    card_issuing: Option<stripe_shared::AccountCardIssuingSettings>,
    card_payments: Option<stripe_shared::AccountCardPaymentsSettings>,
    dashboard: Option<stripe_shared::AccountDashboardSettings>,
    invoices: Option<stripe_shared::AccountInvoicesSettings>,
    payments: Option<stripe_shared::AccountPaymentsSettings>,
    payouts: Option<stripe_shared::AccountPayoutSettings>,
    sepa_debit_payments: Option<stripe_shared::AccountSepaDebitPaymentsSettings>,
    treasury: Option<stripe_shared::AccountTreasurySettings>,
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

    impl Deserialize for AccountSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountSettings>,
        builder: AccountSettingsBuilder,
    }

    impl Visitor for Place<AccountSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountSettingsBuilder {
        type Out = AccountSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bacs_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bacs_debit_payments)
                }
                "branding" => Deserialize::begin(&mut self.branding),
                "card_issuing" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_issuing)
                }
                "card_payments" => Deserialize::begin(&mut self.card_payments),
                "dashboard" => Deserialize::begin(&mut self.dashboard),
                "invoices" => stripe_types::miniserde_helpers::begin_nullable(&mut self.invoices),
                "payments" => Deserialize::begin(&mut self.payments),
                "payouts" => stripe_types::miniserde_helpers::begin_nullable(&mut self.payouts),
                "sepa_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit_payments)
                }
                "treasury" => stripe_types::miniserde_helpers::begin_nullable(&mut self.treasury),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bacs_debit_payments: None,
                branding: None,
                card_issuing: None,
                card_payments: None,
                dashboard: None,
                invoices: None,
                payments: None,
                payouts: None,
                sepa_debit_payments: None,
                treasury: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    bacs_debit_payments,
                    branding: Some(branding),
                    card_issuing,
                    card_payments: Some(card_payments),
                    dashboard: Some(dashboard),
                    invoices,
                    payments: Some(payments),
                    payouts,
                    sepa_debit_payments,
                    treasury,
                } => Some(Self::Out {
                    bacs_debit_payments,
                    branding,
                    card_issuing,
                    card_payments,
                    dashboard,
                    invoices,
                    payments,
                    payouts,
                    sepa_debit_payments,
                    treasury,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for AccountSettings {
        type Builder = AccountSettingsBuilder;
    }

    impl FromValueOpt for AccountSettings {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
