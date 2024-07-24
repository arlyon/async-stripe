// Needed for `json!` usage in tests
#![recursion_limit = "256"]

use stripe::ClientBuilder;

mod deser;
mod enums;
pub mod generated;
mod price;

mod async_tests;
mod blocking;

mod deserialization_fixture;
mod pagination_utils;

pub const STRIPE_MOCK_LINK: &str = "http://localhost:12111";
pub const SECRET: &str = "sk_test_123";

pub fn get_base_test_config() -> ClientBuilder {
    ClientBuilder::new(SECRET).url(STRIPE_MOCK_LINK)
}
