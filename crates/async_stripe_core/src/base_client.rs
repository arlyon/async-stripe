use std::ops::{FromResidual, Try};

use http_types::Request;
use serde::de::DeserializeOwned;

use crate::{error::StripeError, request_strategy::RequestStrategy};

pub trait BaseClient {
    type Response<T>: FromResidual + Try;

    fn execute<T: DeserializeOwned + Send + 'static>(
        &self,
        request: Request,
        strategy: &RequestStrategy,
    ) -> Self::Response<T>;

    fn ok<T>(&self, e: T) -> Self::Response<T>;
    fn err<T>(&self, e: StripeError) -> Self::Response<T>;
    fn map<T, U, F: FnOnce(T) -> U>(&self, a: Self::Response<T>, fun: F) -> Self::Response<U>;
}
