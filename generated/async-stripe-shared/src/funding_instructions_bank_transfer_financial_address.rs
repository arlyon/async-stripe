/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransferFinancialAddress {
    pub aba: Option<stripe_shared::FundingInstructionsBankTransferAbaRecord>,
    pub iban: Option<stripe_shared::FundingInstructionsBankTransferIbanRecord>,
    pub sort_code: Option<stripe_shared::FundingInstructionsBankTransferSortCodeRecord>,
    pub spei: Option<stripe_shared::FundingInstructionsBankTransferSpeiRecord>,
    /// The payment networks supported by this FinancialAddress
    pub supported_networks:
        Option<Vec<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>>,
    pub swift: Option<stripe_shared::FundingInstructionsBankTransferSwiftRecord>,
    /// The type of financial address
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: FundingInstructionsBankTransferFinancialAddressType,
    pub zengin: Option<stripe_shared::FundingInstructionsBankTransferZenginRecord>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FundingInstructionsBankTransferFinancialAddress").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferFinancialAddressBuilder {
    aba: Option<Option<stripe_shared::FundingInstructionsBankTransferAbaRecord>>,
    iban: Option<Option<stripe_shared::FundingInstructionsBankTransferIbanRecord>>,
    sort_code: Option<Option<stripe_shared::FundingInstructionsBankTransferSortCodeRecord>>,
    spei: Option<Option<stripe_shared::FundingInstructionsBankTransferSpeiRecord>>,
    supported_networks:
        Option<Option<Vec<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>>>,
    swift: Option<Option<stripe_shared::FundingInstructionsBankTransferSwiftRecord>>,
    type_: Option<FundingInstructionsBankTransferFinancialAddressType>,
    zengin: Option<Option<stripe_shared::FundingInstructionsBankTransferZenginRecord>>,
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

    impl Deserialize for FundingInstructionsBankTransferFinancialAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransferFinancialAddress>,
        builder: FundingInstructionsBankTransferFinancialAddressBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransferFinancialAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferFinancialAddressBuilder {
                    aba: Deserialize::default(),
                    iban: Deserialize::default(),
                    sort_code: Deserialize::default(),
                    spei: Deserialize::default(),
                    supported_networks: Deserialize::default(),
                    swift: Deserialize::default(),
                    type_: Deserialize::default(),
                    zengin: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aba" => Deserialize::begin(&mut self.builder.aba),
                "iban" => Deserialize::begin(&mut self.builder.iban),
                "sort_code" => Deserialize::begin(&mut self.builder.sort_code),
                "spei" => Deserialize::begin(&mut self.builder.spei),
                "supported_networks" => Deserialize::begin(&mut self.builder.supported_networks),
                "swift" => Deserialize::begin(&mut self.builder.swift),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "zengin" => Deserialize::begin(&mut self.builder.zengin),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(aba),
                Some(iban),
                Some(sort_code),
                Some(spei),
                Some(supported_networks),
                Some(swift),
                Some(type_),
                Some(zengin),
            ) = (
                self.builder.aba.take(),
                self.builder.iban.take(),
                self.builder.sort_code.take(),
                self.builder.spei.take(),
                self.builder.supported_networks.take(),
                self.builder.swift.take(),
                self.builder.type_.take(),
                self.builder.zengin.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(FundingInstructionsBankTransferFinancialAddress {
                aba,
                iban,
                sort_code,
                spei,
                supported_networks,
                swift,
                type_,
                zengin,
            });
            Ok(())
        }
    }
};
/// The payment networks supported by this FinancialAddress
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    Ach,
    Bacs,
    DomesticWireUs,
    Fps,
    Sepa,
    Spei,
    Swift,
    Zengin,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    pub fn as_str(&self) -> &str {
        use FundingInstructionsBankTransferFinancialAddressSupportedNetworks::*;
        match self {
            Ach => "ach",
            Bacs => "bacs",
            DomesticWireUs => "domestic_wire_us",
            Fps => "fps",
            Sepa => "sepa",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferFinancialAddressSupportedNetworks::*;
        match s {
            "ach" => Ok(Ach),
            "bacs" => Ok(Bacs),
            "domestic_wire_us" => Ok(DomesticWireUs),
            "fps" => Ok(Fps),
            "sepa" => Ok(Sepa),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FundingInstructionsBankTransferFinancialAddressSupportedNetworks"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FundingInstructionsBankTransferFinancialAddressSupportedNetworks))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for FundingInstructionsBankTransferFinancialAddressSupportedNetworks
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of financial address
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Aba,
    Iban,
    SortCode,
    Spei,
    Swift,
    Zengin,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FundingInstructionsBankTransferFinancialAddressType {
    pub fn as_str(&self) -> &str {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match self {
            Aba => "aba",
            Iban => "iban",
            SortCode => "sort_code",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match s {
            "aba" => Ok(Aba),
            "iban" => Ok(Iban),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FundingInstructionsBankTransferFinancialAddressType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FundingInstructionsBankTransferFinancialAddressType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FundingInstructionsBankTransferFinancialAddressType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for FundingInstructionsBankTransferFinancialAddressType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FundingInstructionsBankTransferFinancialAddressType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FundingInstructionsBankTransferFinancialAddressType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FundingInstructionsBankTransferFinancialAddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
