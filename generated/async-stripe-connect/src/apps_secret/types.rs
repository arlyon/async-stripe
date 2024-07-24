/// Secret Store is an API that allows Stripe Apps developers to securely persist secrets for use by UI Extensions and app backends.
///
/// The primary resource in Secret Store is a `secret`.
/// Other apps can't view secrets created by an app.
/// Additionally, secrets are scoped to provide further permission control.
///
/// All Dashboard users and the app backend share `account` scoped secrets.
/// Use the `account` scope for secrets that don't change per-user, like a third-party API key.
///
/// A `user` scoped secret is accessible by the app backend and one specific Dashboard user.
/// Use the `user` scope for per-user secrets like per-user OAuth tokens, where different users might have different permissions.
///
/// Related guide: [Store data between page reloads](https://stripe.com/docs/stripe-apps/store-auth-data-custom-objects).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AppsSecret {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If true, indicates that this secret has been deleted
    pub deleted: Option<bool>,
    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_connect::AppsSecretId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A name for the secret that's unique within the scope.
    pub name: String,
    /// The plaintext secret value to be stored.
    pub payload: Option<String>,
    pub scope: stripe_connect::SecretServiceResourceScope,
}
#[doc(hidden)]
pub struct AppsSecretBuilder {
    created: Option<stripe_types::Timestamp>,
    deleted: Option<Option<bool>>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_connect::AppsSecretId>,
    livemode: Option<bool>,
    name: Option<String>,
    payload: Option<Option<String>>,
    scope: Option<stripe_connect::SecretServiceResourceScope>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AppsSecret {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AppsSecret>,
        builder: AppsSecretBuilder,
    }

    impl Visitor for Place<AppsSecret> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AppsSecretBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AppsSecretBuilder {
        type Out = AppsSecret;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "deleted" => Deserialize::begin(&mut self.deleted),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "name" => Deserialize::begin(&mut self.name),
                "payload" => Deserialize::begin(&mut self.payload),
                "scope" => Deserialize::begin(&mut self.scope),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                deleted: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                name: Deserialize::default(),
                payload: Deserialize::default(),
                scope: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                created: self.created?,
                deleted: self.deleted?,
                expires_at: self.expires_at?,
                id: self.id.take()?,
                livemode: self.livemode?,
                name: self.name.take()?,
                payload: self.payload.take()?,
                scope: self.scope.take()?,
            })
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

    impl ObjectDeser for AppsSecret {
        type Builder = AppsSecretBuilder;
    }

    impl FromValueOpt for AppsSecret {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AppsSecretBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "deleted" => b.deleted = Some(FromValueOpt::from_value(v)?),
                    "expires_at" => b.expires_at = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "name" => b.name = Some(FromValueOpt::from_value(v)?),
                    "payload" => b.payload = Some(FromValueOpt::from_value(v)?),
                    "scope" => b.scope = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for AppsSecret {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("AppsSecret", 9)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("deleted", &self.deleted)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("payload", &self.payload)?;
        s.serialize_field("scope", &self.scope)?;

        s.serialize_field("object", "apps.secret")?;
        s.end()
    }
}
impl stripe_types::Object for AppsSecret {
    type Id = stripe_connect::AppsSecretId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(AppsSecretId);
