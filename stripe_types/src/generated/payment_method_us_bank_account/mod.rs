#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<PaymentMethodUsBankAccountAccountHolderType>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<PaymentMethodUsBankAccountAccountType>,
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
    pub networks: Option<stripe_types::UsBankAccountNetworks>,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
    /// Contains information about the future reusability of this PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types::PaymentMethodUsBankAccountStatusDetails>,
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodUsBankAccountAccountHolderType",
            )
        })
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}

impl PaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for PaymentMethodUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodUsBankAccountAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodUsBankAccountAccountType")
        })
    }
}
