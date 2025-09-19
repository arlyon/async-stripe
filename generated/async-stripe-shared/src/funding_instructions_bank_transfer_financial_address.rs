/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug)]
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
                builder: FundingInstructionsBankTransferFinancialAddressBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferFinancialAddressBuilder {
        type Out = FundingInstructionsBankTransferFinancialAddress;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aba" => Deserialize::begin(&mut self.aba),
                "iban" => Deserialize::begin(&mut self.iban),
                "sort_code" => Deserialize::begin(&mut self.sort_code),
                "spei" => Deserialize::begin(&mut self.spei),
                "supported_networks" => Deserialize::begin(&mut self.supported_networks),
                "swift" => Deserialize::begin(&mut self.swift),
                "type" => Deserialize::begin(&mut self.type_),
                "zengin" => Deserialize::begin(&mut self.zengin),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                aba: Deserialize::default(),
                iban: Deserialize::default(),
                sort_code: Deserialize::default(),
                spei: Deserialize::default(),
                supported_networks: Deserialize::default(),
                swift: Deserialize::default(),
                type_: Deserialize::default(),
                zengin: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.aba.take(),
                self.iban.take(),
                self.sort_code.take(),
                self.spei.take(),
                self.supported_networks.take(),
                self.swift.take(),
                self.type_,
                self.zengin.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { aba, iban, sort_code, spei, supported_networks, swift, type_, zengin })
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

    impl ObjectDeser for FundingInstructionsBankTransferFinancialAddress {
        type Builder = FundingInstructionsBankTransferFinancialAddressBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransferFinancialAddress {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferFinancialAddressBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "aba" => b.aba = FromValueOpt::from_value(v),
                    "iban" => b.iban = FromValueOpt::from_value(v),
                    "sort_code" => b.sort_code = FromValueOpt::from_value(v),
                    "spei" => b.spei = FromValueOpt::from_value(v),
                    "supported_networks" => b.supported_networks = FromValueOpt::from_value(v),
                    "swift" => b.swift = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "zengin" => b.zengin = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The payment networks supported by this FinancialAddress
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    Ach,
    Bacs,
    DomesticWireUs,
    Fps,
    Sepa,
    Spei,
    Swift,
    Zengin,
}
impl FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
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
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    FundingInstructionsBankTransferFinancialAddressSupportedNetworks
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for FundingInstructionsBankTransferFinancialAddressSupportedNetworks
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for FundingInstructionsBankTransferFinancialAddressSupportedNetworks"))
    }
}
/// The type of financial address
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Aba,
    Iban,
    SortCode,
    Spei,
    Swift,
    Zengin,
}
impl FundingInstructionsBankTransferFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match self {
            Aba => "aba",
            Iban => "iban",
            SortCode => "sort_code",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferFinancialAddressType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferFinancialAddressType::*;
        match s {
            "aba" => Ok(Aba),
            "iban" => Ok(Iban),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for FundingInstructionsBankTransferFinancialAddressType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FundingInstructionsBankTransferFinancialAddressType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FundingInstructionsBankTransferFinancialAddressType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FundingInstructionsBankTransferFinancialAddressType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FundingInstructionsBankTransferFinancialAddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for FundingInstructionsBankTransferFinancialAddressType",
            )
        })
    }
}
