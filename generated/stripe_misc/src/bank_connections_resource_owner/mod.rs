/// Describes an owner of an account.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceOwner {
    /// The email address of the owner.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::bank_connections_resource_owner::FinancialConnectionsAccountOwnerId,
    /// The full name of the owner.
    pub name: String,
    /// The ownership object that this owner belongs to.
    pub ownership: String,
    /// The raw phone number of the owner.
    pub phone: Option<String>,
    /// The raw physical address of the owner.
    pub raw_address: Option<String>,
    /// The timestamp of the refresh that updated this owner.
    pub refreshed_at: Option<stripe_types::Timestamp>,
}
impl stripe_types::Object for BankConnectionsResourceOwner {
    type Id = stripe_misc::bank_connections_resource_owner::FinancialConnectionsAccountOwnerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnerId);
