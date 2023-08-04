/// Describes a snapshot of the owners of an account at a particular point in time.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceOwnership {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id:
        stripe_misc::bank_connections_resource_ownership::FinancialConnectionsAccountOwnershipId,
    /// A paginated list of owners for this account.
    pub owners: stripe_types::List<stripe_misc::BankConnectionsResourceOwner>,
}
impl stripe_types::Object for BankConnectionsResourceOwnership {
    type Id =
        stripe_misc::bank_connections_resource_ownership::FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsAccountOwnershipId);
