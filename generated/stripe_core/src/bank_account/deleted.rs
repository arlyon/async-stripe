#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedBankAccount {
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_core::bank_account::BankAccountId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedBankAccountObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedBankAccount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DeletedBankAccountObject {
    BankAccount,
}

impl DeletedBankAccountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
        }
    }
}

impl AsRef<str> for DeletedBankAccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedBankAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for DeletedBankAccount {
    type Id = stripe_core::bank_account::BankAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
