/// This is an object representing a Stripe account.
///
/// You can retrieve it to see properties on the account like its current e-mail address or if the account is enabled yet to make live charges.  Some properties, marked below, are available only to platforms that want to [create and manage Express or Custom accounts](https://stripe.com/docs/connect/accounts).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Account {
    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<stripe_types::account::business_profile::BusinessProfile>,
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<AccountBusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types::account::capabilities::Capabilities>,
    /// Whether the account can create live charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<stripe_types::account::company::Company>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<stripe_types::account::controller::Controller>,
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
    /// You can treat this as metadata: it is not used for authentication or messaging account holders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// External accounts (bank accounts and debit cards) currently attached to this account.
    #[serde(default)]
    pub external_accounts: stripe_types::List<stripe_types::external_account::ExternalAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types::account::future_requirements::FutureRequirements>,
    /// Unique identifier for the object.
    pub id: stripe_types::AccountId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types::person::Person>,
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
    pub requirements: Option<stripe_types::account::requirements::Requirements>,
    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<stripe_types::account::settings::Settings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_types::account::tos_acceptance::TosAcceptance>,
    /// The Stripe account type.
    ///
    /// Can be `standard`, `express`, or `custom`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<AccountType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Account {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The business type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl AccountBusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::GovernmentEntity => "government_entity",
            Self::Individual => "individual",
            Self::NonProfit => "non_profit",
        }
    }
}

impl std::str::FromStr for AccountBusinessType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "company" => Ok(Self::Company),
            "government_entity" => Ok(Self::GovernmentEntity),
            "individual" => Ok(Self::Individual),
            "non_profit" => Ok(Self::NonProfit),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountBusinessType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountBusinessType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<AccountBusinessType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountBusinessType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountObject {
    Account,
}

impl AccountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
        }
    }
}

impl std::str::FromStr for AccountObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account" => Ok(Self::Account),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<AccountObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The Stripe account type.
///
/// Can be `standard`, `express`, or `custom`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountType {
    Custom,
    Express,
    Standard,
}

impl AccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Express => "express",
            Self::Standard => "standard",
        }
    }
}

impl std::str::FromStr for AccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "custom" => Ok(Self::Custom),
            "express" => Ok(Self::Express),
            "standard" => Ok(Self::Standard),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<AccountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Account {
    type Id = stripe_types::AccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
pub mod deleted;
pub use deleted::DeletedAccount;
pub mod settings_branding;
pub use settings_branding::SettingsBranding;
pub mod business_profile;
pub use business_profile::BusinessProfile;
pub mod capabilities;
pub use capabilities::Capabilities;
pub mod settings_card_payments;
pub use settings_card_payments::SettingsCardPayments;
pub mod settings_dashboard;
pub use settings_dashboard::SettingsDashboard;
pub mod decline_charge_on;
pub use decline_charge_on::DeclineChargeOn;
pub mod future_requirements;
pub use future_requirements::FutureRequirements;
pub mod settings_payments;
pub use settings_payments::SettingsPayments;
pub mod settings_payouts;
pub use settings_payouts::SettingsPayouts;
pub mod requirements;
pub use requirements::Requirements;
pub mod settings;
pub use settings::Settings;
pub mod tos_acceptance;
pub use tos_acceptance::TosAcceptance;
pub mod controller;
pub use controller::Controller;
pub mod company;
pub use company::Company;
pub mod payout_schedule;
pub use payout_schedule::PayoutSchedule;
