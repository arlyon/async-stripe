// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{AccountId};
use crate::params::{Expand, Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ConnectEmbeddedMethodAccountSessionCreateMethodAccountSession".
///
/// For more details see <https://stripe.com/docs/api/account_sessions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSession {

    /// The ID of the account the AccountSession was created for.
    pub account: String,

    /// The client secret of this AccountSession.
    ///
    /// Used on the client to set up secure access to the given `account`.  The client secret can be used to provide access to `account` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the connected account.
    /// Make sure that you have TLS enabled on any page that includes the client secret.  Refer to our docs to [setup Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components) and learn about how `client_secret` should be handled.
    pub client_secret: String,

    pub components: ConnectEmbeddedAccountSessionCreateComponents,

    /// The timestamp at which this AccountSession will expire.
    pub expires_at: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl AccountSession {

    /// Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.
    pub fn create(client: &Client, params: CreateAccountSession<'_>) -> Response<AccountSession> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/account_sessions", &params)
    }
}

impl Object for AccountSession {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "account_session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {

    pub account_onboarding: ConnectEmbeddedBaseConfigClaim,

    pub payment_details: ConnectEmbeddedPaymentsConfig,

    pub payments: ConnectEmbeddedPaymentsConfig,

    pub payouts: ConnectEmbeddedPayoutsConfig,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedBaseConfigClaim {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedBaseFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedBaseFeatures {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedPaymentsConfig {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedPaymentsFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedPaymentsFeatures {

    /// Whether to allow capturing and cancelling payment intents.
    ///
    /// This is `true` by default.
    pub capture_payments: bool,

    /// Whether to allow responding to disputes, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    pub dispute_management: bool,

    /// Whether to allow sending refunds.
    ///
    /// This is `true` by default.
    pub refund_management: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedPayoutsConfig {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedPayoutsFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedPayoutsFeatures {

    /// Whether to allow payout schedule to be changed.
    ///
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub edit_payout_schedule: bool,

    /// Whether to allow creation of instant payouts.
    ///
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub instant_payouts: bool,

    /// Whether to allow creation of standard payouts.
    ///
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub standard_payouts: bool,
}

/// The parameters for `AccountSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateAccountSession<'a> {

    /// The identifier of the account to create an Account Session for.
    pub account: AccountId,

    /// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
    ///
    /// whether it has been enabled or not).
    pub components: CreateAccountSessionComponents,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl<'a> CreateAccountSession<'a> {
    pub fn new(account: AccountId, components: CreateAccountSessionComponents) -> Self {
        CreateAccountSession {
            account,
            components,
            expand: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponents {

    /// Configuration for the account onboarding embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<CreateAccountSessionComponentsAccountOnboarding>,

    /// Configuration for the payment details embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<CreateAccountSessionComponentsPaymentDetails>,

    /// Configuration for the payments embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountSessionComponentsPayments>,

    /// Configuration for the payouts embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<CreateAccountSessionComponentsPayouts>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsAccountOnboarding {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsAccountOnboardingFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPaymentDetails {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPaymentDetailsFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPayments {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPaymentsFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPayouts {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPayoutsFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsAccountOnboardingFeatures {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPaymentDetailsFeatures {

    /// Whether to allow capturing and cancelling payment intents.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,

    /// Whether to allow responding to disputes, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,

    /// Whether to allow sending refunds.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPaymentsFeatures {

    /// Whether to allow capturing and cancelling payment intents.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,

    /// Whether to allow responding to disputes, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,

    /// Whether to allow sending refunds.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPayoutsFeatures {

    /// Whether to allow payout schedule to be changed.
    ///
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_payout_schedule: Option<bool>,

    /// Whether to allow creation of instant payouts.
    ///
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_payouts: Option<bool>,

    /// Whether to allow creation of standard payouts.
    ///
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_payouts: Option<bool>,
}
