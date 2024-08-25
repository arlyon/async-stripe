use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateAccountSessionBuilder {
    account: String,
    components: CreateAccountSessionComponents,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CreateAccountSessionBuilder {
    fn new(
        account: impl Into<String>,
        components: impl Into<CreateAccountSessionComponents>,
    ) -> Self {
        Self { account: account.into(), components: components.into(), expand: None }
    }
}
/// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
/// whether it has been enabled or not).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateAccountSessionComponents {
    /// Configuration for the account management embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_management: Option<AccountConfigParam>,
    /// Configuration for the account onboarding embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<AccountConfigParam>,
    /// Configuration for the balances embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<PayoutsConfigParam>,
    /// Configuration for the documents embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<BaseConfigParam>,
    /// Configuration for the notification banner embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_banner: Option<AccountConfigParam>,
    /// Configuration for the payment details embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentsConfigParam>,
    /// Configuration for the payments embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsConfigParam>,
    /// Configuration for the payouts embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutsConfigParam>,
    /// Configuration for the payouts list embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_list: Option<BaseConfigParam>,
}
impl CreateAccountSessionComponents {
    pub fn new() -> Self {
        Self {
            account_management: None,
            account_onboarding: None,
            balances: None,
            documents: None,
            notification_banner: None,
            payment_details: None,
            payments: None,
            payouts: None,
            payouts_list: None,
        }
    }
}
impl Default for CreateAccountSessionComponents {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateAccountSession {
    inner: CreateAccountSessionBuilder,
}
impl CreateAccountSession {
    /// Construct a new `CreateAccountSession`.
    pub fn new(
        account: impl Into<String>,
        components: impl Into<CreateAccountSessionComponents>,
    ) -> Self {
        Self { inner: CreateAccountSessionBuilder::new(account.into(), components.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateAccountSession {
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

impl StripeRequest for CreateAccountSession {
    type Output = stripe_connect::AccountSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/account_sessions").form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct AccountFeaturesParam {
    /// Whether to allow platforms to control bank account collection for their connected accounts.
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
}
impl AccountFeaturesParam {
    pub fn new() -> Self {
        Self { external_account_collection: None }
    }
}
impl Default for AccountFeaturesParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct PayoutsFeaturesParam {
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
impl PayoutsFeaturesParam {
    pub fn new() -> Self {
        Self { edit_payout_schedule: None, instant_payouts: None, standard_payouts: None }
    }
}
impl Default for PayoutsFeaturesParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct BaseConfigParam {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub features: Option<miniserde::json::Value>,
}
impl BaseConfigParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct PaymentsFeaturesParam {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,
    /// Whether to allow connected accounts to manage destination charges that are created on behalf of them.
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,
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
        Self {
            capture_payments: None,
            destination_on_behalf_of_charge_management: None,
            dispute_management: None,
            refund_management: None,
        }
    }
}
impl Default for PaymentsFeaturesParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct AccountConfigParam {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<AccountFeaturesParam>,
}
impl AccountConfigParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct PayoutsConfigParam {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PayoutsFeaturesParam>,
}
impl PayoutsConfigParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct PaymentsConfigParam {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PaymentsFeaturesParam>,
}
impl PaymentsConfigParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
