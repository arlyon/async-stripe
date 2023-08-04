#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptionsCustomerBalanceBankTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<stripe_types::PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types:
        Option<Vec<PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,
    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    pub type_: Option<PaymentMethodOptionsCustomerBalanceBankTransferType>,
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

impl PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::*;
        match self {
            Aba => "aba",
            Iban => "iban",
            Sepa => "sepa",
            SortCode => "sort_code",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::*;
        match s {
            "aba" => Ok(Aba),
            "iban" => Ok(Iban),
            "sepa" => Ok(Sepa),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes"))
    }
}
/// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

impl PaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCustomerBalanceBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCustomerBalanceBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCustomerBalanceBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsCustomerBalanceBankTransferType",
            )
        })
    }
}
