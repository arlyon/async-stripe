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
    /// Configuration for the financial account embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<CreateAccountSessionComponentsFinancialAccount>,
    /// Configuration for the financial account transactions embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account_transactions:
        Option<CreateAccountSessionComponentsFinancialAccountTransactions>,
    /// Configuration for the issuing card embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<CreateAccountSessionComponentsIssuingCard>,
    /// Configuration for the issuing cards list embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_cards_list: Option<CreateAccountSessionComponentsIssuingCardsList>,
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
    /// Configuration for the tax registrations embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registrations: Option<BaseConfigParam>,
    /// Configuration for the tax settings embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_settings: Option<BaseConfigParam>,
}
impl CreateAccountSessionComponents {
    pub fn new() -> Self {
        Self {
            account_management: None,
            account_onboarding: None,
            balances: None,
            documents: None,
            financial_account: None,
            financial_account_transactions: None,
            issuing_card: None,
            issuing_cards_list: None,
            notification_banner: None,
            payment_details: None,
            payments: None,
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
/// Configuration for the financial account embedded component.
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
    /// Disables Stripe user authentication for this embedded component.
    /// This value can only be true for accounts where `controller.requirement_collection` is `application`.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don’t set `external_account_collection`, it defaults to true and `disable_stripe_user_authentication` defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether to allow external accounts to be linked for money transfer.
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
/// Configuration for the financial account transactions embedded component.
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
/// Configuration for the issuing card embedded component.
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
/// Configuration for the issuing cards list embedded component.
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
    /// Disables Stripe user authentication for this embedded component.
    /// This feature can only be false for accounts where you’re responsible for collecting updated information when requirements are due or change, like custom accounts.
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
    /// Disables Stripe user authentication for this embedded component.
    /// This value can only be true for accounts where `controller.requirement_collection` is `application`.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don’t set `external_account_collection`, it defaults to true and `disable_stripe_user_authentication` defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether to allow platforms to control bank account collection for their connected accounts.
    /// This feature can only be false for accounts where you’re responsible for collecting updated information when requirements are due or change, like custom accounts.
    /// Otherwise, bank account collection is determined by compliance requirements.
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PayoutsFeaturesParam {
    /// Disables Stripe user authentication for this embedded component.
    /// This value can only be true for accounts where `controller.requirement_collection` is `application`.
    /// The default value is the opposite of the `external_account_collection` value.
    /// For example, if you don’t set `external_account_collection`, it defaults to true and `disable_stripe_user_authentication` defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stripe_user_authentication: Option<bool>,
    /// Whether to allow payout schedule to be changed.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_payout_schedule: Option<bool>,
    /// Whether to allow platforms to control bank account collection for their connected accounts.
    /// This feature can only be false for accounts where you’re responsible for collecting updated information when requirements are due or change, like custom accounts.
    /// Otherwise, bank account collection is determined by compliance requirements.
    /// The default value for this feature is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_collection: Option<bool>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
