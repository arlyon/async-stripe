#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountHolder {
    /// The ID of the Stripe account this account belongs to.
    ///
    /// Should only be present if `account_holder.type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types::Expandable<stripe_types::account::Account>>,
    /// ID of the Stripe customer this account belongs to.
    ///
    /// Present if and only if `account_holder.type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    /// Type of account holder that this account belongs to.
    #[serde(rename = "type")]
    pub type_: AccountHolderType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountHolder {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of account holder that this account belongs to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountHolderType {
    Account,
    Customer,
}

impl AccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Customer => "customer",
        }
    }
}

impl std::str::FromStr for AccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account" => Ok(Self::Account),
            "customer" => Ok(Self::Customer),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountHolderType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<AccountHolderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountHolderType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
