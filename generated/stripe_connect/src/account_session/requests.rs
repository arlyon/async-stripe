#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSession<'a> {
    /// The identifier of the account to create an Account Session for.
    pub account: &'a str,
    /// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
    /// whether it has been enabled or not).
    pub components: CreateAccountSessionComponents<'a>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountSession<'a> {
    pub fn new(account: &'a str, components: CreateAccountSessionComponents<'a>) -> Self {
        Self { account, components, expand: None }
    }
}
/// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
/// whether it has been enabled or not).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSessionComponents<'a> {
    /// Configuration for the account onboarding embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<CreateAccountSessionComponentsAccountOnboarding<'a>>,
    /// Configuration for the payment details embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentsConfigParam>,
    /// Configuration for the payments embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsConfigParam>,
    /// Configuration for the payouts embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<CreateAccountSessionComponentsPayouts>,
}
impl<'a> CreateAccountSessionComponents<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the account onboarding embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsAccountOnboarding<'a> {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<&'a serde_json::Value>,
}
impl<'a> CreateAccountSessionComponentsAccountOnboarding<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, features: None }
    }
}
/// Configuration for the payouts embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsPayouts {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPayoutsFeatures>,
}
impl CreateAccountSessionComponentsPayouts {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSessionComponentsPayoutsFeatures {
    /// Whether to allow payout schedule to be changed.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_payout_schedule: Option<bool>,
    /// Whether to allow creation of instant payouts.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_payouts: Option<bool>,
    /// Whether to allow creation of standard payouts.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_payouts: Option<bool>,
}
impl CreateAccountSessionComponentsPayoutsFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreateAccountSession<'a> {
    /// Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_connect::AccountSession> {
        client.send_form("/account_sessions", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PaymentsFeaturesParam {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,
    /// Whether to allow responding to disputes, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,
    /// Whether to allow sending refunds. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}
impl PaymentsFeaturesParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PaymentsConfigParam {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PaymentsFeaturesParam>,
}
impl PaymentsConfigParam {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, features: None }
    }
}
