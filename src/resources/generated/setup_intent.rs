// ======================================
// This file was automatically generated.
// ======================================

use self::{
    Account, ApiErrors, Application, Currency, Customer, Mandate, PaymentMethod, SetupAttempt,
};
use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId, SetupIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SetupIntent".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntent {
    /// Unique identifier for the object.
    pub id: SetupIntentId,

    /// ID of the Connect application that created the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    /// Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<SetupIntentCancellationReason>,

    /// The client secret of this SetupIntent.
    ///
    /// Used for client-side retrieval using a publishable key.  The client secret can be used to complete payment setup from your frontend.
    /// It should not be stored, logged, embedded in URLs, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The error encountered in the previous SetupIntent confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_setup_error: Option<ApiErrors>,

    /// The most recent SetupAttempt for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_attempt: Option<Expandable<SetupAttempt>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// ID of the multi use Mandate generated by the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Expandable<Mandate>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// If present, this property tells you what actions you need to take in order for your customer to continue payment setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<SetupIntentNextAction>,

    /// The account (if any) for which the setup is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Expandable<Account>>,

    /// ID of the payment method used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Expandable<PaymentMethod>>,

    /// Payment-method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<SetupIntentPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) that this SetupIntent is allowed to set up.
    pub payment_method_types: Vec<String>,

    /// ID of the single_use Mandate generated by the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use_mandate: Option<Expandable<Mandate>>,

    /// [Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
    pub status: SetupIntentStatus,

    /// Indicates how the payment method is intended to be used in the future.
    ///
    /// Use `on_session` if you intend to only reuse the payment method when the customer is in your checkout flow.
    ///
    /// Use `off_session` if your customer may or may not be in your checkout flow.
    /// If not provided, this value defaults to `off_session`.
    pub usage: String,
}

impl SetupIntent {
    /// Returns a list of SetupIntents.
    pub fn list(client: &Client, params: ListSetupIntents<'_>) -> Response<List<SetupIntent>> {
        client.get_query("/setup_intents", &params)
    }

    /// Creates a SetupIntent object.
    ///
    /// After the SetupIntent is created, attach a payment method and [confirm](https://stripe.com/docs/api/setup_intents/confirm)
    /// to collect any required permissions to charge the payment method later.
    pub fn create(client: &Client, params: CreateSetupIntent<'_>) -> Response<SetupIntent> {
        client.post_form("/setup_intents", &params)
    }

    /// Retrieves the details of a SetupIntent that has previously been created.
    ///
    /// Client-side retrieval using a publishable key is allowed when the `client_secret` is provided in the query string.
    /// When retrieved with a publishable key, only a subset of properties will be returned.
    /// Please refer to the [SetupIntent](https://stripe.com/docs/api#setup_intent_object) object reference for more details.
    pub fn retrieve(client: &Client, id: &SetupIntentId, expand: &[&str]) -> Response<SetupIntent> {
        client.get_query(&format!("/setup_intents/{}", id), &Expand { expand })
    }

    /// Updates a SetupIntent object.
    pub fn update(
        client: &Client,
        id: &SetupIntentId,
        params: UpdateSetupIntent<'_>,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}", id), &params)
    }
}

impl Object for SetupIntent {
    type Id = SetupIntentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "setup_intent"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntentNextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<SetupIntentNextActionRedirectToUrl>,

    /// Type of the next action to perform, one of `redirect_to_url` or `use_stripe_sdk`.
    #[serde(rename = "type")]
    pub type_: String,

    /// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    ///
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntentNextActionRedirectToUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// The URL you must redirect your customer to in order to authenticate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SetupIntentPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SetupIntentPaymentMethodOptionsSepaDebit>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsCard {
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure: Option<SetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {}

/// The parameters for `SetupIntent::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSetupIntent<'a> {
    /// Set to `true` to attempt to confirm this SetupIntent immediately.
    ///
    /// This parameter defaults to `false`.
    /// If the payment method attached is a card, a return_url may be provided in case additional authentication is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,

    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// This hash contains details about the Mandate to create.
    ///
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<CreateSetupIntentMandateData>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The Stripe account ID for which this SetupIntent is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// Payment-method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateSetupIntentPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) that this SetupIntent is allowed to use.
    /// If this is not provided, defaults to ["card"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,

    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    ///
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,

    /// If this hash is populated, this SetupIntent will generate a single_use Mandate on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<CreateSetupIntentSingleUse>,
}

impl<'a> CreateSetupIntent<'a> {
    pub fn new() -> Self {
        CreateSetupIntent {
            confirm: Default::default(),
            customer: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            mandate_data: Default::default(),
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_method: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            return_url: Default::default(),
            single_use: Default::default(),
        }
    }
}

/// The parameters for `SetupIntent::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListSetupIntents<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return SetupIntents for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SetupIntentId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return SetupIntents associated with the specified payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SetupIntentId>,
}

impl<'a> ListSetupIntents<'a> {
    pub fn new() -> Self {
        ListSetupIntents {
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_method: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `SetupIntent::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSetupIntent<'a> {
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// Payment-method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateSetupIntentPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) that this SetupIntent is allowed to set up.
    /// If this is not provided, defaults to ["card"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
}

impl<'a> UpdateSetupIntent<'a> {
    pub fn new() -> Self {
        UpdateSetupIntent {
            customer: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            payment_method: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateData {
    pub customer_acceptance: CreateSetupIntentMandateDataCustomerAcceptance,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSetupIntentPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodOptionsSepaDebit>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentSingleUse {
    pub amount: i64,

    pub currency: Currency,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSetupIntentPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebit>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreateSetupIntentMandateDataCustomerAcceptanceOffline>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<CreateSetupIntentMandateDataCustomerAcceptanceOnline>,

    #[serde(rename = "type")]
    pub type_: CreateSetupIntentMandateDataCustomerAcceptanceType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptanceOffline {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptanceOnline {
    pub ip_address: String,

    pub user_agent: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {}

/// An enum representing the possible values of an `CreateSetupIntentMandateDataCustomerAcceptance`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentMandateDataCustomerAcceptanceType {
    Offline,
    Online,
}

impl CreateSetupIntentMandateDataCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentMandateDataCustomerAcceptanceType::Offline => "offline",
            CreateSetupIntentMandateDataCustomerAcceptanceType::Online => "online",
        }
    }
}

impl AsRef<str> for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SetupIntent`'s `cancellation_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
}

impl SetupIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentCancellationReason::Abandoned => "abandoned",
            SetupIntentCancellationReason::Duplicate => "duplicate",
            SetupIntentCancellationReason::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for SetupIntentCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    ChallengeOnly,
}

impl SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::ChallengeOnly => {
                "challenge_only"
            }
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SetupIntent`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}

impl SetupIntentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentStatus::Canceled => "canceled",
            SetupIntentStatus::Processing => "processing",
            SetupIntentStatus::RequiresAction => "requires_action",
            SetupIntentStatus::RequiresConfirmation => "requires_confirmation",
            SetupIntentStatus::RequiresPaymentMethod => "requires_payment_method",
            SetupIntentStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for SetupIntentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
