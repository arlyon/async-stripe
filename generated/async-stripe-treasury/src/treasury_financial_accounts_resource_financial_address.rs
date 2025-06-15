/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    pub aba: Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaRecord>,
    /// The list of networks that the address supports
    pub supported_networks:
        Option<Vec<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks>>,
    /// The type of financial address
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TreasuryFinancialAccountsResourceFinancialAddressType,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressBuilder {
    aba: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaRecord>>,
    supported_networks:
        Option<Option<Vec<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks>>>,
    type_: Option<TreasuryFinancialAccountsResourceFinancialAddressType>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceFinancialAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceFinancialAddress>,
        builder: TreasuryFinancialAccountsResourceFinancialAddressBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceFinancialAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceFinancialAddressBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceFinancialAddressBuilder {
        type Out = TreasuryFinancialAccountsResourceFinancialAddress;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aba" => Deserialize::begin(&mut self.aba),
                "supported_networks" => Deserialize::begin(&mut self.supported_networks),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                aba: Deserialize::default(),
                supported_networks: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(aba), Some(supported_networks), Some(type_)) =
                (self.aba.take(), self.supported_networks.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { aba, supported_networks, type_ })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceFinancialAddress {
        type Builder = TreasuryFinancialAccountsResourceFinancialAddressBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceFinancialAddress {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceFinancialAddressBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "aba" => b.aba = FromValueOpt::from_value(v),
                    "supported_networks" => b.supported_networks = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The list of networks that the address supports
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    Ach,
    UsDomesticWire,
}
impl TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks"))
    }
}
/// The type of financial address
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceFinancialAddressType {
    Aba,
}
impl TreasuryFinancialAccountsResourceFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceFinancialAddressType::*;
        match self {
            Aba => "aba",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceFinancialAddressType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceFinancialAddressType::*;
        match s {
            "aba" => Ok(Aba),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceFinancialAddressType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceFinancialAddressType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryFinancialAccountsResourceFinancialAddressType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryFinancialAccountsResourceFinancialAddressType",
            )
        })
    }
}
