#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodConfigResourceDisplayPreference {
    /// For child configs, whether or not the account's preference will be observed.
    /// If `false`, the parent configuration's default is used.
    pub overridable: Option<bool>,
    /// The account's display preference.
    pub preference: PaymentMethodConfigResourceDisplayPreferencePreference,
    /// The effective display preference value.
    pub value: PaymentMethodConfigResourceDisplayPreferenceValue,
}
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
impl serde::Serialize for PaymentMethodConfigResourceDisplayPreferencePreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
impl serde::Serialize for PaymentMethodConfigResourceDisplayPreferenceValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
