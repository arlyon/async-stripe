/// A portal configuration describes the functionality and behavior of a portal session.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BillingPortalConfiguration {
    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,
    /// ID of the Connect Application that created the configuration.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    pub business_profile: stripe_billing::PortalBusinessProfile,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub default_return_url: Option<String>,
    pub features: stripe_billing::PortalFeatures,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingPortalConfigurationId,
    /// Whether the configuration is the default.
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub login_page: stripe_billing::PortalLoginPage,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
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
    updated: Option<stripe_types::Timestamp>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: BillingPortalConfigurationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BillingPortalConfigurationBuilder {
        type Out = BillingPortalConfiguration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "application" => Deserialize::begin(&mut self.application),
                "business_profile" => Deserialize::begin(&mut self.business_profile),
                "created" => Deserialize::begin(&mut self.created),
                "default_return_url" => Deserialize::begin(&mut self.default_return_url),
                "features" => Deserialize::begin(&mut self.features),
                "id" => Deserialize::begin(&mut self.id),
                "is_default" => Deserialize::begin(&mut self.is_default),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "login_page" => Deserialize::begin(&mut self.login_page),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "updated" => Deserialize::begin(&mut self.updated),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
                updated: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active = self.active.take()?;
            let application = self.application.take()?;
            let business_profile = self.business_profile.take()?;
            let created = self.created.take()?;
            let default_return_url = self.default_return_url.take()?;
            let features = self.features.take()?;
            let id = self.id.take()?;
            let is_default = self.is_default.take()?;
            let livemode = self.livemode.take()?;
            let login_page = self.login_page.take()?;
            let metadata = self.metadata.take()?;
            let updated = self.updated.take()?;

            Some(Self::Out { active, application, business_profile, created, default_return_url, features, id, is_default, livemode, login_page, metadata, updated })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingPortalConfiguration {
        type Builder = BillingPortalConfigurationBuilder;
    }

    impl FromValueOpt for BillingPortalConfiguration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingPortalConfigurationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = Some(FromValueOpt::from_value(v)?),
                    "application" => b.application = Some(FromValueOpt::from_value(v)?),
                    "business_profile" => b.business_profile = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "default_return_url" => b.default_return_url = Some(FromValueOpt::from_value(v)?),
                    "features" => b.features = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "is_default" => b.is_default = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "login_page" => b.login_page = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "updated" => b.updated = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for BillingPortalConfiguration {
    type Id = stripe_billing::BillingPortalConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(BillingPortalConfigurationId, "bpc_");
