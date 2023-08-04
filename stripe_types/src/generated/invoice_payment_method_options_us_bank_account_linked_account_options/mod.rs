#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions:
        Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>>,
}
/// The list of permissions to request.
///
/// The `payment_method` permission must be included.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    Balances,
    PaymentMethod,
    Transactions,
}

impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match self {
            Balances => "balances",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions"))
    }
}
