#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// The default value is `enabled` when Stripe is responsible for negative account balances, and `use_dashboard_rules` otherwise.
    pub instant_payouts: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedInstantPayoutsPromotionFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedInstantPayoutsPromotionFeatures").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: ConnectEmbeddedInstantPayoutsPromotionFeaturesBuilder {
                    disable_stripe_user_authentication: Deserialize::default(),
                    external_account_collection: Deserialize::default(),
                    instant_payouts: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disable_stripe_user_authentication" => {
                    Deserialize::begin(&mut self.builder.disable_stripe_user_authentication)
                }
                "external_account_collection" => {
                    Deserialize::begin(&mut self.builder.external_account_collection)
                }
                "instant_payouts" => Deserialize::begin(&mut self.builder.instant_payouts),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(disable_stripe_user_authentication),
                Some(external_account_collection),
                Some(instant_payouts),
            ) = (
                self.builder.disable_stripe_user_authentication,
                self.builder.external_account_collection,
                self.builder.instant_payouts,
            )
            else {
                return Ok(());
            };
            *self.out = Some(ConnectEmbeddedInstantPayoutsPromotionFeatures {
                disable_stripe_user_authentication,
                external_account_collection,
                instant_payouts,
            });
            Ok(())
        }
    }
};
