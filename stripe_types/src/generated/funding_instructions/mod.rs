/// Each customer has a [`balance`](https://stripe.com/docs/api/customers/object#customer_object-balance) that is
/// automatically applied to future invoices and payments using the `customer_balance` payment method.
/// Customers can fund this balance by initiating a bank transfer to any account in the
/// `financial_addresses` field.
/// Related guide: [Customer balance funding instructions](https://stripe.com/docs/payments/customer-balance/funding-instructions).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructions {
    pub bank_transfer: stripe_types::bank_transfer::BankTransfer,
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
/// The `funding_type` of the returned instructions.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FundingInstructionsFundingType {
    BankTransfer,
}

impl FundingInstructionsFundingType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for FundingInstructionsFundingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(()),
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
impl serde::Serialize for FundingInstructionsFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FundingInstructionsFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FundingInstructionsFundingType")
        })
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FundingInstructionsObject {
    FundingInstructions,
}

impl FundingInstructionsObject {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsObject::*;
        match self {
            FundingInstructions => "funding_instructions",
        }
    }
}

impl std::str::FromStr for FundingInstructionsObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsObject::*;
        match s {
            "funding_instructions" => Ok(FundingInstructions),
            _ => Err(()),
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
impl serde::Serialize for FundingInstructionsObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FundingInstructionsObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FundingInstructionsObject"))
    }
}
