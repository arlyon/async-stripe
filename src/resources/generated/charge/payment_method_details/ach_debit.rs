#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AchDebit {
    /// Type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    pub account_holder_type: Option<AchDebitAccountHolderType>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Routing transit number of the bank account.
    pub routing_number: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AchDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of entity that holds the account.
///
/// This can be either `individual` or `company`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AchDebitAccountHolderType {
    Company,
    Individual,
}

impl AchDebitAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for AchDebitAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AchDebitAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
