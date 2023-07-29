#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    /// The type of account being debited or credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<ReceiptAccountType>,
    /// EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    pub application_cryptogram: Option<String>,
    /// Mnenomic of the Application Identifier.
    pub application_preferred_name: Option<String>,
    /// Identifier for this transaction.
    pub authorization_code: Option<String>,
    /// EMV tag 8A.
    ///
    /// A code returned by the card issuer.
    pub authorization_response_code: Option<String>,
    /// How the cardholder verified ownership of the card.
    pub cardholder_verification_method: Option<String>,
    /// EMV tag 84.
    ///
    /// Similar to the application identifier stored on the integrated circuit chip.
    pub dedicated_file_name: Option<String>,
    /// The outcome of a series of EMV functions performed by the card reader.
    pub terminal_verification_results: Option<String>,
    /// An indication of various EMV functions performed during the transaction.
    pub transaction_status_information: Option<String>,
}
/// The type of account being debited or credited.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReceiptAccountType {
    Checking,
    Savings,
    Unknown,
}

impl ReceiptAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
            Self::Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ReceiptAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "checking" => Ok(Self::Checking),
            "savings" => Ok(Self::Savings),
            "unknown" => Ok(Self::Unknown),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReceiptAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceiptAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReceiptAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceiptAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReceiptAccountType"))
    }
}
