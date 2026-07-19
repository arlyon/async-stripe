#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EphemeralKey {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which the key will expire. Measured in seconds since the Unix epoch.
    pub expires: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::EphemeralKeyId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The key's secret. You can use this value to make authorized requests to the Stripe API.
    pub secret: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EphemeralKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EphemeralKey").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct EphemeralKeyBuilder {
    created: Option<stripe_types::Timestamp>,
    expires: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::EphemeralKeyId>,
    livemode: Option<bool>,
    secret: Option<Option<String>>,
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

    impl Deserialize for EphemeralKey {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<EphemeralKey>,
        builder: EphemeralKeyBuilder,
    }

    impl Visitor for Place<EphemeralKey> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: EphemeralKeyBuilder {
                    created: Deserialize::default(),
                    expires: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    secret: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "expires" => Deserialize::begin(&mut self.builder.expires),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "secret" => Deserialize::begin(&mut self.builder.secret),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(created), Some(expires), Some(id), Some(livemode), Some(secret)) = (
                self.builder.created,
                self.builder.expires,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.secret.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(EphemeralKey { created, expires, id, livemode, secret });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for EphemeralKey {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("EphemeralKey", 6)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("expires", &self.expires)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("secret", &self.secret)?;

        s.serialize_field("object", "ephemeral_key")?;
        s.end()
    }
}
impl stripe_types::Object for EphemeralKey {
    type Id = stripe_misc::EphemeralKeyId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(EphemeralKeyId);
