#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarListDeletedList {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_list_list::RadarValueListId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: RadarListDeletedListObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RadarListDeletedListObject {
    RadarValueList,
}

impl RadarListDeletedListObject {
    pub fn as_str(self) -> &'static str {
        use RadarListDeletedListObject::*;
        match self {
            RadarValueList => "radar.value_list",
        }
    }
}

impl std::str::FromStr for RadarListDeletedListObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RadarListDeletedListObject::*;
        match s {
            "radar.value_list" => Ok(RadarValueList),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for RadarListDeletedListObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RadarListDeletedListObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RadarListDeletedListObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RadarListDeletedListObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RadarListDeletedListObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for RadarListDeletedListObject"))
    }
}
impl stripe_types::Object for RadarListDeletedList {
    type Id = stripe_fraud::radar_list_list::RadarValueListId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
