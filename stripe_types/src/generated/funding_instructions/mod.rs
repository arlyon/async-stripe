/// Each customer has a [`balance`](https://stripe.com/docs/api/customers/object#customer_object-balance) that is
/// automatically applied to future invoices and payments using the `customer_balance` payment method.
/// Customers can fund this balance by initiating a bank transfer to any account in the
/// `financial_addresses` field.
/// Related guide: [Customer Balance - Funding Instructions](https://stripe.com/docs/payments/customer-balance/funding-instructions) to learn more.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FundingInstructions {
    pub bank_transfer: stripe_types::funding_instructions::bank_transfer::BankTransfer,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `funding_type` of the returned instructions.
    pub funding_type: FundingInstructionsFundingType,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FundingInstructionsObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FundingInstructions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The `funding_type` of the returned instructions.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsFundingType {
    BankTransfer,
}

impl FundingInstructionsFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for FundingInstructionsFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsObject {
    FundingInstructions,
}

impl FundingInstructionsObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FundingInstructions => "funding_instructions",
        }
    }
}

impl AsRef<str> for FundingInstructionsObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod bank_transfer;
pub use bank_transfer::BankTransfer;
