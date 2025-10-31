#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedInstantPayoutsPromotionFeatures {
    /// Whether Stripe user authentication is disabled.
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    pub disable_stripe_user_authentication: bool,
    /// Whether external account collection is enabled.
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    pub external_account_collection: bool,
    /// Whether to allow creation of instant payouts.
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    pub instant_payouts: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedInstantPayoutsPromotionFeaturesBuilder {
    disable_stripe_user_authentication: Option<bool>,
    external_account_collection: Option<bool>,
    instant_payouts: Option<bool>,
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

    impl Deserialize for ConnectEmbeddedInstantPayoutsPromotionFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedInstantPayoutsPromotionFeatures>,
        builder: ConnectEmbeddedInstantPayoutsPromotionFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedInstantPayoutsPromotionFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedInstantPayoutsPromotionFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedInstantPayoutsPromotionFeaturesBuilder {
        type Out = ConnectEmbeddedInstantPayoutsPromotionFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disable_stripe_user_authentication" => {
                    Deserialize::begin(&mut self.disable_stripe_user_authentication)
                }
                "external_account_collection" => {
                    Deserialize::begin(&mut self.external_account_collection)
                }
                "instant_payouts" => Deserialize::begin(&mut self.instant_payouts),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                disable_stripe_user_authentication: Deserialize::default(),
                external_account_collection: Deserialize::default(),
                instant_payouts: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(disable_stripe_user_authentication),
                Some(external_account_collection),
                Some(instant_payouts),
            ) = (
                self.disable_stripe_user_authentication,
                self.external_account_collection,
                self.instant_payouts,
            )
            else {
                return None;
            };
            Some(Self::Out {
                disable_stripe_user_authentication,
                external_account_collection,
                instant_payouts,
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

    impl ObjectDeser for ConnectEmbeddedInstantPayoutsPromotionFeatures {
        type Builder = ConnectEmbeddedInstantPayoutsPromotionFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedInstantPayoutsPromotionFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedInstantPayoutsPromotionFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "disable_stripe_user_authentication" => {
                        b.disable_stripe_user_authentication = FromValueOpt::from_value(v)
                    }
                    "external_account_collection" => {
                        b.external_account_collection = FromValueOpt::from_value(v)
                    }
                    "instant_payouts" => b.instant_payouts = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
