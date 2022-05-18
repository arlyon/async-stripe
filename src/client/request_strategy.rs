use std::time::Duration;

use http_types::StatusCode;

#[derive(Clone, Debug)]
pub enum RequestStrategy {
    Once,
    /// Run it once with a given idempotency key.
    Idempotent(String),
    /// This strategy will retry the request up to the
    /// specified number of times using the same, random,
    /// idempotency key, up to n times.
    Retry(u32),
    /// This strategy will retry the request up to the
    /// specified number of times using the same, random,
    /// idempotency key with exponential backoff, up to n times.
    ExponentialBackoff(u32),
}

impl RequestStrategy {
    pub fn test(
        &self,
        status: Option<StatusCode>,
        stripe_should_retry: Option<bool>,
        retry_count: u32,
    ) -> Outcome {
        // if stripe explicitly says not to retry then don't
        if !stripe_should_retry.unwrap_or(true) {
            return Outcome::Stop;
        }

        use RequestStrategy::*;

        match (self, status, retry_count) {
            // a strategy of once or idempotent should run once
            (Once | Idempotent(_), _, 0) => Outcome::Continue(None),

            // requests with idempotency keys that hit client
            // errors usually cannot be solved with retries
            // see: https://stripe.com/docs/error-handling#content-errors
            (_, Some(c), _) if c.is_client_error() => Outcome::Stop,

            // a strategy of retry or exponential backoff should retry with
            // the appropriate delay if the number of retries is less than the max
            (Retry(n), _, x) if x < *n => Outcome::Continue(None),
            (ExponentialBackoff(n), _, x) if x < *n => {
                Outcome::Continue(Some(calculate_backoff(x)))
            }

            // unknown cases should be stopped to prevent infinite loops
            _ => Outcome::Stop,
        }
    }

    #[cfg(feature = "uuid")]
    pub fn idempotent_with_uuid() -> Self {
        use uuid::Uuid;
        Self::Idempotent(Uuid::new_v4().to_string())
    }

    pub fn get_key(&self) -> Option<String> {
        match self {
            RequestStrategy::Once => None,
            RequestStrategy::Idempotent(key) => Some(key.clone()),
            #[cfg(feature = "uuid")]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => {
                Some(uuid::Uuid::new_v4().to_string())
            }
            #[cfg(not(feature = "uuid"))]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => None,
        }
    }
}

fn calculate_backoff(retry_count: u32) -> Duration {
    Duration::from_secs(2_u64.pow(retry_count))
}

#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    Stop,
    Continue(Option<Duration>),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{Outcome, RequestStrategy};

    #[test]
    fn test_idempotent_strategy() {
        let strategy = RequestStrategy::Idempotent("key".to_string());
        assert_eq!(strategy.get_key(), Some("key".to_string()));
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
        assert!(Uuid::parse_str(&strategy.get_key().unwrap()).is_ok());
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
}
