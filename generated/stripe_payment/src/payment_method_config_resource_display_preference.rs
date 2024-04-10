#[derive(Copy, Clone, Debug)]
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Some(Self::Out {
                overridable: self.overridable?,
                preference: self.preference?,
                value: self.value?,
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
                    "overridable" => b.overridable = Some(FromValueOpt::from_value(v)?),
                    "preference" => b.preference = Some(FromValueOpt::from_value(v)?),
                    "value" => b.value = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The account's display preference.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodConfigResourceDisplayPreferencePreference {
    None,
    Off,
    On,
}
impl PaymentMethodConfigResourceDisplayPreferencePreference {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodConfigResourceDisplayPreferencePreference::*;
        match self {
            None => "none",
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for PaymentMethodConfigResourceDisplayPreferencePreference {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodConfigResourceDisplayPreferencePreference::*;
        match s {
            "none" => Ok(None),
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
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
                .map_err(|_| miniserde::Error)?,
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodConfigResourceDisplayPreferencePreference",
            )
        })
    }
}
/// The effective display preference value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodConfigResourceDisplayPreferenceValue {
    Off,
    On,
}
impl PaymentMethodConfigResourceDisplayPreferenceValue {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodConfigResourceDisplayPreferenceValue::*;
        match self {
            Off => "off",
            On => "on",
        }
    }
}

impl std::str::FromStr for PaymentMethodConfigResourceDisplayPreferenceValue {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodConfigResourceDisplayPreferenceValue::*;
        match s {
            "off" => Ok(Off),
            "on" => Ok(On),
            _ => Err(()),
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
            PaymentMethodConfigResourceDisplayPreferenceValue::from_str(s)
                .map_err(|_| miniserde::Error)?,
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodConfigResourceDisplayPreferenceValue",
            )
        })
    }
}
