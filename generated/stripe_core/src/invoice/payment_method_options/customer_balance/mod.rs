#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<
        stripe_core::invoice::payment_method_options::customer_balance::bank_transfer::BankTransfer,
    >,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<CustomerBalanceFundingType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerBalance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceFundingType {
    BankTransfer,
}

impl CustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for CustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod bank_transfer;
pub use bank_transfer::BankTransfer;
