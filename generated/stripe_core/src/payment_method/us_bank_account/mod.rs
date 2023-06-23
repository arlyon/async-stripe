#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct UsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<UsBankAccountAccountHolderType>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<UsBankAccountAccountType>,
    /// The name of the bank.
    pub bank_name: Option<String>,
    /// The ID of the Financial Connections Account used to create the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Contains information about US bank account networks that can be used.
    pub networks: Option<stripe_core::payment_method::us_bank_account::networks::Networks>,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for UsBankAccount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum UsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl UsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum UsBankAccountAccountType {
    Checking,
    Savings,
}

impl UsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for UsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod networks;
pub use networks::Networks;
