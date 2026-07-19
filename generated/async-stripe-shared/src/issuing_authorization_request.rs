#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationRequest {
    /// The `pending_request.amount` at the time of the request, presented in your card's currency and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// Stripe held this amount from your account to fund the authorization if the request was approved.
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingAuthorizationAmountDetails>,
    /// Whether this request was approved.
    pub approved: bool,
    /// A code created by Stripe which is shared with the merchant to validate the authorization.
    /// This field will be populated if the authorization message was approved.
    /// The code typically starts with the letter "S", followed by a six-digit number.
    /// For example, "S498162".
    /// Please note that the code is not guaranteed to be unique across authorizations.
    pub authorization_code: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `pending_request.merchant_amount` at the time of the request, presented in the `merchant_currency` and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub merchant_amount: i64,
    /// The currency that was collected by the merchant and presented to the cardholder for the authorization.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: stripe_types::Currency,
    /// The card network's estimate of the likelihood that an authorization is fraudulent.
    /// Takes on values between 1 and 99.
    pub network_risk_score: Option<i64>,
    /// When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
    pub reason: IssuingAuthorizationRequestReason,
    /// If the `request_history.reason` is `webhook_error` because the direct webhook response is invalid (for example, parsing errors or missing parameters), we surface a more detailed error message via this field.
    pub reason_message: Option<String>,
    /// Time when the card network received an authorization request from the acquirer in UTC.
    /// Referred to by networks as transmission time.
    pub requested_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationRequest").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationRequestBuilder {
    amount: Option<i64>,
    amount_details: Option<Option<stripe_shared::IssuingAuthorizationAmountDetails>>,
    approved: Option<bool>,
    authorization_code: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    merchant_amount: Option<i64>,
    merchant_currency: Option<stripe_types::Currency>,
    network_risk_score: Option<Option<i64>>,
    reason: Option<IssuingAuthorizationRequestReason>,
    reason_message: Option<Option<String>>,
    requested_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationRequest {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationRequest>,
        builder: IssuingAuthorizationRequestBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationRequest> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationRequestBuilder {
                    amount: Deserialize::default(),
                    amount_details: Deserialize::default(),
                    approved: Deserialize::default(),
                    authorization_code: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    merchant_amount: Deserialize::default(),
                    merchant_currency: Deserialize::default(),
                    network_risk_score: Deserialize::default(),
                    reason: Deserialize::default(),
                    reason_message: Deserialize::default(),
                    requested_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_details" => Deserialize::begin(&mut self.builder.amount_details),
                "approved" => Deserialize::begin(&mut self.builder.approved),
                "authorization_code" => Deserialize::begin(&mut self.builder.authorization_code),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "merchant_amount" => Deserialize::begin(&mut self.builder.merchant_amount),
                "merchant_currency" => Deserialize::begin(&mut self.builder.merchant_currency),
                "network_risk_score" => Deserialize::begin(&mut self.builder.network_risk_score),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                "reason_message" => Deserialize::begin(&mut self.builder.reason_message),
                "requested_at" => Deserialize::begin(&mut self.builder.requested_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(amount_details),
                Some(approved),
                Some(authorization_code),
                Some(created),
                Some(currency),
                Some(merchant_amount),
                Some(merchant_currency),
                Some(network_risk_score),
                Some(reason),
                Some(reason_message),
                Some(requested_at),
            ) = (
                self.builder.amount,
                self.builder.amount_details,
                self.builder.approved,
                self.builder.authorization_code.take(),
                self.builder.created,
                self.builder.currency.take(),
                self.builder.merchant_amount,
                self.builder.merchant_currency.take(),
                self.builder.network_risk_score,
                self.builder.reason.take(),
                self.builder.reason_message.take(),
                self.builder.requested_at,
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationRequest {
                amount,
                amount_details,
                approved,
                authorization_code,
                created,
                currency,
                merchant_amount,
                merchant_currency,
                network_risk_score,
                reason,
                reason_message,
                requested_at,
            });
            Ok(())
        }
    }
};
/// When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationRequestReason {
    AccountDisabled,
    CardActive,
    CardCanceled,
    CardExpired,
    CardInactive,
    CardholderBlocked,
    CardholderInactive,
    CardholderVerificationRequired,
    InsecureAuthorizationMethod,
    InsufficientFunds,
    NetworkFallback,
    NotAllowed,
    PinBlocked,
    SpendingControls,
    SuspectedFraud,
    VerificationFailed,
    WebhookApproved,
    WebhookDeclined,
    WebhookError,
    WebhookTimeout,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationRequestReason {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationRequestReason::*;
        match self {
            AccountDisabled => "account_disabled",
            CardActive => "card_active",
            CardCanceled => "card_canceled",
            CardExpired => "card_expired",
            CardInactive => "card_inactive",
            CardholderBlocked => "cardholder_blocked",
            CardholderInactive => "cardholder_inactive",
            CardholderVerificationRequired => "cardholder_verification_required",
            InsecureAuthorizationMethod => "insecure_authorization_method",
            InsufficientFunds => "insufficient_funds",
            NetworkFallback => "network_fallback",
            NotAllowed => "not_allowed",
            PinBlocked => "pin_blocked",
            SpendingControls => "spending_controls",
            SuspectedFraud => "suspected_fraud",
            VerificationFailed => "verification_failed",
            WebhookApproved => "webhook_approved",
            WebhookDeclined => "webhook_declined",
            WebhookError => "webhook_error",
            WebhookTimeout => "webhook_timeout",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationRequestReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationRequestReason::*;
        match s {
            "account_disabled" => Ok(AccountDisabled),
            "card_active" => Ok(CardActive),
            "card_canceled" => Ok(CardCanceled),
            "card_expired" => Ok(CardExpired),
            "card_inactive" => Ok(CardInactive),
            "cardholder_blocked" => Ok(CardholderBlocked),
            "cardholder_inactive" => Ok(CardholderInactive),
            "cardholder_verification_required" => Ok(CardholderVerificationRequired),
            "insecure_authorization_method" => Ok(InsecureAuthorizationMethod),
            "insufficient_funds" => Ok(InsufficientFunds),
            "network_fallback" => Ok(NetworkFallback),
            "not_allowed" => Ok(NotAllowed),
            "pin_blocked" => Ok(PinBlocked),
            "spending_controls" => Ok(SpendingControls),
            "suspected_fraud" => Ok(SuspectedFraud),
            "verification_failed" => Ok(VerificationFailed),
            "webhook_approved" => Ok(WebhookApproved),
            "webhook_declined" => Ok(WebhookDeclined),
            "webhook_error" => Ok(WebhookError),
            "webhook_timeout" => Ok(WebhookTimeout),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationRequestReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationRequestReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationRequestReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationRequestReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingAuthorizationRequestReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingAuthorizationRequestReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationRequestReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationRequestReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
