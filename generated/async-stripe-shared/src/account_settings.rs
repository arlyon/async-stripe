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
    bacs_debit_payments: Option<Option<stripe_shared::AccountBacsDebitPaymentsSettings>>,
    branding: Option<stripe_shared::AccountBrandingSettings>,
    card_issuing: Option<Option<stripe_shared::AccountCardIssuingSettings>>,
    card_payments: Option<stripe_shared::AccountCardPaymentsSettings>,
    dashboard: Option<stripe_shared::AccountDashboardSettings>,
    invoices: Option<Option<stripe_shared::AccountInvoicesSettings>>,
    payments: Option<stripe_shared::AccountPaymentsSettings>,
    payouts: Option<Option<stripe_shared::AccountPayoutSettings>>,
    sepa_debit_payments: Option<Option<stripe_shared::AccountSepaDebitPaymentsSettings>>,
    treasury: Option<Option<stripe_shared::AccountTreasurySettings>>,
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
                builder: AccountSettingsBuilder {
                    bacs_debit_payments: Deserialize::default(),
                    branding: Deserialize::default(),
                    card_issuing: Deserialize::default(),
                    card_payments: Deserialize::default(),
                    dashboard: Deserialize::default(),
                    invoices: Deserialize::default(),
                    payments: Deserialize::default(),
                    payouts: Deserialize::default(),
                    sepa_debit_payments: Deserialize::default(),
                    treasury: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bacs_debit_payments" => Deserialize::begin(&mut self.builder.bacs_debit_payments),
                "branding" => Deserialize::begin(&mut self.builder.branding),
                "card_issuing" => Deserialize::begin(&mut self.builder.card_issuing),
                "card_payments" => Deserialize::begin(&mut self.builder.card_payments),
                "dashboard" => Deserialize::begin(&mut self.builder.dashboard),
                "invoices" => Deserialize::begin(&mut self.builder.invoices),
                "payments" => Deserialize::begin(&mut self.builder.payments),
                "payouts" => Deserialize::begin(&mut self.builder.payouts),
                "sepa_debit_payments" => Deserialize::begin(&mut self.builder.sepa_debit_payments),
                "treasury" => Deserialize::begin(&mut self.builder.treasury),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bacs_debit_payments),
                Some(branding),
                Some(card_issuing),
                Some(card_payments),
                Some(dashboard),
                Some(invoices),
                Some(payments),
                Some(payouts),
                Some(sepa_debit_payments),
                Some(treasury),
            ) = (
                self.builder.bacs_debit_payments.take(),
                self.builder.branding.take(),
                self.builder.card_issuing.take(),
                self.builder.card_payments.take(),
                self.builder.dashboard.take(),
                self.builder.invoices.take(),
                self.builder.payments.take(),
                self.builder.payouts.take(),
                self.builder.sepa_debit_payments.take(),
                self.builder.treasury.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(AccountSettings {
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
            });
            Ok(())
        }
    }
};
