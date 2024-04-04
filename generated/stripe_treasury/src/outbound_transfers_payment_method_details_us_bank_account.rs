#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OutboundTransfersPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type:
        Option<OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// The US bank account network used to send funds.
    pub network: OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}
impl OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType",
            )
        })
    }
}
/// The US bank account network used to send funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}
impl OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork",
            )
        })
    }
}
