use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateAccountLinkBuilder {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    collect: Option<CreateAccountLinkCollect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_options: Option<CreateAccountLinkCollectionOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(rename = "type")]
    type_: CreateAccountLinkType,
}
impl CreateAccountLinkBuilder {
    fn new(account: impl Into<String>, type_: impl Into<CreateAccountLinkType>) -> Self {
        Self {
            account: account.into(),
            collect: None,
            collection_options: None,
            expand: None,
            refresh_url: None,
            return_url: None,
            type_: type_.into(),
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollect::*;
        match s {
            "currently_due" => Ok(CurrentlyDue),
            "eventually_due" => Ok(EventuallyDue),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountLinkCollect {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateAccountLinkCollect"))
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
    pub fn new(fields: impl Into<CreateAccountLinkCollectionOptionsFields>) -> Self {
        Self { fields: fields.into(), future_requirements: None }
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollectionOptionsFields::*;
        match s {
            "currently_due" => Ok(CurrentlyDue),
            "eventually_due" => Ok(EventuallyDue),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountLinkCollectionOptionsFields {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateAccountLinkCollectionOptionsFields")
        })
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollectionOptionsFutureRequirements::*;
        match s {
            "include" => Ok(Include),
            "omit" => Ok(Omit),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountLinkCollectionOptionsFutureRequirements {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateAccountLinkCollectionOptionsFutureRequirements",
            )
        })
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkType::*;
        match s {
            "account_onboarding" => Ok(AccountOnboarding),
            "account_update" => Ok(AccountUpdate),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountLinkType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateAccountLinkType"))
    }
}
/// Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountLink {
    inner: CreateAccountLinkBuilder,
}
impl CreateAccountLink {
    /// Construct a new `CreateAccountLink`.
    pub fn new(account: impl Into<String>, type_: impl Into<CreateAccountLinkType>) -> Self {
        Self { inner: CreateAccountLinkBuilder::new(account.into(), type_.into()) }
    }
    /// The collect parameter is deprecated. Use `collection_options` instead.
    pub fn collect(mut self, collect: impl Into<CreateAccountLinkCollect>) -> Self {
        self.inner.collect = Some(collect.into());
        self
    }
    /// Specifies the requirements that Stripe collects from connected accounts in the Connect Onboarding flow.
    pub fn collection_options(
        mut self,
        collection_options: impl Into<CreateAccountLinkCollectionOptions>,
    ) -> Self {
        self.inner.collection_options = Some(collection_options.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The URL the user will be redirected to if the account link is expired, has been previously-visited, or is otherwise invalid.
    /// The URL you specify should attempt to generate a new account link with the same parameters used to create the original account link, then redirect the user to the new account link's URL so they can continue with Connect Onboarding.
    /// If a new account link cannot be generated or the redirect fails you should display a useful error to the user.
    pub fn refresh_url(mut self, refresh_url: impl Into<String>) -> Self {
        self.inner.refresh_url = Some(refresh_url.into());
        self
    }
    /// The URL that the user will be redirected to upon leaving or completing the linked flow.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
}
impl CreateAccountLink {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateAccountLink {
    type Output = stripe_connect::AccountLink;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/account_links").form(&self.inner)
    }
}
