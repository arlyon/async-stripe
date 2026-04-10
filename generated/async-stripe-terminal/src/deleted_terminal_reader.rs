#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DeletedTerminalReader {
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Device type of the reader.
    pub device_type: DeletedTerminalReaderDeviceType,
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
    device_type: Option<DeletedTerminalReaderDeviceType>,
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
            Self {
                deleted: Deserialize::default(),
                device_type: Deserialize::default(),
                id: Deserialize::default(),
                serial_number: Deserialize::default(),
            }
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
            *self.out = self.builder.take_out();
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
/// Device type of the reader.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DeletedTerminalReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    MobilePhoneReader,
    SimulatedStripeS700,
    SimulatedStripeS710,
    SimulatedWiseposE,
    StripeM2,
    StripeS700,
    StripeS710,
    VerifoneP400,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DeletedTerminalReaderDeviceType {
    pub fn as_str(&self) -> &str {
        use DeletedTerminalReaderDeviceType::*;
        match self {
            BbposChipper2x => "bbpos_chipper2x",
            BbposWisepad3 => "bbpos_wisepad3",
            BbposWiseposE => "bbpos_wisepos_e",
            MobilePhoneReader => "mobile_phone_reader",
            SimulatedStripeS700 => "simulated_stripe_s700",
            SimulatedStripeS710 => "simulated_stripe_s710",
            SimulatedWiseposE => "simulated_wisepos_e",
            StripeM2 => "stripe_m2",
            StripeS700 => "stripe_s700",
            StripeS710 => "stripe_s710",
            VerifoneP400 => "verifone_P400",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DeletedTerminalReaderDeviceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedTerminalReaderDeviceType::*;
        match s {
            "bbpos_chipper2x" => Ok(BbposChipper2x),
            "bbpos_wisepad3" => Ok(BbposWisepad3),
            "bbpos_wisepos_e" => Ok(BbposWiseposE),
            "mobile_phone_reader" => Ok(MobilePhoneReader),
            "simulated_stripe_s700" => Ok(SimulatedStripeS700),
            "simulated_stripe_s710" => Ok(SimulatedStripeS710),
            "simulated_wisepos_e" => Ok(SimulatedWiseposE),
            "stripe_m2" => Ok(StripeM2),
            "stripe_s700" => Ok(StripeS700),
            "stripe_s710" => Ok(StripeS710),
            "verifone_P400" => Ok(VerifoneP400),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DeletedTerminalReaderDeviceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DeletedTerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DeletedTerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeletedTerminalReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DeletedTerminalReaderDeviceType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DeletedTerminalReaderDeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DeletedTerminalReaderDeviceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DeletedTerminalReaderDeviceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedTerminalReaderDeviceType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DeletedTerminalReaderDeviceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DeletedTerminalReaderDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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
