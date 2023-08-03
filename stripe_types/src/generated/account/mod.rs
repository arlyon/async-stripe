/// This is an object representing a Stripe account.
///
/// You can retrieve it to see properties on the account like its current requirements or if the account is enabled to make live charges or receive payouts.  For Custom accounts, the properties below are always returned.
/// For other accounts, some properties are returned until that account has started to go through Connect Onboarding.
/// Once you create an [Account Link](https://stripe.com/docs/api/account_links) for a Standard or Express account, some parameters are no longer returned.
/// These are marked as **Custom Only** or **Custom and Express** below.
/// Learn about the differences [between accounts](https://stripe.com/docs/connect/accounts).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<stripe_types::AccountBusinessProfile>,
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<AccountBusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types::AccountCapabilities>,
    /// Whether the account can create live charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<stripe_types::LegalEntityCompany>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<stripe_types::AccountUnificationAccountController>,
    /// The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Time at which the account was connected.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::Timestamp>,
    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<stripe_types::Currency>,
    /// Whether account details have been submitted.
    ///
    /// Standard accounts cannot receive payouts before this is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,
    /// An email address associated with the account.
    ///
    /// It's not used for authentication and Stripe doesn't market to this field without explicit approval from the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// External accounts (bank accounts and debit cards) currently attached to this account.
    #[serde(default)]
    pub external_accounts: stripe_types::List<stripe_types::ExternalAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types::AccountFutureRequirements>,
    /// Unique identifier for the object.
    pub id: stripe_types::account::AccountId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types::Person>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: AccountObject,
    /// Whether Stripe can send payouts to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types::AccountRequirements>,
    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<stripe_types::AccountSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_types::AccountTosAcceptance>,
    /// The Stripe account type.
    ///
    /// Can be `standard`, `express`, or `custom`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<AccountType>,
}
/// The business type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl AccountBusinessType {
    pub fn as_str(self) -> &'static str {
        use AccountBusinessType::*;
        match self {
            Company => "company",
            GovernmentEntity => "government_entity",
            Individual => "individual",
            NonProfit => "non_profit",
        }
    }
}

impl std::str::FromStr for AccountBusinessType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountBusinessType::*;
        match s {
            "company" => Ok(Company),
            "government_entity" => Ok(GovernmentEntity),
            "individual" => Ok(Individual),
            "non_profit" => Ok(NonProfit),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountBusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountBusinessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountBusinessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for AccountBusinessType"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountObject {
    Account,
}

impl AccountObject {
    pub fn as_str(self) -> &'static str {
        use AccountObject::*;
        match self {
            Account => "account",
        }
    }
}

impl std::str::FromStr for AccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountObject::*;
        match s {
            "account" => Ok(Account),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for AccountObject"))
    }
}
/// The Stripe account type.
///
/// Can be `standard`, `express`, or `custom`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountType {
    Custom,
    Express,
    Standard,
}

impl AccountType {
    pub fn as_str(self) -> &'static str {
        use AccountType::*;
        match self {
            Custom => "custom",
            Express => "express",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for AccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountType::*;
        match s {
            "custom" => Ok(Custom),
            "express" => Ok(Express),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for AccountType"))
    }
}
impl stripe_types::Object for Account {
    type Id = stripe_types::account::AccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(AccountId, "acct_");
