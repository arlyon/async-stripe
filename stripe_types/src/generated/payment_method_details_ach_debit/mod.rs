#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsAchDebit {
    /// Type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodDetailsAchDebitAccountHolderType>,
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
/// Type of entity that holds the account.
///
/// This can be either `individual` or `company`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsAchDebitAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodDetailsAchDebitAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsAchDebitAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsAchDebitAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsAchDebitAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsAchDebitAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsAchDebitAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsAchDebitAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsAchDebitAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsAchDebitAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsAchDebitAccountHolderType",
            )
        })
    }
}
