#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingNetworkTokenDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingNetworkTokenDevice").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingNetworkTokenDeviceBuilder {
                    device_fingerprint: Deserialize::default(),
                    ip_address: Deserialize::default(),
                    location: Deserialize::default(),
                    name: Deserialize::default(),
                    phone_number: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "device_fingerprint" => Deserialize::begin(&mut self.builder.device_fingerprint),
                "ip_address" => Deserialize::begin(&mut self.builder.ip_address),
                "location" => Deserialize::begin(&mut self.builder.location),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone_number" => Deserialize::begin(&mut self.builder.phone_number),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(device_fingerprint),
                Some(ip_address),
                Some(location),
                Some(name),
                Some(phone_number),
                Some(type_),
            ) = (
                self.builder.device_fingerprint.take(),
                self.builder.ip_address.take(),
                self.builder.location.take(),
                self.builder.name.take(),
                self.builder.phone_number.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingNetworkTokenDevice {
                device_fingerprint,
                ip_address,
                location,
                name,
                phone_number,
                type_,
            });
            Ok(())
        }
    }
};
/// The type of device used for tokenization.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingNetworkTokenDeviceType {
    Other,
    Phone,
    Watch,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingNetworkTokenDeviceType {
    pub fn as_str(&self) -> &str {
        use IssuingNetworkTokenDeviceType::*;
        match self {
            Other => "other",
            Phone => "phone",
            Watch => "watch",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenDeviceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenDeviceType::*;
        match s {
            "other" => Ok(Other),
            "phone" => Ok(Phone),
            "watch" => Ok(Watch),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingNetworkTokenDeviceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingNetworkTokenDeviceType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingNetworkTokenDeviceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingNetworkTokenDeviceType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingNetworkTokenDeviceType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenDeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
