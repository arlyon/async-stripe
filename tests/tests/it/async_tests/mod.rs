use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;

use bytes::Bytes;
use futures_util::TryFutureExt;
use stripe::CustomizedStripeRequest;

use crate::{SECRET, STRIPE_MOCK_LINK};

mod pagination;

/// For testing async clients generically. This could be avoided if we could use
/// `&dyn StripeClient`, but currently traits with async functions cannot be object safe.
#[derive(Clone)]
pub enum StripeClient {
    Hyper(stripe::Client),
    AsyncStd(Box<stripe::async_std::Client>),
}

#[derive(Debug)]
// Code is used for debugging errors!
#[allow(dead_code)]
pub struct SimpleStripeClientError(String);

impl stripe_client_core::StripeClientErr for SimpleStripeClientError {
    fn deserialize_err(msg: impl Display) -> Self {
        SimpleStripeClientError(msg.to_string())
    }
}

impl stripe_client_core::StripeClient for StripeClient {
    type Err = SimpleStripeClientError;

    // See AFIT stabilization PR (https://github.com/rust-lang/rust/pull/115822)
    #[allow(refining_impl_trait)]
    fn execute(
        &self,
        req: CustomizedStripeRequest,
    ) -> Pin<Box<dyn Future<Output = Result<Bytes, SimpleStripeClientError>> + '_>> {
        match self {
            StripeClient::Hyper(c) => {
                Box::pin(c.execute(req).map_err(|err| SimpleStripeClientError(err.to_string())))
            }
            StripeClient::AsyncStd(c) => {
                Box::pin(c.execute(req).map_err(|err| SimpleStripeClientError(err.to_string())))
            }
        }
    }
}

impl StripeClient {
    fn hyper() -> Self {
        Self::Hyper(stripe::ClientBuilder::new(SECRET).url(STRIPE_MOCK_LINK).build().unwrap())
    }

    fn async_std() -> Self {
        Self::AsyncStd(Box::new(
            stripe::async_std::ClientBuilder::new(SECRET).url(STRIPE_MOCK_LINK).build().unwrap(),
        ))
    }
}

/// Run the test closure on all async clients
pub fn test_with_all_clients<TestFn, Fut>(test: TestFn)
where
    TestFn: Fn(StripeClient) -> Fut,
    Fut: Future<Output = ()>,
{
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        test(StripeClient::hyper()).await;
        test(StripeClient::async_std()).await;
    });
}
