//! Setup for the stripe mock service.

use stripe::Client;

pub fn get_client() -> Client {
    Client::from_url("http://localhost:12111", "sk_test_123")
}
