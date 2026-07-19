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
/// Related guide: [Store data between page reloads](https://docs.stripe.com/stripe-apps/store-auth-data-custom-objects).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// A name for the secret that's unique within the scope.
    pub name: String,
    /// The plaintext secret value to be stored.
    pub payload: Option<String>,
    pub scope: stripe_connect::SecretServiceResourceScope,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AppsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AppsSecret").finish_non_exhaustive()
    }
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
                builder: AppsSecretBuilder {
                    created: Deserialize::default(),
                    deleted: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    name: Deserialize::default(),
                    payload: Deserialize::default(),
                    scope: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "deleted" => Deserialize::begin(&mut self.builder.deleted),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "name" => Deserialize::begin(&mut self.builder.name),
                "payload" => Deserialize::begin(&mut self.builder.payload),
                "scope" => Deserialize::begin(&mut self.builder.scope),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(deleted),
                Some(expires_at),
                Some(id),
                Some(livemode),
                Some(name),
                Some(payload),
                Some(scope),
            ) = (
                self.builder.created,
                self.builder.deleted,
                self.builder.expires_at,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.name.take(),
                self.builder.payload.take(),
                self.builder.scope.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(AppsSecret {
                created,
                deleted,
                expires_at,
                id,
                livemode,
                name,
                payload,
                scope,
            });
            Ok(())
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
