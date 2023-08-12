/// A portal configuration describes the functionality and behavior of a portal session.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalConfiguration {
    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,
    /// ID of the Connect Application that created the configuration.
    pub application: Option<stripe_types::Expandable<stripe_types::Application>>,
    pub business_profile: stripe_billing::PortalBusinessProfile,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub default_return_url: Option<String>,
    pub features: stripe_billing::PortalFeatures,
    /// Unique identifier for the object.
    pub id: stripe_billing::portal_configuration::BillingPortalConfigurationId,
    /// Whether the configuration is the default.
    ///
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub login_page: stripe_billing::PortalLoginPage,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
}
impl stripe_types::Object for PortalConfiguration {
    type Id = stripe_billing::portal_configuration::BillingPortalConfigurationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(BillingPortalConfigurationId, "bpc_");
#[cfg(feature = "portal_configuration")]
mod requests;
#[cfg(feature = "portal_configuration")]
pub use requests::*;
