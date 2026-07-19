#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodConfigResourceDisplayPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodConfigResourceDisplayPreference").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodConfigResourceDisplayPreferenceBuilder {
                    overridable: Deserialize::default(),
                    preference: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "overridable" => Deserialize::begin(&mut self.builder.overridable),
                "preference" => Deserialize::begin(&mut self.builder.preference),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(overridable), Some(preference), Some(value)) = (
                self.builder.overridable,
                self.builder.preference.take(),
                self.builder.value.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodConfigResourceDisplayPreference {
                overridable,
                preference,
                value,
            });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodConfigResourceDisplayPreferencePreference))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentMethodConfigResourceDisplayPreferencePreference>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodConfigResourceDisplayPreferencePreference::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodConfigResourceDisplayPreferenceValue))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodConfigResourceDisplayPreferenceValue> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodConfigResourceDisplayPreferenceValue::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
