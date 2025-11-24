#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodConfigResourceDisplayPreference {
    /// For child configs, whether or not the account's preference will be observed.
    /// If `false`, the parent configuration's default is used.
    pub overridable: Option<bool>,
    /// The account's display preference.
    pub preference: PaymentMethodConfigResourceDisplayPreferencePreference,
    /// The effective display preference value.
    pub value: PaymentMethodConfigResourceDisplayPreferenceValue,
}
#[doc(hidden)]
pub struct PaymentMethodConfigResourceDisplayPreferenceBuilder {
    overridable: Option<Option<bool>>,
    preference: Option<PaymentMethodConfigResourceDisplayPreferencePreference>,
    value: Option<PaymentMethodConfigResourceDisplayPreferenceValue>,
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

    impl Deserialize for PaymentMethodConfigResourceDisplayPreference {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfigResourceDisplayPreference>,
        builder: PaymentMethodConfigResourceDisplayPreferenceBuilder,
    }

    impl Visitor for Place<PaymentMethodConfigResourceDisplayPreference> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodConfigResourceDisplayPreferenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodConfigResourceDisplayPreferenceBuilder {
        type Out = PaymentMethodConfigResourceDisplayPreference;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "overridable" => Deserialize::begin(&mut self.overridable),
                "preference" => Deserialize::begin(&mut self.preference),
                "value" => Deserialize::begin(&mut self.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                overridable: Deserialize::default(),
                preference: Deserialize::default(),
                value: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(overridable), Some(preference), Some(value)) =
                (self.overridable, self.preference.take(), self.value.take())
            else {
                return None;
            };
            Some(Self::Out { overridable, preference, value })
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

    impl ObjectDeser for PaymentMethodConfigResourceDisplayPreference {
        type Builder = PaymentMethodConfigResourceDisplayPreferenceBuilder;
    }

    impl FromValueOpt for PaymentMethodConfigResourceDisplayPreference {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodConfigResourceDisplayPreferenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "overridable" => b.overridable = FromValueOpt::from_value(v),
                    "preference" => b.preference = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The account's display preference.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodConfigResourceDisplayPreferencePreference {
    None,
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodConfigResourceDisplayPreferencePreference {
    pub fn as_str(&self) -> &str {
        use PaymentMethodConfigResourceDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodConfigResourceDisplayPreferencePreference {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodConfigResourceDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodConfigResourceDisplayPreferencePreference"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentMethodConfigResourceDisplayPreferencePreference>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodConfigResourceDisplayPreferencePreference::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodConfigResourceDisplayPreferencePreference);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The effective display preference value.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodConfigResourceDisplayPreferenceValue {
    Off,
    On,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodConfigResourceDisplayPreferenceValue {
    pub fn as_str(&self) -> &str {
        use PaymentMethodConfigResourceDisplayPreferenceValue::*;
        match self {
            Off => "off",
            On => "on",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodConfigResourceDisplayPreferenceValue {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodConfigResourceDisplayPreferenceValue::*;
        match s {
            "off" => Ok(Off),
            "on" => Ok(On),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodConfigResourceDisplayPreferenceValue"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodConfigResourceDisplayPreferenceValue> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodConfigResourceDisplayPreferenceValue::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodConfigResourceDisplayPreferenceValue);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
