/// To share the contents of a `File` object with non-Stripe users, you can
/// create a `FileLink`. `FileLink`s contain a URL that you can use to
/// retrieve the contents of the file without authentication.
///
/// For more details see <<https://stripe.com/docs/api/file_links/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FileLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FileLink").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FileLinkBuilder {
                    created: Deserialize::default(),
                    expired: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    file: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "expired" => Deserialize::begin(&mut self.builder.expired),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "file" => Deserialize::begin(&mut self.builder.file),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.created,
                self.builder.expired,
                self.builder.expires_at,
                self.builder.file.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.url.take(),
            )
            else {
                return Ok(());
            };
            *self.out =
                Some(FileLink { created, expired, expires_at, file, id, livemode, metadata, url });
            Ok(())
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
