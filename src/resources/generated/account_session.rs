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

    pub balances: ConnectEmbeddedPayoutsConfig,

    pub disputes_list: ConnectEmbeddedDisputesListConfig,

    pub documents: ConnectEmbeddedBaseConfigClaim,

    pub financial_account: ConnectEmbeddedFinancialAccountConfigClaim,

    pub financial_account_transactions: ConnectEmbeddedFinancialAccountTransactionsConfigClaim,

    pub issuing_card: ConnectEmbeddedIssuingCardConfigClaim,

    pub issuing_cards_list: ConnectEmbeddedIssuingCardsListConfigClaim,

    pub notification_banner: ConnectEmbeddedAccountConfigClaim,

    pub payment_details: ConnectEmbeddedPaymentsConfigClaim,

    pub payment_disputes: ConnectEmbeddedPaymentDisputesConfig,

    pub payments: ConnectEmbeddedPaymentsConfigClaim,

    pub payouts: ConnectEmbeddedPayoutsConfig,

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

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    pub disable_stripe_user_authentication: bool,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
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
pub struct ConnectEmbeddedDisputesListConfig {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedDisputesListFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedDisputesListFeatures {

    /// Whether to allow capturing and cancelling payment intents.
    ///
    /// This is `true` by default.
    pub capture_payments: bool,

    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    pub destination_on_behalf_of_charge_management: bool,

    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    pub dispute_management: bool,

    /// Whether sending refunds is enabled.
    ///
    /// This is `true` by default.
    pub refund_management: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedFinancialAccountConfigClaim {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedFinancialAccountFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedFinancialAccountFeatures {

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    pub disable_stripe_user_authentication: bool,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    pub external_account_collection: bool,

    /// Whether to allow sending money.
    pub send_money: bool,

    /// Whether to allow transferring balance.
    pub transfer_balance: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedFinancialAccountTransactionsConfigClaim {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedFinancialAccountTransactionsFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedFinancialAccountTransactionsFeatures {

    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedIssuingCardConfigClaim {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedIssuingCardFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedIssuingCardFeatures {

    /// Whether to allow card management features.
    pub card_management: bool,

    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,

    /// Whether to allow cardholder management features.
    pub cardholder_management: bool,

    /// Whether to allow spend control management features.
    pub spend_control_management: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedIssuingCardsListConfigClaim {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedIssuingCardsListFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedIssuingCardsListFeatures {

    /// Whether to allow card management features.
    pub card_management: bool,

    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,

    /// Whether to allow cardholder management features.
    pub cardholder_management: bool,

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    pub disable_stripe_user_authentication: bool,

    /// Whether to allow spend control management features.
    pub spend_control_management: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedPaymentDisputesConfig {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    pub features: ConnectEmbeddedPaymentDisputesFeatures,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedPaymentDisputesFeatures {

    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    pub destination_on_behalf_of_charge_management: bool,

    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    pub dispute_management: bool,

    /// Whether sending refunds is enabled.
    ///
    /// This is `true` by default.
    pub refund_management: bool,
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

    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    pub destination_on_behalf_of_charge_management: bool,

    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    pub dispute_management: bool,

    /// Whether sending refunds is enabled.
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

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    pub disable_stripe_user_authentication: bool,

    /// Whether to allow payout schedule to be changed.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    pub edit_payout_schedule: bool,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    pub external_account_collection: bool,

    /// Whether to allow creation of instant payouts.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    pub instant_payouts: bool,

    /// Whether to allow creation of standard payouts.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
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

    /// Configuration for the [account management](/connect/supported-embedded-components/account-management/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_management: Option<CreateAccountSessionComponentsAccountManagement>,

    /// Configuration for the [account onboarding](/connect/supported-embedded-components/account-onboarding/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<CreateAccountSessionComponentsAccountOnboarding>,

    /// Configuration for the [balances](/connect/supported-embedded-components/balances/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<CreateAccountSessionComponentsBalances>,

    /// Configuration for the [disputes list](/connect/supported-embedded-components/disputes-list/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disputes_list: Option<CreateAccountSessionComponentsDisputesList>,

    /// Configuration for the [documents](/connect/supported-embedded-components/documents/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateAccountSessionComponentsDocuments>,

    /// Configuration for the [financial account](/connect/supported-embedded-components/financial-account/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<CreateAccountSessionComponentsFinancialAccount>,

    /// Configuration for the [financial account transactions](/connect/supported-embedded-components/financial-account-transactions/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account_transactions: Option<CreateAccountSessionComponentsFinancialAccountTransactions>,

    /// Configuration for the [issuing card](/connect/supported-embedded-components/issuing-card/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<CreateAccountSessionComponentsIssuingCard>,

    /// Configuration for the [issuing cards list](/connect/supported-embedded-components/issuing-cards-list/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_cards_list: Option<CreateAccountSessionComponentsIssuingCardsList>,

    /// Configuration for the [notification banner](/connect/supported-embedded-components/notification-banner/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_banner: Option<CreateAccountSessionComponentsNotificationBanner>,

    /// Configuration for the [payment details](/connect/supported-embedded-components/payment-details/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<CreateAccountSessionComponentsPaymentDetails>,

    /// Configuration for the [payment disputes](/connect/supported-embedded-components/payment-disputes/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_disputes: Option<CreateAccountSessionComponentsPaymentDisputes>,

    /// Configuration for the [payments](/connect/supported-embedded-components/payments/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountSessionComponentsPayments>,

    /// Configuration for the [payouts](/connect/supported-embedded-components/payouts/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<CreateAccountSessionComponentsPayouts>,

    /// Configuration for the [payouts list](/connect/supported-embedded-components/payouts-list/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_list: Option<CreateAccountSessionComponentsPayoutsList>,

    /// Configuration for the [tax registrations](/connect/supported-embedded-components/tax-registrations/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registrations: Option<CreateAccountSessionComponentsTaxRegistrations>,

    /// Configuration for the [tax settings](/connect/supported-embedded-components/tax-settings/) embedded component.
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
pub struct CreateAccountSessionComponentsDisputesList {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsDisputesListFeatures>,
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
pub struct CreateAccountSessionComponentsFinancialAccount {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsFinancialAccountFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsFinancialAccountTransactions {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsFinancialAccountTransactionsFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsIssuingCard {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsIssuingCardFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsIssuingCardsList {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsIssuingCardsListFeatures>,
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
pub struct CreateAccountSessionComponentsPaymentDisputes {

    /// Whether the embedded component is enabled.
    pub enabled: bool,

    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPaymentDisputesFeatures>,
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

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsAccountOnboardingFeatures {

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsBalancesFeatures {

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,

    /// Whether to allow payout schedule to be changed.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_payout_schedule: Option<bool>,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,

    /// Whether to allow creation of instant payouts.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_payouts: Option<bool>,

    /// Whether to allow creation of standard payouts.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_payouts: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsDisputesListFeatures {

    /// Whether to allow capturing and cancelling payment intents.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,

    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,

    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,

    /// Whether sending refunds is enabled.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsDocumentsFeatures {
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsFinancialAccountFeatures {

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,

    /// Whether to allow sending money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_money: Option<bool>,

    /// Whether to allow transferring balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_balance: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsFinancialAccountTransactionsFeatures {

    /// Whether to allow card spend dispute management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_spend_dispute_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsIssuingCardFeatures {

    /// Whether to allow card management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_management: Option<bool>,

    /// Whether to allow card spend dispute management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_spend_dispute_management: Option<bool>,

    /// Whether to allow cardholder management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_management: Option<bool>,

    /// Whether to allow spend control management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_control_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsIssuingCardsListFeatures {

    /// Whether to allow card management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_management: Option<bool>,

    /// Whether to allow card spend dispute management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_spend_dispute_management: Option<bool>,

    /// Whether to allow cardholder management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_management: Option<bool>,

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,

    /// Whether to allow spend control management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_control_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsNotificationBannerFeatures {

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
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

    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,

    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,

    /// Whether sending refunds is enabled.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPaymentDisputesFeatures {

    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,

    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,

    /// Whether sending refunds is enabled.
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

    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    ///
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,

    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,

    /// Whether sending refunds is enabled.
    ///
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsPayoutsFeatures {

    /// Whether Stripe user authentication is disabled.
    ///
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,

    /// Whether to allow payout schedule to be changed.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_payout_schedule: Option<bool>,

    /// Whether external account collection is enabled.
    ///
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,

    /// Whether to allow creation of instant payouts.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_payouts: Option<bool>,

    /// Whether to allow creation of standard payouts.
    ///
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
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
