#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_management: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub account_onboarding: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub balance_report: stripe_connect::ConnectEmbeddedBaseConfigClaim,
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
    pub payment_method_settings: stripe_connect::ConnectEmbeddedPaymentMethodSettingsConfigClaim,
    pub payments: stripe_connect::ConnectEmbeddedPaymentsConfigClaim,
    pub payout_details: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub payout_reconciliation_report: stripe_connect::ConnectEmbeddedBaseConfigClaim,
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
    balance_report: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
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
    payment_method_settings:
        Option<stripe_connect::ConnectEmbeddedPaymentMethodSettingsConfigClaim>,
    payments: Option<stripe_connect::ConnectEmbeddedPaymentsConfigClaim>,
    payout_details: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    payout_reconciliation_report: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    payouts: Option<stripe_connect::ConnectEmbeddedPayoutsConfig>,
    payouts_list: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    tax_registrations: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    tax_settings: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
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
                builder: ConnectEmbeddedAccountSessionCreateComponentsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedAccountSessionCreateComponentsBuilder {
        type Out = ConnectEmbeddedAccountSessionCreateComponents;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_management" => Deserialize::begin(&mut self.account_management),
                "account_onboarding" => Deserialize::begin(&mut self.account_onboarding),
                "balance_report" => Deserialize::begin(&mut self.balance_report),
                "balances" => Deserialize::begin(&mut self.balances),
                "disputes_list" => Deserialize::begin(&mut self.disputes_list),
                "documents" => Deserialize::begin(&mut self.documents),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "financial_account_transactions" => {
                    Deserialize::begin(&mut self.financial_account_transactions)
                }
                "instant_payouts_promotion" => {
                    Deserialize::begin(&mut self.instant_payouts_promotion)
                }
                "issuing_card" => Deserialize::begin(&mut self.issuing_card),
                "issuing_cards_list" => Deserialize::begin(&mut self.issuing_cards_list),
                "notification_banner" => Deserialize::begin(&mut self.notification_banner),
                "payment_details" => Deserialize::begin(&mut self.payment_details),
                "payment_disputes" => Deserialize::begin(&mut self.payment_disputes),
                "payment_method_settings" => Deserialize::begin(&mut self.payment_method_settings),
                "payments" => Deserialize::begin(&mut self.payments),
                "payout_details" => Deserialize::begin(&mut self.payout_details),
                "payout_reconciliation_report" => {
                    Deserialize::begin(&mut self.payout_reconciliation_report)
                }
                "payouts" => Deserialize::begin(&mut self.payouts),
                "payouts_list" => Deserialize::begin(&mut self.payouts_list),
                "tax_registrations" => Deserialize::begin(&mut self.tax_registrations),
                "tax_settings" => Deserialize::begin(&mut self.tax_settings),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_management: None,
                account_onboarding: None,
                balance_report: None,
                balances: None,
                disputes_list: None,
                documents: None,
                financial_account: None,
                financial_account_transactions: None,
                instant_payouts_promotion: None,
                issuing_card: None,
                issuing_cards_list: None,
                notification_banner: None,
                payment_details: None,
                payment_disputes: None,
                payment_method_settings: None,
                payments: None,
                payout_details: None,
                payout_reconciliation_report: None,
                payouts: None,
                payouts_list: None,
                tax_registrations: None,
                tax_settings: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    account_management: Some(account_management),
                    account_onboarding: Some(account_onboarding),
                    balance_report: Some(balance_report),
                    balances: Some(balances),
                    disputes_list: Some(disputes_list),
                    documents: Some(documents),
                    financial_account: Some(financial_account),
                    financial_account_transactions: Some(financial_account_transactions),
                    instant_payouts_promotion: Some(instant_payouts_promotion),
                    issuing_card: Some(issuing_card),
                    issuing_cards_list: Some(issuing_cards_list),
                    notification_banner: Some(notification_banner),
                    payment_details: Some(payment_details),
                    payment_disputes: Some(payment_disputes),
                    payment_method_settings: Some(payment_method_settings),
                    payments: Some(payments),
                    payout_details: Some(payout_details),
                    payout_reconciliation_report: Some(payout_reconciliation_report),
                    payouts: Some(payouts),
                    payouts_list: Some(payouts_list),
                    tax_registrations: Some(tax_registrations),
                    tax_settings: Some(tax_settings),
                } => Some(Self::Out {
                    account_management,
                    account_onboarding,
                    balance_report,
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
                    payment_method_settings,
                    payments,
                    payout_details,
                    payout_reconciliation_report,
                    payouts,
                    payouts_list,
                    tax_registrations,
                    tax_settings,
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

    impl ObjectDeser for ConnectEmbeddedAccountSessionCreateComponents {
        type Builder = ConnectEmbeddedAccountSessionCreateComponentsBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedAccountSessionCreateComponents {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
