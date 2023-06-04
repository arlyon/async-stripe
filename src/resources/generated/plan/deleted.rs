#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedPlan {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: crate::plan::PlanId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedPlanObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedPlan {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for DeletedPlan {
    type Id = crate::plan::PlanId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
