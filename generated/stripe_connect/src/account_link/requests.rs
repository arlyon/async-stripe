#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountLink<'a> {
    /// The identifier of the account to create an account link for.
    pub account: &'a str,
    /// The collect parameter is deprecated. Use `collection_options` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect: Option<CreateAccountLinkCollect>,
    /// Specifies the requirements that Stripe collects from connected accounts in the Connect Onboarding flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_options: Option<CreateAccountLinkCollectionOptions>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The URL the user will be redirected to if the account link is expired, has been previously-visited, or is otherwise invalid.
    /// The URL you specify should attempt to generate a new account link with the same parameters used to create the original account link, then redirect the user to the new account link's URL so they can continue with Connect Onboarding.
    /// If a new account link cannot be generated or the redirect fails you should display a useful error to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<&'a str>,
    /// The URL that the user will be redirected to upon leaving or completing the linked flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// The type of account link the user is requesting.
    /// Possible values are `account_onboarding` or `account_update`.
    #[serde(rename = "type")]
    pub type_: CreateAccountLinkType,
}
impl<'a> CreateAccountLink<'a> {
    pub fn new(account: &'a str, type_: CreateAccountLinkType) -> Self {
        Self {
            account,
            collect: None,
            collection_options: None,
            expand: None,
            refresh_url: None,
            return_url: None,
            type_,
        }
    }
}
/// The collect parameter is deprecated. Use `collection_options` instead.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountLinkCollect {
    CurrentlyDue,
    EventuallyDue,
}
impl CreateAccountLinkCollect {
    pub fn as_str(self) -> &'static str {
        use CreateAccountLinkCollect::*;
        match self {
            CurrentlyDue => "currently_due",
            EventuallyDue => "eventually_due",
        }
    }
}

impl std::str::FromStr for CreateAccountLinkCollect {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollect::*;
        match s {
            "currently_due" => Ok(CurrentlyDue),
            "eventually_due" => Ok(EventuallyDue),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateAccountLinkCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountLinkCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateAccountLinkCollect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specifies the requirements that Stripe collects from connected accounts in the Connect Onboarding flow.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountLinkCollectionOptions {
    /// Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and eventually_due requirements (`eventually_due`).
    /// If you don't specify `collection_options`, the default value is `currently_due`.
    pub fields: CreateAccountLinkCollectionOptionsFields,
    /// Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding.
    /// The default value is `omit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<CreateAccountLinkCollectionOptionsFutureRequirements>,
}
impl CreateAccountLinkCollectionOptions {
    pub fn new(fields: CreateAccountLinkCollectionOptionsFields) -> Self {
        Self { fields, future_requirements: None }
    }
}
/// Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and eventually_due requirements (`eventually_due`).
/// If you don't specify `collection_options`, the default value is `currently_due`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountLinkCollectionOptionsFields {
    CurrentlyDue,
    EventuallyDue,
}
impl CreateAccountLinkCollectionOptionsFields {
    pub fn as_str(self) -> &'static str {
        use CreateAccountLinkCollectionOptionsFields::*;
        match self {
            CurrentlyDue => "currently_due",
            EventuallyDue => "eventually_due",
        }
    }
}

impl std::str::FromStr for CreateAccountLinkCollectionOptionsFields {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollectionOptionsFields::*;
        match s {
            "currently_due" => Ok(CurrentlyDue),
            "eventually_due" => Ok(EventuallyDue),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateAccountLinkCollectionOptionsFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountLinkCollectionOptionsFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateAccountLinkCollectionOptionsFields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding.
/// The default value is `omit`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountLinkCollectionOptionsFutureRequirements {
    Include,
    Omit,
}
impl CreateAccountLinkCollectionOptionsFutureRequirements {
    pub fn as_str(self) -> &'static str {
        use CreateAccountLinkCollectionOptionsFutureRequirements::*;
        match self {
            Include => "include",
            Omit => "omit",
        }
    }
}

impl std::str::FromStr for CreateAccountLinkCollectionOptionsFutureRequirements {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollectionOptionsFutureRequirements::*;
        match s {
            "include" => Ok(Include),
            "omit" => Ok(Omit),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateAccountLinkCollectionOptionsFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountLinkCollectionOptionsFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateAccountLinkCollectionOptionsFutureRequirements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The type of account link the user is requesting.
/// Possible values are `account_onboarding` or `account_update`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountLinkType {
    AccountOnboarding,
    AccountUpdate,
}
impl CreateAccountLinkType {
    pub fn as_str(self) -> &'static str {
        use CreateAccountLinkType::*;
        match self {
            AccountOnboarding => "account_onboarding",
            AccountUpdate => "account_update",
        }
    }
}

impl std::str::FromStr for CreateAccountLinkType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkType::*;
        match s {
            "account_onboarding" => Ok(AccountOnboarding),
            "account_update" => Ok(AccountUpdate),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateAccountLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateAccountLinkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateAccountLink<'a> {
    /// Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_connect::AccountLink> {
        client.send_form("/account_links", self, http_types::Method::Post)
    }
}
