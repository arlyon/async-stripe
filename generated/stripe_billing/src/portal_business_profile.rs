#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PortalBusinessProfile {
    /// The messaging shown to customers in the portal.
    pub headline: Option<String>,
    /// A link to the business’s publicly available privacy policy.
    pub privacy_policy_url: Option<String>,
    /// A link to the business’s publicly available terms of service.
    pub terms_of_service_url: Option<String>,
}
