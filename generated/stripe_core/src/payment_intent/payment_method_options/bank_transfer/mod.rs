#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<stripe_core::payment_intent::payment_method_options::bank_transfer::eu_bank_transfer::EuBankTransfer>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<Vec<BankTransferRequestedAddressTypes>>,
    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: Option<BankTransferType>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BankTransferRequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl BankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for BankTransferRequestedAddressTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "iban" => Ok(Self::Iban),
            "sepa" => Ok(Self::Sepa),
            "sort_code" => Ok(Self::SortCode),
            "spei" => Ok(Self::Spei),
            "zengin" => Ok(Self::Zengin),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BankTransferRequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankTransferRequestedAddressTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BankTransferRequestedAddressTypes")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransferRequestedAddressTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<BankTransferRequestedAddressTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankTransferRequestedAddressTypes::from_str(s)?);
        Ok(())
    }
}
/// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl BankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl std::str::FromStr for BankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eu_bank_transfer" => Ok(Self::EuBankTransfer),
            "gb_bank_transfer" => Ok(Self::GbBankTransfer),
            "jp_bank_transfer" => Ok(Self::JpBankTransfer),
            "mx_bank_transfer" => Ok(Self::MxBankTransfer),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BankTransferType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransferType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<BankTransferType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankTransferType::from_str(s)?);
        Ok(())
    }
}
pub mod eu_bank_transfer;
pub use eu_bank_transfer::EuBankTransfer;
