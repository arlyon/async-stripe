/// Metadata of an uploaded file
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceFileMetadata {
    /// Creation time of the object (in seconds since the Unix epoch).
    pub created_at: stripe_types::Timestamp,
    /// The original name of the uploaded file (e.g. `receipt.png`).
    pub filename: String,
    /// The size (in bytes) of the uploaded file.
    pub size: u64,
    /// The format of the uploaded file.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceFileMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceFileMetadata").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceFileMetadataBuilder {
    created_at: Option<stripe_types::Timestamp>,
    filename: Option<String>,
    size: Option<u64>,
    type_: Option<String>,
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

    impl Deserialize for TerminalReaderReaderResourceFileMetadata {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceFileMetadata>,
        builder: TerminalReaderReaderResourceFileMetadataBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceFileMetadata> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceFileMetadataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceFileMetadataBuilder {
        type Out = TerminalReaderReaderResourceFileMetadata;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created_at" => Deserialize::begin(&mut self.created_at),
                "filename" => Deserialize::begin(&mut self.filename),
                "size" => Deserialize::begin(&mut self.size),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { created_at: None, filename: None, size: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(created_at), Some(filename), Some(size), Some(type_)) =
                (self.created_at, self.filename.take(), self.size, self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { created_at, filename, size, type_ })
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

    impl ObjectDeser for TerminalReaderReaderResourceFileMetadata {
        type Builder = TerminalReaderReaderResourceFileMetadataBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceFileMetadata {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceFileMetadataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created_at" => b.created_at = FromValueOpt::from_value(v),
                    "filename" => b.filename = FromValueOpt::from_value(v),
                    "size" => b.size = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
