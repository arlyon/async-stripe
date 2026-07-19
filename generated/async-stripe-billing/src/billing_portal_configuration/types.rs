/// A portal configuration describes the functionality and behavior you embed in a portal session.
/// Related guide: [Configure the customer portal](/customer-management/configure-portal).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingPortalConfiguration {
    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,
    /// ID of the Connect Application that created the configuration.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    pub business_profile: stripe_billing::PortalBusinessProfile,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://docs.stripe.com/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub default_return_url: Option<String>,
    pub features: stripe_billing::PortalFeatures,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingPortalConfigurationId,
    /// Whether the configuration is the default.
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub login_page: stripe_billing::PortalLoginPage,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The name of the configuration.
    pub name: Option<String>,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingPortalConfiguration").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingPortalConfigurationBuilder {
    active: Option<bool>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    business_profile: Option<stripe_billing::PortalBusinessProfile>,
    created: Option<stripe_types::Timestamp>,
    default_return_url: Option<Option<String>>,
    features: Option<stripe_billing::PortalFeatures>,
    id: Option<stripe_billing::BillingPortalConfigurationId>,
    is_default: Option<bool>,
    livemode: Option<bool>,
    login_page: Option<stripe_billing::PortalLoginPage>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    name: Option<Option<String>>,
    updated: Option<stripe_types::Timestamp>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for BillingPortalConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingPortalConfiguration>,
        builder: BillingPortalConfigurationBuilder,
    }

    impl Visitor for Place<BillingPortalConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingPortalConfigurationBuilder {
                    active: Deserialize::default(),
                    application: Deserialize::default(),
                    business_profile: Deserialize::default(),
                    created: Deserialize::default(),
                    default_return_url: Deserialize::default(),
                    features: Deserialize::default(),
                    id: Deserialize::default(),
                    is_default: Deserialize::default(),
                    livemode: Deserialize::default(),
                    login_page: Deserialize::default(),
                    metadata: Deserialize::default(),
                    name: Deserialize::default(),
                    updated: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.builder.active),
                "application" => Deserialize::begin(&mut self.builder.application),
                "business_profile" => Deserialize::begin(&mut self.builder.business_profile),
                "created" => Deserialize::begin(&mut self.builder.created),
                "default_return_url" => Deserialize::begin(&mut self.builder.default_return_url),
                "features" => Deserialize::begin(&mut self.builder.features),
                "id" => Deserialize::begin(&mut self.builder.id),
                "is_default" => Deserialize::begin(&mut self.builder.is_default),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "login_page" => Deserialize::begin(&mut self.builder.login_page),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "name" => Deserialize::begin(&mut self.builder.name),
                "updated" => Deserialize::begin(&mut self.builder.updated),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(active),
                Some(application),
                Some(business_profile),
                Some(created),
                Some(default_return_url),
                Some(features),
                Some(id),
                Some(is_default),
                Some(livemode),
                Some(login_page),
                Some(metadata),
                Some(name),
                Some(updated),
            ) = (
                self.builder.active,
                self.builder.application.take(),
                self.builder.business_profile.take(),
                self.builder.created,
                self.builder.default_return_url.take(),
                self.builder.features.take(),
                self.builder.id.take(),
                self.builder.is_default,
                self.builder.livemode,
                self.builder.login_page.take(),
                self.builder.metadata.take(),
                self.builder.name.take(),
                self.builder.updated,
            )
            else {
                return Ok(());
            };
            *self.out = Some(BillingPortalConfiguration {
                active,
                application,
                business_profile,
                created,
                default_return_url,
                features,
                id,
                is_default,
                livemode,
                login_page,
                metadata,
                name,
                updated,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingPortalConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingPortalConfiguration", 14)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("business_profile", &self.business_profile)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("default_return_url", &self.default_return_url)?;
        s.serialize_field("features", &self.features)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("is_default", &self.is_default)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("login_page", &self.login_page)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("updated", &self.updated)?;

        s.serialize_field("object", "billing_portal.configuration")?;
        s.end()
    }
}
impl stripe_types::Object for BillingPortalConfiguration {
    type Id = stripe_billing::BillingPortalConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingPortalConfigurationId);
