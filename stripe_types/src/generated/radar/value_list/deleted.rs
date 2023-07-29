#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedValueList {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::radar::value_list::RadarValueListId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedValueListObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedValueListObject {
    RadarValueList,
}

impl DeletedValueListObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RadarValueList => "radar.value_list",
        }
    }
}

impl std::str::FromStr for DeletedValueListObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "radar.value_list" => Ok(Self::RadarValueList),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedValueListObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedValueListObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedValueListObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedValueListObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedValueListObject"))
    }
}
impl stripe_types::Object for DeletedValueList {
    type Id = stripe_types::radar::value_list::RadarValueListId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
