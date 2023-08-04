/// Each customer has a [`balance`](https://stripe.com/docs/api/customers/object#customer_object-balance) that is
/// automatically applied to future invoices and payments using the `customer_balance` payment method.
/// Customers can fund this balance by initiating a bank transfer to any account in the
/// `financial_addresses` field.
/// Related guide: [Customer balance funding instructions](https://stripe.com/docs/payments/customer-balance/funding-instructions).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructions {
    pub bank_transfer: stripe_types::FundingInstructionsBankTransfer,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `funding_type` of the returned instructions.
    pub funding_type:
        CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
/// The `funding_type` of the returned instructions.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType {
    BankTransfer,
}

impl CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType {
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr
    for CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(()),
        }
    }
}

impl AsRef<str>
    for CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructionsFundingType"))
    }
}
