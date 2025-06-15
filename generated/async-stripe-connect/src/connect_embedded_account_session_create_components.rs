#[derive(Copy, Clone, Debug)]
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
    pub issuing_card: stripe_connect::ConnectEmbeddedIssuingCardConfigClaim,
    pub issuing_cards_list: stripe_connect::ConnectEmbeddedIssuingCardsListConfigClaim,
    pub notification_banner: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub payment_details: stripe_connect::ConnectEmbeddedPaymentsConfigClaim,
    pub payment_disputes: stripe_connect::ConnectEmbeddedPaymentDisputesConfig,
    pub payments: stripe_connect::ConnectEmbeddedPaymentsConfigClaim,
    pub payouts: stripe_connect::ConnectEmbeddedPayoutsConfig,
    pub payouts_list: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub tax_registrations: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub tax_settings: stripe_connect::ConnectEmbeddedBaseConfigClaim,
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
    issuing_card: Option<stripe_connect::ConnectEmbeddedIssuingCardConfigClaim>,
    issuing_cards_list: Option<stripe_connect::ConnectEmbeddedIssuingCardsListConfigClaim>,
    notification_banner: Option<stripe_connect::ConnectEmbeddedAccountConfigClaim>,
    payment_details: Option<stripe_connect::ConnectEmbeddedPaymentsConfigClaim>,
    payment_disputes: Option<stripe_connect::ConnectEmbeddedPaymentDisputesConfig>,
    payments: Option<stripe_connect::ConnectEmbeddedPaymentsConfigClaim>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                "balances" => Deserialize::begin(&mut self.balances),
                "disputes_list" => Deserialize::begin(&mut self.disputes_list),
                "documents" => Deserialize::begin(&mut self.documents),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "financial_account_transactions" => {
                    Deserialize::begin(&mut self.financial_account_transactions)
                }
                "issuing_card" => Deserialize::begin(&mut self.issuing_card),
                "issuing_cards_list" => Deserialize::begin(&mut self.issuing_cards_list),
                "notification_banner" => Deserialize::begin(&mut self.notification_banner),
                "payment_details" => Deserialize::begin(&mut self.payment_details),
                "payment_disputes" => Deserialize::begin(&mut self.payment_disputes),
                "payments" => Deserialize::begin(&mut self.payments),
                "payouts" => Deserialize::begin(&mut self.payouts),
                "payouts_list" => Deserialize::begin(&mut self.payouts_list),
                "tax_registrations" => Deserialize::begin(&mut self.tax_registrations),
                "tax_settings" => Deserialize::begin(&mut self.tax_settings),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_management: Deserialize::default(),
                account_onboarding: Deserialize::default(),
                balances: Deserialize::default(),
                disputes_list: Deserialize::default(),
                documents: Deserialize::default(),
                financial_account: Deserialize::default(),
                financial_account_transactions: Deserialize::default(),
                issuing_card: Deserialize::default(),
                issuing_cards_list: Deserialize::default(),
                notification_banner: Deserialize::default(),
                payment_details: Deserialize::default(),
                payment_disputes: Deserialize::default(),
                payments: Deserialize::default(),
                payouts: Deserialize::default(),
                payouts_list: Deserialize::default(),
                tax_registrations: Deserialize::default(),
                tax_settings: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_management),
                Some(account_onboarding),
                Some(balances),
                Some(disputes_list),
                Some(documents),
                Some(financial_account),
                Some(financial_account_transactions),
                Some(issuing_card),
                Some(issuing_cards_list),
                Some(notification_banner),
                Some(payment_details),
                Some(payment_disputes),
                Some(payments),
                Some(payouts),
                Some(payouts_list),
                Some(tax_registrations),
                Some(tax_settings),
            ) = (
                self.account_management,
                self.account_onboarding,
                self.balances,
                self.disputes_list,
                self.documents,
                self.financial_account,
                self.financial_account_transactions,
                self.issuing_card,
                self.issuing_cards_list,
                self.notification_banner,
                self.payment_details,
                self.payment_disputes,
                self.payments,
                self.payouts,
                self.payouts_list,
                self.tax_registrations,
                self.tax_settings,
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_management,
                account_onboarding,
                balances,
                disputes_list,
                documents,
                financial_account,
                financial_account_transactions,
                issuing_card,
                issuing_cards_list,
                notification_banner,
                payment_details,
                payment_disputes,
                payments,
                payouts,
                payouts_list,
                tax_registrations,
                tax_settings,
            })
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

    impl ObjectDeser for ConnectEmbeddedAccountSessionCreateComponents {
        type Builder = ConnectEmbeddedAccountSessionCreateComponentsBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedAccountSessionCreateComponents {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedAccountSessionCreateComponentsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_management" => b.account_management = FromValueOpt::from_value(v),
                    "account_onboarding" => b.account_onboarding = FromValueOpt::from_value(v),
                    "balances" => b.balances = FromValueOpt::from_value(v),
                    "disputes_list" => b.disputes_list = FromValueOpt::from_value(v),
                    "documents" => b.documents = FromValueOpt::from_value(v),
                    "financial_account" => b.financial_account = FromValueOpt::from_value(v),
                    "financial_account_transactions" => {
                        b.financial_account_transactions = FromValueOpt::from_value(v)
                    }
                    "issuing_card" => b.issuing_card = FromValueOpt::from_value(v),
                    "issuing_cards_list" => b.issuing_cards_list = FromValueOpt::from_value(v),
                    "notification_banner" => b.notification_banner = FromValueOpt::from_value(v),
                    "payment_details" => b.payment_details = FromValueOpt::from_value(v),
                    "payment_disputes" => b.payment_disputes = FromValueOpt::from_value(v),
                    "payments" => b.payments = FromValueOpt::from_value(v),
                    "payouts" => b.payouts = FromValueOpt::from_value(v),
                    "payouts_list" => b.payouts_list = FromValueOpt::from_value(v),
                    "tax_registrations" => b.tax_registrations = FromValueOpt::from_value(v),
                    "tax_settings" => b.tax_settings = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
