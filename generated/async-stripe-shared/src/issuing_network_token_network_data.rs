#[derive(Clone, Debug)]
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
    clippy::let_unit_value,
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
                builder: IssuingNetworkTokenNetworkDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenNetworkDataBuilder {
        type Out = IssuingNetworkTokenNetworkData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "device" => Deserialize::begin(&mut self.device),
                "mastercard" => Deserialize::begin(&mut self.mastercard),
                "type" => Deserialize::begin(&mut self.type_),
                "visa" => Deserialize::begin(&mut self.visa),
                "wallet_provider" => Deserialize::begin(&mut self.wallet_provider),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                device: Deserialize::default(),
                mastercard: Deserialize::default(),
                type_: Deserialize::default(),
                visa: Deserialize::default(),
                wallet_provider: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(device), Some(mastercard), Some(type_), Some(visa), Some(wallet_provider)) = (
                self.device.take(),
                self.mastercard.take(),
                self.type_,
                self.visa.take(),
                self.wallet_provider.take(),
            ) else {
                return None;
            };
            Some(Self::Out { device, mastercard, type_, visa, wallet_provider })
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

    impl ObjectDeser for IssuingNetworkTokenNetworkData {
        type Builder = IssuingNetworkTokenNetworkDataBuilder;
    }

    impl FromValueOpt for IssuingNetworkTokenNetworkData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingNetworkTokenNetworkDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "device" => b.device = FromValueOpt::from_value(v),
                    "mastercard" => b.mastercard = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "visa" => b.visa = FromValueOpt::from_value(v),
                    "wallet_provider" => b.wallet_provider = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The network that the token is associated with.
/// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenNetworkDataType {
    Mastercard,
    Visa,
}
impl IssuingNetworkTokenNetworkDataType {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenNetworkDataType::*;
        match self {
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenNetworkDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenNetworkDataType::*;
        match s {
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingNetworkTokenNetworkDataType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingNetworkTokenNetworkDataType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingNetworkTokenNetworkDataType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingNetworkTokenNetworkDataType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenNetworkDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingNetworkTokenNetworkDataType")
        })
    }
}
