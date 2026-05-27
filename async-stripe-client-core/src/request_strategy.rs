use std::time::Duration;

// Fall back to Stripe's documented transient status codes when
// `Stripe-Should-Retry` is absent.
// See: https://docs.stripe.com/error-low-level#status-codes
fn retryable_status(status: u16) -> bool {
    matches!(status, 409 | 424 | 429 | 500..=504)
}

/// Possible strategies for sending Stripe API requests, including retry behavior
/// and use of idempotency keys.
#[derive(Clone, Debug)]
pub enum RequestStrategy {
    /// Run the request once.
    Once,
    /// Run it once with a given idempotency key.
    Idempotent(IdempotencyKey),
    /// This strategy will try the request up to the specified
    /// number of total attempts using the same, random,
    /// idempotency key.
    Retry(u32),
    /// This strategy will try the request up to the specified
    /// number of total attempts using the same, random,
    /// idempotency key with exponential backoff.
    ExponentialBackoff(u32),
}

impl RequestStrategy {
    /// Decide if we should retry this request, or stop.
    pub fn test(
        &self,
        // Status would be much better as a newtype, but we want to be library
        // agnostic. We could reimplement our own `StripeStatusCode`, but that
        // feels a bit like reinventing the wheel...
        status: Option<u16>,
        stripe_should_retry: Option<bool>,
        retry_count: u32,
    ) -> Outcome {
        use RequestStrategy::*;

        match stripe_should_retry {
            // If Stripe explicitly says not to retry, never retry
            Some(false) => return Outcome::Stop,
            // If Stripe explicitly says to retry, continue with retry logic
            Some(true) => {}
            // If header is absent, fall back to retryable status codes below.
            None => {}
        }

        if let Some(s) = status
            && !retryable_status(s)
        {
            return Outcome::Stop;
        }

        match (self, retry_count) {
            // a strategy of once or idempotent should run once
            (Once | Idempotent(_), 0) => Outcome::Continue(None),

            // a strategy of retry or exponential backoff should retry with
            // the appropriate delay if the number of retries is less than the max
            (Retry(n), x) if x < *n => Outcome::Continue(None),
            (ExponentialBackoff(n), x) if x < *n => Outcome::Continue(Some(calculate_backoff(x))),

            // unknown cases should be stopped to prevent infinite loops
            _ => Outcome::Stop,
        }
    }

    /// Send the request once with a generated UUID.
    #[cfg(feature = "uuid")]
    pub fn idempotent_with_uuid() -> Self {
        Self::Idempotent(IdempotencyKey::new_uuid_v4())
    }

    /// Extract the current idempotency key to use for the next request, if any.
    pub fn get_key(&self) -> Option<IdempotencyKey> {
        match self {
            RequestStrategy::Once => None,
            RequestStrategy::Idempotent(key) => Some(key.clone()),
            #[cfg(feature = "uuid")]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => {
                Some(IdempotencyKey::new_uuid_v4())
            }
            #[cfg(not(feature = "uuid"))]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
/// Represents valid idempotency key
/// - Cannot be empty
/// - Cannot be longer than 255 charachters
pub struct IdempotencyKey(String);

#[derive(Debug, thiserror::Error)]
/// Error that can be returned when constructing [`IdempotencyKey`]
pub enum IdempotentKeyError {
    #[error("Idempotency Key cannot be empty")]
    /// Idempotency key cannot be empty
    EmptyKey,
    #[error("Idempotency key cannot be longer than 255 characters (you supplied: {0})")]
    /// Idempotency key cannot be longer than 255 characters
    KeyTooLong(usize),
}

impl IdempotencyKey {
    /// Creates new validated idempotency key.
    ///
    /// # Errors
    /// This function returns error when they key is empty or when
    /// its longer than 255 characters
    pub fn new(val: impl AsRef<str>) -> Result<Self, IdempotentKeyError> {
        let val = val.as_ref();
        if val.is_empty() {
            Err(IdempotentKeyError::EmptyKey)
        } else if val.len() > 255 {
            Err(IdempotentKeyError::KeyTooLong(val.len()))
        } else {
            Ok(Self(val.to_owned()))
        }
    }

    #[cfg(feature = "uuid")]
    /// Generates new UUID as new idempotency key
    pub fn new_uuid_v4() -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();
        Self(uuid)
    }

    /// Borrows self as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes self and returns inner string
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for IdempotencyKey {
    type Error = IdempotentKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for IdempotencyKey {
    #[inline]
    fn from(value: uuid::Uuid) -> Self {
        Self(value.to_string())
    }
}

fn calculate_backoff(retry_count: u32) -> Duration {
    Duration::from_secs(2_u64.pow(retry_count))
}

/// Representation of whether to retry the API request, including a potential waiting period.
#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    /// Do not retry the requests, return the last error.
    Stop,
    /// Send another request, either immediately or after sleeping for the given `Duration`
    /// if provided.
    Continue(Option<Duration>),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{Outcome, RequestStrategy};
    use crate::IdempotencyKey;

    #[test]
    fn test_idempotent_strategy() {
        let key: IdempotencyKey = "key".to_string().try_into().unwrap();
        let strategy = RequestStrategy::Idempotent(key.clone());
        assert_eq!(strategy.get_key(), Some(key));
    }

    #[test]
    fn test_once_strategy() {
        let strategy = RequestStrategy::Once;
        assert_eq!(strategy.get_key(), None);
        assert_eq!(strategy.test(None, None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 1), Outcome::Stop);
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_uuid_idempotency() {
        use uuid::Uuid;
        let strategy = RequestStrategy::Retry(3);
        assert!(Uuid::parse_str(strategy.get_key().unwrap().as_str()).is_ok());
    }

    #[test]
    #[cfg(not(feature = "uuid"))]
    fn test_uuid_idempotency() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.get_key(), None);
    }

    #[test]
    fn test_retry_strategy() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.test(None, None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 1), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 2), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 3), Outcome::Stop);
        assert_eq!(strategy.test(None, None, 4), Outcome::Stop);
    }

    #[test]
    fn test_backoff_strategy() {
        let strategy = RequestStrategy::ExponentialBackoff(3);
        assert_eq!(strategy.test(None, None, 0), Outcome::Continue(Some(Duration::from_secs(1))));
        assert_eq!(strategy.test(None, None, 1), Outcome::Continue(Some(Duration::from_secs(2))));
        assert_eq!(strategy.test(None, None, 2), Outcome::Continue(Some(Duration::from_secs(4))));
        assert_eq!(strategy.test(None, None, 3), Outcome::Stop);
        assert_eq!(strategy.test(None, None, 4), Outcome::Stop);
    }

    #[test]
    fn test_retry_header() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.test(None, Some(false), 0), Outcome::Stop);
    }

    #[test]
    fn test_stripe_should_retry_true() {
        let strategy = RequestStrategy::Retry(3);
        // When Stripe-Should-Retry is true, should retry retryable status codes
        assert_eq!(strategy.test(Some(500), Some(true), 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(503), Some(true), 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(409), Some(true), 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(424), Some(true), 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(429), Some(true), 0), Outcome::Continue(None));
        // Except for non-retryable client errors (4xx)
        assert_eq!(strategy.test(Some(400), Some(true), 0), Outcome::Stop);
        assert_eq!(strategy.test(Some(404), Some(true), 0), Outcome::Stop);
    }

    #[test]
    fn test_stripe_should_retry_false() {
        let strategy = RequestStrategy::Retry(3);
        // When Stripe-Should-Retry is false, never retry
        assert_eq!(strategy.test(Some(429), Some(false), 0), Outcome::Stop);
        assert_eq!(strategy.test(Some(500), Some(false), 0), Outcome::Stop);
        assert_eq!(strategy.test(Some(200), Some(false), 0), Outcome::Stop);
    }

    #[test]
    fn test_stripe_should_retry_absent_429() {
        let strategy = RequestStrategy::Retry(3);
        // When header is absent and status is 429, should retry
        assert_eq!(strategy.test(Some(429), None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(429), None, 1), Outcome::Continue(None));
    }

    #[test]
    fn test_stripe_should_retry_absent_500() {
        let strategy = RequestStrategy::Retry(3);
        // When header is absent and status is 5xx, should retry
        assert_eq!(strategy.test(Some(500), None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(502), None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(503), None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(504), None, 0), Outcome::Continue(None));
    }

    #[test]
    fn test_stripe_should_retry_absent_4xx() {
        let strategy = RequestStrategy::Retry(3);
        // When header is absent and status is 4xx, should NOT retry
        assert_eq!(strategy.test(Some(400), None, 0), Outcome::Stop);
        assert_eq!(strategy.test(Some(404), None, 0), Outcome::Stop);
    }

    #[test]
    fn test_stripe_should_retry_absent_409() {
        let strategy = RequestStrategy::Retry(3);
        // When header is absent and status is 409, should retry
        assert_eq!(strategy.test(Some(409), None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(409), None, 1), Outcome::Continue(None));
    }

    #[test]
    fn test_stripe_should_retry_absent_424() {
        let strategy = RequestStrategy::Retry(3);
        // When header is absent and status is 424, should retry
        assert_eq!(strategy.test(Some(424), None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(Some(424), None, 1), Outcome::Continue(None));
    }

    #[test]
    fn test_backoff_with_stripe_should_retry() {
        let strategy = RequestStrategy::ExponentialBackoff(3);
        // Test that exponential backoff works with Stripe-Should-Retry=true
        assert_eq!(
            strategy.test(Some(500), Some(true), 0),
            Outcome::Continue(Some(Duration::from_secs(1)))
        );
        assert_eq!(
            strategy.test(Some(500), Some(true), 1),
            Outcome::Continue(Some(Duration::from_secs(2)))
        );
        assert_eq!(
            strategy.test(Some(500), Some(true), 2),
            Outcome::Continue(Some(Duration::from_secs(4)))
        );
        assert_eq!(strategy.test(Some(500), Some(true), 3), Outcome::Stop);
    }

    #[test]
    fn test_backoff_with_429_no_header() {
        let strategy = RequestStrategy::ExponentialBackoff(3);
        // Test that exponential backoff works with 429 when header is absent
        assert_eq!(
            strategy.test(Some(429), None, 0),
            Outcome::Continue(Some(Duration::from_secs(1)))
        );
        assert_eq!(
            strategy.test(Some(429), None, 1),
            Outcome::Continue(Some(Duration::from_secs(2)))
        );
    }
}
