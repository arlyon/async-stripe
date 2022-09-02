use crate::{error::StripeError, request_strategy::RequestStrategy};
use http_types::Request;
use serde::de::DeserializeOwned;

#[maybe_async::maybe_async(?Send)]
pub trait BaseClient {
    async fn execute<T: DeserializeOwned + Send + 'static>(
        &self,
        request: Request,
        strategy: &RequestStrategy,
    ) -> Result<T, StripeError>;
}
