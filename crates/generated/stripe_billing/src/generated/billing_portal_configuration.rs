// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_core::{
    ids::BillingPortalConfigurationId,
    params::{Expandable, Metadata, Object, Timestamp},
};
use serde::{Deserialize, Serialize};

use crate::resources::{Application, PortalFeatures};

/// The resource representing a Stripe "PortalConfiguration".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingPortalConfiguration {
    /// Unique identifier for the object.
    pub id: BillingPortalConfigurationId,

    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,

    /// ID of the Connect Application that created the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    pub business_profile: PortalBusinessProfile,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<String>,

    pub features: PortalFeatures,

    /// Whether the configuration is the default.
    ///
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: Timestamp,
}

impl Object for BillingPortalConfiguration {
    type Id = BillingPortalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing_portal.configuration"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalBusinessProfile {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,

    /// A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,

    /// A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
}
