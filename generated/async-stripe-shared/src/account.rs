/// For new integrations, we recommend using the [Accounts v2 API](/api/v2/core/accounts), in place of /v1/accounts and /v1/customers to represent a user.
///
/// This is an object representing a Stripe account. You can retrieve it to see
/// properties on the account like its current requirements or if the account is
/// enabled to make live charges or receive payouts.
///
/// For accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection).
/// is `application`, which includes Custom accounts, the properties below are always
/// returned.
///
/// For accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection).
/// is `stripe`, which includes Standard and Express accounts, some properties are only returned
/// until you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions)
/// to start Connect Onboarding. Learn about the [differences between accounts](/connect/accounts).
///
/// For more details see <<https://stripe.com/docs/api/accounts/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Account {
    /// Business information about the account.
    pub business_profile: Option<stripe_shared::AccountBusinessProfile>,
    /// The business type.
    pub business_type: Option<stripe_shared::AccountBusinessType>,
    pub capabilities: Option<stripe_shared::AccountCapabilities>,
    /// Whether the account can process charges.
    pub charges_enabled: Option<bool>,
    pub company: Option<stripe_shared::LegalEntityCompany>,
    pub controller: Option<stripe_shared::AccountUnificationAccountController>,
    /// The account's country.
    pub country: Option<String>,
    /// Time at which the account was connected. Measured in seconds since the Unix epoch.
    pub created: Option<stripe_types::Timestamp>,
    /// Three-letter ISO currency code representing the default currency for the account.
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    pub default_currency: Option<stripe_types::Currency>,
    /// Whether account details have been submitted.
    /// Accounts with Stripe Dashboard access, which includes Standard accounts, cannot receive payouts before this is true.
    /// Accounts where this is false should be directed to [an onboarding flow](/connect/onboarding) to finish submitting account details.
    pub details_submitted: Option<bool>,
    /// An email address associated with the account.
    /// It's not used for authentication and Stripe doesn't market to this field without explicit approval from the platform.
    pub email: Option<String>,
    /// External accounts (bank accounts and debit cards) currently attached to this account.
    /// External accounts are only returned for requests where `controller[is_controller]` is true.
    pub external_accounts: Option<stripe_types::List<stripe_shared::ExternalAccount>>,
    pub future_requirements: Option<stripe_shared::AccountFutureRequirements>,
    /// The groups associated with the account.
    pub groups: Option<stripe_shared::AccountGroupMembership>,
    /// Unique identifier for the object.
    pub id: stripe_shared::AccountId,
    pub individual: Option<stripe_shared::Person>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Whether the funds in this account can be paid out.
    pub payouts_enabled: Option<bool>,
    pub requirements: Option<stripe_shared::AccountRequirements>,
    /// Options for customizing how the account functions within Stripe.
    pub settings: Option<stripe_shared::AccountSettings>,
    pub tos_acceptance: Option<stripe_shared::AccountTosAcceptance>,
    /// The Stripe account type. Can be `standard`, `express`, `custom`, or `none`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: Option<AccountType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Account").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountBuilder {
    business_profile: Option<Option<stripe_shared::AccountBusinessProfile>>,
    business_type: Option<Option<stripe_shared::AccountBusinessType>>,
    capabilities: Option<Option<stripe_shared::AccountCapabilities>>,
    charges_enabled: Option<Option<bool>>,
    company: Option<Option<stripe_shared::LegalEntityCompany>>,
    controller: Option<Option<stripe_shared::AccountUnificationAccountController>>,
    country: Option<Option<String>>,
    created: Option<Option<stripe_types::Timestamp>>,
    default_currency: Option<Option<stripe_types::Currency>>,
    details_submitted: Option<Option<bool>>,
    email: Option<Option<String>>,
    external_accounts: Option<Option<stripe_types::List<stripe_shared::ExternalAccount>>>,
    future_requirements: Option<Option<stripe_shared::AccountFutureRequirements>>,
    groups: Option<Option<stripe_shared::AccountGroupMembership>>,
    id: Option<stripe_shared::AccountId>,
    individual: Option<Option<stripe_shared::Person>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    payouts_enabled: Option<Option<bool>>,
    requirements: Option<Option<stripe_shared::AccountRequirements>>,
    settings: Option<Option<stripe_shared::AccountSettings>>,
    tos_acceptance: Option<Option<stripe_shared::AccountTosAcceptance>>,
    type_: Option<Option<AccountType>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for Account {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Account>,
        builder: AccountBuilder,
    }

    impl Visitor for Place<Account> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountBuilder {
                    business_profile: Deserialize::default(),
                    business_type: Deserialize::default(),
                    capabilities: Deserialize::default(),
                    charges_enabled: Deserialize::default(),
                    company: Deserialize::default(),
                    controller: Deserialize::default(),
                    country: Deserialize::default(),
                    created: Deserialize::default(),
                    default_currency: Deserialize::default(),
                    details_submitted: Deserialize::default(),
                    email: Deserialize::default(),
                    external_accounts: Deserialize::default(),
                    future_requirements: Deserialize::default(),
                    groups: Deserialize::default(),
                    id: Deserialize::default(),
                    individual: Deserialize::default(),
                    metadata: Deserialize::default(),
                    payouts_enabled: Deserialize::default(),
                    requirements: Deserialize::default(),
                    settings: Deserialize::default(),
                    tos_acceptance: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "business_profile" => Deserialize::begin(&mut self.builder.business_profile),
                "business_type" => Deserialize::begin(&mut self.builder.business_type),
                "capabilities" => Deserialize::begin(&mut self.builder.capabilities),
                "charges_enabled" => Deserialize::begin(&mut self.builder.charges_enabled),
                "company" => Deserialize::begin(&mut self.builder.company),
                "controller" => Deserialize::begin(&mut self.builder.controller),
                "country" => Deserialize::begin(&mut self.builder.country),
                "created" => Deserialize::begin(&mut self.builder.created),
                "default_currency" => Deserialize::begin(&mut self.builder.default_currency),
                "details_submitted" => Deserialize::begin(&mut self.builder.details_submitted),
                "email" => Deserialize::begin(&mut self.builder.email),
                "external_accounts" => Deserialize::begin(&mut self.builder.external_accounts),
                "future_requirements" => Deserialize::begin(&mut self.builder.future_requirements),
                "groups" => Deserialize::begin(&mut self.builder.groups),
                "id" => Deserialize::begin(&mut self.builder.id),
                "individual" => Deserialize::begin(&mut self.builder.individual),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "payouts_enabled" => Deserialize::begin(&mut self.builder.payouts_enabled),
                "requirements" => Deserialize::begin(&mut self.builder.requirements),
                "settings" => Deserialize::begin(&mut self.builder.settings),
                "tos_acceptance" => Deserialize::begin(&mut self.builder.tos_acceptance),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(business_profile),
                Some(business_type),
                Some(capabilities),
                Some(charges_enabled),
                Some(company),
                Some(controller),
                Some(country),
                Some(created),
                Some(default_currency),
                Some(details_submitted),
                Some(email),
                Some(external_accounts),
                Some(future_requirements),
                Some(groups),
                Some(id),
                Some(individual),
                Some(metadata),
                Some(payouts_enabled),
                Some(requirements),
                Some(settings),
                Some(tos_acceptance),
                Some(type_),
            ) = (
                self.builder.business_profile.take(),
                self.builder.business_type.take(),
                self.builder.capabilities.take(),
                self.builder.charges_enabled,
                self.builder.company.take(),
                self.builder.controller.take(),
                self.builder.country.take(),
                self.builder.created,
                self.builder.default_currency.take(),
                self.builder.details_submitted,
                self.builder.email.take(),
                self.builder.external_accounts.take(),
                self.builder.future_requirements.take(),
                self.builder.groups.take(),
                self.builder.id.take(),
                self.builder.individual.take(),
                self.builder.metadata.take(),
                self.builder.payouts_enabled,
                self.builder.requirements.take(),
                self.builder.settings.take(),
                self.builder.tos_acceptance.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(Account {
                business_profile,
                business_type,
                capabilities,
                charges_enabled,
                company,
                controller,
                country,
                created,
                default_currency,
                details_submitted,
                email,
                external_accounts,
                future_requirements,
                groups,
                id,
                individual,
                metadata,
                payouts_enabled,
                requirements,
                settings,
                tos_acceptance,
                type_,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Account {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Account", 23)?;
        s.serialize_field("business_profile", &self.business_profile)?;
        s.serialize_field("business_type", &self.business_type)?;
        s.serialize_field("capabilities", &self.capabilities)?;
        s.serialize_field("charges_enabled", &self.charges_enabled)?;
        s.serialize_field("company", &self.company)?;
        s.serialize_field("controller", &self.controller)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("default_currency", &self.default_currency)?;
        s.serialize_field("details_submitted", &self.details_submitted)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("external_accounts", &self.external_accounts)?;
        s.serialize_field("future_requirements", &self.future_requirements)?;
        s.serialize_field("groups", &self.groups)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("individual", &self.individual)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("payouts_enabled", &self.payouts_enabled)?;
        s.serialize_field("requirements", &self.requirements)?;
        s.serialize_field("settings", &self.settings)?;
        s.serialize_field("tos_acceptance", &self.tos_acceptance)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "account")?;
        s.end()
    }
}
/// The Stripe account type. Can be `standard`, `express`, `custom`, or `none`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountType {
    Custom,
    Express,
    None,
    Standard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountType {
    pub fn as_str(&self) -> &str {
        use AccountType::*;
        match self {
            Custom => "custom",
            Express => "express",
            None => "none",
            Standard => "standard",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountType::*;
        match s {
            "custom" => Ok(Custom),
            "express" => Ok(Express),
            "none" => Ok(None),
            "standard" => Ok(Standard),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AccountType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for AccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<AccountType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for Account {
    type Id = stripe_shared::AccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(AccountId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountBusinessType {
    pub fn as_str(&self) -> &str {
        use AccountBusinessType::*;
        match self {
            Company => "company",
            GovernmentEntity => "government_entity",
            Individual => "individual",
            NonProfit => "non_profit",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountBusinessType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountBusinessType::*;
        match s {
            "company" => Ok(Company),
            "government_entity" => Ok(GovernmentEntity),
            "individual" => Ok(Individual),
            "non_profit" => Ok(NonProfit),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AccountBusinessType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountBusinessType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for AccountBusinessType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<AccountBusinessType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountBusinessType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountBusinessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
