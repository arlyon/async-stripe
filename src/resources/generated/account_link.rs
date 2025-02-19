// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::AccountId;
use crate::params::{Expand, Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "AccountLink".
///
/// For more details see <https://stripe.com/docs/api/account_links/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The timestamp at which this account link will expire.
    pub expires_at: Timestamp,

    /// The URL for the account link.
    pub url: String,
}

impl AccountLink {
    /// Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.
    pub fn create(client: &Client, params: CreateAccountLink<'_>) -> Response<AccountLink> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/account_links", &params)
    }
}

impl Object for AccountLink {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "account_link"
    }
}

/// The parameters for `AccountLink::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateAccountLink<'a> {
    /// The identifier of the account to create an account link for.
    pub account: AccountId,

    /// The collect parameter is deprecated.
    ///
    /// Use `collection_options` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect: Option<AccountLinkCollect>,

    /// Specifies the requirements that Stripe collects from connected accounts in the Connect Onboarding flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_options: Option<CreateAccountLinkCollectionOptions>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The URL the user will be redirected to if the account link is expired, has been previously-visited, or is otherwise invalid.
    ///
    /// The URL you specify should attempt to generate a new account link with the same parameters used to create the original account link, then redirect the user to the new account link's URL so they can continue with Connect Onboarding.
    /// If a new account link cannot be generated or the redirect fails you should display a useful error to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<&'a str>,

    /// The URL that the user will be redirected to upon leaving or completing the linked flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,

    /// The type of account link the user is requesting.
    ///
    /// Possible values are `account_onboarding` or `account_update`.
    #[serde(rename = "type")]
    pub type_: AccountLinkType,
}

impl<'a> CreateAccountLink<'a> {
    pub fn new(account: AccountId, type_: AccountLinkType) -> Self {
        CreateAccountLink {
            account,
            collect: Default::default(),
            collection_options: Default::default(),
            expand: Default::default(),
            refresh_url: Default::default(),
            return_url: Default::default(),
            type_,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountLinkCollectionOptions {
    /// Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and eventually_due requirements (`eventually_due`).
    ///
    /// If you don't specify `collection_options`, the default value is `currently_due`.
    pub fields: CreateAccountLinkCollectionOptionsFields,

    /// Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding.
    ///
    /// The default value is `omit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<CreateAccountLinkCollectionOptionsFutureRequirements>,
}

/// An enum representing the possible values of an `CreateAccountLink`'s `collect` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountLinkCollect {
    CurrentlyDue,
    EventuallyDue,
}

impl AccountLinkCollect {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountLinkCollect::CurrentlyDue => "currently_due",
            AccountLinkCollect::EventuallyDue => "eventually_due",
        }
    }
}

impl AsRef<str> for AccountLinkCollect {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountLinkCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountLinkCollect {
    fn default() -> Self {
        Self::CurrentlyDue
    }
}

/// An enum representing the possible values of an `CreateAccountLink`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountLinkType {
    AccountOnboarding,
    AccountUpdate,
}

impl AccountLinkType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountLinkType::AccountOnboarding => "account_onboarding",
            AccountLinkType::AccountUpdate => "account_update",
        }
    }
}

impl AsRef<str> for AccountLinkType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountLinkType {
    fn default() -> Self {
        Self::AccountOnboarding
    }
}

/// An enum representing the possible values of an `CreateAccountLinkCollectionOptions`'s `fields` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountLinkCollectionOptionsFields {
    CurrentlyDue,
    EventuallyDue,
}

impl CreateAccountLinkCollectionOptionsFields {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateAccountLinkCollectionOptionsFields::CurrentlyDue => "currently_due",
            CreateAccountLinkCollectionOptionsFields::EventuallyDue => "eventually_due",
        }
    }
}

impl AsRef<str> for CreateAccountLinkCollectionOptionsFields {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountLinkCollectionOptionsFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateAccountLinkCollectionOptionsFields {
    fn default() -> Self {
        Self::CurrentlyDue
    }
}

/// An enum representing the possible values of an `CreateAccountLinkCollectionOptions`'s `future_requirements` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountLinkCollectionOptionsFutureRequirements {
    Include,
    Omit,
}

impl CreateAccountLinkCollectionOptionsFutureRequirements {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateAccountLinkCollectionOptionsFutureRequirements::Include => "include",
            CreateAccountLinkCollectionOptionsFutureRequirements::Omit => "omit",
        }
    }
}

impl AsRef<str> for CreateAccountLinkCollectionOptionsFutureRequirements {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountLinkCollectionOptionsFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateAccountLinkCollectionOptionsFutureRequirements {
    fn default() -> Self {
        Self::Include
    }
}
