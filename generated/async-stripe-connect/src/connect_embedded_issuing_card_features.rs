#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedIssuingCardFeatures {
    /// Whether to allow card management features.
    pub card_management: bool,
    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,
    /// Whether to allow cardholder management features.
    pub cardholder_management: bool,
    /// Whether to allow spend control management features.
    pub spend_control_management: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedIssuingCardFeaturesBuilder {
    card_management: Option<bool>,
    card_spend_dispute_management: Option<bool>,
    cardholder_management: Option<bool>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConnectEmbeddedIssuingCardFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedIssuingCardFeatures>,
        builder: ConnectEmbeddedIssuingCardFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedIssuingCardFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedIssuingCardFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedIssuingCardFeaturesBuilder {
        type Out = ConnectEmbeddedIssuingCardFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_management" => Deserialize::begin(&mut self.card_management),
                "card_spend_dispute_management" => {
                    Deserialize::begin(&mut self.card_spend_dispute_management)
                }
                "cardholder_management" => Deserialize::begin(&mut self.cardholder_management),
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
                spend_control_management: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(card_management),
                Some(card_spend_dispute_management),
                Some(cardholder_management),
                Some(spend_control_management),
            ) = (
                self.card_management,
                self.card_spend_dispute_management,
                self.cardholder_management,
                self.spend_control_management,
            )
            else {
                return None;
            };
            Some(Self::Out {
                card_management,
                card_spend_dispute_management,
                cardholder_management,
                spend_control_management,
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

    impl ObjectDeser for ConnectEmbeddedIssuingCardFeatures {
        type Builder = ConnectEmbeddedIssuingCardFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedIssuingCardFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedIssuingCardFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_management" => b.card_management = FromValueOpt::from_value(v),
                    "card_spend_dispute_management" => {
                        b.card_spend_dispute_management = FromValueOpt::from_value(v)
                    }
                    "cardholder_management" => {
                        b.cardholder_management = FromValueOpt::from_value(v)
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
