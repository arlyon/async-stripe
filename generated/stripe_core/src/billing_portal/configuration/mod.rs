/// A portal configuration describes the functionality and behavior of a portal session.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Configuration {
    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,
    /// ID of the Connect Application that created the configuration.
    pub application: Option<stripe_types::Expandable<stripe_types::application::Application>>,
    pub business_profile:
        stripe_core::billing_portal::configuration::business_profile::BusinessProfile,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub default_return_url: Option<String>,
    pub features: stripe_core::billing_portal::configuration::features::Features,
    /// Unique identifier for the object.
    pub id: stripe_core::billing_portal::configuration::BillingPortalConfigurationId,
    /// Whether the configuration is the default.
    ///
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub login_page: stripe_core::billing_portal::configuration::login_page::LoginPage,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ConfigurationObject,
    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Configuration {
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
pub enum ConfigurationObject {
    #[serde(rename = "billing_portal.configuration")]
    BillingPortalConfiguration,
}

impl ConfigurationObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BillingPortalConfiguration => "billing_portal.configuration",
        }
    }
}

impl AsRef<str> for ConfigurationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfigurationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Configuration {
    type Id = stripe_core::billing_portal::configuration::BillingPortalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(BillingPortalConfigurationId, "bpc_");
pub mod business_profile;
pub mod requests;
pub use business_profile::BusinessProfile;
pub mod features;
pub use features::Features;
pub mod login_page;
pub use login_page::LoginPage;
