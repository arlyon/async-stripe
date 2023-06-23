#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RequestHistory {
    /// The `pending_request.amount` at the time of the request, presented in your card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// Stripe held this amount from your account to fund the authorization if the request was approved.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_core::issuing::authorization::amount_details::AmountDetails>,
    /// Whether this request was approved.
    pub approved: bool,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `pending_request.merchant_amount` at the time of the request, presented in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    /// The currency that was collected by the merchant and presented to the cardholder for the authorization.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: stripe_types::Currency,
    /// The reason for the approval or decline.
    pub reason: RequestHistoryReason,
    /// If approve/decline decision is directly responsed to the webhook with json payload and if the response is invalid (e.g., parsing errors), we surface the detailed message via this field.
    pub reason_message: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RequestHistory {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The reason for the approval or decline.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum RequestHistoryReason {
    AccountDisabled,
    CardActive,
    CardInactive,
    CardholderInactive,
    CardholderVerificationRequired,
    InsufficientFunds,
    NotAllowed,
    SpendingControls,
    SuspectedFraud,
    VerificationFailed,
    WebhookApproved,
    WebhookDeclined,
    WebhookError,
    WebhookTimeout,
}

impl RequestHistoryReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountDisabled => "account_disabled",
            Self::CardActive => "card_active",
            Self::CardInactive => "card_inactive",
            Self::CardholderInactive => "cardholder_inactive",
            Self::CardholderVerificationRequired => "cardholder_verification_required",
            Self::InsufficientFunds => "insufficient_funds",
            Self::NotAllowed => "not_allowed",
            Self::SpendingControls => "spending_controls",
            Self::SuspectedFraud => "suspected_fraud",
            Self::VerificationFailed => "verification_failed",
            Self::WebhookApproved => "webhook_approved",
            Self::WebhookDeclined => "webhook_declined",
            Self::WebhookError => "webhook_error",
            Self::WebhookTimeout => "webhook_timeout",
        }
    }
}

impl AsRef<str> for RequestHistoryReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RequestHistoryReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
