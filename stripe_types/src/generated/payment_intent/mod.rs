/// A PaymentIntent guides you through the process of collecting a payment from your customer.
/// We recommend that you create exactly one PaymentIntent for each order or
/// customer session in your system.
///
/// You can reference the PaymentIntent later to see the history of payment attempts for a particular session.  A PaymentIntent transitions through [multiple statuses](https://stripe.com/docs/payments/intents#intent-statuses) throughout its lifetime as it interfaces with Stripe.js to perform authentication flows and ultimately creates at most one successful charge.  Related guide: [Payment Intents API](https://stripe.com/docs/payments/payment-intents).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntent {
    /// Amount intended to be collected by this PaymentIntent.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Amount that can be captured from this PaymentIntent.
    pub amount_capturable: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<stripe_types::PaymentFlowsAmountDetails>,
    /// Amount that was collected by this PaymentIntent.
    pub amount_received: i64,
    /// ID of the Connect application that created the PaymentIntent.
    pub application: Option<stripe_types::Expandable<stripe_types::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    pub application_fee_amount: Option<i64>,
    /// Settings to configure compatible payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub automatic_payment_methods:
        Option<stripe_types::PaymentFlowsAutomaticPaymentMethodsPaymentIntent>,
    /// Populated when `status` is `canceled`, this is the time at which the PaymentIntent was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Reason for cancellation of this PaymentIntent, either user-provided (`duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`) or generated by Stripe internally (`failed_invoice`, `void_invoice`, or `automatic`).
    pub cancellation_reason: Option<PaymentIntentCancellationReason>,
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: PaymentIntentCaptureMethod,
    /// The client secret of this PaymentIntent.
    ///
    /// Used for client-side retrieval using a publishable key.
    /// The client secret can be used to complete a payment from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.  Refer to our docs to [accept a payment](https://stripe.com/docs/payments/accept-a-payment?ui=elements) and learn about how `client_secret` should be handled.
    pub client_secret: Option<String>,
    pub confirmation_method: PaymentIntentConfirmationMethod,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the Customer this PaymentIntent belongs to, if one exists.
    ///
    /// Payment methods attached to other Customers cannot be used with this PaymentIntent.
    ///
    /// If present in combination with [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage), this PaymentIntent's payment method will be attached to the Customer after the PaymentIntent has been confirmed and any required actions from the user are complete.
    pub customer: Option<stripe_types::Expandable<stripe_types::Customer>>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::payment_intent::PaymentIntentId,
    /// ID of the invoice that created this PaymentIntent, if it exists.
    pub invoice: Option<stripe_types::Expandable<stripe_types::Invoice>>,
    /// The payment error encountered in the previous PaymentIntent confirmation.
    ///
    /// It will be cleared if the PaymentIntent is later updated for any reason.
    pub last_payment_error: Option<Box<stripe_types::ApiErrors>>,
    /// The latest charge created by this payment intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_charge: Option<stripe_types::Expandable<stripe_types::Charge>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// For more information, see the [documentation](https://stripe.com/docs/payments/payment-intents/creating-payment-intents#storing-information-in-metadata).
    pub metadata: std::collections::HashMap<String, String>,
    /// If present, this property tells you what actions you need to take in order for your customer to fulfill a payment using the provided source.
    pub next_action: Option<stripe_types::PaymentIntentNextAction>,
    /// The account (if any) for which the funds of the PaymentIntent are intended.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::Account>>,
    /// ID of the payment method used in this PaymentIntent.
    pub payment_method: Option<stripe_types::Expandable<stripe_types::PaymentMethod>>,
    /// Payment-method-specific configuration for this PaymentIntent.
    pub payment_method_options: Option<stripe_types::PaymentIntentPaymentMethodOptions>,
    /// The list of payment method types (e.g.
    ///
    /// card) that this PaymentIntent is allowed to use.
    pub payment_method_types: Vec<String>,
    /// If present, this property tells you about the processing state of the payment.
    pub processing: Option<stripe_types::PaymentIntentProcessing>,
    /// Email address that the receipt for the resulting payment will be sent to.
    ///
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    pub receipt_email: Option<String>,
    /// ID of the review associated with this PaymentIntent, if any.
    pub review: Option<stripe_types::Expandable<stripe_types::RadarReview>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentIntentSetupFutureUsage>,
    /// Shipping information for this PaymentIntent.
    pub shipping: Option<stripe_types::Shipping>,
    /// This is a legacy field that will be removed in the future.
    ///
    /// It is the ID of the Source object that is associated with this PaymentIntent, if one was supplied.
    pub source: Option<stripe_types::Expandable<stripe_types::PaymentSource>>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers’ statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// Status of this PaymentIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `requires_capture`, `canceled`, or `succeeded`.
    ///
    /// Read more about each PaymentIntent [status](https://stripe.com/docs/payments/intents#intent-statuses).
    pub status: PaymentIntentStatus,
    /// The data with which to automatically create a Transfer when the payment is finalized.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    pub transfer_data: Option<stripe_types::TransferData>,
    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts) for details.
    pub transfer_group: Option<String>,
}
/// Reason for cancellation of this PaymentIntent, either user-provided (`duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`) or generated by Stripe internally (`failed_invoice`, `void_invoice`, or `automatic`).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentCancellationReason {
    Abandoned,
    Automatic,
    Duplicate,
    FailedInvoice,
    Fraudulent,
    RequestedByCustomer,
    VoidInvoice,
}

impl PaymentIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentCancellationReason::*;
        match self {
            Abandoned => "abandoned",
            Automatic => "automatic",
            Duplicate => "duplicate",
            FailedInvoice => "failed_invoice",
            Fraudulent => "fraudulent",
            RequestedByCustomer => "requested_by_customer",
            VoidInvoice => "void_invoice",
        }
    }
}

impl std::str::FromStr for PaymentIntentCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentCancellationReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "automatic" => Ok(Automatic),
            "duplicate" => Ok(Duplicate),
            "failed_invoice" => Ok(FailedInvoice),
            "fraudulent" => Ok(Fraudulent),
            "requested_by_customer" => Ok(RequestedByCustomer),
            "void_invoice" => Ok(VoidInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentIntentCancellationReason")
        })
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

impl PaymentIntentCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentIntentCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentCaptureMethod"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentConfirmationMethod {
    Automatic,
    Manual,
}

impl PaymentIntentConfirmationMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentConfirmationMethod::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentIntentConfirmationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentConfirmationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentConfirmationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentConfirmationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentConfirmationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentConfirmationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentConfirmationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentIntentConfirmationMethod")
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentSetupFutureUsage {
    OffSession,
    OnSession,
}

impl PaymentIntentSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentIntentSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentIntentSetupFutureUsage")
        })
    }
}
/// Status of this PaymentIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `requires_capture`, `canceled`, or `succeeded`.
///
/// Read more about each PaymentIntent [status](https://stripe.com/docs/payments/intents#intent-statuses).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}

impl PaymentIntentStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresAction => "requires_action",
            RequiresCapture => "requires_capture",
            RequiresConfirmation => "requires_confirmation",
            RequiresPaymentMethod => "requires_payment_method",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for PaymentIntentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_action" => Ok(RequiresAction),
            "requires_capture" => Ok(RequiresCapture),
            "requires_confirmation" => Ok(RequiresConfirmation),
            "requires_payment_method" => Ok(RequiresPaymentMethod),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentStatus"))
    }
}
impl stripe_types::Object for PaymentIntent {
    type Id = stripe_types::payment_intent::PaymentIntentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PaymentIntentId, "pi_");
