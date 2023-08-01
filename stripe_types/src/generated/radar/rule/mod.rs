#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Rule {
    /// The action taken on the payment.
    pub action: String,
    /// Unique identifier for the object.
    pub id: stripe_types::radar::rule::RuleId,
    /// The predicate to evaluate the payment against.
    pub predicate: String,
}
impl stripe_types::Object for Rule {
    type Id = stripe_types::radar::rule::RuleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RuleId);
