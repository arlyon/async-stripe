#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
}
impl stripe_types::Object for Application {
    type Id = stripe_shared::ApplicationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplicationId);
