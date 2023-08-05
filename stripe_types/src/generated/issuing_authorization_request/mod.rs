#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingAuthorizationRequest {
    /// The `pending_request.amount` at the time of the request, presented in your card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// Stripe held this amount from your account to fund the authorization if the request was approved.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_types::IssuingAuthorizationAmountDetails>,
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
    /// When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
    pub reason: IssuingAuthorizationRequestReason,
    /// If approve/decline decision is directly responsed to the webhook with json payload and if the response is invalid (e.g., parsing errors), we surface the detailed message via this field.
    pub reason_message: Option<String>,
}
/// When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationRequestReason {
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

impl IssuingAuthorizationRequestReason {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationRequestReason::*;
        match self {
            AccountDisabled => "account_disabled",
            CardActive => "card_active",
            CardInactive => "card_inactive",
            CardholderInactive => "cardholder_inactive",
            CardholderVerificationRequired => "cardholder_verification_required",
            InsufficientFunds => "insufficient_funds",
            NotAllowed => "not_allowed",
            SpendingControls => "spending_controls",
            SuspectedFraud => "suspected_fraud",
            VerificationFailed => "verification_failed",
            WebhookApproved => "webhook_approved",
            WebhookDeclined => "webhook_declined",
            WebhookError => "webhook_error",
            WebhookTimeout => "webhook_timeout",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationRequestReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationRequestReason::*;
        match s {
            "account_disabled" => Ok(AccountDisabled),
            "card_active" => Ok(CardActive),
            "card_inactive" => Ok(CardInactive),
            "cardholder_inactive" => Ok(CardholderInactive),
            "cardholder_verification_required" => Ok(CardholderVerificationRequired),
            "insufficient_funds" => Ok(InsufficientFunds),
            "not_allowed" => Ok(NotAllowed),
            "spending_controls" => Ok(SpendingControls),
            "suspected_fraud" => Ok(SuspectedFraud),
            "verification_failed" => Ok(VerificationFailed),
            "webhook_approved" => Ok(WebhookApproved),
            "webhook_declined" => Ok(WebhookDeclined),
            "webhook_error" => Ok(WebhookError),
            "webhook_timeout" => Ok(WebhookTimeout),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingAuthorizationRequestReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingAuthorizationRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationRequestReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationRequestReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationRequestReason")
        })
    }
}
