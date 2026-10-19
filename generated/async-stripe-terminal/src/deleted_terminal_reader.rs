#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DeletedTerminalReader {
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Device type of the reader.
    pub device_type: stripe_shared::DeviceType,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalReaderId,
    /// Serial number of the reader.
    pub serial_number: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeletedTerminalReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeletedTerminalReader").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DeletedTerminalReaderBuilder {
    deleted: Option<stripe_types::AlwaysTrue>,
    device_type: Option<stripe_shared::DeviceType>,
    id: Option<stripe_terminal::TerminalReaderId>,
    serial_number: Option<String>,
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

    impl Deserialize for DeletedTerminalReader {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedTerminalReader>,
        builder: DeletedTerminalReaderBuilder,
    }

    impl Visitor for Place<DeletedTerminalReader> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DeletedTerminalReaderBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DeletedTerminalReaderBuilder {
        type Out = DeletedTerminalReader;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "deleted" => Deserialize::begin(&mut self.deleted),
                "device_type" => Deserialize::begin(&mut self.device_type),
                "id" => Deserialize::begin(&mut self.id),
                "serial_number" => Deserialize::begin(&mut self.serial_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { deleted: None, device_type: None, id: None, serial_number: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(deleted), Some(device_type), Some(id), Some(serial_number)) =
                (self.deleted, self.device_type.take(), self.id.take(), self.serial_number.take())
            else {
                return None;
            };
            Some(Self::Out { deleted, device_type, id, serial_number })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for DeletedTerminalReader {
        type Builder = DeletedTerminalReaderBuilder;
    }

    impl FromValueOpt for DeletedTerminalReader {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DeletedTerminalReaderBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "deleted" => b.deleted = FromValueOpt::from_value(v),
                    "device_type" => b.device_type = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "serial_number" => b.serial_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for DeletedTerminalReader {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("DeletedTerminalReader", 5)?;
        s.serialize_field("deleted", &self.deleted)?;
        s.serialize_field("device_type", &self.device_type)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("serial_number", &self.serial_number)?;

        s.serialize_field("object", "terminal.reader")?;
        s.end()
    }
}
impl stripe_types::Object for DeletedTerminalReader {
    type Id = stripe_terminal::TerminalReaderId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
/// Device type of the reader.
pub type DeletedTerminalReaderDeviceType = stripe_shared::DeviceType;
