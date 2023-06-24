#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialConnections {
    /// The list of permissions to request.
    ///
    /// The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<FinancialConnectionsPermissions>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialConnections {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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
        match self {
            Self::Balances => "balances",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "balances" => Ok(Self::Balances),
            "payment_method" => Ok(Self::PaymentMethod),
            "transactions" => Ok(Self::Transactions),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsPermissions")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialConnectionsPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FinancialConnectionsPermissions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsPermissions::from_str(s)?);
        Ok(())
    }
}
