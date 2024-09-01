#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EphemeralKey {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which the key will expire. Measured in seconds since the Unix epoch.
    pub expires: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::EphemeralKeyId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The key's secret. You can use this value to make authorized requests to the Stripe API.
    pub secret: Option<String>,
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
                builder: EphemeralKeyBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for EphemeralKeyBuilder {
        type Out = EphemeralKey;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "expires" => Deserialize::begin(&mut self.expires),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "secret" => Deserialize::begin(&mut self.secret),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                expires: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                secret: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(created), Some(expires), Some(id), Some(livemode), Some(secret)) =
                (self.created, self.expires, self.id.take(), self.livemode, self.secret.take())
            else {
                return None;
            };
            Some(Self::Out { created, expires, id, livemode, secret })
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

    impl ObjectDeser for EphemeralKey {
        type Builder = EphemeralKeyBuilder;
    }

    impl FromValueOpt for EphemeralKey {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = EphemeralKeyBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "expires" => b.expires = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "secret" => b.secret = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
