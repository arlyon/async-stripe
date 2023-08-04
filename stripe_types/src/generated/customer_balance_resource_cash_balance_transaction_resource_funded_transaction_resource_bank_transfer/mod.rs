#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>,
#[serde(skip_serializing_if = "Option::is_none")]
pub gb_bank_transfer: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,
#[serde(skip_serializing_if = "Option::is_none")]
pub jp_bank_transfer: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>,
    /// The user-supplied reference field on the bank transfer.
pub reference: Option<String>,
    /// The funding method type used to fund the customer balance.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[serde(rename = "type")]
pub type_: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_transfer: Option<stripe_types::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>,

}
/// The funding method type used to fund the customer balance.
///
/// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType
{
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

impl
    CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType
{
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::*;
        match s {
    "eu_bank_transfer" => Ok(EuBankTransfer),
"gb_bank_transfer" => Ok(GbBankTransfer),
"jp_bank_transfer" => Ok(JpBankTransfer),
"mx_bank_transfer" => Ok(MxBankTransfer),
"us_bank_transfer" => Ok(UsBankTransfer),
_ => Err(())

        }
    }
}

impl AsRef<str> for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType"))
    }
}
