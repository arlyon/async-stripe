#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<stripe_types::InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<InvoicePaymentMethodOptionsCustomerBalanceFundingType>,
}
/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsCustomerBalanceFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsCustomerBalanceFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for InvoicePaymentMethodOptionsCustomerBalanceFundingType",
            )
        })
    }
}
