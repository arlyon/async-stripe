#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: stripe_types::application::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
}
impl stripe_types::Object for Application {
    type Id = stripe_types::application::ApplicationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(ApplicationId, "ca_");
