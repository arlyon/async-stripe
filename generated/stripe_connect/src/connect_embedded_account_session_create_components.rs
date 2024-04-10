#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_onboarding: stripe_connect::ConnectEmbeddedBaseConfigClaim,
    pub payment_details: stripe_connect::ConnectEmbeddedPaymentsConfig,
    pub payments: stripe_connect::ConnectEmbeddedPaymentsConfig,
    pub payouts: stripe_connect::ConnectEmbeddedPayoutsConfig,
}
#[doc(hidden)]
pub struct ConnectEmbeddedAccountSessionCreateComponentsBuilder {
    account_onboarding: Option<stripe_connect::ConnectEmbeddedBaseConfigClaim>,
    payment_details: Option<stripe_connect::ConnectEmbeddedPaymentsConfig>,
    payments: Option<stripe_connect::ConnectEmbeddedPaymentsConfig>,
    payouts: Option<stripe_connect::ConnectEmbeddedPayoutsConfig>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
                "account_onboarding" => Deserialize::begin(&mut self.account_onboarding),
                "payment_details" => Deserialize::begin(&mut self.payment_details),
                "payments" => Deserialize::begin(&mut self.payments),
                "payouts" => Deserialize::begin(&mut self.payouts),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_onboarding: Deserialize::default(),
                payment_details: Deserialize::default(),
                payments: Deserialize::default(),
                payouts: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account_onboarding: self.account_onboarding?,
                payment_details: self.payment_details?,
                payments: self.payments?,
                payouts: self.payouts?,
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
                    "account_onboarding" => {
                        b.account_onboarding = Some(FromValueOpt::from_value(v)?)
                    }
                    "payment_details" => b.payment_details = Some(FromValueOpt::from_value(v)?),
                    "payments" => b.payments = Some(FromValueOpt::from_value(v)?),
                    "payouts" => b.payouts = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
