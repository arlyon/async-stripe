#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Rule {
    /// The action taken on the payment.
    pub action: String,
    /// Unique identifier for the object.
    pub id: stripe_types::radar::rule::RuleId,
    /// The predicate to evaluate the payment against.
    pub predicate: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Rule {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

impl stripe_types::Object for Rule {
    type Id = stripe_types::radar::rule::RuleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RuleId);
