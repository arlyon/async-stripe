#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FinancialConnections {
    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<FinancialConnectionsPermissions>>,
}
/// The list of permissions to request.
///
/// The `payment_method` permission must be included.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialConnectionsPermissions {
    Balances,
    PaymentMethod,
    Transactions,
}

impl FinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsPermissions::*;
        match self {
            Balances => "balances",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FinancialConnectionsPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsPermissions")
        })
    }
}
