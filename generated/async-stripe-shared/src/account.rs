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
#[derive(Clone, Debug)]
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountBuilder {
        type Out = Account;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "business_profile" => Deserialize::begin(&mut self.business_profile),
                "business_type" => Deserialize::begin(&mut self.business_type),
                "capabilities" => Deserialize::begin(&mut self.capabilities),
                "charges_enabled" => Deserialize::begin(&mut self.charges_enabled),
                "company" => Deserialize::begin(&mut self.company),
                "controller" => Deserialize::begin(&mut self.controller),
                "country" => Deserialize::begin(&mut self.country),
                "created" => Deserialize::begin(&mut self.created),
                "default_currency" => Deserialize::begin(&mut self.default_currency),
                "details_submitted" => Deserialize::begin(&mut self.details_submitted),
                "email" => Deserialize::begin(&mut self.email),
                "external_accounts" => Deserialize::begin(&mut self.external_accounts),
                "future_requirements" => Deserialize::begin(&mut self.future_requirements),
                "groups" => Deserialize::begin(&mut self.groups),
                "id" => Deserialize::begin(&mut self.id),
                "individual" => Deserialize::begin(&mut self.individual),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "payouts_enabled" => Deserialize::begin(&mut self.payouts_enabled),
                "requirements" => Deserialize::begin(&mut self.requirements),
                "settings" => Deserialize::begin(&mut self.settings),
                "tos_acceptance" => Deserialize::begin(&mut self.tos_acceptance),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.business_profile.take(),
                self.business_type,
                self.capabilities,
                self.charges_enabled,
                self.company.take(),
                self.controller,
                self.country.take(),
                self.created,
                self.default_currency.take(),
                self.details_submitted,
                self.email.take(),
                self.external_accounts.take(),
                self.future_requirements.take(),
                self.groups.take(),
                self.id.take(),
                self.individual.take(),
                self.metadata.take(),
                self.payouts_enabled,
                self.requirements.take(),
                self.settings.take(),
                self.tos_acceptance.take(),
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out {
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Account {
        type Builder = AccountBuilder;
    }

    impl FromValueOpt for Account {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "business_profile" => b.business_profile = FromValueOpt::from_value(v),
                    "business_type" => b.business_type = FromValueOpt::from_value(v),
                    "capabilities" => b.capabilities = FromValueOpt::from_value(v),
                    "charges_enabled" => b.charges_enabled = FromValueOpt::from_value(v),
                    "company" => b.company = FromValueOpt::from_value(v),
                    "controller" => b.controller = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "default_currency" => b.default_currency = FromValueOpt::from_value(v),
                    "details_submitted" => b.details_submitted = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "external_accounts" => b.external_accounts = FromValueOpt::from_value(v),
                    "future_requirements" => b.future_requirements = FromValueOpt::from_value(v),
                    "groups" => b.groups = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "individual" => b.individual = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "payouts_enabled" => b.payouts_enabled = FromValueOpt::from_value(v),
                    "requirements" => b.requirements = FromValueOpt::from_value(v),
                    "settings" => b.settings = FromValueOpt::from_value(v),
                    "tos_acceptance" => b.tos_acceptance = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountType {
    Custom,
    Express,
    None,
    Standard,
}
impl AccountType {
    pub fn as_str(self) -> &'static str {
        use AccountType::*;
        match self {
            Custom => "custom",
            Express => "express",
            None => "none",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for AccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountType::*;
        match s {
            "custom" => Ok(Custom),
            "express" => Ok(Express),
            "none" => Ok(None),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountType"))
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountBusinessType::*;
        match s {
            "company" => Ok(Company),
            "government_entity" => Ok(GovernmentEntity),
            "individual" => Ok(Individual),
            "non_profit" => Ok(NonProfit),
            _ => Err(stripe_types::StripeParseError),
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
impl miniserde::Deserialize for AccountBusinessType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountBusinessType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountBusinessType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountBusinessType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountBusinessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountBusinessType"))
    }
}
