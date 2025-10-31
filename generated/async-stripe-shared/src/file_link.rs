/// To share the contents of a `File` object with non-Stripe users, you can
/// create a `FileLink`. `FileLink`s contain a URL that you can use to
/// retrieve the contents of the file without authentication.
///
/// For more details see <<https://stripe.com/docs/api/file_links/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FileLink {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Returns if the link is already expired.
    pub expired: bool,
    /// Time that the link expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The file object this link points to.
    pub file: stripe_types::Expandable<stripe_shared::File>,
    /// Unique identifier for the object.
    pub id: stripe_shared::FileLinkId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}
#[doc(hidden)]
pub struct FileLinkBuilder {
    created: Option<stripe_types::Timestamp>,
    expired: Option<bool>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    file: Option<stripe_types::Expandable<stripe_shared::File>>,
    id: Option<stripe_shared::FileLinkId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    url: Option<Option<String>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FileLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FileLink>,
        builder: FileLinkBuilder,
    }

    impl Visitor for Place<FileLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: FileLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for FileLinkBuilder {
        type Out = FileLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "expired" => Deserialize::begin(&mut self.expired),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "file" => Deserialize::begin(&mut self.file),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                expired: Deserialize::default(),
                expires_at: Deserialize::default(),
                file: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(expired),
                Some(expires_at),
                Some(file),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(url),
            ) = (
                self.created,
                self.expired,
                self.expires_at,
                self.file.take(),
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { created, expired, expires_at, file, id, livemode, metadata, url })
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

    impl ObjectDeser for FileLink {
        type Builder = FileLinkBuilder;
    }

    impl FromValueOpt for FileLink {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FileLinkBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "expired" => b.expired = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "file" => b.file = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FileLink {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FileLink", 9)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("expired", &self.expired)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("file", &self.file)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "file_link")?;
        s.end()
    }
}
impl stripe_types::Object for FileLink {
    type Id = stripe_shared::FileLinkId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FileLinkId);
