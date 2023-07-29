#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NextActionDisplayBankTransferInstructions {
    /// The remaining amount that needs to be transferred to complete the payment.
    pub amount_remaining: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// A list of financial addresses that can be used to fund the customer balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<
        Vec<stripe_types::funding_instructions::bank_transfer::financial_address::FinancialAddress>,
    >,
    /// A link to a hosted page that guides your customer through completing the transfer.
    pub hosted_instructions_url: Option<String>,
    /// A string identifying this payment.
    ///
    /// Instruct your customer to include this code in the reference or memo field of their bank transfer.
    pub reference: Option<String>,
    /// Type of bank transfer.
    #[serde(rename = "type")]
    pub type_: NextActionDisplayBankTransferInstructionsType,
}
/// Type of bank transfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NextActionDisplayBankTransferInstructionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl NextActionDisplayBankTransferInstructionsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl std::str::FromStr for NextActionDisplayBankTransferInstructionsType {
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

impl AsRef<str> for NextActionDisplayBankTransferInstructionsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for NextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for NextActionDisplayBankTransferInstructionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NextActionDisplayBankTransferInstructionsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for NextActionDisplayBankTransferInstructionsType",
            )
        })
    }
}
