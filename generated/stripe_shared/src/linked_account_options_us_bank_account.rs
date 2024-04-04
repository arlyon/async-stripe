#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LinkedAccountOptionsUsBankAccount {
    /// The list of permissions to request. The `payment_method` permission must be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<LinkedAccountOptionsUsBankAccountPermissions>>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<LinkedAccountOptionsUsBankAccountPrefetch>>,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
/// The list of permissions to request. The `payment_method` permission must be included.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LinkedAccountOptionsUsBankAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl LinkedAccountOptionsUsBankAccountPermissions {
    pub fn as_str(self) -> &'static str {
        use LinkedAccountOptionsUsBankAccountPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsUsBankAccountPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsUsBankAccountPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for LinkedAccountOptionsUsBankAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LinkedAccountOptionsUsBankAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for LinkedAccountOptionsUsBankAccountPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsUsBankAccountPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for LinkedAccountOptionsUsBankAccountPermissions",
            )
        })
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LinkedAccountOptionsUsBankAccountPrefetch {
    Balances,
    Transactions,
}
impl LinkedAccountOptionsUsBankAccountPrefetch {
    pub fn as_str(self) -> &'static str {
        use LinkedAccountOptionsUsBankAccountPrefetch::*;
        match self {
            Balances => "balances",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsUsBankAccountPrefetch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsUsBankAccountPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for LinkedAccountOptionsUsBankAccountPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LinkedAccountOptionsUsBankAccountPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for LinkedAccountOptionsUsBankAccountPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsUsBankAccountPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for LinkedAccountOptionsUsBankAccountPrefetch")
        })
    }
}
