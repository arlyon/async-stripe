/// You can configure [webhook endpoints](https://docs.stripe.com/webhooks/) via the API to be
/// notified about events that happen in your Stripe account or connected
/// accounts.
///
/// Most users configure webhooks from [the dashboard](https://dashboard.stripe.com/webhooks), which provides a user interface for registering and testing your webhook endpoints.
///
/// Related guide: [Setting up webhooks](https://docs.stripe.com/webhooks/configure)
///
/// For more details see <<https://stripe.com/docs/api/webhook_endpoints/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct WebhookEndpoint {
    /// The API version events are rendered as for this webhook endpoint.
    pub api_version: Option<String>,
    /// The ID of the associated Connect application.
    pub application: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// An optional description of what the webhook is used for.
    pub description: Option<String>,
    /// The list of events to enable for this endpoint.
    /// `['*']` indicates that all events are enabled, except those that require explicit selection.
    pub enabled_events: Vec<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::WebhookEndpointId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The endpoint's secret, used to generate [webhook signatures](https://docs.stripe.com/webhooks/signatures).
    /// Only returned at creation.
    pub secret: Option<String>,
    /// The status of the webhook. It can be `enabled` or `disabled`.
    pub status: String,
    /// The URL of the webhook endpoint.
    pub url: String,
}
#[doc(hidden)]
pub struct WebhookEndpointBuilder {
    api_version: Option<Option<String>>,
    application: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    description: Option<Option<String>>,
    enabled_events: Option<Vec<String>>,
    id: Option<stripe_misc::WebhookEndpointId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    secret: Option<Option<String>>,
    status: Option<String>,
    url: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for WebhookEndpoint {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<WebhookEndpoint>,
        builder: WebhookEndpointBuilder,
    }

    impl Visitor for Place<WebhookEndpoint> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: WebhookEndpointBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for WebhookEndpointBuilder {
        type Out = WebhookEndpoint;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "api_version" => Deserialize::begin(&mut self.api_version),
                "application" => Deserialize::begin(&mut self.application),
                "created" => Deserialize::begin(&mut self.created),
                "description" => Deserialize::begin(&mut self.description),
                "enabled_events" => Deserialize::begin(&mut self.enabled_events),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "secret" => Deserialize::begin(&mut self.secret),
                "status" => Deserialize::begin(&mut self.status),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                api_version: Deserialize::default(),
                application: Deserialize::default(),
                created: Deserialize::default(),
                description: Deserialize::default(),
                enabled_events: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                secret: Deserialize::default(),
                status: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(api_version),
                Some(application),
                Some(created),
                Some(description),
                Some(enabled_events),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(secret),
                Some(status),
                Some(url),
            ) = (
                self.api_version.take(),
                self.application.take(),
                self.created,
                self.description.take(),
                self.enabled_events.take(),
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.secret.take(),
                self.status.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                api_version,
                application,
                created,
                description,
                enabled_events,
                id,
                livemode,
                metadata,
                secret,
                status,
                url,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for WebhookEndpoint {
        type Builder = WebhookEndpointBuilder;
    }

    impl FromValueOpt for WebhookEndpoint {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = WebhookEndpointBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "api_version" => b.api_version = FromValueOpt::from_value(v),
                    "application" => b.application = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "enabled_events" => b.enabled_events = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "secret" => b.secret = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for WebhookEndpoint {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("WebhookEndpoint", 12)?;
        s.serialize_field("api_version", &self.api_version)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("enabled_events", &self.enabled_events)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("secret", &self.secret)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "webhook_endpoint")?;
        s.end()
    }
}
impl stripe_types::Object for WebhookEndpoint {
    type Id = stripe_misc::WebhookEndpointId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(WebhookEndpointId);
