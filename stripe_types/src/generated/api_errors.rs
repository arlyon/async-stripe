#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ApiErrors {
    /// For card errors, the ID of the failed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,
    /// For some errors that could be handled programmatically, a short string indicating the [error code](https://stripe.com/docs/error-codes) reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// For card errors resulting from a card issuer decline, a short string indicating the [card issuer's reason for the decline](https://stripe.com/docs/declines#issuer-declines) if they provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<String>,
    /// A URL to more information about the [error code](https://stripe.com/docs/error-codes) reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,
    /// A human-readable message providing more details about the error.
    ///
    /// For card errors, these messages can be shown to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// If the error is parameter-specific, the parameter related to the error.
    ///
    /// For example, you can use this to display a message near the correct form field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<stripe_types::payment_intent::PaymentIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<stripe_types::payment_method::PaymentMethod>,
    /// If the error is specific to the type of payment method, the payment method type that had a problem.
    ///
    /// This field is only populated for invoice-related errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    /// A URL to the request log entry in your dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_log_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<stripe_types::setup_intent::SetupIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<stripe_types::payment_source::PaymentSource>,
    /// The type of error returned.
    ///
    /// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`.
    #[serde(rename = "type")]
    pub type_: ApiErrorsType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApiErrors {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of error returned.
///
/// One of `api_error`, `card_error`, `idempotency_error`, or `invalid_request_error`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ApiErrorsType {
    ApiError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
}

impl ApiErrorsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApiError => "api_error",
            Self::CardError => "card_error",
            Self::IdempotencyError => "idempotency_error",
            Self::InvalidRequestError => "invalid_request_error",
        }
    }
}

impl std::str::FromStr for ApiErrorsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "api_error" => Ok(Self::ApiError),
            "card_error" => Ok(Self::CardError),
            "idempotency_error" => Ok(Self::IdempotencyError),
            "invalid_request_error" => Ok(Self::InvalidRequestError),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ApiErrorsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApiErrorsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ApiErrorsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ApiErrorsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ApiErrorsType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApiErrorsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ApiErrorsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApiErrorsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
