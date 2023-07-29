#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedValueListItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::radar::value_list_item::RadarValueListItemId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedValueListItemObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedValueListItemObject {
    RadarValueListItem,
}

impl DeletedValueListItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RadarValueListItem => "radar.value_list_item",
        }
    }
}

impl std::str::FromStr for DeletedValueListItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "radar.value_list_item" => Ok(Self::RadarValueListItem),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedValueListItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedValueListItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedValueListItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedValueListItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedValueListItemObject"))
    }
}
impl stripe_types::Object for DeletedValueListItem {
    type Id = stripe_types::radar::value_list_item::RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
