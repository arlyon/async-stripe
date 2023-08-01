/// A SetupIntent guides you through the process of setting up and saving a customer's payment credentials for future payments.
/// For example, you could use a SetupIntent to set up and save your customer's card without immediately collecting a payment.
/// Later, you can use [PaymentIntents](https://stripe.com/docs/api#payment_intents) to drive the payment flow.
///
/// Create a SetupIntent as soon as you're ready to collect your customer's payment credentials.
/// Do not maintain long-lived, unconfirmed SetupIntents as they may no longer be valid.
/// The SetupIntent then transitions through multiple [statuses](https://stripe.com/docs/payments/intents#intent-statuses) as it guides
/// you through the setup process.
///
/// Successful SetupIntents result in payment credentials that are optimized for future payments.
/// For example, cardholders in [certain regions](/guides/strong-customer-authentication) may need to be run through
/// [Strong Customer Authentication](https://stripe.com/docs/strong-customer-authentication) at the time of payment method collection
/// in order to streamline later [off-session payments](https://stripe.com/docs/payments/setup-intents).
/// If the SetupIntent is used with a [Customer](https://stripe.com/docs/api#setup_intent_object-customer), upon success,
/// it will automatically attach the resulting payment method to that Customer.
/// We recommend using SetupIntents or [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage) on
/// PaymentIntents to save payment methods in order to prevent saving invalid or unoptimized payment methods.
///
/// By using SetupIntents, you ensure that your customers experience the minimum set of required friction,
/// even as regulations change over time.
///
/// Related guide: [Setup Intents API](https://stripe.com/docs/payments/setup-intents).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetupIntent {
    /// ID of the Connect application that created the SetupIntent.
    pub application: Option<stripe_types::Expandable<stripe_types::application::Application>>,
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    /// Settings for automatic payment methods compatible with this Setup Intent.
    pub automatic_payment_methods: Option<
        stripe_types::automatic_payment_methods_setup_intent::AutomaticPaymentMethodsSetupIntent,
    >,
    /// Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
    pub cancellation_reason: Option<SetupIntentCancellationReason>,
    /// The client secret of this SetupIntent.
    ///
    /// Used for client-side retrieval using a publishable key.  The client secret can be used to complete payment setup from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub flow_directions: Option<Vec<SetupIntentFlowDirections>>,
    /// Unique identifier for the object.
    pub id: stripe_types::setup_intent::SetupIntentId,
    /// The error encountered in the previous SetupIntent confirmation.
    pub last_setup_error: Option<Box<stripe_types::api_errors::ApiErrors>>,
    /// The most recent SetupAttempt for this SetupIntent.
    pub latest_attempt: Option<stripe_types::Expandable<stripe_types::setup_attempt::SetupAttempt>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the multi use Mandate generated by the SetupIntent.
    pub mandate: Option<stripe_types::Expandable<stripe_types::mandate::Mandate>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If present, this property tells you what actions you need to take in order for your customer to continue payment setup.
    pub next_action: Option<stripe_types::setup_intent::next_action::NextAction>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SetupIntentObject,
    /// The account (if any) for which the setup is intended.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::account::Account>>,
    /// ID of the payment method used with this SetupIntent.
    pub payment_method:
        Option<stripe_types::Expandable<stripe_types::payment_method::PaymentMethod>>,
    /// Payment-method-specific configuration for this SetupIntent.
    pub payment_method_options:
        Option<stripe_types::setup_intent::payment_method_options::PaymentMethodOptions>,
    /// The list of payment method types (e.g.
    ///
    /// card) that this SetupIntent is allowed to set up.
    pub payment_method_types: Vec<String>,
    /// ID of the single_use Mandate generated by the SetupIntent.
    pub single_use_mandate: Option<stripe_types::Expandable<stripe_types::mandate::Mandate>>,
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
/// Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
}

impl SetupIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        use SetupIntentCancellationReason::*;
        match self {
            Abandoned => "abandoned",
            Duplicate => "duplicate",
            RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for SetupIntentCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentCancellationReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "duplicate" => Ok(Duplicate),
            "requested_by_customer" => Ok(RequestedByCustomer),
            _ => Err(()),
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
impl serde::Serialize for SetupIntentCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SetupIntentCancellationReason")
        })
    }
}
/// Indicates the directions of money movement for which this payment method is intended to be used.
///
/// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
///
/// Include `outbound` if you intend to use the payment method as the destination to send funds to.
/// You can include both if you intend to use the payment method for both purposes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetupIntentFlowDirections {
    Inbound,
    Outbound,
}

impl SetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        use SetupIntentFlowDirections::*;
        match self {
            Inbound => "inbound",
            Outbound => "outbound",
        }
    }
}

impl std::str::FromStr for SetupIntentFlowDirections {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentFlowDirections::*;
        match s {
            "inbound" => Ok(Inbound),
            "outbound" => Ok(Outbound),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SetupIntentFlowDirections {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentFlowDirections {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentFlowDirections"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetupIntentObject {
    SetupIntent,
}

impl SetupIntentObject {
    pub fn as_str(self) -> &'static str {
        use SetupIntentObject::*;
        match self {
            SetupIntent => "setup_intent",
        }
    }
}

impl std::str::FromStr for SetupIntentObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentObject::*;
        match s {
            "setup_intent" => Ok(SetupIntent),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SetupIntentObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentObject"))
    }
}
/// [Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        use SetupIntentStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresAction => "requires_action",
            RequiresConfirmation => "requires_confirmation",
            RequiresPaymentMethod => "requires_payment_method",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for SetupIntentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_action" => Ok(RequiresAction),
            "requires_confirmation" => Ok(RequiresConfirmation),
            "requires_payment_method" => Ok(RequiresPaymentMethod),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
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
impl serde::Serialize for SetupIntentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentStatus"))
    }
}
impl stripe_types::Object for SetupIntent {
    type Id = stripe_types::setup_intent::SetupIntentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SetupIntentId, "seti_");
pub mod next_action;
pub use next_action::NextAction;
pub mod next_action_redirect_to_url;
pub use next_action_redirect_to_url::NextActionRedirectToUrl;
pub mod payment_method_options;
pub use payment_method_options::PaymentMethodOptions;
