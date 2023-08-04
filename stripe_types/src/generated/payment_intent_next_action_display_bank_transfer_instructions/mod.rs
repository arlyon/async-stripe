#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionDisplayBankTransferInstructions {
    /// The remaining amount that needs to be transferred to complete the payment.
    pub amount_remaining: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// A list of financial addresses that can be used to fund the customer balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses:
        Option<Vec<stripe_types::FundingInstructionsBankTransferFinancialAddress>>,
    /// A link to a hosted page that guides your customer through completing the transfer.
    pub hosted_instructions_url: Option<String>,
    /// A string identifying this payment.
    ///
    /// Instruct your customer to include this code in the reference or memo field of their bank transfer.
    pub reference: Option<String>,
    /// Type of bank transfer.
    #[serde(rename = "type")]
    pub type_: PaymentIntentNextActionDisplayBankTransferInstructionsType,
}
/// Type of bank transfer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentNextActionDisplayBankTransferInstructionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

impl PaymentIntentNextActionDisplayBankTransferInstructionsType {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentNextActionDisplayBankTransferInstructionsType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentNextActionDisplayBankTransferInstructionsType::*;
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

impl AsRef<str> for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentNextActionDisplayBankTransferInstructionsType",
            )
        })
    }
}
