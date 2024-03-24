/// This is an object representing a Stripe account. You can retrieve it to see
/// properties on the account like its current requirements or if the account is
/// enabled to make live charges or receive payouts.
///
/// For Custom accounts, the properties below are always returned.
/// For other accounts, some properties are returned until that.
/// account has started to go through Connect Onboarding.
/// Once you create an [Account Link](https://stripe.com/docs/api/account_links).
/// for a Standard or Express account, some parameters are no longer returned.
/// These are marked as **Custom Only** or **Custom and Express**.
/// below. Learn about the differences [between accounts](https://stripe.com/docs/connect/accounts).
///
/// For more details see <<https://stripe.com/docs/api/accounts/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Account {
    /// Business information about the account.
    pub business_profile: Option<stripe_shared::AccountBusinessProfile>,
    /// The business type.
    pub business_type: Option<stripe_shared::AccountBusinessType>,
    pub capabilities: Option<stripe_shared::AccountCapabilities>,
    /// Whether the account can create live charges.
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
    /// Standard accounts cannot receive payouts before this is true.
    pub details_submitted: Option<bool>,
    /// An email address associated with the account.
    /// It's not used for authentication and Stripe doesn't market to this field without explicit approval from the platform.
    pub email: Option<String>,
    /// External accounts (bank accounts and debit cards) currently attached to this account
    pub external_accounts: Option<stripe_types::List<stripe_shared::ExternalAccount>>,
    pub future_requirements: Option<stripe_shared::AccountFutureRequirements>,
    /// Unique identifier for the object.
    pub id: stripe_shared::AccountId,
    pub individual: Option<stripe_shared::Person>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Whether Stripe can send payouts to this account.
    pub payouts_enabled: Option<bool>,
    pub requirements: Option<stripe_shared::AccountRequirements>,
    /// Options for customizing how the account functions within Stripe.
    pub settings: Option<stripe_shared::AccountSettings>,
    pub tos_acceptance: Option<stripe_shared::AccountTosAcceptance>,
    /// The Stripe account type. Can be `standard`, `express`, or `custom`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<stripe_shared::AccountType>,
}
#[cfg(feature = "min-ser")]
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
    id: Option<stripe_shared::AccountId>,
    individual: Option<Option<stripe_shared::Person>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    payouts_enabled: Option<Option<bool>>,
    requirements: Option<Option<stripe_shared::AccountRequirements>>,
    settings: Option<Option<stripe_shared::AccountSettings>>,
    tos_acceptance: Option<Option<stripe_shared::AccountTosAcceptance>>,
    type_: Option<Option<stripe_shared::AccountType>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            let business_profile = self.business_profile.take()?;
            let business_type = self.business_type.take()?;
            let capabilities = self.capabilities.take()?;
            let charges_enabled = self.charges_enabled.take()?;
            let company = self.company.take()?;
            let controller = self.controller.take()?;
            let country = self.country.take()?;
            let created = self.created.take()?;
            let default_currency = self.default_currency.take()?;
            let details_submitted = self.details_submitted.take()?;
            let email = self.email.take()?;
            let external_accounts = self.external_accounts.take()?;
            let future_requirements = self.future_requirements.take()?;
            let id = self.id.take()?;
            let individual = self.individual.take()?;
            let metadata = self.metadata.take()?;
            let payouts_enabled = self.payouts_enabled.take()?;
            let requirements = self.requirements.take()?;
            let settings = self.settings.take()?;
            let tos_acceptance = self.tos_acceptance.take()?;
            let type_ = self.type_.take()?;

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

    impl<'a> Map for Builder<'a> {
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
                    "business_profile" => b.business_profile = Some(FromValueOpt::from_value(v)?),
                    "business_type" => b.business_type = Some(FromValueOpt::from_value(v)?),
                    "capabilities" => b.capabilities = Some(FromValueOpt::from_value(v)?),
                    "charges_enabled" => b.charges_enabled = Some(FromValueOpt::from_value(v)?),
                    "company" => b.company = Some(FromValueOpt::from_value(v)?),
                    "controller" => b.controller = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "default_currency" => b.default_currency = Some(FromValueOpt::from_value(v)?),
                    "details_submitted" => b.details_submitted = Some(FromValueOpt::from_value(v)?),
                    "email" => b.email = Some(FromValueOpt::from_value(v)?),
                    "external_accounts" => b.external_accounts = Some(FromValueOpt::from_value(v)?),
                    "future_requirements" => b.future_requirements = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "individual" => b.individual = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "payouts_enabled" => b.payouts_enabled = Some(FromValueOpt::from_value(v)?),
                    "requirements" => b.requirements = Some(FromValueOpt::from_value(v)?),
                    "settings" => b.settings = Some(FromValueOpt::from_value(v)?),
                    "tos_acceptance" => b.tos_acceptance = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for Account {
    type Id = stripe_shared::AccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(AccountId, "acct_");
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
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountBusinessType"))
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

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(AccountBusinessType);
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

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(AccountType);
