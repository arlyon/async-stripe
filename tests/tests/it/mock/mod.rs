//! Setup and teardown for the stripe mock service.

use stripe::Client;

#[allow(dead_code)]
pub fn with_client<T>(test: T) -> ()
where
    T: FnOnce(&Client) -> () + std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(|| {
        let client = get_client();
        test(&client)
    });

    assert!(result.is_ok())
}

#[allow(dead_code)]
pub fn get_client() -> Client {
    Client::from_url("http://localhost:12111", "sk_test_123")
}
