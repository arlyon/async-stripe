#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextActionDisplayBankTransferInstructions {
    /// The remaining amount that needs to be transferred to complete the payment.
    pub amount_remaining: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<crate::Currency>,
    /// A list of financial addresses that can be used to fund the customer balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<
        Vec<crate::funding_instructions::bank_transfer::financial_address::FinancialAddress>,
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextActionDisplayBankTransferInstructions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of bank transfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
