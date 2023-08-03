#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarListDeletedListItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_list_list_item::RadarValueListItemId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: RadarListDeletedListItemObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RadarListDeletedListItemObject {
    RadarValueListItem,
}

impl RadarListDeletedListItemObject {
    pub fn as_str(self) -> &'static str {
        use RadarListDeletedListItemObject::*;
        match self {
            RadarValueListItem => "radar.value_list_item",
        }
    }
}

impl std::str::FromStr for RadarListDeletedListItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RadarListDeletedListItemObject::*;
        match s {
            "radar.value_list_item" => Ok(RadarValueListItem),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for RadarListDeletedListItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RadarListDeletedListItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RadarListDeletedListItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RadarListDeletedListItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RadarListDeletedListItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for RadarListDeletedListItemObject"))
    }
}
impl stripe_types::Object for RadarListDeletedListItem {
    type Id = stripe_fraud::radar_list_list_item::RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
