#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedProduct {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::product::ProductId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedProductObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedProductObject {
    Product,
}

impl DeletedProductObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Product => "product",
        }
    }
}

impl std::str::FromStr for DeletedProductObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "product" => Ok(Self::Product),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedProductObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedProductObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedProductObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedProductObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedProductObject"))
    }
}
impl stripe_types::Object for DeletedProduct {
    type Id = stripe_types::product::ProductId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
