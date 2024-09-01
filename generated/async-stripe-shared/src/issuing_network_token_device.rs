#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenDevice {
    /// An obfuscated ID derived from the device ID.
    pub device_fingerprint: Option<String>,
    /// The IP address of the device at provisioning time.
    pub ip_address: Option<String>,
    /// The geographic latitude/longitude coordinates of the device at provisioning time.
    /// The format is [+-]decimal/[+-]decimal.
    pub location: Option<String>,
    /// The name of the device used for tokenization.
    pub name: Option<String>,
    /// The phone number of the device used for tokenization.
    pub phone_number: Option<String>,
    /// The type of device used for tokenization.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<IssuingNetworkTokenDeviceType>,
}
#[doc(hidden)]
pub struct IssuingNetworkTokenDeviceBuilder {
    device_fingerprint: Option<Option<String>>,
    ip_address: Option<Option<String>>,
    location: Option<Option<String>>,
    name: Option<Option<String>>,
    phone_number: Option<Option<String>>,
    type_: Option<Option<IssuingNetworkTokenDeviceType>>,
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

    impl Deserialize for IssuingNetworkTokenDevice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenDevice>,
        builder: IssuingNetworkTokenDeviceBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenDevice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingNetworkTokenDeviceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenDeviceBuilder {
        type Out = IssuingNetworkTokenDevice;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "device_fingerprint" => Deserialize::begin(&mut self.device_fingerprint),
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "location" => Deserialize::begin(&mut self.location),
                "name" => Deserialize::begin(&mut self.name),
                "phone_number" => Deserialize::begin(&mut self.phone_number),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                device_fingerprint: Deserialize::default(),
                ip_address: Deserialize::default(),
                location: Deserialize::default(),
                name: Deserialize::default(),
                phone_number: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(device_fingerprint),
                Some(ip_address),
                Some(location),
                Some(name),
                Some(phone_number),
                Some(type_),
            ) = (
                self.device_fingerprint.take(),
                self.ip_address.take(),
                self.location.take(),
                self.name.take(),
                self.phone_number.take(),
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out { device_fingerprint, ip_address, location, name, phone_number, type_ })
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

    impl ObjectDeser for IssuingNetworkTokenDevice {
        type Builder = IssuingNetworkTokenDeviceBuilder;
    }

    impl FromValueOpt for IssuingNetworkTokenDevice {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingNetworkTokenDeviceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "device_fingerprint" => b.device_fingerprint = FromValueOpt::from_value(v),
                    "ip_address" => b.ip_address = FromValueOpt::from_value(v),
                    "location" => b.location = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "phone_number" => b.phone_number = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of device used for tokenization.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenDeviceType {
    Other,
    Phone,
    Watch,
}
impl IssuingNetworkTokenDeviceType {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenDeviceType::*;
        match self {
            Other => "other",
            Phone => "phone",
            Watch => "watch",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenDeviceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenDeviceType::*;
        match s {
            "other" => Ok(Other),
            "phone" => Ok(Phone),
            "watch" => Ok(Watch),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingNetworkTokenDeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingNetworkTokenDeviceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingNetworkTokenDeviceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingNetworkTokenDeviceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingNetworkTokenDeviceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingNetworkTokenDeviceType")
        })
    }
}
