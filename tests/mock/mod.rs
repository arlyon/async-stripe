//! Setup and teardown for the stripe mock service.

use lazy_static::lazy_static;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref STRIPE_MOCK_PROCESS: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
}

pub fn with_client<T>(test: T) -> ()
where
    T: FnOnce(&stripe::Client) -> () + std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(|| {
        let client = stripe::Client::from_url("http://localhost:12111", "sk_test_123");
        test(&client)
    });

    assert!(result.is_ok())
}
