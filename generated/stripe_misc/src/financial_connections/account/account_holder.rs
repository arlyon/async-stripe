#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountHolder {
    /// The ID of the Stripe account this account belongs to.
    ///
    /// Should only be present if `account_holder.type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types::Expandable<stripe_core::account::Account>>,
    /// ID of the Stripe customer this account belongs to.
    ///
    /// Present if and only if `account_holder.type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<stripe_types::Expandable<stripe_core::customer::Customer>>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
