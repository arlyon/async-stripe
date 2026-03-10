use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountLinkCollect {
    CurrentlyDue,
    EventuallyDue,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountLinkCollect {
    pub fn as_str(&self) -> &str {
        use CreateAccountLinkCollect::*;
        match self {
            CurrentlyDue => "currently_due",
            EventuallyDue => "eventually_due",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountLinkCollect {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollect::*;
        match s {
            "currently_due" => Ok(CurrentlyDue),
            "eventually_due" => Ok(EventuallyDue),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateAccountLinkCollect");
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Specifies the requirements that Stripe collects from connected accounts in the Connect Onboarding flow.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateAccountLinkCollectionOptions {
    /// Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and eventually_due requirements (`eventually_due`).
    /// If you don't specify `collection_options`, the default value is `currently_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<CreateAccountLinkCollectionOptionsFields>,
    /// Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding.
    /// The default value is `omit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<CreateAccountLinkCollectionOptionsFutureRequirements>,
}
impl CreateAccountLinkCollectionOptions {
    pub fn new() -> Self {
        Self { fields: None, future_requirements: None }
    }
}
impl Default for CreateAccountLinkCollectionOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies whether the platform collects only currently_due requirements (`currently_due`) or both currently_due and eventually_due requirements (`eventually_due`).
/// If you don't specify `collection_options`, the default value is `currently_due`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountLinkCollectionOptionsFields {
    CurrentlyDue,
    EventuallyDue,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountLinkCollectionOptionsFields {
    pub fn as_str(&self) -> &str {
        use CreateAccountLinkCollectionOptionsFields::*;
        match self {
            CurrentlyDue => "currently_due",
            EventuallyDue => "eventually_due",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountLinkCollectionOptionsFields {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollectionOptionsFields::*;
        match s {
            "currently_due" => Ok(CurrentlyDue),
            "eventually_due" => Ok(EventuallyDue),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountLinkCollectionOptionsFields"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Specifies whether the platform collects future_requirements in addition to requirements in Connect Onboarding.
/// The default value is `omit`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountLinkCollectionOptionsFutureRequirements {
    Include,
    Omit,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountLinkCollectionOptionsFutureRequirements {
    pub fn as_str(&self) -> &str {
        use CreateAccountLinkCollectionOptionsFutureRequirements::*;
        match self {
            Include => "include",
            Omit => "omit",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountLinkCollectionOptionsFutureRequirements {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkCollectionOptionsFutureRequirements::*;
        match s {
            "include" => Ok(Include),
            "omit" => Ok(Omit),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountLinkCollectionOptionsFutureRequirements"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of account link the user is requesting.
///
/// You can create Account Links of type `account_update` only for connected accounts where your platform is responsible for collecting requirements, including Custom accounts.
/// You can't create them for accounts that have access to a Stripe-hosted Dashboard.
/// If you use [Connect embedded components](/connect/get-started-connect-embedded-components), you can include components that allow your connected accounts to update their own information.
/// For an account without Stripe-hosted Dashboard access where Stripe is liable for negative balances, you must use embedded components.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountLinkType {
    AccountOnboarding,
    AccountUpdate,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountLinkType {
    pub fn as_str(&self) -> &str {
        use CreateAccountLinkType::*;
        match self {
            AccountOnboarding => "account_onboarding",
            AccountUpdate => "account_update",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountLinkType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountLinkType::*;
        match s {
            "account_onboarding" => Ok(AccountOnboarding),
            "account_update" => Ok(AccountUpdate),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateAccountLinkType");
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
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
