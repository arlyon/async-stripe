#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedPlan {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::plan::PlanId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedPlanObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedPlanObject {
    Plan,
}

impl DeletedPlanObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plan => "plan",
        }
    }
}

impl std::str::FromStr for DeletedPlanObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plan" => Ok(Self::Plan),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedPlanObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedPlanObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedPlanObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedPlanObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedPlanObject"))
    }
}
impl stripe_types::Object for DeletedPlan {
    type Id = stripe_types::plan::PlanId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
