#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPhysicalBundleFeatures {
    /// The policy for how to use card logo images in a card design with this physical bundle.
    pub card_logo: stripe_shared::IssuingPhysicalBundleFeaturesCardLogo,
    /// The policy for how to use carrier letter text in a card design with this physical bundle.
    pub carrier_text: stripe_shared::IssuingPhysicalBundleFeaturesCardLogo,
    /// The policy for how to use a second line on a card with this physical bundle.
    pub second_line: stripe_shared::IssuingPhysicalBundleFeaturesCardLogo,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPhysicalBundleFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingPhysicalBundleFeatures").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingPhysicalBundleFeaturesBuilder {
    card_logo: Option<stripe_shared::IssuingPhysicalBundleFeaturesCardLogo>,
    carrier_text: Option<stripe_shared::IssuingPhysicalBundleFeaturesCardLogo>,
    second_line: Option<stripe_shared::IssuingPhysicalBundleFeaturesCardLogo>,
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

    impl Deserialize for IssuingPhysicalBundleFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPhysicalBundleFeatures>,
        builder: IssuingPhysicalBundleFeaturesBuilder,
    }

    impl Visitor for Place<IssuingPhysicalBundleFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPhysicalBundleFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPhysicalBundleFeaturesBuilder {
        type Out = IssuingPhysicalBundleFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_logo" => Deserialize::begin(&mut self.card_logo),
                "carrier_text" => Deserialize::begin(&mut self.carrier_text),
                "second_line" => Deserialize::begin(&mut self.second_line),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card_logo: None, carrier_text: None, second_line: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card_logo), Some(carrier_text), Some(second_line)) =
                (self.card_logo.take(), self.carrier_text.take(), self.second_line.take())
            else {
                return None;
            };
            Some(Self::Out { card_logo, carrier_text, second_line })
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

    impl ObjectDeser for IssuingPhysicalBundleFeatures {
        type Builder = IssuingPhysicalBundleFeaturesBuilder;
    }

    impl FromValueOpt for IssuingPhysicalBundleFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPhysicalBundleFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_logo" => b.card_logo = FromValueOpt::from_value(v),
                    "carrier_text" => b.carrier_text = FromValueOpt::from_value(v),
                    "second_line" => b.second_line = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The policy for how to use card logo images in a card design with this physical bundle.
pub type IssuingPhysicalBundleFeaturesCardLogo =
    stripe_shared::IssuingPhysicalBundleFeaturesCardLogo;
/// The policy for how to use carrier letter text in a card design with this physical bundle.
pub type IssuingPhysicalBundleFeaturesCarrierText =
    stripe_shared::IssuingPhysicalBundleFeaturesCardLogo;
/// The policy for how to use a second line on a card with this physical bundle.
pub type IssuingPhysicalBundleFeaturesSecondLine =
    stripe_shared::IssuingPhysicalBundleFeaturesCardLogo;
