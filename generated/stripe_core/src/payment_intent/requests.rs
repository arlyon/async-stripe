impl stripe_core::payment_intent::PaymentIntent {
    /// Search for PaymentIntents you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    ///
    /// Under normal operating conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
    /// Search functionality is not available to merchants in India.
    pub fn search(
        client: &stripe::Client,
        params: SearchPaymentIntent,
    ) -> stripe::Response<SearchReturned> {
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
        client: &stripe::Client,
        params: CreatePaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form("/payment_intents", params, http_types::Method::Post)
    }
    /// Returns a list of PaymentIntents.
    pub fn list(
        client: &stripe::Client,
        params: ListPaymentIntent,
    ) -> stripe::Response<stripe_types::List<stripe_core::payment_intent::PaymentIntent>> {
        client.get_query("/payment_intents", params)
    }
    /// Retrieves the details of a PaymentIntent that has previously been created.
    ///
    /// Client-side retrieval using a publishable key is allowed when the `client_secret` is provided in the query string.
    /// When retrieved with a publishable key, only a subset of properties will be returned.
    /// Please refer to the [payment intent](https://stripe.com/docs/api#payment_intent_object) object reference for more details.
    pub fn retrieve(
        client: &stripe::Client,
        intent: &str,
        params: RetrievePaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
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
        client: &stripe::Client,
        intent: &str,
        params: UpdatePaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
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
        client: &stripe::Client,
        intent: &str,
        params: ConfirmPaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
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
        client: &stripe::Client,
        intent: &str,
        params: CancelPaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
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
        client: &stripe::Client,
        intent: &str,
        params: CapturePaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
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
        client: &stripe::Client,
        intent: &str,
        params: IncrementAuthorizationPaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/increment_authorization", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Verifies microdeposits on a PaymentIntent object.
    pub fn verify_microdeposits(
        client: &stripe::Client,
        intent: &str,
        params: VerifyMicrodepositsPaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
        client.send_form(
            &format!("/payment_intents/{intent}/verify_microdeposits", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Manually reconcile the remaining amount for a customer_balance PaymentIntent.
    pub fn apply_customer_balance(
        client: &stripe::Client,
        intent: &str,
        params: ApplyCustomerBalancePaymentIntent,
    ) -> stripe::Response<stripe_core::payment_intent::PaymentIntent> {
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SearchReturnedObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "search_result" => Ok(Self::SearchResult),

            _ => Err(()),
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
impl serde::Serialize for SearchReturnedObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchReturnedObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SearchReturnedObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SearchReturnedObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<SearchReturnedObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SearchReturnedObject::from_str(s)?);
        Ok(())
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Set to `true` to indicate that the customer is not in your checkout flow during this payment attempt, and therefore is unable to authenticate.
    ///
    /// This parameter is intended for scenarios where you collect card details and [charge them later](https://stripe.com/docs/payments/cards/charging-saved-cards).
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<OffSession>,
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
    pub radar_options: Option<RadarOptions<'a>>,
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
    pub shipping: Option<OptionalFieldsShipping<'a>>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentConfirmationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentConfirmationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub online: Option<OnlineParam<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: Type,
}
impl<'a> CreatePaymentIntentMandateDataCustomerAcceptance<'a> {
    pub fn new(type_: Type) -> Self {
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
/// If provided, this hash will be used to create a PaymentMethod.
///
/// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam<'a>>,
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
    pub au_becs_debit: Option<AuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<BacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePaymentIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePaymentIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Boleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreatePaymentIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<Eps>,
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
    pub klarna: Option<Klarna>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    pub radar_options: Option<RadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<Sofort>,
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
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataBancontact {}
impl CreatePaymentIntentPaymentMethodDataBancontact {
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
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataCustomerBalance {}
impl CreatePaymentIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,
    /// The customer's bank.
    pub bank: CreatePaymentIntentPaymentMethodDataFpxBank,
}
impl CreatePaymentIntentPaymentMethodDataFpx {
    pub fn new(bank: CreatePaymentIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodDataFpxBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "affin_bank" => Ok(Self::AffinBank),
            "agrobank" => Ok(Self::Agrobank),
            "alliance_bank" => Ok(Self::AllianceBank),
            "ambank" => Ok(Self::Ambank),
            "bank_islam" => Ok(Self::BankIslam),
            "bank_muamalat" => Ok(Self::BankMuamalat),
            "bank_of_china" => Ok(Self::BankOfChina),
            "bank_rakyat" => Ok(Self::BankRakyat),
            "bsn" => Ok(Self::Bsn),
            "cimb" => Ok(Self::Cimb),
            "deutsche_bank" => Ok(Self::DeutscheBank),
            "hong_leong_bank" => Ok(Self::HongLeongBank),
            "hsbc" => Ok(Self::Hsbc),
            "kfh" => Ok(Self::Kfh),
            "maybank2e" => Ok(Self::Maybank2e),
            "maybank2u" => Ok(Self::Maybank2u),
            "ocbc" => Ok(Self::Ocbc),
            "pb_enterprise" => Ok(Self::PbEnterprise),
            "public_bank" => Ok(Self::PublicBank),
            "rhb" => Ok(Self::Rhb),
            "standard_chartered" => Ok(Self::StandardChartered),
            "uob" => Ok(Self::Uob),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodDataFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodDataIdealBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abn_amro" => Ok(Self::AbnAmro),
            "asn_bank" => Ok(Self::AsnBank),
            "bunq" => Ok(Self::Bunq),
            "handelsbanken" => Ok(Self::Handelsbanken),
            "ing" => Ok(Self::Ing),
            "knab" => Ok(Self::Knab),
            "moneyou" => Ok(Self::Moneyou),
            "rabobank" => Ok(Self::Rabobank),
            "regiobank" => Ok(Self::Regiobank),
            "revolut" => Ok(Self::Revolut),
            "sns_bank" => Ok(Self::SnsBank),
            "triodos_bank" => Ok(Self::TriodosBank),
            "van_lanschot" => Ok(Self::VanLanschot),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodDataIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodDataP24Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alior_bank" => Ok(Self::AliorBank),
            "bank_millennium" => Ok(Self::BankMillennium),
            "bank_nowy_bfg_sa" => Ok(Self::BankNowyBfgSa),
            "bank_pekao_sa" => Ok(Self::BankPekaoSa),
            "banki_spbdzielcze" => Ok(Self::BankiSpbdzielcze),
            "blik" => Ok(Self::Blik),
            "bnp_paribas" => Ok(Self::BnpParibas),
            "boz" => Ok(Self::Boz),
            "citi_handlowy" => Ok(Self::CitiHandlowy),
            "credit_agricole" => Ok(Self::CreditAgricole),
            "envelobank" => Ok(Self::Envelobank),
            "etransfer_pocztowy24" => Ok(Self::EtransferPocztowy24),
            "getin_bank" => Ok(Self::GetinBank),
            "ideabank" => Ok(Self::Ideabank),
            "ing" => Ok(Self::Ing),
            "inteligo" => Ok(Self::Inteligo),
            "mbank_mtransfer" => Ok(Self::MbankMtransfer),
            "nest_przelew" => Ok(Self::NestPrzelew),
            "noble_pay" => Ok(Self::NoblePay),
            "pbac_z_ipko" => Ok(Self::PbacZIpko),
            "plus_bank" => Ok(Self::PlusBank),
            "santander_przelew24" => Ok(Self::SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(Self::TmobileUsbugiBankowe),
            "toyota_bank" => Ok(Self::ToyotaBank),
            "volkswagen_bank" => Ok(Self::VolkswagenBank),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodDataP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acss_debit" => Ok(Self::AcssDebit),
            "affirm" => Ok(Self::Affirm),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "blik" => Ok(Self::Blik),
            "boleto" => Ok(Self::Boleto),
            "customer_balance" => Ok(Self::CustomerBalance),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "konbini" => Ok(Self::Konbini),
            "link" => Ok(Self::Link),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "paynow" => Ok(Self::Paynow),
            "pix" => Ok(Self::Pix),
            "promptpay" => Ok(Self::Promptpay),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "us_bank_account" => Ok(Self::UsBankAccount),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
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
    pub blik: Option<PaymentIntentPaymentMethodOptionsParam<'a>>,
    /// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentIntentPaymentMethodOptionsBoleto>,
    /// Configuration for any card payments attempted on this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreatePaymentIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodOptionsParam>,
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
    pub mandate_options: Option<PaymentIntentPaymentMethodOptionsMandateOptionsParam<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<PreferredLanguage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl CreatePaymentIntentPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
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
    pub capture_method: Option<CaptureMethod>,
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
    pub network: Option<Network>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<RequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
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
    pub interval: Interval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
}
impl CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: u64,
        interval: Interval,
        type_: CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_count" => Ok(Self::FixedCount),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub amount_type: AmountType,
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
    pub supported_types: Option<&'a [SupportedTypes]>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: AmountType,
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
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "sporadic" => Ok(Self::Sporadic),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub funding_type: Option<FundingType>,
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
    pub eu_bank_transfer: Option<EuBankTransferParams<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<&'a [RequestedAddressTypes]>,
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
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eu_bank_transfer" => Ok(Self::EuBankTransfer),
            "gb_bank_transfer" => Ok(Self::GbBankTransfer),
            "jp_bank_transfer" => Ok(Self::JpBankTransfer),
            "mx_bank_transfer" => Ok(Self::MxBankTransfer),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<PreferredLocale>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "fr" => Ok(Self::Fr),
            "it" => Ok(Self::It),
            "nl" => Ok(Self::Nl),
            "pl" => Ok(Self::Pl),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<LinkedAccountOptionsParam<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<NetworksOptionsParam<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> CreatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
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
    pub client: Client,
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
    pub fn new(client: Client) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CreatePaymentIntentSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for CreatePaymentIntentSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    pub shipping: Option<OptionalFieldsShipping<'a>>,
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
    pub transfer_data: Option<TransferDataUpdateParams>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If provided, this hash will be used to create a PaymentMethod.
///
/// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam<'a>>,
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
    pub au_becs_debit: Option<AuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<BacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdatePaymentIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdatePaymentIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Boleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdatePaymentIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<Eps>,
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
    pub klarna: Option<Klarna>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    pub radar_options: Option<RadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<Sofort>,
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
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataBancontact {}
impl UpdatePaymentIntentPaymentMethodDataBancontact {
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
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataCustomerBalance {}
impl UpdatePaymentIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,
    /// The customer's bank.
    pub bank: UpdatePaymentIntentPaymentMethodDataFpxBank,
}
impl UpdatePaymentIntentPaymentMethodDataFpx {
    pub fn new(bank: UpdatePaymentIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodDataFpxBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "affin_bank" => Ok(Self::AffinBank),
            "agrobank" => Ok(Self::Agrobank),
            "alliance_bank" => Ok(Self::AllianceBank),
            "ambank" => Ok(Self::Ambank),
            "bank_islam" => Ok(Self::BankIslam),
            "bank_muamalat" => Ok(Self::BankMuamalat),
            "bank_of_china" => Ok(Self::BankOfChina),
            "bank_rakyat" => Ok(Self::BankRakyat),
            "bsn" => Ok(Self::Bsn),
            "cimb" => Ok(Self::Cimb),
            "deutsche_bank" => Ok(Self::DeutscheBank),
            "hong_leong_bank" => Ok(Self::HongLeongBank),
            "hsbc" => Ok(Self::Hsbc),
            "kfh" => Ok(Self::Kfh),
            "maybank2e" => Ok(Self::Maybank2e),
            "maybank2u" => Ok(Self::Maybank2u),
            "ocbc" => Ok(Self::Ocbc),
            "pb_enterprise" => Ok(Self::PbEnterprise),
            "public_bank" => Ok(Self::PublicBank),
            "rhb" => Ok(Self::Rhb),
            "standard_chartered" => Ok(Self::StandardChartered),
            "uob" => Ok(Self::Uob),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodDataFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodDataIdealBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abn_amro" => Ok(Self::AbnAmro),
            "asn_bank" => Ok(Self::AsnBank),
            "bunq" => Ok(Self::Bunq),
            "handelsbanken" => Ok(Self::Handelsbanken),
            "ing" => Ok(Self::Ing),
            "knab" => Ok(Self::Knab),
            "moneyou" => Ok(Self::Moneyou),
            "rabobank" => Ok(Self::Rabobank),
            "regiobank" => Ok(Self::Regiobank),
            "revolut" => Ok(Self::Revolut),
            "sns_bank" => Ok(Self::SnsBank),
            "triodos_bank" => Ok(Self::TriodosBank),
            "van_lanschot" => Ok(Self::VanLanschot),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodDataIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodDataP24Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alior_bank" => Ok(Self::AliorBank),
            "bank_millennium" => Ok(Self::BankMillennium),
            "bank_nowy_bfg_sa" => Ok(Self::BankNowyBfgSa),
            "bank_pekao_sa" => Ok(Self::BankPekaoSa),
            "banki_spbdzielcze" => Ok(Self::BankiSpbdzielcze),
            "blik" => Ok(Self::Blik),
            "bnp_paribas" => Ok(Self::BnpParibas),
            "boz" => Ok(Self::Boz),
            "citi_handlowy" => Ok(Self::CitiHandlowy),
            "credit_agricole" => Ok(Self::CreditAgricole),
            "envelobank" => Ok(Self::Envelobank),
            "etransfer_pocztowy24" => Ok(Self::EtransferPocztowy24),
            "getin_bank" => Ok(Self::GetinBank),
            "ideabank" => Ok(Self::Ideabank),
            "ing" => Ok(Self::Ing),
            "inteligo" => Ok(Self::Inteligo),
            "mbank_mtransfer" => Ok(Self::MbankMtransfer),
            "nest_przelew" => Ok(Self::NestPrzelew),
            "noble_pay" => Ok(Self::NoblePay),
            "pbac_z_ipko" => Ok(Self::PbacZIpko),
            "plus_bank" => Ok(Self::PlusBank),
            "santander_przelew24" => Ok(Self::SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(Self::TmobileUsbugiBankowe),
            "toyota_bank" => Ok(Self::ToyotaBank),
            "volkswagen_bank" => Ok(Self::VolkswagenBank),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodDataP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acss_debit" => Ok(Self::AcssDebit),
            "affirm" => Ok(Self::Affirm),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "blik" => Ok(Self::Blik),
            "boleto" => Ok(Self::Boleto),
            "customer_balance" => Ok(Self::CustomerBalance),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "konbini" => Ok(Self::Konbini),
            "link" => Ok(Self::Link),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "paynow" => Ok(Self::Paynow),
            "pix" => Ok(Self::Pix),
            "promptpay" => Ok(Self::Promptpay),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "us_bank_account" => Ok(Self::UsBankAccount),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
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
    pub blik: Option<PaymentIntentPaymentMethodOptionsParam<'a>>,
    /// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdatePaymentIntentPaymentMethodOptionsBoleto>,
    /// Configuration for any card payments attempted on this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdatePaymentIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodOptionsParam>,
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
    pub mandate_options: Option<PaymentIntentPaymentMethodOptionsMandateOptionsParam<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<PreferredLanguage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl UpdatePaymentIntentPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
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
    pub capture_method: Option<CaptureMethod>,
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
    pub network: Option<Network>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<RequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
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
    pub interval: Interval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
}
impl UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: u64,
        interval: Interval,
        type_: UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_count" => Ok(Self::FixedCount),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub amount_type: AmountType,
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
    pub supported_types: Option<&'a [SupportedTypes]>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: AmountType,
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
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "sporadic" => Ok(Self::Sporadic),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub funding_type: Option<FundingType>,
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
    pub eu_bank_transfer: Option<EuBankTransferParams<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<&'a [RequestedAddressTypes]>,
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
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eu_bank_transfer" => Ok(Self::EuBankTransfer),
            "gb_bank_transfer" => Ok(Self::GbBankTransfer),
            "jp_bank_transfer" => Ok(Self::JpBankTransfer),
            "mx_bank_transfer" => Ok(Self::MxBankTransfer),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<PreferredLocale>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "fr" => Ok(Self::Fr),
            "it" => Ok(Self::It),
            "nl" => Ok(Self::Nl),
            "pl" => Ok(Self::Pl),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<LinkedAccountOptionsParam<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<NetworksOptionsParam<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> UpdatePaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
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
    pub client: Client,
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
    pub fn new(client: Client) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for UpdatePaymentIntentSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for UpdatePaymentIntentSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub off_session: Option<OffSession>,
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
    pub radar_options: Option<RadarOptions<'a>>,
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
    pub shipping: Option<OptionalFieldsShipping<'a>>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub online: Option<OnlineParam<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: Type,
}
impl<'a> ConfirmPaymentIntentSecretKeyParamCustomerAcceptance<'a> {
    pub fn new(type_: Type) -> Self {
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "online" => Ok(Self::Online),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentClientKeyParamCustomerAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If provided, this hash will be used to create a PaymentMethod.
///
/// The new PaymentMethod will appear in the [payment_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-payment_method) property on the PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodParam<'a>>,
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
    pub au_becs_debit: Option<AuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<BacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<ConfirmPaymentIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetailsInnerParams<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<ConfirmPaymentIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Boleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<ConfirmPaymentIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<Eps>,
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
    pub klarna: Option<Klarna>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
    pub radar_options: Option<RadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<Sofort>,
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
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataBancontact {}
impl ConfirmPaymentIntentPaymentMethodDataBancontact {
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
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataCustomerBalance {}
impl ConfirmPaymentIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,
    /// The customer's bank.
    pub bank: ConfirmPaymentIntentPaymentMethodDataFpxBank,
}
impl ConfirmPaymentIntentPaymentMethodDataFpx {
    pub fn new(bank: ConfirmPaymentIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodDataFpxBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "affin_bank" => Ok(Self::AffinBank),
            "agrobank" => Ok(Self::Agrobank),
            "alliance_bank" => Ok(Self::AllianceBank),
            "ambank" => Ok(Self::Ambank),
            "bank_islam" => Ok(Self::BankIslam),
            "bank_muamalat" => Ok(Self::BankMuamalat),
            "bank_of_china" => Ok(Self::BankOfChina),
            "bank_rakyat" => Ok(Self::BankRakyat),
            "bsn" => Ok(Self::Bsn),
            "cimb" => Ok(Self::Cimb),
            "deutsche_bank" => Ok(Self::DeutscheBank),
            "hong_leong_bank" => Ok(Self::HongLeongBank),
            "hsbc" => Ok(Self::Hsbc),
            "kfh" => Ok(Self::Kfh),
            "maybank2e" => Ok(Self::Maybank2e),
            "maybank2u" => Ok(Self::Maybank2u),
            "ocbc" => Ok(Self::Ocbc),
            "pb_enterprise" => Ok(Self::PbEnterprise),
            "public_bank" => Ok(Self::PublicBank),
            "rhb" => Ok(Self::Rhb),
            "standard_chartered" => Ok(Self::StandardChartered),
            "uob" => Ok(Self::Uob),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodDataFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodDataIdealBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abn_amro" => Ok(Self::AbnAmro),
            "asn_bank" => Ok(Self::AsnBank),
            "bunq" => Ok(Self::Bunq),
            "handelsbanken" => Ok(Self::Handelsbanken),
            "ing" => Ok(Self::Ing),
            "knab" => Ok(Self::Knab),
            "moneyou" => Ok(Self::Moneyou),
            "rabobank" => Ok(Self::Rabobank),
            "regiobank" => Ok(Self::Regiobank),
            "revolut" => Ok(Self::Revolut),
            "sns_bank" => Ok(Self::SnsBank),
            "triodos_bank" => Ok(Self::TriodosBank),
            "van_lanschot" => Ok(Self::VanLanschot),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodDataIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodDataP24Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alior_bank" => Ok(Self::AliorBank),
            "bank_millennium" => Ok(Self::BankMillennium),
            "bank_nowy_bfg_sa" => Ok(Self::BankNowyBfgSa),
            "bank_pekao_sa" => Ok(Self::BankPekaoSa),
            "banki_spbdzielcze" => Ok(Self::BankiSpbdzielcze),
            "blik" => Ok(Self::Blik),
            "bnp_paribas" => Ok(Self::BnpParibas),
            "boz" => Ok(Self::Boz),
            "citi_handlowy" => Ok(Self::CitiHandlowy),
            "credit_agricole" => Ok(Self::CreditAgricole),
            "envelobank" => Ok(Self::Envelobank),
            "etransfer_pocztowy24" => Ok(Self::EtransferPocztowy24),
            "getin_bank" => Ok(Self::GetinBank),
            "ideabank" => Ok(Self::Ideabank),
            "ing" => Ok(Self::Ing),
            "inteligo" => Ok(Self::Inteligo),
            "mbank_mtransfer" => Ok(Self::MbankMtransfer),
            "nest_przelew" => Ok(Self::NestPrzelew),
            "noble_pay" => Ok(Self::NoblePay),
            "pbac_z_ipko" => Ok(Self::PbacZIpko),
            "plus_bank" => Ok(Self::PlusBank),
            "santander_przelew24" => Ok(Self::SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(Self::TmobileUsbugiBankowe),
            "toyota_bank" => Ok(Self::ToyotaBank),
            "volkswagen_bank" => Ok(Self::VolkswagenBank),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodDataP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acss_debit" => Ok(Self::AcssDebit),
            "affirm" => Ok(Self::Affirm),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "blik" => Ok(Self::Blik),
            "boleto" => Ok(Self::Boleto),
            "customer_balance" => Ok(Self::CustomerBalance),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "konbini" => Ok(Self::Konbini),
            "link" => Ok(Self::Link),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "paynow" => Ok(Self::Paynow),
            "pix" => Ok(Self::Pix),
            "promptpay" => Ok(Self::Promptpay),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "us_bank_account" => Ok(Self::UsBankAccount),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
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
    pub blik: Option<PaymentIntentPaymentMethodOptionsParam<'a>>,
    /// If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<ConfirmPaymentIntentPaymentMethodOptionsBoleto>,
    /// Configuration for any card payments attempted on this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ConfirmPaymentIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodOptionsParam>,
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
    pub mandate_options: Option<PaymentIntentPaymentMethodOptionsMandateOptionsParam<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsAffirmSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr
    for ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsAlipaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<PreferredLanguage>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsBancontactSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
}
impl ConfirmPaymentIntentPaymentMethodOptionsBoleto {
    pub fn new() -> Self {
        Self::default()
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
    pub capture_method: Option<CaptureMethod>,
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
    pub network: Option<Network>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<RequestThreeDSecure>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
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
    pub interval: Interval,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
}
impl ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlan {
    pub fn new(
        count: u64,
        interval: Interval,
        type_: ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType,
    ) -> Self {
        Self { count, interval, type_ }
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_count" => Ok(Self::FixedCount),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsCardInstallmentsPlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub amount_type: AmountType,
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
    pub supported_types: Option<&'a [SupportedTypes]>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: AmountType,
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
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "sporadic" => Ok(Self::Sporadic),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub funding_type: Option<FundingType>,
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
    pub eu_bank_transfer: Option<EuBankTransferParams<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<&'a [RequestedAddressTypes]>,
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
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eu_bank_transfer" => Ok(Self::EuBankTransfer),
            "gb_bank_transfer" => Ok(Self::GbBankTransfer),
            "jp_bank_transfer" => Ok(Self::JpBankTransfer),
            "mx_bank_transfer" => Ok(Self::MxBankTransfer),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsFpxSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsGiropaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsIdealSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<PreferredLocale>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub capture_method: Option<CaptureMethod>,
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
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsOxxoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsP24SetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsPaynowSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsPixSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsPromptpaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub setup_future_usage: Option<SetupFutureUsage>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "fr" => Ok(Self::Fr),
            "it" => Ok(Self::It),
            "nl" => Ok(Self::Nl),
            "pl" => Ok(Self::Pl),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsSofortSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmPaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<LinkedAccountOptionsParam<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<NetworksOptionsParam<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<VerificationMethod>,
}
impl<'a> ConfirmPaymentIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
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
    pub client: Client,
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
    pub fn new(client: Client) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConfirmPaymentIntentSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for ConfirmPaymentIntentSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CancelPaymentIntentCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abandoned" => Ok(Self::Abandoned),
            "duplicate" => Ok(Self::Duplicate),
            "fraudulent" => Ok(Self::Fraudulent),
            "requested_by_customer" => Ok(Self::RequestedByCustomer),

            _ => Err(()),
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
impl serde::Serialize for CancelPaymentIntentCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub transfer_data: Option<TransferDataUpdateParams>,
}
impl<'a> CapturePaymentIntent<'a> {
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// The parameters used to automatically create a Transfer when the payment is captured.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataUpdateParams>,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OnlineParam<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: &'a str,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: &'a str,
}
impl<'a> OnlineParam<'a> {
    pub fn new(ip_address: &'a str, user_agent: &'a str) -> Self {
        Self { ip_address, user_agent }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Offline,
    Online,
}

impl Type {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "offline" => Ok(Self::Offline),
            "online" => Ok(Self::Online),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum OffSession {
    Bool(bool),
    OneOff,
    Recurring,
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PaymentMethodParam<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> PaymentMethodParam<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> AuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> BacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Address<'a> {
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
impl<'a> Address<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Boleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> Boleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Bank {
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

impl Bank {
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

impl std::str::FromStr for Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arzte_und_apotheker_bank" => Ok(Self::ArzteUndApothekerBank),
            "austrian_anadi_bank_ag" => Ok(Self::AustrianAnadiBankAg),
            "bank_austria" => Ok(Self::BankAustria),
            "bankhaus_carl_spangler" => Ok(Self::BankhausCarlSpangler),
            "bankhaus_schelhammer_und_schattera_ag" => Ok(Self::BankhausSchelhammerUndSchatteraAg),
            "bawag_psk_ag" => Ok(Self::BawagPskAg),
            "bks_bank_ag" => Ok(Self::BksBankAg),
            "brull_kallmus_bank_ag" => Ok(Self::BrullKallmusBankAg),
            "btv_vier_lander_bank" => Ok(Self::BtvVierLanderBank),
            "capital_bank_grawe_gruppe_ag" => Ok(Self::CapitalBankGraweGruppeAg),
            "deutsche_bank_ag" => Ok(Self::DeutscheBankAg),
            "dolomitenbank" => Ok(Self::Dolomitenbank),
            "easybank_ag" => Ok(Self::EasybankAg),
            "erste_bank_und_sparkassen" => Ok(Self::ErsteBankUndSparkassen),
            "hypo_alpeadriabank_international_ag" => Ok(Self::HypoAlpeadriabankInternationalAg),
            "hypo_bank_burgenland_aktiengesellschaft" => {
                Ok(Self::HypoBankBurgenlandAktiengesellschaft)
            }
            "hypo_noe_lb_fur_niederosterreich_u_wien" => {
                Ok(Self::HypoNoeLbFurNiederosterreichUWien)
            }
            "hypo_oberosterreich_salzburg_steiermark" => {
                Ok(Self::HypoOberosterreichSalzburgSteiermark)
            }
            "hypo_tirol_bank_ag" => Ok(Self::HypoTirolBankAg),
            "hypo_vorarlberg_bank_ag" => Ok(Self::HypoVorarlbergBankAg),
            "marchfelder_bank" => Ok(Self::MarchfelderBank),
            "oberbank_ag" => Ok(Self::OberbankAg),
            "raiffeisen_bankengruppe_osterreich" => Ok(Self::RaiffeisenBankengruppeOsterreich),
            "schoellerbank_ag" => Ok(Self::SchoellerbankAg),
            "sparda_bank_wien" => Ok(Self::SpardaBankWien),
            "volksbank_gruppe" => Ok(Self::VolksbankGruppe),
            "volkskreditbank_ag" => Ok(Self::VolkskreditbankAg),
            "vr_bank_braunau" => Ok(Self::VrBankBraunau),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountHolderType {
    Company,
    Individual,
}

impl AccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl std::str::FromStr for AccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "company" => Ok(Self::Company),
            "individual" => Ok(Self::Individual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DateOfBirth {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl DateOfBirth {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> RadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> SepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Country {
    At,
    Be,
    De,
    Es,
    It,
    Nl,
}

impl Country {
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

impl std::str::FromStr for Country {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AT" => Ok(Self::At),
            "BE" => Ok(Self::Be),
            "DE" => Ok(Self::De),
            "ES" => Ok(Self::Es),
            "IT" => Ok(Self::It),
            "NL" => Ok(Self::Nl),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Country {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Country {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountType {
    Checking,
    Savings,
}

impl AccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl std::str::FromStr for AccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "checking" => Ok(Self::Checking),
            "savings" => Ok(Self::Savings),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl PaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for PaymentSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combined" => Ok(Self::Combined),
            "interval" => Ok(Self::Interval),
            "sporadic" => Ok(Self::Sporadic),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
    Business,
    Personal,
}

impl TransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl std::str::FromStr for TransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business" => Ok(Self::Business),
            "personal" => Ok(Self::Personal),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for SetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl VerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for VerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "instant" => Ok(Self::Instant),
            "microdeposits" => Ok(Self::Microdeposits),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CaptureMethod {
    Manual,
}

impl CaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for CaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl PreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl std::str::FromStr for PreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "nl" => Ok(Self::Nl),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PaymentIntentPaymentMethodOptionsParam<'a> {
    /// The 6-digit BLIK code that a customer has generated using their banking application.
    ///
    /// Can only be set on confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}
impl<'a> PaymentIntentPaymentMethodOptionsParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interval {
    Month,
}

impl Interval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Month => "month",
        }
    }
}

impl std::str::FromStr for Interval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "month" => Ok(Self::Month),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Interval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AmountType {
    Fixed,
    Maximum,
}

impl AmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for AmountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed" => Ok(Self::Fixed),
            "maximum" => Ok(Self::Maximum),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SupportedTypes {
    India,
}

impl SupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl std::str::FromStr for SupportedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "india" => Ok(Self::India),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Network {
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

impl Network {
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

impl std::str::FromStr for Network {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amex" => Ok(Self::Amex),
            "cartes_bancaires" => Ok(Self::CartesBancaires),
            "diners" => Ok(Self::Diners),
            "discover" => Ok(Self::Discover),
            "interac" => Ok(Self::Interac),
            "jcb" => Ok(Self::Jcb),
            "mastercard" => Ok(Self::Mastercard),
            "unionpay" => Ok(Self::Unionpay),
            "unknown" => Ok(Self::Unknown),
            "visa" => Ok(Self::Visa),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Network {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Network {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestThreeDSecure {
    Any,
    Automatic,
}

impl RequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl std::str::FromStr for RequestThreeDSecure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "any" => Ok(Self::Any),
            "automatic" => Ok(Self::Automatic),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for RequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for RequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PaymentMethodOptionsParam {
    /// Request ability to capture this payment beyond the standard [authorization validity window](https://stripe.com/docs/terminal/features/extended-authorizations#authorization-validity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<bool>,
    /// Request ability to [increment](https://stripe.com/docs/terminal/features/incremental-authorizations) this PaymentIntent if the combination of MCC and card brand is eligible.
    ///
    /// Check [incremental_authorization_supported](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-card_present-incremental_authorization_supported) in the [Confirm](https://stripe.com/docs/api/payment_intents/confirm) response to verify support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization_support: Option<bool>,
}
impl PaymentMethodOptionsParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct EuBankTransferParams<'a> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> EuBankTransferParams<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl RequestedAddressTypes {
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

impl std::str::FromStr for RequestedAddressTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "iban" => Ok(Self::Iban),
            "sepa" => Ok(Self::Sepa),
            "sort_code" => Ok(Self::SortCode),
            "spei" => Ok(Self::Spei),
            "zengin" => Ok(Self::Zengin),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for RequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for RequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FundingType {
    BankTransfer,
}

impl FundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for FundingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bank_transfer" => Ok(Self::BankTransfer),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PreferredLocale {
    DaMinusDk,
    DeMinusAt,
    DeMinusCh,
    DeMinusDe,
    EnMinusAt,
    EnMinusAu,
    EnMinusBe,
    EnMinusCa,
    EnMinusCh,
    EnMinusDe,
    EnMinusDk,
    EnMinusEs,
    EnMinusFi,
    EnMinusFr,
    EnMinusGb,
    EnMinusIe,
    EnMinusIt,
    EnMinusNl,
    EnMinusNo,
    EnMinusNz,
    EnMinusPl,
    EnMinusPt,
    EnMinusSe,
    EnMinusUs,
    EsMinusEs,
    EsMinusUs,
    FiMinusFi,
    FrMinusBe,
    FrMinusCa,
    FrMinusCh,
    FrMinusFr,
    ItMinusCh,
    ItMinusIt,
    NbMinusNo,
    NlMinusBe,
    NlMinusNl,
    PlMinusPl,
    PtMinusPt,
    SvMinusFi,
    SvMinusSe,
}

impl PreferredLocale {
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

impl std::str::FromStr for PreferredLocale {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "da-DK" => Ok(Self::DaMinusDk),
            "de-AT" => Ok(Self::DeMinusAt),
            "de-CH" => Ok(Self::DeMinusCh),
            "de-DE" => Ok(Self::DeMinusDe),
            "en-AT" => Ok(Self::EnMinusAt),
            "en-AU" => Ok(Self::EnMinusAu),
            "en-BE" => Ok(Self::EnMinusBe),
            "en-CA" => Ok(Self::EnMinusCa),
            "en-CH" => Ok(Self::EnMinusCh),
            "en-DE" => Ok(Self::EnMinusDe),
            "en-DK" => Ok(Self::EnMinusDk),
            "en-ES" => Ok(Self::EnMinusEs),
            "en-FI" => Ok(Self::EnMinusFi),
            "en-FR" => Ok(Self::EnMinusFr),
            "en-GB" => Ok(Self::EnMinusGb),
            "en-IE" => Ok(Self::EnMinusIe),
            "en-IT" => Ok(Self::EnMinusIt),
            "en-NL" => Ok(Self::EnMinusNl),
            "en-NO" => Ok(Self::EnMinusNo),
            "en-NZ" => Ok(Self::EnMinusNz),
            "en-PL" => Ok(Self::EnMinusPl),
            "en-PT" => Ok(Self::EnMinusPt),
            "en-SE" => Ok(Self::EnMinusSe),
            "en-US" => Ok(Self::EnMinusUs),
            "es-ES" => Ok(Self::EsMinusEs),
            "es-US" => Ok(Self::EsMinusUs),
            "fi-FI" => Ok(Self::FiMinusFi),
            "fr-BE" => Ok(Self::FrMinusBe),
            "fr-CA" => Ok(Self::FrMinusCa),
            "fr-CH" => Ok(Self::FrMinusCh),
            "fr-FR" => Ok(Self::FrMinusFr),
            "it-CH" => Ok(Self::ItMinusCh),
            "it-IT" => Ok(Self::ItMinusIt),
            "nb-NO" => Ok(Self::NbMinusNo),
            "nl-BE" => Ok(Self::NlMinusBe),
            "nl-NL" => Ok(Self::NlMinusNl),
            "pl-PL" => Ok(Self::PlMinusPl),
            "pt-PT" => Ok(Self::PtMinusPt),
            "sv-FI" => Ok(Self::SvMinusFi),
            "sv-SE" => Ok(Self::SvMinusSe),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PreferredLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Permissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl Permissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for Permissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "balances" => Ok(Self::Balances),
            "ownership" => Ok(Self::Ownership),
            "payment_method" => Ok(Self::PaymentMethod),
            "transactions" => Ok(Self::Transactions),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Permissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Requested {
    Ach,
    UsDomesticWire,
}

impl Requested {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for Requested {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach" => Ok(Self::Ach),
            "us_domestic_wire" => Ok(Self::UsDomesticWire),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Requested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Requested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Requested {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Client {
    Android,
    Ios,
    Web,
}

impl Client {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl std::str::FromStr for Client {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "android" => Ok(Self::Android),
            "ios" => Ok(Self::Ios),
            "web" => Ok(Self::Web),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Client {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct TransferDataUpdateParams {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl TransferDataUpdateParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BillingDetailsInnerParams<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address<'a>>,
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
impl<'a> BillingDetailsInnerParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Eps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Bank>,
}
impl Eps {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct Klarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
}
impl Klarna {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Sofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Country,
}
impl Sofort {
    pub fn new(country: Country) -> Self {
        Self { country }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsParam<'a> {
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
    pub payment_schedule: Option<PaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
}
impl<'a> PaymentIntentPaymentMethodOptionsMandateOptionsParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct LinkedAccountOptionsParam<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<&'a [Permissions]>,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> LinkedAccountOptionsParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct NetworksOptionsParam<'a> {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<&'a [Requested]>,
}
impl<'a> NetworksOptionsParam<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OptionalFieldsShipping<'a> {
    /// Shipping address.
    pub address: Address<'a>,
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
impl<'a> OptionalFieldsShipping<'a> {
    pub fn new(address: Address<'a>, name: &'a str) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name,
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
