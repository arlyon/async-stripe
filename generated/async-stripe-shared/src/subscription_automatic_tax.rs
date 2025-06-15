#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionAutomaticTax {
    /// If Stripe disabled automatic tax, this enum describes why.
    pub disabled_reason: Option<SubscriptionAutomaticTaxDisabledReason>,
    /// Whether Stripe automatically computes tax on this subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<stripe_shared::ConnectAccountReference>,
}
#[doc(hidden)]
pub struct SubscriptionAutomaticTaxBuilder {
    disabled_reason: Option<Option<SubscriptionAutomaticTaxDisabledReason>>,
    enabled: Option<bool>,
    liability: Option<Option<stripe_shared::ConnectAccountReference>>,
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

    impl Deserialize for SubscriptionAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionAutomaticTax>,
        builder: SubscriptionAutomaticTaxBuilder,
    }

    impl Visitor for Place<SubscriptionAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionAutomaticTaxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionAutomaticTaxBuilder {
        type Out = SubscriptionAutomaticTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disabled_reason" => Deserialize::begin(&mut self.disabled_reason),
                "enabled" => Deserialize::begin(&mut self.enabled),
                "liability" => Deserialize::begin(&mut self.liability),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                disabled_reason: Deserialize::default(),
                enabled: Deserialize::default(),
                liability: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(disabled_reason), Some(enabled), Some(liability)) =
                (self.disabled_reason, self.enabled, self.liability.take())
            else {
                return None;
            };
            Some(Self::Out { disabled_reason, enabled, liability })
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

    impl ObjectDeser for SubscriptionAutomaticTax {
        type Builder = SubscriptionAutomaticTaxBuilder;
    }

    impl FromValueOpt for SubscriptionAutomaticTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionAutomaticTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "disabled_reason" => b.disabled_reason = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "liability" => b.liability = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// If Stripe disabled automatic tax, this enum describes why.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionAutomaticTaxDisabledReason {
    RequiresLocationInputs,
}
impl SubscriptionAutomaticTaxDisabledReason {
    pub fn as_str(self) -> &'static str {
        use SubscriptionAutomaticTaxDisabledReason::*;
        match self {
            RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for SubscriptionAutomaticTaxDisabledReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionAutomaticTaxDisabledReason::*;
        match s {
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionAutomaticTaxDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionAutomaticTaxDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionAutomaticTaxDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionAutomaticTaxDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionAutomaticTaxDisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionAutomaticTaxDisabledReason::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionAutomaticTaxDisabledReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionAutomaticTaxDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SubscriptionAutomaticTaxDisabledReason")
        })
    }
}
