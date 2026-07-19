#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_management: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub account_onboarding: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub balances: stripe_connect::ConnectEmbeddedPayoutsConfig,
    pub disputes_list: stripe_connect::ConnectEmbeddedDisputesListConfig,
    pub documents: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub financial_account: stripe_connect::ConnectEmbeddedFinancialAccountConfigClaim,
    pub financial_account_transactions:
        stripe_connect::ConnectEmbeddedFinancialAccountTransactionsConfigClaim,
    pub instant_payouts_promotion: stripe_connect::ConnectEmbeddedInstantPayoutsPromotionConfig,
    pub issuing_card: stripe_connect::ConnectEmbeddedIssuingCardConfigClaim,
    pub issuing_cards_list: stripe_connect::ConnectEmbeddedIssuingCardsListConfigClaim,
    pub notification_banner: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub payment_details: stripe_connect::ConnectEmbeddedPaymentsConfigClaim,
    pub payment_disputes: stripe_connect::ConnectEmbeddedPaymentDisputesConfig,
    pub payments: stripe_connect::ConnectEmbeddedPaymentsConfigClaim,
    pub payout_details: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub payouts: stripe_connect::ConnectEmbeddedPayoutsConfig,
    pub payouts_list: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub tax_registrations: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub tax_settings: stripe_connect::ConnectEmbeddedBaseConfigClaim,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedAccountSessionCreateComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedAccountSessionCreateComponents").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedAccountSessionCreateComponentsBuilder {
    account_management: Option<stripe_connect::ConnectEmbeddedAccountConfigClaim>,
    account_onboarding: Option<stripe_connect::ConnectEmbeddedAccountConfigClaim>,
    balances: Option<stripe_connect::ConnectEmbeddedPayoutsConfig>,
    disputes_list: Option<stripe_connect::ConnectEmbeddedDisputesListConfig>,
    documents: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    financial_account: Option<stripe_connect::ConnectEmbeddedFinancialAccountConfigClaim>,
    financial_account_transactions:
        Option<stripe_connect::ConnectEmbeddedFinancialAccountTransactionsConfigClaim>,
    instant_payouts_promotion: Option<stripe_connect::ConnectEmbeddedInstantPayoutsPromotionConfig>,
    issuing_card: Option<stripe_connect::ConnectEmbeddedIssuingCardConfigClaim>,
    issuing_cards_list: Option<stripe_connect::ConnectEmbeddedIssuingCardsListConfigClaim>,
    notification_banner: Option<stripe_connect::ConnectEmbeddedAccountConfigClaim>,
    payment_details: Option<stripe_connect::ConnectEmbeddedPaymentsConfigClaim>,
    payment_disputes: Option<stripe_connect::ConnectEmbeddedPaymentDisputesConfig>,
    payments: Option<stripe_connect::ConnectEmbeddedPaymentsConfigClaim>,
    payout_details: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    payouts: Option<stripe_connect::ConnectEmbeddedPayoutsConfig>,
    payouts_list: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    tax_registrations: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    tax_settings: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
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

    impl Deserialize for ConnectEmbeddedAccountSessionCreateComponents {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedAccountSessionCreateComponents>,
        builder: ConnectEmbeddedAccountSessionCreateComponentsBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedAccountSessionCreateComponents> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedAccountSessionCreateComponentsBuilder {
                    account_management: Deserialize::default(),
                    account_onboarding: Deserialize::default(),
                    balances: Deserialize::default(),
                    disputes_list: Deserialize::default(),
                    documents: Deserialize::default(),
                    financial_account: Deserialize::default(),
                    financial_account_transactions: Deserialize::default(),
                    instant_payouts_promotion: Deserialize::default(),
                    issuing_card: Deserialize::default(),
                    issuing_cards_list: Deserialize::default(),
                    notification_banner: Deserialize::default(),
                    payment_details: Deserialize::default(),
                    payment_disputes: Deserialize::default(),
                    payments: Deserialize::default(),
                    payout_details: Deserialize::default(),
                    payouts: Deserialize::default(),
                    payouts_list: Deserialize::default(),
                    tax_registrations: Deserialize::default(),
                    tax_settings: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_management" => Deserialize::begin(&mut self.builder.account_management),
                "account_onboarding" => Deserialize::begin(&mut self.builder.account_onboarding),
                "balances" => Deserialize::begin(&mut self.builder.balances),
                "disputes_list" => Deserialize::begin(&mut self.builder.disputes_list),
                "documents" => Deserialize::begin(&mut self.builder.documents),
                "financial_account" => Deserialize::begin(&mut self.builder.financial_account),
                "financial_account_transactions" => {
                    Deserialize::begin(&mut self.builder.financial_account_transactions)
                }
                "instant_payouts_promotion" => {
                    Deserialize::begin(&mut self.builder.instant_payouts_promotion)
                }
                "issuing_card" => Deserialize::begin(&mut self.builder.issuing_card),
                "issuing_cards_list" => Deserialize::begin(&mut self.builder.issuing_cards_list),
                "notification_banner" => Deserialize::begin(&mut self.builder.notification_banner),
                "payment_details" => Deserialize::begin(&mut self.builder.payment_details),
                "payment_disputes" => Deserialize::begin(&mut self.builder.payment_disputes),
                "payments" => Deserialize::begin(&mut self.builder.payments),
                "payout_details" => Deserialize::begin(&mut self.builder.payout_details),
                "payouts" => Deserialize::begin(&mut self.builder.payouts),
                "payouts_list" => Deserialize::begin(&mut self.builder.payouts_list),
                "tax_registrations" => Deserialize::begin(&mut self.builder.tax_registrations),
                "tax_settings" => Deserialize::begin(&mut self.builder.tax_settings),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_management),
                Some(account_onboarding),
                Some(balances),
                Some(disputes_list),
                Some(documents),
                Some(financial_account),
                Some(financial_account_transactions),
                Some(instant_payouts_promotion),
                Some(issuing_card),
                Some(issuing_cards_list),
                Some(notification_banner),
                Some(payment_details),
                Some(payment_disputes),
                Some(payments),
                Some(payout_details),
                Some(payouts),
                Some(payouts_list),
                Some(tax_registrations),
                Some(tax_settings),
            ) = (
                self.builder.account_management,
                self.builder.account_onboarding,
                self.builder.balances,
                self.builder.disputes_list,
                self.builder.documents,
                self.builder.financial_account,
                self.builder.financial_account_transactions,
                self.builder.instant_payouts_promotion,
                self.builder.issuing_card,
                self.builder.issuing_cards_list,
                self.builder.notification_banner,
                self.builder.payment_details,
                self.builder.payment_disputes,
                self.builder.payments,
                self.builder.payout_details,
                self.builder.payouts,
                self.builder.payouts_list,
                self.builder.tax_registrations,
                self.builder.tax_settings,
            )
            else {
                return Ok(());
            };
            *self.out = Some(ConnectEmbeddedAccountSessionCreateComponents {
                account_management,
                account_onboarding,
                balances,
                disputes_list,
                documents,
                financial_account,
                financial_account_transactions,
                instant_payouts_promotion,
                issuing_card,
                issuing_cards_list,
                notification_banner,
                payment_details,
                payment_disputes,
                payments,
                payout_details,
                payouts,
                payouts_list,
                tax_registrations,
                tax_settings,
            });
            Ok(())
        }
    }
};
