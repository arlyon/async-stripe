/// This is an object representing a Stripe account. You can retrieve it to see
/// properties on the account like its current requirements or if the account is
/// enabled to make live charges or receive payouts.
///
/// For Custom accounts, the properties below are always returned.
/// For other accounts, some properties are returned until that.
/// account has started to go through Connect Onboarding.
/// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions),.
/// some properties are only returned for Custom accounts.
/// Learn about the differences [between accounts](https://stripe.com/docs/connect/accounts).
///
/// For more details see <<https://stripe.com/docs/api/accounts/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<stripe_shared::AccountBusinessProfile>,
    /// The business type.
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property is only returned for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<stripe_shared::AccountBusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_shared::AccountCapabilities>,
    /// Whether the account can create live charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<stripe_shared::LegalEntityCompany>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<stripe_shared::AccountUnificationAccountController>,
    /// The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Time at which the account was connected. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::Timestamp>,
    /// Three-letter ISO currency code representing the default currency for the account.
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<stripe_types::Currency>,
    /// Whether account details have been submitted.
    /// Standard accounts cannot receive payouts before this is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,
    /// An email address associated with the account.
    /// It's not used for authentication and Stripe doesn't market to this field without explicit approval from the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// External accounts (bank accounts and debit cards) currently attached to this account.
    /// External accounts are only returned for requests where `controller[is_controller]` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<stripe_types::List<stripe_shared::ExternalAccount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_shared::AccountFutureRequirements>,
    /// Unique identifier for the object.
    pub id: stripe_shared::AccountId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_shared::Person>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Whether Stripe can send payouts to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_shared::AccountRequirements>,
    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<stripe_shared::AccountSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_shared::AccountTosAcceptance>,
    /// The Stripe account type. Can be `standard`, `express`, or `custom`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_shared::AccountType>,
}
impl stripe_types::Object for Account {
    type Id = stripe_shared::AccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(AccountId);
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountBusinessType"))
    }
}
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountType"))
    }
}
