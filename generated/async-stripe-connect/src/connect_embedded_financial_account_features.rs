#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedFinancialAccountFeatures {
    /// Disables Stripe user authentication for this embedded component.
    /// This value can only be true for accounts where `controller.requirement_collection` is `application`.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don’t set `external_account_collection`, it defaults to true and `disable_stripe_user_authentication` defaults to false.
    pub disable_stripe_user_authentication: bool,
    /// Whether to allow external accounts to be linked for money transfer.
    pub external_account_collection: bool,
    /// Whether to allow sending money.
    pub send_money: bool,
    /// Whether to allow transferring balance.
    pub transfer_balance: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedFinancialAccountFeaturesBuilder {
    disable_stripe_user_authentication: Option<bool>,
    external_account_collection: Option<bool>,
    send_money: Option<bool>,
    transfer_balance: Option<bool>,
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

    impl Deserialize for ConnectEmbeddedFinancialAccountFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedFinancialAccountFeatures>,
        builder: ConnectEmbeddedFinancialAccountFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedFinancialAccountFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedFinancialAccountFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedFinancialAccountFeaturesBuilder {
        type Out = ConnectEmbeddedFinancialAccountFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disable_stripe_user_authentication" => {
                    Deserialize::begin(&mut self.disable_stripe_user_authentication)
                }
                "external_account_collection" => {
                    Deserialize::begin(&mut self.external_account_collection)
                }
                "send_money" => Deserialize::begin(&mut self.send_money),
                "transfer_balance" => Deserialize::begin(&mut self.transfer_balance),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                disable_stripe_user_authentication: Deserialize::default(),
                external_account_collection: Deserialize::default(),
                send_money: Deserialize::default(),
                transfer_balance: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(disable_stripe_user_authentication),
                Some(external_account_collection),
                Some(send_money),
                Some(transfer_balance),
            ) = (
                self.disable_stripe_user_authentication,
                self.external_account_collection,
                self.send_money,
                self.transfer_balance,
            )
            else {
                return None;
            };
            Some(Self::Out {
                disable_stripe_user_authentication,
                external_account_collection,
                send_money,
                transfer_balance,
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

    impl ObjectDeser for ConnectEmbeddedFinancialAccountFeatures {
        type Builder = ConnectEmbeddedFinancialAccountFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedFinancialAccountFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedFinancialAccountFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "disable_stripe_user_authentication" => {
                        b.disable_stripe_user_authentication = FromValueOpt::from_value(v)
                    }
                    "external_account_collection" => {
                        b.external_account_collection = FromValueOpt::from_value(v)
                    }
                    "send_money" => b.send_money = FromValueOpt::from_value(v),
                    "transfer_balance" => b.transfer_balance = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
