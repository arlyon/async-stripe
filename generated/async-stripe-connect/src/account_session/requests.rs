use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
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
pub struct CreateAccountSessionComponents {
    /// Configuration for the [account management](/connect/supported-embedded-components/account-management/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_management: Option<AccountConfigParam>,
    /// Configuration for the [account onboarding](/connect/supported-embedded-components/account-onboarding/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<AccountConfigParam>,
    /// Configuration for the [balance report](/connect/supported-embedded-components/financial-reports#balance-report) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_report: Option<BaseConfigParam>,
    /// Configuration for the [balances](/connect/supported-embedded-components/balances/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<PayoutsConfigParam>,
    /// Configuration for the [disputes list](/connect/supported-embedded-components/disputes-list/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disputes_list: Option<CreateAccountSessionComponentsDisputesList>,
    /// Configuration for the [documents](/connect/supported-embedded-components/documents/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<BaseConfigParam>,
    /// Configuration for the [financial account](/connect/supported-embedded-components/financial-account/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<CreateAccountSessionComponentsFinancialAccount>,
    /// Configuration for the [financial account transactions](/connect/supported-embedded-components/financial-account-transactions/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account_transactions:
        Option<CreateAccountSessionComponentsFinancialAccountTransactions>,
    /// Configuration for the [instant payouts promotion](/connect/supported-embedded-components/instant-payouts-promotion/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_payouts_promotion: Option<CreateAccountSessionComponentsInstantPayoutsPromotion>,
    /// Configuration for the [issuing card](/connect/supported-embedded-components/issuing-card/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<CreateAccountSessionComponentsIssuingCard>,
    /// Configuration for the [issuing cards list](/connect/supported-embedded-components/issuing-cards-list/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_cards_list: Option<CreateAccountSessionComponentsIssuingCardsList>,
    /// Configuration for the [notification banner](/connect/supported-embedded-components/notification-banner/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_banner: Option<AccountConfigParam>,
    /// Configuration for the [payment details](/connect/supported-embedded-components/payment-details/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<CreateAccountSessionComponentsPaymentDetails>,
    /// Configuration for the [payment disputes](/connect/supported-embedded-components/payment-disputes/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_disputes: Option<CreateAccountSessionComponentsPaymentDisputes>,
    /// Configuration for the [payments](/connect/supported-embedded-components/payments/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountSessionComponentsPayments>,
    /// Configuration for the [payout details](/connect/supported-embedded-components/payout-details/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_details: Option<BaseConfigParam>,
    /// Configuration for the [payout reconciliation report](/connect/supported-embedded-components/financial-reports#payout-reconciliation-report) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_reconciliation_report: Option<BaseConfigParam>,
    /// Configuration for the [payouts](/connect/supported-embedded-components/payouts/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutsConfigParam>,
    /// Configuration for the [payouts list](/connect/supported-embedded-components/payouts-list/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_list: Option<BaseConfigParam>,
    /// Configuration for the [tax registrations](/connect/supported-embedded-components/tax-registrations/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registrations: Option<BaseConfigParam>,
    /// Configuration for the [tax settings](/connect/supported-embedded-components/tax-settings/) embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_settings: Option<BaseConfigParam>,
}
impl CreateAccountSessionComponents {
    pub fn new() -> Self {
        Self {
            account_management: None,
            account_onboarding: None,
            balance_report: None,
            balances: None,
            disputes_list: None,
            documents: None,
            financial_account: None,
            financial_account_transactions: None,
            instant_payouts_promotion: None,
            issuing_card: None,
            issuing_cards_list: None,
            notification_banner: None,
            payment_details: None,
            payment_disputes: None,
            payments: None,
            payout_details: None,
            payout_reconciliation_report: None,
            payouts: None,
            payouts_list: None,
            tax_registrations: None,
            tax_settings: None,
        }
    }
}
impl Default for CreateAccountSessionComponents {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [disputes list](/connect/supported-embedded-components/disputes-list/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsDisputesList {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsDisputesListFeatures>,
}
impl CreateAccountSessionComponentsDisputesList {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsDisputesListFeatures {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,
    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,
    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,
    /// Whether sending refunds is enabled. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}
impl CreateAccountSessionComponentsDisputesListFeatures {
    pub fn new() -> Self {
        Self {
            capture_payments: None,
            destination_on_behalf_of_charge_management: None,
            dispute_management: None,
            refund_management: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsDisputesListFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [financial account](/connect/supported-embedded-components/financial-account/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsFinancialAccount {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsFinancialAccountFeatures>,
}
impl CreateAccountSessionComponentsFinancialAccount {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsFinancialAccountFeatures {
    /// Whether Stripe user authentication is disabled.
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether external account collection is enabled.
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
impl CreateAccountSessionComponentsFinancialAccountFeatures {
    pub fn new() -> Self {
        Self {
            disable_stripe_user_authentication: None,
            external_account_collection: None,
            send_money: None,
            transfer_balance: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsFinancialAccountFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [financial account transactions](/connect/supported-embedded-components/financial-account-transactions/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsFinancialAccountTransactions {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsFinancialAccountTransactionsFeatures>,
}
impl CreateAccountSessionComponentsFinancialAccountTransactions {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsFinancialAccountTransactionsFeatures {
    /// Whether to allow card spend dispute management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_spend_dispute_management: Option<bool>,
}
impl CreateAccountSessionComponentsFinancialAccountTransactionsFeatures {
    pub fn new() -> Self {
        Self { card_spend_dispute_management: None }
    }
}
impl Default for CreateAccountSessionComponentsFinancialAccountTransactionsFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [instant payouts promotion](/connect/supported-embedded-components/instant-payouts-promotion/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsInstantPayoutsPromotion {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsInstantPayoutsPromotionFeatures>,
}
impl CreateAccountSessionComponentsInstantPayoutsPromotion {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsInstantPayoutsPromotionFeatures {
    /// Whether Stripe user authentication is disabled.
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether external account collection is enabled.
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
    /// Whether to allow creation of instant payouts.
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_payouts: Option<bool>,
}
impl CreateAccountSessionComponentsInstantPayoutsPromotionFeatures {
    pub fn new() -> Self {
        Self {
            disable_stripe_user_authentication: None,
            external_account_collection: None,
            instant_payouts: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsInstantPayoutsPromotionFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [issuing card](/connect/supported-embedded-components/issuing-card/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsIssuingCard {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsIssuingCardFeatures>,
}
impl CreateAccountSessionComponentsIssuingCard {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
impl CreateAccountSessionComponentsIssuingCardFeatures {
    pub fn new() -> Self {
        Self {
            card_management: None,
            card_spend_dispute_management: None,
            cardholder_management: None,
            spend_control_management: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsIssuingCardFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [issuing cards list](/connect/supported-embedded-components/issuing-cards-list/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsIssuingCardsList {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsIssuingCardsListFeatures>,
}
impl CreateAccountSessionComponentsIssuingCardsList {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether to allow spend control management features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_control_management: Option<bool>,
}
impl CreateAccountSessionComponentsIssuingCardsListFeatures {
    pub fn new() -> Self {
        Self {
            card_management: None,
            card_spend_dispute_management: None,
            cardholder_management: None,
            disable_stripe_user_authentication: None,
            spend_control_management: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsIssuingCardsListFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [payment details](/connect/supported-embedded-components/payment-details/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsPaymentDetails {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPaymentDetailsFeatures>,
}
impl CreateAccountSessionComponentsPaymentDetails {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsPaymentDetailsFeatures {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,
    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,
    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,
    /// Whether sending refunds is enabled. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}
impl CreateAccountSessionComponentsPaymentDetailsFeatures {
    pub fn new() -> Self {
        Self {
            capture_payments: None,
            destination_on_behalf_of_charge_management: None,
            dispute_management: None,
            refund_management: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsPaymentDetailsFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [payment disputes](/connect/supported-embedded-components/payment-disputes/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsPaymentDisputes {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPaymentDisputesFeatures>,
}
impl CreateAccountSessionComponentsPaymentDisputes {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsPaymentDisputesFeatures {
    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,
    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,
    /// Whether sending refunds is enabled. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}
impl CreateAccountSessionComponentsPaymentDisputesFeatures {
    pub fn new() -> Self {
        Self {
            destination_on_behalf_of_charge_management: None,
            dispute_management: None,
            refund_management: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsPaymentDisputesFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for the [payments](/connect/supported-embedded-components/payments/) embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsPayments {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// The list of features enabled in the embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateAccountSessionComponentsPaymentsFeatures>,
}
impl CreateAccountSessionComponentsPayments {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// The list of features enabled in the embedded component.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSessionComponentsPaymentsFeatures {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_payments: Option<bool>,
    /// Whether connected accounts can manage destination charges that are created on behalf of them.
    /// This is `false` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_on_behalf_of_charge_management: Option<bool>,
    /// Whether responding to disputes is enabled, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_management: Option<bool>,
    /// Whether sending refunds is enabled. This is `true` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_management: Option<bool>,
}
impl CreateAccountSessionComponentsPaymentsFeatures {
    pub fn new() -> Self {
        Self {
            capture_payments: None,
            destination_on_behalf_of_charge_management: None,
            dispute_management: None,
            refund_management: None,
        }
    }
}
impl Default for CreateAccountSessionComponentsPaymentsFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.
#[derive(Clone, Debug, serde::Serialize)]
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
pub struct AccountFeaturesParam {
    /// Whether Stripe user authentication is disabled.
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether external account collection is enabled.
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
}
impl AccountFeaturesParam {
    pub fn new() -> Self {
        Self { disable_stripe_user_authentication: None, external_account_collection: None }
    }
}
impl Default for AccountFeaturesParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct BaseConfigParam {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    /// An empty list, because this embedded component has no features.
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
pub struct PayoutsFeaturesParam {
    /// Whether Stripe user authentication is disabled.
    /// This value can only be `true` for accounts where `controller.requirement_collection` is `application` for the account.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don't set `external_account_collection`, it defaults to `true` and `disable_stripe_user_authentication` defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether to allow payout schedule to be changed.
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_payout_schedule: Option<bool>,
    /// Whether external account collection is enabled.
    /// This feature can only be `false` for accounts where you’re responsible for collecting updated information when requirements are due or change, like Custom accounts.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
    /// Whether to allow creation of instant payouts.
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_payouts: Option<bool>,
    /// Whether to allow creation of standard payouts.
    /// Defaults to `true` when `controller.losses.payments` is set to `stripe` for the account, otherwise `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_payouts: Option<bool>,
}
impl PayoutsFeaturesParam {
    pub fn new() -> Self {
        Self {
            disable_stripe_user_authentication: None,
            edit_payout_schedule: None,
            external_account_collection: None,
            instant_payouts: None,
            standard_payouts: None,
        }
    }
}
impl Default for PayoutsFeaturesParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
