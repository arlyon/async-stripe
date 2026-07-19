#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedIssuingCardsListFeatures {
    /// Whether to allow card management features.
    pub card_management: bool,
    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,
    /// Whether to allow cardholder management features.
    pub cardholder_management: bool,
    /// Whether Stripe user authentication is disabled.
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    pub disable_stripe_user_authentication: bool,
    /// Whether to allow spend control management features.
    pub spend_control_management: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedIssuingCardsListFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedIssuingCardsListFeatures").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedIssuingCardsListFeaturesBuilder {
    card_management: Option<bool>,
    card_spend_dispute_management: Option<bool>,
    cardholder_management: Option<bool>,
    disable_stripe_user_authentication: Option<bool>,
    spend_control_management: Option<bool>,
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

    impl Deserialize for ConnectEmbeddedIssuingCardsListFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedIssuingCardsListFeatures>,
        builder: ConnectEmbeddedIssuingCardsListFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedIssuingCardsListFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedIssuingCardsListFeaturesBuilder {
                    card_management: Deserialize::default(),
                    card_spend_dispute_management: Deserialize::default(),
                    cardholder_management: Deserialize::default(),
                    disable_stripe_user_authentication: Deserialize::default(),
                    spend_control_management: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_management" => Deserialize::begin(&mut self.builder.card_management),
                "card_spend_dispute_management" => {
                    Deserialize::begin(&mut self.builder.card_spend_dispute_management)
                }
                "cardholder_management" => {
                    Deserialize::begin(&mut self.builder.cardholder_management)
                }
                "disable_stripe_user_authentication" => {
                    Deserialize::begin(&mut self.builder.disable_stripe_user_authentication)
                }
                "spend_control_management" => {
                    Deserialize::begin(&mut self.builder.spend_control_management)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(card_management),
                Some(card_spend_dispute_management),
                Some(cardholder_management),
                Some(disable_stripe_user_authentication),
                Some(spend_control_management),
            ) = (
                self.builder.card_management,
                self.builder.card_spend_dispute_management,
                self.builder.cardholder_management,
                self.builder.disable_stripe_user_authentication,
                self.builder.spend_control_management,
            )
            else {
                return Ok(());
            };
            *self.out = Some(ConnectEmbeddedIssuingCardsListFeatures {
                card_management,
                card_spend_dispute_management,
                cardholder_management,
                disable_stripe_user_authentication,
                spend_control_management,
            });
            Ok(())
        }
    }
};
