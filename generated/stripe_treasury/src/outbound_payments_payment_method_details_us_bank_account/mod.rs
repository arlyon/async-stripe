#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// The US bank account network used to send funds.
    pub network: OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

impl OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType"))
    }
}
/// The US bank account network used to send funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork"))
    }
}
