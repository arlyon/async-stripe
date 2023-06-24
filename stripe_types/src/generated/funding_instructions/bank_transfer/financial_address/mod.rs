/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban:
        Option<stripe_types::funding_instructions::bank_transfer::financial_address::iban::Iban>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<
        stripe_types::funding_instructions::bank_transfer::financial_address::sort_code::SortCode,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei:
        Option<stripe_types::funding_instructions::bank_transfer::financial_address::spei::Spei>,
    /// The payment networks supported by this FinancialAddress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<FinancialAddressSupportedNetworks>>,
    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: FinancialAddressType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<
        stripe_types::funding_instructions::bank_transfer::financial_address::zengin::Zengin,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAddress {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The payment networks supported by this FinancialAddress.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAddressSupportedNetworks {
    Bacs,
    Fps,
    Sepa,
    Spei,
    Zengin,
}

impl FinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bacs => "bacs",
            Self::Fps => "fps",
            Self::Sepa => "sepa",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FinancialAddressSupportedNetworks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bacs" => Ok(Self::Bacs),
            "fps" => Ok(Self::Fps),
            "sepa" => Ok(Self::Sepa),
            "spei" => Ok(Self::Spei),
            "zengin" => Ok(Self::Zengin),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FinancialAddressSupportedNetworks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAddressSupportedNetworks {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialAddressSupportedNetworks")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAddressSupportedNetworks {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FinancialAddressSupportedNetworks> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialAddressSupportedNetworks::from_str(s)?);
        Ok(())
    }
}
/// The type of financial address.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAddressType {
    Iban,
    SortCode,
    Spei,
    Zengin,
}

impl FinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for FinancialAddressType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "iban" => Ok(Self::Iban),
            "sort_code" => Ok(Self::SortCode),
            "spei" => Ok(Self::Spei),
            "zengin" => Ok(Self::Zengin),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FinancialAddressType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FinancialAddressType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAddressType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FinancialAddressType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialAddressType::from_str(s)?);
        Ok(())
    }
}
pub mod iban;
pub use iban::Iban;
pub mod sort_code;
pub use sort_code::SortCode;
pub mod spei;
pub use spei::Spei;
pub mod zengin;
pub use zengin::Zengin;
