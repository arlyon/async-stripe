#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_management: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub account_onboarding: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub balances: stripe_connect::ConnectEmbeddedPayoutsConfigClaim,
    pub documents: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub notification_banner: stripe_connect::ConnectEmbeddedAccountConfigClaim,
    pub payment_details: stripe_connect::ConnectEmbeddedPaymentsConfigClaim,
    pub payments: stripe_connect::ConnectEmbeddedPaymentsConfigClaim,
    pub payouts: stripe_connect::ConnectEmbeddedPayoutsConfigClaim,
    pub payouts_list: stripe_connect::ConnectEmbeddedBaseConfigClaim,
}
#[doc(hidden)]
pub struct ConnectEmbeddedAccountSessionCreateComponentsBuilder {
    account_management: Option<stripe_connect::ConnectEmbeddedAccountConfigClaim>,
    account_onboarding: Option<stripe_connect::ConnectEmbeddedAccountConfigClaim>,
    balances: Option<stripe_connect::ConnectEmbeddedPayoutsConfigClaim>,
    documents: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    notification_banner: Option<stripe_connect::ConnectEmbeddedAccountConfigClaim>,
    payment_details: Option<stripe_connect::ConnectEmbeddedPaymentsConfigClaim>,
    payments: Option<stripe_connect::ConnectEmbeddedPaymentsConfigClaim>,
    payouts: Option<stripe_connect::ConnectEmbeddedPayoutsConfigClaim>,
    payouts_list: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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
                "documents" => Deserialize::begin(&mut self.documents),
                "notification_banner" => Deserialize::begin(&mut self.notification_banner),
                "payment_details" => Deserialize::begin(&mut self.payment_details),
                "payments" => Deserialize::begin(&mut self.payments),
                "payouts" => Deserialize::begin(&mut self.payouts),
                "payouts_list" => Deserialize::begin(&mut self.payouts_list),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_management: Deserialize::default(),
                account_onboarding: Deserialize::default(),
                balances: Deserialize::default(),
                documents: Deserialize::default(),
                notification_banner: Deserialize::default(),
                payment_details: Deserialize::default(),
                payments: Deserialize::default(),
                payouts: Deserialize::default(),
                payouts_list: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_management),
                Some(account_onboarding),
                Some(balances),
                Some(documents),
                Some(notification_banner),
                Some(payment_details),
                Some(payments),
                Some(payouts),
                Some(payouts_list),
            ) = (
                self.account_management,
                self.account_onboarding,
                self.balances,
                self.documents,
                self.notification_banner,
                self.payment_details,
                self.payments,
                self.payouts,
                self.payouts_list,
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_management,
                account_onboarding,
                balances,
                documents,
                notification_banner,
                payment_details,
                payments,
                payouts,
                payouts_list,
            })
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
                    "documents" => b.documents = FromValueOpt::from_value(v),
                    "notification_banner" => b.notification_banner = FromValueOpt::from_value(v),
                    "payment_details" => b.payment_details = FromValueOpt::from_value(v),
                    "payments" => b.payments = FromValueOpt::from_value(v),
                    "payouts" => b.payouts = FromValueOpt::from_value(v),
                    "payouts_list" => b.payouts_list = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
