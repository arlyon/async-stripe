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

    pub account_management: ConnectEmbeddedAccountConfigClaim,

    pub account_onboarding: ConnectEmbeddedAccountConfigClaim,

    pub balances: ConnectEmbeddedPayoutsConfigClaim,

    pub documents: ConnectEmbeddedBaseConfigClaim,

    pub notification_banner: ConnectEmbeddedAccountConfigClaim,

    pub payment_details: ConnectEmbeddedPaymentsConfigClaim,

    pub payments: ConnectEmbeddedPaymentsConfigClaim,

    pub payouts: ConnectEmbeddedPayoutsConfigClaim,

    pub payouts_list: ConnectEmbeddedBaseConfigClaim,

    pub tax_registrations: ConnectEmbeddedBaseConfigClaim,

    pub tax_settings: ConnectEmbeddedBaseConfigClaim,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedAccountConfigClaim {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedAccountFeaturesClaim,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedAccountFeaturesClaim {

    /// Whether to allow platforms to control bank account collection for their connected accounts.
    ///
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    pub external_account_collection: bool,
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
pub struct ConnectEmbeddedPaymentsConfigClaim {

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

    /// Whether to allow connected accounts to manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    pub destination_on_behalf_of_charge_management: bool,

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
pub struct ConnectEmbeddedPayoutsConfigClaim {

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

    /// Whether to allow platforms to control bank account collection for their connected accounts.
    ///
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    pub external_account_collection: bool,

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

    /// Configuration for the account management embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_management: Option<CreateAccountSessionComponentsAccountManagement>,

    /// Configuration for the account onboarding embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<CreateAccountSessionComponentsAccountOnboarding>,

    /// Configuration for the balances embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<CreateAccountSessionComponentsBalances>,

    /// Configuration for the documents embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateAccountSessionComponentsDocuments>,

    /// Configuration for the notification banner embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_banner: Option<CreateAccountSessionComponentsNotificationBanner>,

    /// Configuration for the payment details embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<CreateAccountSessionComponentsPaymentDetails>,

    /// Configuration for the payments embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountSessionComponentsPayments>,

    /// Configuration for the payouts embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<CreateAccountSessionComponentsPayouts>,

    /// Configuration for the payouts list embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_list: Option<CreateAccountSessionComponentsPayoutsList>,

    /// Configuration for the tax registrations embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registrations: Option<CreateAccountSessionComponentsTaxRegistrations>,

    /// Configuration for the tax settings embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_settings: Option<CreateAccountSessionComponentsTaxSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsAccountManagement {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsAccountManagementFeatures>,
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
pub struct CreateAccountSessionComponentsBalances {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsBalancesFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsDocuments {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsDocumentsFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsNotificationBanner {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsNotificationBannerFeatures>,
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
pub struct CreateAccountSessionComponentsPayoutsList {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPayoutsListFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsTaxRegistrations {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsTaxRegistrationsFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsTaxSettings {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsTaxSettingsFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsAccountManagementFeatures {

    /// Whether to allow platforms to control bank account collection for their connected accounts.
    ///
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsAccountOnboardingFeatures {

    /// Whether to allow platforms to control bank account collection for their connected accounts.
    ///
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsBalancesFeatures {

    /// Whether to allow payout schedule to be changed.
    ///
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_payout_schedule: Option<bool>,

    /// Whether to allow platforms to control bank account collection for their connected accounts.
    ///
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsDocumentsFeatures {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsNotificationBannerFeatures {

    /// Whether to allow platforms to control bank account collection for their connected accounts.
    ///
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPaymentDetailsFeatures {

    /// Whether to allow capturing and cancelling payment intents.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,

    /// Whether to allow connected accounts to manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,

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

    /// Whether to allow connected accounts to manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,

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

    /// Whether to allow platforms to control bank account collection for their connected accounts.
    ///
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPayoutsListFeatures {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsTaxRegistrationsFeatures {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsTaxSettingsFeatures {
}
