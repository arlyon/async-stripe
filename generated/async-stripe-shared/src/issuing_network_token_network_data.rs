#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenNetworkData {
    pub device: Option<stripe_shared::IssuingNetworkTokenDevice>,
    pub mastercard: Option<stripe_shared::IssuingNetworkTokenMastercard>,
    /// The network that the token is associated with.
    /// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: IssuingNetworkTokenNetworkDataType,
    pub visa: Option<stripe_shared::IssuingNetworkTokenVisa>,
    pub wallet_provider: Option<stripe_shared::IssuingNetworkTokenWalletProvider>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingNetworkTokenNetworkData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingNetworkTokenNetworkData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingNetworkTokenNetworkDataBuilder {
    device: Option<Option<stripe_shared::IssuingNetworkTokenDevice>>,
    mastercard: Option<Option<stripe_shared::IssuingNetworkTokenMastercard>>,
    type_: Option<IssuingNetworkTokenNetworkDataType>,
    visa: Option<Option<stripe_shared::IssuingNetworkTokenVisa>>,
    wallet_provider: Option<Option<stripe_shared::IssuingNetworkTokenWalletProvider>>,
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

    impl Deserialize for IssuingNetworkTokenNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenNetworkData>,
        builder: IssuingNetworkTokenNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingNetworkTokenNetworkDataBuilder {
                    device: Deserialize::default(),
                    mastercard: Deserialize::default(),
                    type_: Deserialize::default(),
                    visa: Deserialize::default(),
                    wallet_provider: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "device" => Deserialize::begin(&mut self.builder.device),
                "mastercard" => Deserialize::begin(&mut self.builder.mastercard),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "visa" => Deserialize::begin(&mut self.builder.visa),
                "wallet_provider" => Deserialize::begin(&mut self.builder.wallet_provider),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(device), Some(mastercard), Some(type_), Some(visa), Some(wallet_provider)) = (
                self.builder.device.take(),
                self.builder.mastercard.take(),
                self.builder.type_.take(),
                self.builder.visa.take(),
                self.builder.wallet_provider.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(IssuingNetworkTokenNetworkData {
                device,
                mastercard,
                type_,
                visa,
                wallet_provider,
            });
            Ok(())
        }
    }
};
/// The network that the token is associated with.
/// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingNetworkTokenNetworkDataType {
    Mastercard,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingNetworkTokenNetworkDataType {
    pub fn as_str(&self) -> &str {
        use IssuingNetworkTokenNetworkDataType::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenNetworkDataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenNetworkDataType::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingNetworkTokenNetworkDataType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingNetworkTokenNetworkDataType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingNetworkTokenNetworkDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingNetworkTokenNetworkDataType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingNetworkTokenNetworkDataType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingNetworkTokenNetworkDataType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenNetworkDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
