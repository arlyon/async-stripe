
/// Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.
pub fn create(client: &stripe::Client, params: CreateAccountLink) -> stripe::Response<stripe_connect::AccountLink> {
    client.send_form("/account_links", params, http_types::Method::Post)
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountLink<'a> {
    /// The identifier of the account to create an account link for.
    pub account: &'a str,
    /// Which information the platform needs to collect from the user.
    ///
    /// One of `currently_due` or `eventually_due`.
    /// Default is `currently_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect: Option<CreateAccountLinkCollect>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
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
    pub type_: CreateAccountLinkType,
}
impl<'a> CreateAccountLink<'a> {
    pub fn new(account: &'a str, type_: CreateAccountLinkType) -> Self {
        Self { account, collect: Default::default(), expand: Default::default(), refresh_url: Default::default(), return_url: Default::default(), type_ }
    }
}
/// Which information the platform needs to collect from the user.
///
/// One of `currently_due` or `eventually_due`.
/// Default is `currently_due`.
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

impl AsRef<str> for CreateAccountLinkCollect {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// The type of account link the user is requesting.
///
/// Possible values are `account_onboarding` or `account_update`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountLinkType {
    AccountOnboarding,
    AccountUpdate,
    CustomAccountUpdate,
    CustomAccountVerification,
}

impl CreateAccountLinkType {
    pub fn as_str(self) -> &'static str {
        use CreateAccountLinkType::*;
        match self {
            AccountOnboarding => "account_onboarding",
            AccountUpdate => "account_update",
            CustomAccountUpdate => "custom_account_update",
            CustomAccountVerification => "custom_account_verification",
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
            "custom_account_update" => Ok(CustomAccountUpdate),
            "custom_account_verification" => Ok(CustomAccountVerification),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateAccountLinkType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
