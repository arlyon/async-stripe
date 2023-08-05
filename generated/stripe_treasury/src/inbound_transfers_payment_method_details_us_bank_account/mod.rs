#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// The US bank account network used to debit funds.
    pub network: InboundTransfersPaymentMethodDetailsUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InboundTransfersPaymentMethodDetailsUsBankAccountAccountType"))
    }
}
/// The US bank account network used to debit funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
}

impl InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
        }
    }
}

impl std::str::FromStr for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork"))
    }
}
