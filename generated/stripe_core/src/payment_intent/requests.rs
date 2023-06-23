use stripe::{Client, Response};

impl stripe_core::payment_intent::PaymentIntent {
    /// Search for PaymentIntents you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    ///
    /// Under normal operating conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
    /// Search functionality is not available to merchants in India.
    pub fn search(client: &Client, params: SearchPaymentIntent) -> Response<SearchReturned> {
        client.get_query("/payment_intents/search", params)
    }
    /// Creates a PaymentIntent object.
    ///
    /// After the PaymentIntent is created, attach a payment method and [confirm](https://stripe.com/docs/api/payment_intents/confirm)
    /// to continue the payment.
    ///
    /// You can read more about the different payment flows available via the Payment Intents API [here](https://stripe.com/docs/payments/payment-intents).  When `confirm=true` is used during creation, it is equivalent to creating and confirming the PaymentIntent in the same call.
    /// You may use any parameters available in the [confirm API](https://stripe.com/docs/api/payment_intents/confirm) when `confirm=true` is supplied.
    pub fn create(
        client: &Client,
        params: CreatePaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form("/payment_intents", params, http_types::Method::Post)
    }
    /// Returns a list of PaymentIntents.
    pub fn list(
        client: &Client,
        params: ListPaymentIntent,
    ) -> Response<stripe_types::List<stripe_core::payment_intent::PaymentIntent>> {
        client.get_query("/payment_intents", params)
    }
    /// Retrieves the details of a PaymentIntent that has previously been created.
    ///
    /// Client-side retrieval using a publishable key is allowed when the `client_secret` is provided in the query string.
    /// When retrieved with a publishable key, only a subset of properties will be returned.
    /// Please refer to the [payment intent](https://stripe.com/docs/api#payment_intent_object) object reference for more details.
    pub fn retrieve(
        client: &Client,
        intent: &str,
        params: RetrievePaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.get_query(&format!("/payment_intents/{intent}", intent = intent), params)
    }
    /// Updates properties on a PaymentIntent object without confirming.
    ///
    /// Depending on which properties you update, you may need to confirm the
    /// PaymentIntent again.
    ///
    /// For example, updating the `payment_method` will always require you to confirm the PaymentIntent again.
    /// If you prefer to update and confirm at the same time, we recommend updating properties via the [confirm API](https://stripe.com/docs/api/payment_intents/confirm) instead.
    pub fn update(
        client: &Client,
        intent: &str,
        params: UpdatePaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Confirm that your customer intends to pay with current or provided
    /// payment method.
    ///
    /// Upon confirmation, the PaymentIntent will attempt to initiate a payment. If the selected payment method requires additional authentication steps, the PaymentIntent will transition to the `requires_action` status and suggest additional actions via `next_action`.
    /// If payment fails, the PaymentIntent will transition to the `requires_payment_method` status.
    /// If payment succeeds, the PaymentIntent will transition to the `succeeded` status (or `requires_capture`, if `capture_method` is set to `manual`). If the `confirmation_method` is `automatic`, payment may be attempted using our [client SDKs](https://stripe.com/docs/stripe-js/reference#stripe-handle-card-payment) and the PaymentIntent’s [client_secret](https://stripe.com/docs/api#payment_intent_object-client_secret). After `next_action`s are handled by the client, no additional confirmation is required to complete the payment. If the `confirmation_method` is `manual`, all payment attempts must be initiated using a secret key. If any actions are required for the payment, the PaymentIntent will return to the `requires_confirmation` state after those actions are completed.
    /// Your server needs to then explicitly re-confirm the PaymentIntent to initiate the next payment attempt.
    /// Read the [expanded documentation](https://stripe.com/docs/payments/payment-intents/web-manual) to learn more about manual confirmation.
    pub fn confirm(
        client: &Client,
        intent: &str,
        params: ConfirmPaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/confirm", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// A PaymentIntent object can be canceled when it is in one of these statuses: `requires_payment_method`, `requires_capture`, `requires_confirmation`, `requires_action`, or `processing`.
    ///
    /// Once canceled, no additional charges will be made by the PaymentIntent and any operations on the PaymentIntent will fail with an error.
    /// For PaymentIntents with `status=’requires_capture’`, the remaining `amount_capturable` will automatically be refunded.
    /// You cannot cancel the PaymentIntent for a Checkout Session.
    /// [Expire the Checkout Session](https://stripe.com/docs/api/checkout/sessions/expire) instead.
    pub fn cancel(
        client: &Client,
        intent: &str,
        params: CancelPaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/cancel", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Capture the funds of an existing uncaptured PaymentIntent when its status is `requires_capture`.
    ///
    /// Uncaptured PaymentIntents will be canceled a set number of days after they are created (7 by default).
    ///
    /// Learn more about [separate authorization and capture](https://stripe.com/docs/payments/capture-later).
    pub fn capture(
        client: &Client,
        intent: &str,
        params: CapturePaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/capture", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Perform an incremental authorization on an eligible
    /// [PaymentIntent](https://stripe.com/docs/api/payment_intents/object).
    ///
    /// To be eligible, the PaymentIntent’s status must be `requires_capture` and [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) must be `true`.  Incremental authorizations attempt to increase the authorized amount on your customer’s card to the new, higher `amount` provided.
    /// As with the initial authorization, incremental authorizations may be declined.
    /// A single PaymentIntent can call this endpoint multiple times to further increase the authorized amount.  If the incremental authorization succeeds, the PaymentIntent object is returned with the updated [amount](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-amount). If the incremental authorization fails, a [card_declined](https://stripe.com/docs/error-codes#card-declined) error is returned, and no fields on the PaymentIntent or Charge are updated.
    /// The PaymentIntent object remains capturable for the previously authorized amount.  Each PaymentIntent can have a maximum of 10 incremental authorization attempts, including declines. Once captured, a PaymentIntent can no longer be incremented.  Learn more about [incremental authorizations](https://stripe.com/docs/terminal/features/incremental-authorizations).
    pub fn increment_authorization(
        client: &Client,
        intent: &str,
        params: IncrementAuthorizationPaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/increment_authorization", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Verifies microdeposits on a PaymentIntent object.
    pub fn verify_microdeposits(
        client: &Client,
        intent: &str,
        params: VerifyMicrodepositsPaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/verify_microdeposits", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Manually reconcile the remaining amount for a customer_balance PaymentIntent.
    pub fn apply_customer_balance(
        client: &Client,
        intent: &str,
        params: ApplyCustomerBalancePaymentIntent,
    ) -> Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/apply_customer_balance", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SearchReturned {
    pub data: Vec<stripe_core::payment_intent::PaymentIntent>,
    pub has_more: bool,
    pub next_page: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SearchReturnedObject,
    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SearchReturned {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SearchReturnedObject {
    SearchResult,
}

impl SearchReturnedObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SearchResult => "search_result",
        }
    }
}

impl AsRef<str> for SearchReturnedObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchPaymentIntent<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for pagination across multiple pages of results.
    ///
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<&'a str>,
    /// The search query string.
    ///
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for payment intents](https://stripe.com/docs/search#query-fields-for-payment-intents).
    pub query: &'a str,
}
impl<'a> SearchPaymentIntent<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            expand: Default::default(),
            limit: Default::default(),
            page: Default::default(),
            query,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntent<'a> {
    /// Amount intended to be collected by this PaymentIntent.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// When enabled, this PaymentIntent will accept payment methods that you have enabled in the Dashboard and are compatible with this PaymentIntent's other parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<CreatePaymentIntentAutomaticPaymentMethods>,
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentIntentCaptureMethod>,
    /// Set to `true` to attempt to [confirm](https://stripe.com/docs/api/payment_intents/confirm) this PaymentIntent immediately.
    ///
    /// This parameter defaults to `false`.
    /// When creating and confirming a PaymentIntent at the same time, parameters available in the [confirm](https://stripe.com/docs/api/payment_intents/confirm) API may also be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_method: Option<CreatePaymentIntentConfirmationMethod>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the Customer this PaymentIntent belongs to, if one exists.
    ///
    /// Payment methods attached to other Customers cannot be used with this PaymentIntent.
    ///
    /// If present in combination with [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage), this PaymentIntent's payment method will be attached to the Customer after the PaymentIntent has been confirmed and any required actions from the user are complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set to `true` to fail the payment attempt if the PaymentIntent transitions into `requires_action`.
    ///
    /// This parameter is intended for simpler integrations that do not handle customer actions, like [saving cards without authentication](https://stripe.com/docs/payments/save-card-without-authentication).
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_on_requires_action: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// ID of the mandate to be used for this payment.
    ///
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<&'a str>,
    /// This hash contains details about the Mandate to create.
    ///
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<CreatePaymentIntentMandateData<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// Set to `true` to indicate that the customer is not in your checkout flow during this payment attempt, and therefore is unable to authenticate.
    ///
    /// This parameter is intended for scenarios where you collect card details and [charge them later](https://stripe.com/docs/payments/cards/charging-saved-cards).
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<CreatePaymentIntentOffSession>,
    /// The Stripe account ID for which these funds are intended.
    ///
    /// For details, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// ID of the payment method (a PaymentMethod, Card, or [compatible Source](https://stripe.com/docs/payments/payment-methods#compatibility) object) to attach to this PaymentIntent.
    ///
    /// If neither the `payment_method` parameter nor the `source` parameter are provided with `confirm=true`, `source` will be automatically populated with `customer.default_source` to improve the migration experience for users of the Charges API.
    ///
    /// We recommend that you explicitly provide the `payment_method` going forward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// If provided, this hash will be used to create a PaymentMethod.
    ///
    /// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<CreatePaymentIntentPaymentMethodData<'a>>,
    /// Payment-method-specific configuration for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreatePaymentIntentPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) that this PaymentIntent is allowed to use.
    /// If this is not provided, defaults to ["card"].
    /// Use automatic_payment_methods to manage payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [&'a str]>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreatePaymentIntentRadarOptions<'a>>,
    /// Email address that the receipt for the resulting payment will be sent to.
    ///
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    ///
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentSetupFutureUsage>,
    /// Shipping information for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreatePaymentIntentShipping<'a>>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// The parameters used to automatically create a Transfer when the payment succeeds.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreatePaymentIntentTransferData<'a>>,
    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    /// Set to `true` only when using manual confirmation and the iOS or Android SDKs to handle additional authentication steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<bool>,
}
impl<'a> CreatePaymentIntent<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self {
            amount,
            application_fee_amount: Default::default(),
            automatic_payment_methods: Default::default(),
            capture_method: Default::default(),
            confirm: Default::default(),
            confirmation_method: Default::default(),
            currency,
            customer: Default::default(),
            description: Default::default(),
            error_on_requires_action: Default::default(),
            expand: Default::default(),
            mandate: Default::default(),
            mandate_data: Default::default(),
            metadata: Default::default(),
            off_session: Default::default(),
            on_behalf_of: Default::default(),
            payment_method: Default::default(),
            payment_method_data: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            radar_options: Default::default(),
            receipt_email: Default::default(),
            return_url: Default::default(),
            setup_future_usage: Default::default(),
            shipping: Default::default(),
            statement_descriptor: Default::default(),
            statement_descriptor_suffix: Default::default(),
            transfer_data: Default::default(),
            transfer_group: Default::default(),
            use_stripe_sdk: Default::default(),
        }
    }
}
/// When enabled, this PaymentIntent will accept payment methods that you have enabled in the Dashboard and are compatible with this PaymentIntent's other parameters.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentAutomaticPaymentMethods {
    /// Whether this feature is enabled.
    pub enabled: bool,
}
impl CreatePaymentIntentAutomaticPaymentMethods {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentCaptureMethod {
    Automatic,
    Manual,
}

impl CreatePaymentIntentCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentConfirmationMethod {
    Automatic,
    Manual,
}

impl CreatePaymentIntentConfirmationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentConfirmationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentConfirmationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This hash contains details about the Mandate to create.
///
/// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentMandateData<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: CreatePaymentIntentMandateDataCustomerAcceptance<'a>,
}
impl<'a> CreatePaymentIntentMandateData<'a> {
    pub fn new(customer_acceptance: CreatePaymentIntentMandateDataCustomerAcceptance<'a>) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentMandateDataCustomerAcceptance<'a> {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreatePaymentIntentMandateDataCustomerAcceptanceOffline>,
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<CreatePaymentIntentMandateDataCustomerAcceptanceOnline<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CreatePaymentIntentMandateDataCustomerAcceptanceType,
}
impl<'a> CreatePaymentIntentMandateDataCustomerAcceptance<'a> {
    pub fn new(type_: CreatePaymentIntentMandateDataCustomerAcceptanceType) -> Self {
        Self {
            accepted_at: Default::default(),
            offline: Default::default(),
            online: Default::default(),
            type_,
        }
    }
}
/// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentMandateDataCustomerAcceptanceOffline {}
impl CreatePaymentIntentMandateDataCustomerAcceptanceOffline {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentMandateDataCustomerAcceptanceOnline<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: &'a str,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: &'a str,
}
impl<'a> CreatePaymentIntentMandateDataCustomerAcceptanceOnline<'a> {
    pub fn new(ip_address: &'a str, user_agent: &'a str) -> Self {
        Self { ip_address, user_agent }
    }
}
/// The type of customer acceptance information included with the Mandate.
///
/// One of `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentMandateDataCustomerAcceptanceType {
    Offline,
    Online,
}

impl CreatePaymentIntentMandateDataCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentMandateDataCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentMandateDataCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Set to `true` to indicate that the customer is not in your checkout flow during this payment attempt, and therefore is unable to authenticate.
///
/// This parameter is intended for scenarios where you collect card details and [charge them later](https://stripe.com/docs/payments/cards/charging-saved-cards).
/// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreatePaymentIntentOffSession {
    Bool(bool),
    OneOff,
    Recurring,
}
/// If provided, this hash will be used to create a PaymentMethod.
///
/// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePaymentIntentPaymentMethodDataAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreatePaymentIntentPaymentMethodDataAffirm>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreatePaymentIntentPaymentMethodDataAfterpayClearpay>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreatePaymentIntentPaymentMethodDataAlipay>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreatePaymentIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePaymentIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePaymentIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreatePaymentIntentPaymentMethodDataBillingDetails<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePaymentIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreatePaymentIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePaymentIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePaymentIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreatePaymentIntentPaymentMethodDataGiropay>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreatePaymentIntentPaymentMethodDataGrabpay>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePaymentIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<CreatePaymentIntentPaymentMethodDataInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePaymentIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreatePaymentIntentPaymentMethodDataKonbini>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreatePaymentIntentPaymentMethodDataLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreatePaymentIntentPaymentMethodDataOxxo>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePaymentIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreatePaymentIntentPaymentMethodDataPaynow>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreatePaymentIntentPaymentMethodDataPix>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreatePaymentIntentPaymentMethodDataPromptpay>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreatePaymentIntentPaymentMethodDataRadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePaymentIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePaymentIntentPaymentMethodDataSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreatePaymentIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreatePaymentIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreatePaymentIntentPaymentMethodDataWechatPay>,
}
impl<'a> CreatePaymentIntentPaymentMethodData<'a> {
    pub fn new(type_: CreatePaymentIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            paynow: Default::default(),
            pix: Default::default(),
            promptpay: Default::default(),
            radar_options: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            type_,
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> CreatePaymentIntentPaymentMethodDataAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataAffirm {}
impl CreatePaymentIntentPaymentMethodDataAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataAfterpayClearpay {}
impl CreatePaymentIntentPaymentMethodDataAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataAlipay {}
impl CreatePaymentIntentPaymentMethodDataAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> CreatePaymentIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataBancontact {}
impl CreatePaymentIntentPaymentMethodDataBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreatePaymentIntentPaymentMethodDataBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataBillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataBlik {}
impl CreatePaymentIntentPaymentMethodDataBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> CreatePaymentIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataCustomerBalance {}
impl CreatePaymentIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentIntentPaymentMethodDataEpsBank>,
}
impl CreatePaymentIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl CreatePaymentIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreatePaymentIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: CreatePaymentIntentPaymentMethodDataFpxBank,
}
impl CreatePaymentIntentPaymentMethodDataFpx {
    pub fn new(bank: CreatePaymentIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// Account holder type for FPX transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl CreatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl CreatePaymentIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataGiropay {}
impl CreatePaymentIntentPaymentMethodDataGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataGrabpay {}
impl CreatePaymentIntentPaymentMethodDataGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentIntentPaymentMethodDataIdealBank>,
}
impl CreatePaymentIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl CreatePaymentIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataInteracPresent {}
impl CreatePaymentIntentPaymentMethodDataInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreatePaymentIntentPaymentMethodDataKlarnaDob>,
}
impl CreatePaymentIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Customer's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl CreatePaymentIntentPaymentMethodDataKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataKonbini {}
impl CreatePaymentIntentPaymentMethodDataKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataLink {}
impl CreatePaymentIntentPaymentMethodDataLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataOxxo {}
impl CreatePaymentIntentPaymentMethodDataOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentIntentPaymentMethodDataP24Bank>,
}
impl CreatePaymentIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl CreatePaymentIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataPaynow {}
impl CreatePaymentIntentPaymentMethodDataPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataPix {}
impl CreatePaymentIntentPaymentMethodDataPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataPromptpay {}
impl CreatePaymentIntentPaymentMethodDataPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodDataRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> CreatePaymentIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreatePaymentIntentPaymentMethodDataSofortCountry,
}
impl CreatePaymentIntentPaymentMethodDataSofort {
    pub fn new(country: CreatePaymentIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl CreatePaymentIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl CreatePaymentIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<CreatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreatePaymentIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl CreatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl CreatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataWechatPay {}
impl CreatePaymentIntentPaymentMethodDataWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration for this PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePaymentIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreatePaymentIntentPaymentMethodOptionsAffirm>,
    /// If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreatePaymentIntentPaymentMethodOptionsAfterpayClearpay<'a>>,
    /// If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreatePaymentIntentPaymentMethodOptionsAlipay>,
    /// If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreatePaymentIntentPaymentMethodOptionsAuBecsDebit>,
    /// If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePaymentIntentPaymentMethodOptionsBacsDebit>,
    /// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePaymentIntentPaymentMethodOptionsBancontact>,
    /// If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePaymentIntentPaymentMethodOptionsBlik<'a>>,
    /// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentIntentPaymentMethodOptionsBoleto>,
    /// Configuration for any card payments attempted on this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreatePaymentIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CreatePaymentIntentPaymentMethodOptionsCardPresent>,
    /// If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreatePaymentIntentPaymentMethodOptionsCustomerBalance<'a>>,
    /// If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePaymentIntentPaymentMethodOptionsEps>,
    /// If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePaymentIntentPaymentMethodOptionsFpx>,
    /// If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreatePaymentIntentPaymentMethodOptionsGiropay>,
    /// If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreatePaymentIntentPaymentMethodOptionsGrabpay>,
    /// If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePaymentIntentPaymentMethodOptionsIdeal>,
    /// If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<CreatePaymentIntentPaymentMethodOptionsInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePaymentIntentPaymentMethodOptionsKlarna>,
    /// If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreatePaymentIntentPaymentMethodOptionsKonbini<'a>>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreatePaymentIntentPaymentMethodOptionsLink<'a>>,
    /// If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreatePaymentIntentPaymentMethodOptionsOxxo>,
    /// If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePaymentIntentPaymentMethodOptionsP24>,
    /// If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreatePaymentIntentPaymentMethodOptionsPaynow>,
    /// If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreatePaymentIntentPaymentMethodOptionsPix>,
    /// If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreatePaymentIntentPaymentMethodOptionsPromptpay>,
    /// If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePaymentIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePaymentIntentPaymentMethodOptionsSofort>,
    /// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreatePaymentIntentPaymentMethodOptionsUsBankAccount<'a>>,
    /// If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreatePaymentIntentPaymentMethodOptionsWechatPay<'a>>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsAffirm {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    Manual,
}

impl CreatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsAfterpayClearpay<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<CreatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsAfterpayClearpay<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Manual,
}

impl CreatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl CreatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsAuBecsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsBacsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl CreatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl CreatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsBlik<'a> {
    /// The 6-digit BLIK code that a customer has generated using their banking application.
    ///
    /// Can only be set on confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsBlik<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsBoleto {
    /// The number of calendar days before a Boleto voucher expires.
    ///
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto invoice will expire on Wednesday at 23:59 America/Sao_Paulo time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration for any card payments attempted on this PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCard<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentIntentPaymentMethodOptionsCardCaptureMethod>,
    /// A single-use `cvc_update` Token that represents a card CVC value.
    ///
    /// When provided, the CVC value will be verified during the card payment attempt.
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_token: Option<&'a str>,
    /// Installment configuration for payments attempted on this PaymentIntent (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreatePaymentIntentPaymentMethodOptionsCardInstallments>,
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter indicates that a transaction will be marked
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this PaymentIntent on.
    ///
    /// Depends on the available networks of the card attached to the PaymentIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreatePaymentIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    Manual,
}

impl CreatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Installment configuration for payments attempted on this PaymentIntent (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this PaymentIntent.
    /// This will cause the response to contain a list of available installment plans.
    /// Setting to false will prevent any selected plan from applying to a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The selected installment plan to use for this payment attempt.
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan>,
}
impl CreatePaymentIntentPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The selected installment plan to use for this payment attempt.
/// This parameter can only be provided during confirmation.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
}
impl CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: u64,
        interval: CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval,
        type_: CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
    }
}
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}

impl CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Month => "month",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}

impl CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedCount => "fixed_count",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        interval: CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: stripe_types::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            description: Default::default(),
            end_date: Default::default(),
            interval,
            interval_count: Default::default(),
            reference,
            start_date,
            supported_types: Default::default(),
        }
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Sporadic => "sporadic",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this PaymentIntent on.
///
/// Depends on the available networks of the card attached to the PaymentIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl CreatePaymentIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCardPresent {
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    ///
    /// Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization_support: Option<bool>,
}
impl CreatePaymentIntentPaymentMethodOptionsCardPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<CreatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<&'a [CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes]>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType,

}
impl<'a> CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new(
        type_: CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType,
    ) -> Self {
        Self {
            eu_bank_transfer: Default::default(),
            requested_address_types: Default::default(),
            type_,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str>
    for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl CreatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsEps {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsFpx {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsFpx {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsGiropay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsGrabpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl CreatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsInteracPresent {}
impl CreatePaymentIntentPaymentMethodOptionsInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<CreatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl CreatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Preferred language of the Klarna authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    #[serde(rename = "da-DK")]
    DaMinusDk,
    #[serde(rename = "de-AT")]
    DeMinusAt,
    #[serde(rename = "de-CH")]
    DeMinusCh,
    #[serde(rename = "de-DE")]
    DeMinusDe,
    #[serde(rename = "en-AT")]
    EnMinusAt,
    #[serde(rename = "en-AU")]
    EnMinusAu,
    #[serde(rename = "en-BE")]
    EnMinusBe,
    #[serde(rename = "en-CA")]
    EnMinusCa,
    #[serde(rename = "en-CH")]
    EnMinusCh,
    #[serde(rename = "en-DE")]
    EnMinusDe,
    #[serde(rename = "en-DK")]
    EnMinusDk,
    #[serde(rename = "en-ES")]
    EnMinusEs,
    #[serde(rename = "en-FI")]
    EnMinusFi,
    #[serde(rename = "en-FR")]
    EnMinusFr,
    #[serde(rename = "en-GB")]
    EnMinusGb,
    #[serde(rename = "en-IE")]
    EnMinusIe,
    #[serde(rename = "en-IT")]
    EnMinusIt,
    #[serde(rename = "en-NL")]
    EnMinusNl,
    #[serde(rename = "en-NO")]
    EnMinusNo,
    #[serde(rename = "en-NZ")]
    EnMinusNz,
    #[serde(rename = "en-PL")]
    EnMinusPl,
    #[serde(rename = "en-PT")]
    EnMinusPt,
    #[serde(rename = "en-SE")]
    EnMinusSe,
    #[serde(rename = "en-US")]
    EnMinusUs,
    #[serde(rename = "es-ES")]
    EsMinusEs,
    #[serde(rename = "es-US")]
    EsMinusUs,
    #[serde(rename = "fi-FI")]
    FiMinusFi,
    #[serde(rename = "fr-BE")]
    FrMinusBe,
    #[serde(rename = "fr-CA")]
    FrMinusCa,
    #[serde(rename = "fr-CH")]
    FrMinusCh,
    #[serde(rename = "fr-FR")]
    FrMinusFr,
    #[serde(rename = "it-CH")]
    ItMinusCh,
    #[serde(rename = "it-IT")]
    ItMinusIt,
    #[serde(rename = "nb-NO")]
    NbMinusNo,
    #[serde(rename = "nl-BE")]
    NlMinusBe,
    #[serde(rename = "nl-NL")]
    NlMinusNl,
    #[serde(rename = "pl-PL")]
    PlMinusPl,
    #[serde(rename = "pt-PT")]
    PtMinusPt,
    #[serde(rename = "sv-FI")]
    SvMinusFi,
    #[serde(rename = "sv-SE")]
    SvMinusSe,
}

impl CreatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DaMinusDk => "da-DK",
            Self::DeMinusAt => "de-AT",
            Self::DeMinusCh => "de-CH",
            Self::DeMinusDe => "de-DE",
            Self::EnMinusAt => "en-AT",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusBe => "en-BE",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusCh => "en-CH",
            Self::EnMinusDe => "en-DE",
            Self::EnMinusDk => "en-DK",
            Self::EnMinusEs => "en-ES",
            Self::EnMinusFi => "en-FI",
            Self::EnMinusFr => "en-FR",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIt => "en-IT",
            Self::EnMinusNl => "en-NL",
            Self::EnMinusNo => "en-NO",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusPl => "en-PL",
            Self::EnMinusPt => "en-PT",
            Self::EnMinusSe => "en-SE",
            Self::EnMinusUs => "en-US",
            Self::EsMinusEs => "es-ES",
            Self::EsMinusUs => "es-US",
            Self::FiMinusFi => "fi-FI",
            Self::FrMinusBe => "fr-BE",
            Self::FrMinusCa => "fr-CA",
            Self::FrMinusCh => "fr-CH",
            Self::FrMinusFr => "fr-FR",
            Self::ItMinusCh => "it-CH",
            Self::ItMinusIt => "it-IT",
            Self::NbMinusNo => "nb-NO",
            Self::NlMinusBe => "nl-BE",
            Self::NlMinusNl => "nl-NL",
            Self::PlMinusPl => "pl-PL",
            Self::PtMinusPt => "pt-PT",
            Self::SvMinusFi => "sv-FI",
            Self::SvMinusSe => "sv-SE",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsKonbini<'a> {
    /// An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores.
    ///
    /// Must not consist of only zeroes and could be rejected in case of insufficient uniqueness.
    /// We recommend to use the customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<&'a str>,
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    ///
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    /// Defaults to 3 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// The timestamp at which the Konbini payment instructions will expire.
    ///
    /// Only one of `expires_after_days` or `expires_at` may be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// A product descriptor of up to 22 characters, which will appear to customers at the convenience store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsKonbini<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsLink<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentIntentPaymentMethodOptionsLinkCaptureMethod>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl CreatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl CreatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl CreatePaymentIntentPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsPaynow {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsPix {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    ///
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    /// The timestamp at which the Pix expires (between 10 and 1209600 seconds in the future).
    ///
    /// Defaults to 1 day in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsPromptpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreatePaymentIntentPaymentMethodOptionsSepaDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsSepaDebitMandateOptions {}
impl CreatePaymentIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsSofort {
    /// Language shown to the payer on redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<CreatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Language shown to the payer on redirect.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl CreatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Pl => "pl",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl CreatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for network related functions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Triggers validations to run across the selected networks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsWechatPay<'a> {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<&'a str>,
    /// The client type that the end customer will pay from.
    pub client: CreatePaymentIntentPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsWechatPay<'a> {
    pub fn new(client: CreatePaymentIntentPaymentMethodOptionsWechatPayClient) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// The client type that the end customer will pay from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl CreatePaymentIntentPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl CreatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> CreatePaymentIntentRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentIntentSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreatePaymentIntentSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentIntentSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentIntentSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Shipping information for this PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentShipping<'a> {
    /// Shipping address.
    pub address: CreatePaymentIntentShippingAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> CreatePaymentIntentShipping<'a> {
    pub fn new(address: CreatePaymentIntentShippingAddress<'a>, name: &'a str) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name,
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentShippingAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreatePaymentIntentShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The parameters used to automatically create a Transfer when the payment succeeds.
/// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentTransferData<'a> {
    /// The amount that will be transferred automatically when a charge succeeds.
    /// The amount is capped at the total transaction amount and if no amount is set,
    /// the full amount is transferred.
    ///
    /// If you intend to collect a fee and you need a more robust reporting experience, using
    /// [application_fee_amount](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-application_fee_amount)
    /// might be a better fit for your integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account.
    ///
    /// The ID of the resulting transfer will be returned on the successful charge's `transfer` field.
    pub destination: &'a str,
}
impl<'a> CreatePaymentIntentTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListPaymentIntent<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return PaymentIntents for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListPaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentIntent<'a> {
    /// The client secret of the PaymentIntent.
    ///
    /// Required if a publishable key is used to retrieve the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntent<'a> {
    /// Amount intended to be collected by this PaymentIntent.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdatePaymentIntentCaptureMethod>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the Customer this PaymentIntent belongs to, if one exists.
    ///
    /// Payment methods attached to other Customers cannot be used with this PaymentIntent.
    ///
    /// If present in combination with [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage), this PaymentIntent's payment method will be attached to the Customer after the PaymentIntent has been confirmed and any required actions from the user are complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// ID of the payment method (a PaymentMethod, Card, or [compatible Source](https://stripe.com/docs/payments/payment-methods/transitioning#compatibility) object) to attach to this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// If provided, this hash will be used to create a PaymentMethod.
    ///
    /// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<UpdatePaymentIntentPaymentMethodData<'a>>,
    /// Payment-method-specific configuration for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdatePaymentIntentPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) that this PaymentIntent is allowed to use.
    /// Use automatic_payment_methods to manage payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [&'a str]>,
    /// Email address that the receipt for the resulting payment will be sent to.
    ///
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentSetupFutureUsage>,
    /// Shipping information for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<UpdatePaymentIntentShipping<'a>>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// The parameters used to automatically create a Transfer when the payment succeeds.
    ///
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdatePaymentIntentTransferData>,
    /// A string that identifies the resulting payment as part of a group.
    ///
    /// `transfer_group` may only be provided if it has not been set.
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> UpdatePaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentCaptureMethod {
    Automatic,
    Manual,
}

impl UpdatePaymentIntentCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If provided, this hash will be used to create a PaymentMethod.
///
/// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdatePaymentIntentPaymentMethodDataAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<UpdatePaymentIntentPaymentMethodDataAffirm>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<UpdatePaymentIntentPaymentMethodDataAfterpayClearpay>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdatePaymentIntentPaymentMethodDataAlipay>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdatePaymentIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdatePaymentIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdatePaymentIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<UpdatePaymentIntentPaymentMethodDataBillingDetails<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdatePaymentIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdatePaymentIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdatePaymentIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdatePaymentIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdatePaymentIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<UpdatePaymentIntentPaymentMethodDataGiropay>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<UpdatePaymentIntentPaymentMethodDataGrabpay>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdatePaymentIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<UpdatePaymentIntentPaymentMethodDataInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdatePaymentIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<UpdatePaymentIntentPaymentMethodDataKonbini>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdatePaymentIntentPaymentMethodDataLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdatePaymentIntentPaymentMethodDataOxxo>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdatePaymentIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<UpdatePaymentIntentPaymentMethodDataPaynow>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<UpdatePaymentIntentPaymentMethodDataPix>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<UpdatePaymentIntentPaymentMethodDataPromptpay>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<UpdatePaymentIntentPaymentMethodDataRadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdatePaymentIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdatePaymentIntentPaymentMethodDataSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdatePaymentIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdatePaymentIntentPaymentMethodDataWechatPay>,
}
impl<'a> UpdatePaymentIntentPaymentMethodData<'a> {
    pub fn new(type_: UpdatePaymentIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            paynow: Default::default(),
            pix: Default::default(),
            promptpay: Default::default(),
            radar_options: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            type_,
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataAffirm {}
impl UpdatePaymentIntentPaymentMethodDataAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataAfterpayClearpay {}
impl UpdatePaymentIntentPaymentMethodDataAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataAlipay {}
impl UpdatePaymentIntentPaymentMethodDataAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataBancontact {}
impl UpdatePaymentIntentPaymentMethodDataBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePaymentIntentPaymentMethodDataBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataBillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataBlik {}
impl UpdatePaymentIntentPaymentMethodDataBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataCustomerBalance {}
impl UpdatePaymentIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdatePaymentIntentPaymentMethodDataEpsBank>,
}
impl UpdatePaymentIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl UpdatePaymentIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdatePaymentIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: UpdatePaymentIntentPaymentMethodDataFpxBank,
}
impl UpdatePaymentIntentPaymentMethodDataFpx {
    pub fn new(bank: UpdatePaymentIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// Account holder type for FPX transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl UpdatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl UpdatePaymentIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataGiropay {}
impl UpdatePaymentIntentPaymentMethodDataGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataGrabpay {}
impl UpdatePaymentIntentPaymentMethodDataGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdatePaymentIntentPaymentMethodDataIdealBank>,
}
impl UpdatePaymentIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl UpdatePaymentIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataInteracPresent {}
impl UpdatePaymentIntentPaymentMethodDataInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<UpdatePaymentIntentPaymentMethodDataKlarnaDob>,
}
impl UpdatePaymentIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Customer's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl UpdatePaymentIntentPaymentMethodDataKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataKonbini {}
impl UpdatePaymentIntentPaymentMethodDataKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataLink {}
impl UpdatePaymentIntentPaymentMethodDataLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataOxxo {}
impl UpdatePaymentIntentPaymentMethodDataOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdatePaymentIntentPaymentMethodDataP24Bank>,
}
impl UpdatePaymentIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl UpdatePaymentIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataPaynow {}
impl UpdatePaymentIntentPaymentMethodDataPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataPix {}
impl UpdatePaymentIntentPaymentMethodDataPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataPromptpay {}
impl UpdatePaymentIntentPaymentMethodDataPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: UpdatePaymentIntentPaymentMethodDataSofortCountry,
}
impl UpdatePaymentIntentPaymentMethodDataSofort {
    pub fn new(country: UpdatePaymentIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl UpdatePaymentIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl UpdatePaymentIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataWechatPay {}
impl UpdatePaymentIntentPaymentMethodDataWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration for this PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdatePaymentIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<UpdatePaymentIntentPaymentMethodOptionsAffirm>,
    /// If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpay<'a>>,
    /// If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdatePaymentIntentPaymentMethodOptionsAlipay>,
    /// If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdatePaymentIntentPaymentMethodOptionsAuBecsDebit>,
    /// If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdatePaymentIntentPaymentMethodOptionsBacsDebit>,
    /// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdatePaymentIntentPaymentMethodOptionsBancontact>,
    /// If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdatePaymentIntentPaymentMethodOptionsBlik<'a>>,
    /// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdatePaymentIntentPaymentMethodOptionsBoleto>,
    /// Configuration for any card payments attempted on this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdatePaymentIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<UpdatePaymentIntentPaymentMethodOptionsCardPresent>,
    /// If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdatePaymentIntentPaymentMethodOptionsCustomerBalance<'a>>,
    /// If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdatePaymentIntentPaymentMethodOptionsEps>,
    /// If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdatePaymentIntentPaymentMethodOptionsFpx>,
    /// If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<UpdatePaymentIntentPaymentMethodOptionsGiropay>,
    /// If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<UpdatePaymentIntentPaymentMethodOptionsGrabpay>,
    /// If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdatePaymentIntentPaymentMethodOptionsIdeal>,
    /// If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<UpdatePaymentIntentPaymentMethodOptionsInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdatePaymentIntentPaymentMethodOptionsKlarna>,
    /// If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<UpdatePaymentIntentPaymentMethodOptionsKonbini<'a>>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdatePaymentIntentPaymentMethodOptionsLink<'a>>,
    /// If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdatePaymentIntentPaymentMethodOptionsOxxo>,
    /// If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdatePaymentIntentPaymentMethodOptionsP24>,
    /// If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<UpdatePaymentIntentPaymentMethodOptionsPaynow>,
    /// If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<UpdatePaymentIntentPaymentMethodOptionsPix>,
    /// If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<UpdatePaymentIntentPaymentMethodOptionsPromptpay>,
    /// If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdatePaymentIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdatePaymentIntentPaymentMethodOptionsSofort>,
    /// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdatePaymentIntentPaymentMethodOptionsUsBankAccount<'a>>,
    /// If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdatePaymentIntentPaymentMethodOptionsWechatPay<'a>>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsAffirm {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    Manual,
}

impl UpdatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpay<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpay<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Manual,
}

impl UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsAuBecsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsBacsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl UpdatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsBlik<'a> {
    /// The 6-digit BLIK code that a customer has generated using their banking application.
    ///
    /// Can only be set on confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsBlik<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsBoleto {
    /// The number of calendar days before a Boleto voucher expires.
    ///
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto invoice will expire on Wednesday at 23:59 America/Sao_Paulo time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration for any card payments attempted on this PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCard<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdatePaymentIntentPaymentMethodOptionsCardCaptureMethod>,
    /// A single-use `cvc_update` Token that represents a card CVC value.
    ///
    /// When provided, the CVC value will be verified during the card payment attempt.
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_token: Option<&'a str>,
    /// Installment configuration for payments attempted on this PaymentIntent (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<UpdatePaymentIntentPaymentMethodOptionsCardInstallments>,
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter indicates that a transaction will be marked
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this PaymentIntent on.
    ///
    /// Depends on the available networks of the card attached to the PaymentIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<UpdatePaymentIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    Manual,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Installment configuration for payments attempted on this PaymentIntent (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this PaymentIntent.
    /// This will cause the response to contain a list of available installment plans.
    /// Setting to false will prevent any selected plan from applying to a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The selected installment plan to use for this payment attempt.
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan>,
}
impl UpdatePaymentIntentPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The selected installment plan to use for this payment attempt.
/// This parameter can only be provided during confirmation.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
}
impl UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: u64,
        interval: UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval,
        type_: UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
    }
}
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Month => "month",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedCount => "fixed_count",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        interval: UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: stripe_types::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            description: Default::default(),
            end_date: Default::default(),
            interval,
            interval_count: Default::default(),
            reference,
            start_date,
            supported_types: Default::default(),
        }
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Sporadic => "sporadic",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this PaymentIntent on.
///
/// Depends on the available networks of the card attached to the PaymentIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCardPresent {
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    ///
    /// Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization_support: Option<bool>,
}
impl UpdatePaymentIntentPaymentMethodOptionsCardPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<&'a [UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes]>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType,

}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new(
        type_: UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType,
    ) -> Self {
        Self {
            eu_bank_transfer: Default::default(),
            requested_address_types: Default::default(),
            type_,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str>
    for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsEps {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsFpx {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsFpx {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsGiropay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsGrabpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsInteracPresent {}
impl UpdatePaymentIntentPaymentMethodOptionsInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<UpdatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl UpdatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Preferred language of the Klarna authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    #[serde(rename = "da-DK")]
    DaMinusDk,
    #[serde(rename = "de-AT")]
    DeMinusAt,
    #[serde(rename = "de-CH")]
    DeMinusCh,
    #[serde(rename = "de-DE")]
    DeMinusDe,
    #[serde(rename = "en-AT")]
    EnMinusAt,
    #[serde(rename = "en-AU")]
    EnMinusAu,
    #[serde(rename = "en-BE")]
    EnMinusBe,
    #[serde(rename = "en-CA")]
    EnMinusCa,
    #[serde(rename = "en-CH")]
    EnMinusCh,
    #[serde(rename = "en-DE")]
    EnMinusDe,
    #[serde(rename = "en-DK")]
    EnMinusDk,
    #[serde(rename = "en-ES")]
    EnMinusEs,
    #[serde(rename = "en-FI")]
    EnMinusFi,
    #[serde(rename = "en-FR")]
    EnMinusFr,
    #[serde(rename = "en-GB")]
    EnMinusGb,
    #[serde(rename = "en-IE")]
    EnMinusIe,
    #[serde(rename = "en-IT")]
    EnMinusIt,
    #[serde(rename = "en-NL")]
    EnMinusNl,
    #[serde(rename = "en-NO")]
    EnMinusNo,
    #[serde(rename = "en-NZ")]
    EnMinusNz,
    #[serde(rename = "en-PL")]
    EnMinusPl,
    #[serde(rename = "en-PT")]
    EnMinusPt,
    #[serde(rename = "en-SE")]
    EnMinusSe,
    #[serde(rename = "en-US")]
    EnMinusUs,
    #[serde(rename = "es-ES")]
    EsMinusEs,
    #[serde(rename = "es-US")]
    EsMinusUs,
    #[serde(rename = "fi-FI")]
    FiMinusFi,
    #[serde(rename = "fr-BE")]
    FrMinusBe,
    #[serde(rename = "fr-CA")]
    FrMinusCa,
    #[serde(rename = "fr-CH")]
    FrMinusCh,
    #[serde(rename = "fr-FR")]
    FrMinusFr,
    #[serde(rename = "it-CH")]
    ItMinusCh,
    #[serde(rename = "it-IT")]
    ItMinusIt,
    #[serde(rename = "nb-NO")]
    NbMinusNo,
    #[serde(rename = "nl-BE")]
    NlMinusBe,
    #[serde(rename = "nl-NL")]
    NlMinusNl,
    #[serde(rename = "pl-PL")]
    PlMinusPl,
    #[serde(rename = "pt-PT")]
    PtMinusPt,
    #[serde(rename = "sv-FI")]
    SvMinusFi,
    #[serde(rename = "sv-SE")]
    SvMinusSe,
}

impl UpdatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DaMinusDk => "da-DK",
            Self::DeMinusAt => "de-AT",
            Self::DeMinusCh => "de-CH",
            Self::DeMinusDe => "de-DE",
            Self::EnMinusAt => "en-AT",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusBe => "en-BE",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusCh => "en-CH",
            Self::EnMinusDe => "en-DE",
            Self::EnMinusDk => "en-DK",
            Self::EnMinusEs => "en-ES",
            Self::EnMinusFi => "en-FI",
            Self::EnMinusFr => "en-FR",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIt => "en-IT",
            Self::EnMinusNl => "en-NL",
            Self::EnMinusNo => "en-NO",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusPl => "en-PL",
            Self::EnMinusPt => "en-PT",
            Self::EnMinusSe => "en-SE",
            Self::EnMinusUs => "en-US",
            Self::EsMinusEs => "es-ES",
            Self::EsMinusUs => "es-US",
            Self::FiMinusFi => "fi-FI",
            Self::FrMinusBe => "fr-BE",
            Self::FrMinusCa => "fr-CA",
            Self::FrMinusCh => "fr-CH",
            Self::FrMinusFr => "fr-FR",
            Self::ItMinusCh => "it-CH",
            Self::ItMinusIt => "it-IT",
            Self::NbMinusNo => "nb-NO",
            Self::NlMinusBe => "nl-BE",
            Self::NlMinusNl => "nl-NL",
            Self::PlMinusPl => "pl-PL",
            Self::PtMinusPt => "pt-PT",
            Self::SvMinusFi => "sv-FI",
            Self::SvMinusSe => "sv-SE",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsKonbini<'a> {
    /// An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores.
    ///
    /// Must not consist of only zeroes and could be rejected in case of insufficient uniqueness.
    /// We recommend to use the customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<&'a str>,
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    ///
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    /// Defaults to 3 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// The timestamp at which the Konbini payment instructions will expire.
    ///
    /// Only one of `expires_after_days` or `expires_at` may be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// A product descriptor of up to 22 characters, which will appear to customers at the convenience store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsKonbini<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsLink<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdatePaymentIntentPaymentMethodOptionsLinkCaptureMethod>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl UpdatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl UpdatePaymentIntentPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsPaynow {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsPix {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    ///
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    /// The timestamp at which the Pix expires (between 10 and 1209600 seconds in the future).
    ///
    /// Defaults to 1 day in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsPromptpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdatePaymentIntentPaymentMethodOptionsSepaDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsSepaDebitMandateOptions {}
impl UpdatePaymentIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsSofort {
    /// Language shown to the payer on redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<UpdatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<UpdatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Language shown to the payer on redirect.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl UpdatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Pl => "pl",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for network related functions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Triggers validations to run across the selected networks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsWechatPay<'a> {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<&'a str>,
    /// The client type that the end customer will pay from.
    pub client: UpdatePaymentIntentPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsWechatPay<'a> {
    pub fn new(client: UpdatePaymentIntentPaymentMethodOptionsWechatPayClient) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// The client type that the end customer will pay from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl UpdatePaymentIntentPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl UpdatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentIntentSetupFutureUsage {
    OffSession,
    OnSession,
}

impl UpdatePaymentIntentSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for UpdatePaymentIntentSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentIntentSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Shipping information for this PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentShipping<'a> {
    /// Shipping address.
    pub address: UpdatePaymentIntentShippingAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentShipping<'a> {
    pub fn new(address: UpdatePaymentIntentShippingAddress<'a>, name: &'a str) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name,
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentShippingAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdatePaymentIntentShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The parameters used to automatically create a Transfer when the payment succeeds.
///
/// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentTransferData {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl UpdatePaymentIntentTransferData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntent<'a> {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ConfirmPaymentIntentCaptureMethod>,
    /// Set to `true` to fail the payment attempt if the PaymentIntent transitions into `requires_action`.
    ///
    /// This parameter is intended for simpler integrations that do not handle customer actions, like [saving cards without authentication](https://stripe.com/docs/payments/save-card-without-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_on_requires_action: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// ID of the mandate to be used for this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<&'a str>,
    /// This hash contains details about the Mandate to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<ConfirmPaymentIntentMandateData<'a>>,
    /// Set to `true` to indicate that the customer is not in your checkout flow during this payment attempt, and therefore is unable to authenticate.
    ///
    /// This parameter is intended for scenarios where you collect card details and [charge them later](https://stripe.com/docs/payments/cards/charging-saved-cards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<ConfirmPaymentIntentOffSession>,
    /// ID of the payment method (a PaymentMethod, Card, or [compatible Source](https://stripe.com/docs/payments/payment-methods/transitioning#compatibility) object) to attach to this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// If provided, this hash will be used to create a PaymentMethod.
    ///
    /// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<ConfirmPaymentIntentPaymentMethodData<'a>>,
    /// Payment-method-specific configuration for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<ConfirmPaymentIntentPaymentMethodOptions<'a>>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<ConfirmPaymentIntentRadarOptions<'a>>,
    /// Email address that the receipt for the resulting payment will be sent to.
    ///
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter is only used for cards and other redirect-based payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentSetupFutureUsage>,
    /// Shipping information for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ConfirmPaymentIntentShipping<'a>>,
    /// Set to `true` only when using manual confirmation and the iOS or Android SDKs to handle additional authentication steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<bool>,
}
impl<'a> ConfirmPaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentCaptureMethod {
    Automatic,
    Manual,
}

impl ConfirmPaymentIntentCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This hash contains details about the Mandate to create.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ConfirmPaymentIntentMandateData<'a> {
    SecretKeyParam(ConfirmPaymentIntentSecretKeyParam<'a>),
    ClientKeyParam(ConfirmPaymentIntentClientKeyParam<'a>),
}
/// This hash contains details about the Mandate to create.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentSecretKeyParam<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: ConfirmPaymentIntentSecretKeyParamCustomerAcceptance<'a>,
}
impl<'a> ConfirmPaymentIntentSecretKeyParam<'a> {
    pub fn new(
        customer_acceptance: ConfirmPaymentIntentSecretKeyParamCustomerAcceptance<'a>,
    ) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentSecretKeyParamCustomerAcceptance<'a> {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceOffline>,
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceOnline<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceType,
}
impl<'a> ConfirmPaymentIntentSecretKeyParamCustomerAcceptance<'a> {
    pub fn new(type_: ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceType) -> Self {
        Self {
            accepted_at: Default::default(),
            offline: Default::default(),
            online: Default::default(),
            type_,
        }
    }
}
/// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceOffline {}
impl ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceOffline {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceOnline<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: &'a str,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: &'a str,
}
impl<'a> ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceOnline<'a> {
    pub fn new(ip_address: &'a str, user_agent: &'a str) -> Self {
        Self { ip_address, user_agent }
    }
}
/// The type of customer acceptance information included with the Mandate.
///
/// One of `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceType {
    Offline,
    Online,
}

impl ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentSecretKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This hash contains details about the Mandate to create.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentClientKeyParam<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: ConfirmPaymentIntentClientKeyParamCustomerAcceptance<'a>,
}
impl<'a> ConfirmPaymentIntentClientKeyParam<'a> {
    pub fn new(
        customer_acceptance: ConfirmPaymentIntentClientKeyParamCustomerAcceptance<'a>,
    ) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentClientKeyParamCustomerAcceptance<'a> {
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    pub online: ConfirmPaymentIntentClientKeyParamCustomerAcceptanceOnline<'a>,
    /// The type of customer acceptance information included with the Mandate.
    #[serde(rename = "type")]
    pub type_: ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType,
}
impl<'a> ConfirmPaymentIntentClientKeyParamCustomerAcceptance<'a> {
    pub fn new(
        online: ConfirmPaymentIntentClientKeyParamCustomerAcceptanceOnline<'a>,
        type_: ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType,
    ) -> Self {
        Self { online, type_ }
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentClientKeyParamCustomerAcceptanceOnline<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentClientKeyParamCustomerAcceptanceOnline<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of customer acceptance information included with the Mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType {
    Online,
}

impl ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Set to `true` to indicate that the customer is not in your checkout flow during this payment attempt, and therefore is unable to authenticate.
///
/// This parameter is intended for scenarios where you collect card details and [charge them later](https://stripe.com/docs/payments/cards/charging-saved-cards).
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ConfirmPaymentIntentOffSession {
    Bool(bool),
    OneOff,
    Recurring,
}
/// If provided, this hash will be used to create a PaymentMethod.
///
/// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<ConfirmPaymentIntentPaymentMethodDataAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<ConfirmPaymentIntentPaymentMethodDataAffirm>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<ConfirmPaymentIntentPaymentMethodDataAfterpayClearpay>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<ConfirmPaymentIntentPaymentMethodDataAlipay>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<ConfirmPaymentIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<ConfirmPaymentIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<ConfirmPaymentIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<ConfirmPaymentIntentPaymentMethodDataBillingDetails<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<ConfirmPaymentIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<ConfirmPaymentIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<ConfirmPaymentIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<ConfirmPaymentIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<ConfirmPaymentIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<ConfirmPaymentIntentPaymentMethodDataGiropay>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<ConfirmPaymentIntentPaymentMethodDataGrabpay>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<ConfirmPaymentIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<ConfirmPaymentIntentPaymentMethodDataInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<ConfirmPaymentIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<ConfirmPaymentIntentPaymentMethodDataKonbini>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<ConfirmPaymentIntentPaymentMethodDataLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<ConfirmPaymentIntentPaymentMethodDataOxxo>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<ConfirmPaymentIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<ConfirmPaymentIntentPaymentMethodDataPaynow>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<ConfirmPaymentIntentPaymentMethodDataPix>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<ConfirmPaymentIntentPaymentMethodDataPromptpay>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<ConfirmPaymentIntentPaymentMethodDataRadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmPaymentIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<ConfirmPaymentIntentPaymentMethodDataSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: ConfirmPaymentIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmPaymentIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<ConfirmPaymentIntentPaymentMethodDataWechatPay>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodData<'a> {
    pub fn new(type_: ConfirmPaymentIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            paynow: Default::default(),
            pix: Default::default(),
            promptpay: Default::default(),
            radar_options: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            type_,
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataAffirm {}
impl ConfirmPaymentIntentPaymentMethodDataAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataAfterpayClearpay {}
impl ConfirmPaymentIntentPaymentMethodDataAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataAlipay {}
impl ConfirmPaymentIntentPaymentMethodDataAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataBancontact {}
impl ConfirmPaymentIntentPaymentMethodDataBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ConfirmPaymentIntentPaymentMethodDataBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataBillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataBlik {}
impl ConfirmPaymentIntentPaymentMethodDataBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataCustomerBalance {}
impl ConfirmPaymentIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmPaymentIntentPaymentMethodDataEpsBank>,
}
impl ConfirmPaymentIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl ConfirmPaymentIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<ConfirmPaymentIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: ConfirmPaymentIntentPaymentMethodDataFpxBank,
}
impl ConfirmPaymentIntentPaymentMethodDataFpx {
    pub fn new(bank: ConfirmPaymentIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// Account holder type for FPX transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl ConfirmPaymentIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl ConfirmPaymentIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataGiropay {}
impl ConfirmPaymentIntentPaymentMethodDataGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataGrabpay {}
impl ConfirmPaymentIntentPaymentMethodDataGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmPaymentIntentPaymentMethodDataIdealBank>,
}
impl ConfirmPaymentIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl ConfirmPaymentIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataInteracPresent {}
impl ConfirmPaymentIntentPaymentMethodDataInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<ConfirmPaymentIntentPaymentMethodDataKlarnaDob>,
}
impl ConfirmPaymentIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Customer's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl ConfirmPaymentIntentPaymentMethodDataKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataKonbini {}
impl ConfirmPaymentIntentPaymentMethodDataKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataLink {}
impl ConfirmPaymentIntentPaymentMethodDataLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataOxxo {}
impl ConfirmPaymentIntentPaymentMethodDataOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmPaymentIntentPaymentMethodDataP24Bank>,
}
impl ConfirmPaymentIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl ConfirmPaymentIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataPaynow {}
impl ConfirmPaymentIntentPaymentMethodDataPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataPix {}
impl ConfirmPaymentIntentPaymentMethodDataPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataPromptpay {}
impl ConfirmPaymentIntentPaymentMethodDataPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: ConfirmPaymentIntentPaymentMethodDataSofortCountry,
}
impl ConfirmPaymentIntentPaymentMethodDataSofort {
    pub fn new(country: ConfirmPaymentIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl ConfirmPaymentIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl ConfirmPaymentIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataWechatPay {}
impl ConfirmPaymentIntentPaymentMethodDataWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration for this PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<ConfirmPaymentIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<ConfirmPaymentIntentPaymentMethodOptionsAffirm>,
    /// If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpay<'a>>,
    /// If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<ConfirmPaymentIntentPaymentMethodOptionsAlipay>,
    /// If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebit>,
    /// If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<ConfirmPaymentIntentPaymentMethodOptionsBacsDebit>,
    /// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<ConfirmPaymentIntentPaymentMethodOptionsBancontact>,
    /// If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<ConfirmPaymentIntentPaymentMethodOptionsBlik<'a>>,
    /// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<ConfirmPaymentIntentPaymentMethodOptionsBoleto>,
    /// Configuration for any card payments attempted on this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ConfirmPaymentIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<ConfirmPaymentIntentPaymentMethodOptionsCardPresent>,
    /// If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<ConfirmPaymentIntentPaymentMethodOptionsCustomerBalance<'a>>,
    /// If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<ConfirmPaymentIntentPaymentMethodOptionsEps>,
    /// If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<ConfirmPaymentIntentPaymentMethodOptionsFpx>,
    /// If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<ConfirmPaymentIntentPaymentMethodOptionsGiropay>,
    /// If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<ConfirmPaymentIntentPaymentMethodOptionsGrabpay>,
    /// If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<ConfirmPaymentIntentPaymentMethodOptionsIdeal>,
    /// If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<ConfirmPaymentIntentPaymentMethodOptionsInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<ConfirmPaymentIntentPaymentMethodOptionsKlarna>,
    /// If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<ConfirmPaymentIntentPaymentMethodOptionsKonbini<'a>>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<ConfirmPaymentIntentPaymentMethodOptionsLink<'a>>,
    /// If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<ConfirmPaymentIntentPaymentMethodOptionsOxxo>,
    /// If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<ConfirmPaymentIntentPaymentMethodOptionsP24>,
    /// If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<ConfirmPaymentIntentPaymentMethodOptionsPaynow>,
    /// If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<ConfirmPaymentIntentPaymentMethodOptionsPix>,
    /// If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<ConfirmPaymentIntentPaymentMethodOptionsPromptpay>,
    /// If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmPaymentIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<ConfirmPaymentIntentPaymentMethodOptionsSofort>,
    /// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmPaymentIntentPaymentMethodOptionsUsBankAccount<'a>>,
    /// If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<ConfirmPaymentIntentPaymentMethodOptionsWechatPay<'a>>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsAffirm {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ConfirmPaymentIntentPaymentMethodOptionsAffirmCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    Manual,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsAffirmCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpay<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpay<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Manual,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsBacsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<ConfirmPaymentIntentPaymentMethodOptionsBancontactPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl ConfirmPaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsBlik<'a> {
    /// The 6-digit BLIK code that a customer has generated using their banking application.
    ///
    /// Can only be set on confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsBlik<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsBoleto {
    /// The number of calendar days before a Boleto voucher expires.
    ///
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto invoice will expire on Wednesday at 23:59 America/Sao_Paulo time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsBoletoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration for any card payments attempted on this PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCard<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ConfirmPaymentIntentPaymentMethodOptionsCardCaptureMethod>,
    /// A single-use `cvc_update` Token that represents a card CVC value.
    ///
    /// When provided, the CVC value will be verified during the card payment attempt.
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_token: Option<&'a str>,
    /// Installment configuration for payments attempted on this PaymentIntent (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<ConfirmPaymentIntentPaymentMethodOptionsCardInstallments>,
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter indicates that a transaction will be marked
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this PaymentIntent on.
    ///
    /// Depends on the available networks of the card attached to the PaymentIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<ConfirmPaymentIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<ConfirmPaymentIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsCardSetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardCaptureMethod {
    Manual,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Installment configuration for payments attempted on this PaymentIntent (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this PaymentIntent.
    /// This will cause the response to contain a list of available installment plans.
    /// Setting to false will prevent any selected plan from applying to a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The selected installment plan to use for this payment attempt.
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlan>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsCardInstallments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The selected installment plan to use for this payment attempt.
/// This parameter can only be provided during confirmation.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
}
impl ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: u64,
        interval: ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval,
        type_: ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
    }
}
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Month => "month",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedCount => "fixed_count",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        interval: ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: stripe_types::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            description: Default::default(),
            end_date: Default::default(),
            interval,
            interval_count: Default::default(),
            reference,
            start_date,
            supported_types: Default::default(),
        }
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Sporadic => "sporadic",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this PaymentIntent on.
///
/// Depends on the available networks of the card attached to the PaymentIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCardPresent {
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    ///
    /// Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization_support: Option<bool>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsCardPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<&'a [ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes]>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType,

}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new(
        type_: ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType,
    ) -> Self {
        Self {
            eu_bank_transfer: Default::default(),
            requested_address_types: Default::default(),
            type_,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str>
    for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsEps {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsEpsSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsFpx {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsFpxSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsFpx {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsGiropay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsGrabpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsIdealSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsInteracPresent {}
impl ConfirmPaymentIntentPaymentMethodOptionsInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ConfirmPaymentIntentPaymentMethodOptionsKlarnaCaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<ConfirmPaymentIntentPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl ConfirmPaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Preferred language of the Klarna authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    #[serde(rename = "da-DK")]
    DaMinusDk,
    #[serde(rename = "de-AT")]
    DeMinusAt,
    #[serde(rename = "de-CH")]
    DeMinusCh,
    #[serde(rename = "de-DE")]
    DeMinusDe,
    #[serde(rename = "en-AT")]
    EnMinusAt,
    #[serde(rename = "en-AU")]
    EnMinusAu,
    #[serde(rename = "en-BE")]
    EnMinusBe,
    #[serde(rename = "en-CA")]
    EnMinusCa,
    #[serde(rename = "en-CH")]
    EnMinusCh,
    #[serde(rename = "en-DE")]
    EnMinusDe,
    #[serde(rename = "en-DK")]
    EnMinusDk,
    #[serde(rename = "en-ES")]
    EnMinusEs,
    #[serde(rename = "en-FI")]
    EnMinusFi,
    #[serde(rename = "en-FR")]
    EnMinusFr,
    #[serde(rename = "en-GB")]
    EnMinusGb,
    #[serde(rename = "en-IE")]
    EnMinusIe,
    #[serde(rename = "en-IT")]
    EnMinusIt,
    #[serde(rename = "en-NL")]
    EnMinusNl,
    #[serde(rename = "en-NO")]
    EnMinusNo,
    #[serde(rename = "en-NZ")]
    EnMinusNz,
    #[serde(rename = "en-PL")]
    EnMinusPl,
    #[serde(rename = "en-PT")]
    EnMinusPt,
    #[serde(rename = "en-SE")]
    EnMinusSe,
    #[serde(rename = "en-US")]
    EnMinusUs,
    #[serde(rename = "es-ES")]
    EsMinusEs,
    #[serde(rename = "es-US")]
    EsMinusUs,
    #[serde(rename = "fi-FI")]
    FiMinusFi,
    #[serde(rename = "fr-BE")]
    FrMinusBe,
    #[serde(rename = "fr-CA")]
    FrMinusCa,
    #[serde(rename = "fr-CH")]
    FrMinusCh,
    #[serde(rename = "fr-FR")]
    FrMinusFr,
    #[serde(rename = "it-CH")]
    ItMinusCh,
    #[serde(rename = "it-IT")]
    ItMinusIt,
    #[serde(rename = "nb-NO")]
    NbMinusNo,
    #[serde(rename = "nl-BE")]
    NlMinusBe,
    #[serde(rename = "nl-NL")]
    NlMinusNl,
    #[serde(rename = "pl-PL")]
    PlMinusPl,
    #[serde(rename = "pt-PT")]
    PtMinusPt,
    #[serde(rename = "sv-FI")]
    SvMinusFi,
    #[serde(rename = "sv-SE")]
    SvMinusSe,
}

impl ConfirmPaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DaMinusDk => "da-DK",
            Self::DeMinusAt => "de-AT",
            Self::DeMinusCh => "de-CH",
            Self::DeMinusDe => "de-DE",
            Self::EnMinusAt => "en-AT",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusBe => "en-BE",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusCh => "en-CH",
            Self::EnMinusDe => "en-DE",
            Self::EnMinusDk => "en-DK",
            Self::EnMinusEs => "en-ES",
            Self::EnMinusFi => "en-FI",
            Self::EnMinusFr => "en-FR",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIt => "en-IT",
            Self::EnMinusNl => "en-NL",
            Self::EnMinusNo => "en-NO",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusPl => "en-PL",
            Self::EnMinusPt => "en-PT",
            Self::EnMinusSe => "en-SE",
            Self::EnMinusUs => "en-US",
            Self::EsMinusEs => "es-ES",
            Self::EsMinusUs => "es-US",
            Self::FiMinusFi => "fi-FI",
            Self::FrMinusBe => "fr-BE",
            Self::FrMinusCa => "fr-CA",
            Self::FrMinusCh => "fr-CH",
            Self::FrMinusFr => "fr-FR",
            Self::ItMinusCh => "it-CH",
            Self::ItMinusIt => "it-IT",
            Self::NbMinusNo => "nb-NO",
            Self::NlMinusBe => "nl-BE",
            Self::NlMinusNl => "nl-NL",
            Self::PlMinusPl => "pl-PL",
            Self::PtMinusPt => "pt-PT",
            Self::SvMinusFi => "sv-FI",
            Self::SvMinusSe => "sv-SE",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsKonbini<'a> {
    /// An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores.
    ///
    /// Must not consist of only zeroes and could be rejected in case of insufficient uniqueness.
    /// We recommend to use the customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<&'a str>,
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    ///
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    /// Defaults to 3 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// The timestamp at which the Konbini payment instructions will expire.
    ///
    /// Only one of `expires_after_days` or `expires_at` may be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// A product descriptor of up to 22 characters, which will appear to customers at the convenience store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsKonbini<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsLink<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<ConfirmPaymentIntentPaymentMethodOptionsLinkCaptureMethod>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsLinkSetupFutureUsage>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl ConfirmPaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsPaynow {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsPix {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    ///
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    /// The timestamp at which the Pix expires (between 10 and 1209600 seconds in the future).
    ///
    /// Defaults to 1 day in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsPixSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsPromptpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmPaymentIntentPaymentMethodOptionsSepaDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsSepaDebitMandateOptions {}
impl ConfirmPaymentIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsSofort {
    /// Language shown to the payer on redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<ConfirmPaymentIntentPaymentMethodOptionsSofortPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<ConfirmPaymentIntentPaymentMethodOptionsSofortSetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Language shown to the payer on redirect.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl ConfirmPaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Pl => "pl",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<&'a [ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions]>,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
#[serde(skip_serializing_if = "Option::is_none")]
pub return_url: Option<&'a str>,

}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for network related functions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Triggers validations to run across the selected networks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsWechatPay<'a> {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<&'a str>,
    /// The client type that the end customer will pay from.
    pub client: ConfirmPaymentIntentPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<ConfirmPaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsWechatPay<'a> {
    pub fn new(client: ConfirmPaymentIntentPaymentMethodOptionsWechatPayClient) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// The client type that the end customer will pay from.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl ConfirmPaymentIntentPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl ConfirmPaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmPaymentIntentSetupFutureUsage {
    OffSession,
    OnSession,
}

impl ConfirmPaymentIntentSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmPaymentIntentSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmPaymentIntentSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Shipping information for this PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentShipping<'a> {
    /// Shipping address.
    pub address: ConfirmPaymentIntentShippingAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentShipping<'a> {
    pub fn new(address: ConfirmPaymentIntentShippingAddress<'a>, name: &'a str) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name,
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentShippingAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> ConfirmPaymentIntentShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelPaymentIntent<'a> {
    /// Reason for canceling this PaymentIntent.
    ///
    /// Possible values are `duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<CancelPaymentIntentCancellationReason>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelPaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Reason for canceling this PaymentIntent.
///
/// Possible values are `duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelPaymentIntentCancellationReason {
    Abandoned,
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

impl CancelPaymentIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abandoned => "abandoned",
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for CancelPaymentIntentCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CancelPaymentIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CapturePaymentIntent<'a> {
    /// The amount to capture from the PaymentIntent, which must be less than or equal to the original amount.
    ///
    /// Any additional amount will be automatically refunded.
    /// Defaults to the full `amount_capturable` if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_to_capture: Option<i64>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// The parameters used to automatically create a Transfer when the payment
    /// is captured.
    ///
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CapturePaymentIntentTransferData>,
}
impl<'a> CapturePaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The parameters used to automatically create a Transfer when the payment
/// is captured.
///
/// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CapturePaymentIntentTransferData {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl CapturePaymentIntentTransferData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct IncrementAuthorizationPaymentIntent<'a> {
    /// The updated total amount you intend to collect from the cardholder.
    ///
    /// This amount must be greater than the currently authorized amount.
    pub amount: i64,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// The parameters used to automatically create a Transfer when the payment is captured.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<IncrementAuthorizationPaymentIntentTransferData>,
}
impl<'a> IncrementAuthorizationPaymentIntent<'a> {
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            application_fee_amount: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            statement_descriptor: Default::default(),
            transfer_data: Default::default(),
        }
    }
}
/// The parameters used to automatically create a Transfer when the payment is captured.
/// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct IncrementAuthorizationPaymentIntentTransferData {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl IncrementAuthorizationPaymentIntentTransferData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct VerifyMicrodepositsPaymentIntent<'a> {
    /// Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<&'a [i64]>,
    /// A six-character code starting with SM present in the microdeposit sent to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor_code: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> VerifyMicrodepositsPaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ApplyCustomerBalancePaymentIntent<'a> {
    /// Amount intended to be applied to this PaymentIntent from the customer’s cash balance.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    ///
    /// The maximum amount is the amount of the PaymentIntent.
    ///
    /// When omitted, the amount defaults to the remaining amount requested on the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ApplyCustomerBalancePaymentIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
