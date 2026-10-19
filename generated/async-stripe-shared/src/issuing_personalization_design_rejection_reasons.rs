#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPersonalizationDesignRejectionReasons {
    /// The reason(s) the card logo was rejected.
    pub card_logo: Option<Vec<stripe_shared::CardLogo>>,
    /// The reason(s) the carrier text was rejected.
    pub carrier_text: Option<Vec<stripe_shared::CarrierText>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingPersonalizationDesignRejectionReasons").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingPersonalizationDesignRejectionReasonsBuilder {
    card_logo: Option<Option<Vec<stripe_shared::CardLogo>>>,
    carrier_text: Option<Option<Vec<stripe_shared::CarrierText>>>,
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

    impl Deserialize for IssuingPersonalizationDesignRejectionReasons {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPersonalizationDesignRejectionReasons>,
        builder: IssuingPersonalizationDesignRejectionReasonsBuilder,
    }

    impl Visitor for Place<IssuingPersonalizationDesignRejectionReasons> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPersonalizationDesignRejectionReasonsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPersonalizationDesignRejectionReasonsBuilder {
        type Out = IssuingPersonalizationDesignRejectionReasons;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_logo" => Deserialize::begin(&mut self.card_logo),
                "carrier_text" => Deserialize::begin(&mut self.carrier_text),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card_logo: Some(None), carrier_text: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card_logo), Some(carrier_text)) =
                (self.card_logo.take(), self.carrier_text.take())
            else {
                return None;
            };
            Some(Self::Out { card_logo, carrier_text })
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

    impl ObjectDeser for IssuingPersonalizationDesignRejectionReasons {
        type Builder = IssuingPersonalizationDesignRejectionReasonsBuilder;
    }

    impl FromValueOpt for IssuingPersonalizationDesignRejectionReasons {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPersonalizationDesignRejectionReasonsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_logo" => b.card_logo = FromValueOpt::from_value(v),
                    "carrier_text" => b.carrier_text = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reason(s) the card logo was rejected.
pub type IssuingPersonalizationDesignRejectionReasonsCardLogo = stripe_shared::CardLogo;
/// The reason(s) the carrier text was rejected.
pub type IssuingPersonalizationDesignRejectionReasonsCarrierText = stripe_shared::CarrierText;
