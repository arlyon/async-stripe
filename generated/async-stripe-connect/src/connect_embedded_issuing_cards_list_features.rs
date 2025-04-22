#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedIssuingCardsListFeatures {
    /// Whether to allow card management features.
    pub card_management: bool,
    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,
    /// Whether to allow cardholder management features.
    pub cardholder_management: bool,
    /// Disables Stripe user authentication for this embedded component.
    /// This feature can only be false for accounts where you’re responsible for collecting updated information when requirements are due or change, like custom accounts.
    pub disable_stripe_user_authentication: bool,
    /// Whether to allow spend control management features.
    pub spend_control_management: bool,
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
                builder: ConnectEmbeddedIssuingCardsListFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedIssuingCardsListFeaturesBuilder {
        type Out = ConnectEmbeddedIssuingCardsListFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_management" => Deserialize::begin(&mut self.card_management),
                "card_spend_dispute_management" => {
                    Deserialize::begin(&mut self.card_spend_dispute_management)
                }
                "cardholder_management" => Deserialize::begin(&mut self.cardholder_management),
                "disable_stripe_user_authentication" => {
                    Deserialize::begin(&mut self.disable_stripe_user_authentication)
                }
                "spend_control_management" => {
                    Deserialize::begin(&mut self.spend_control_management)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card_management: Deserialize::default(),
                card_spend_dispute_management: Deserialize::default(),
                cardholder_management: Deserialize::default(),
                disable_stripe_user_authentication: Deserialize::default(),
                spend_control_management: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(card_management),
                Some(card_spend_dispute_management),
                Some(cardholder_management),
                Some(disable_stripe_user_authentication),
                Some(spend_control_management),
            ) = (
                self.card_management,
                self.card_spend_dispute_management,
                self.cardholder_management,
                self.disable_stripe_user_authentication,
                self.spend_control_management,
            )
            else {
                return None;
            };
            Some(Self::Out {
                card_management,
                card_spend_dispute_management,
                cardholder_management,
                disable_stripe_user_authentication,
                spend_control_management,
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

    impl ObjectDeser for ConnectEmbeddedIssuingCardsListFeatures {
        type Builder = ConnectEmbeddedIssuingCardsListFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedIssuingCardsListFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedIssuingCardsListFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_management" => b.card_management = FromValueOpt::from_value(v),
                    "card_spend_dispute_management" => {
                        b.card_spend_dispute_management = FromValueOpt::from_value(v)
                    }
                    "cardholder_management" => {
                        b.cardholder_management = FromValueOpt::from_value(v)
                    }
                    "disable_stripe_user_authentication" => {
                        b.disable_stripe_user_authentication = FromValueOpt::from_value(v)
                    }
                    "spend_control_management" => {
                        b.spend_control_management = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
