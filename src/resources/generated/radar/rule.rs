#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Rule {
    /// The action taken on the payment.
    pub action: String,
    /// Unique identifier for the object.
    pub id: crate::radar::rule::RuleId,
    /// The predicate to evaluate the payment against.
    pub predicate: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Rule {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

impl crate::Object for Rule {
    type Id = crate::radar::rule::RuleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(RuleId);
