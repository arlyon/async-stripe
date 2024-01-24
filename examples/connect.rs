//! Stripe Connect
//! ==============
//!
//! Reference: <https://stripe.com/docs/api/account_links>
//!
//! This example shows how to manange a Stripe connect account,
//! initiating an account link which can be used to onboard a
//! new user to your application.
//!
//! Node: To get started, you'll need to make sure you have signed up to
//! use stripe connect and configure branding settings with an icon and a
//! brand color. See more: <https://dashboard.stripe.com/connect/accounts/overview>

use stripe::{
    Account, AccountLink, AccountLinkType, AccountType, Client, CreateAccount,
    CreateAccountCapabilities, CreateAccountCapabilitiesCardPayments,
    CreateAccountCapabilitiesTransfers, CreateAccountLink,
};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let account = Account::create(
        &client,
        CreateAccount {
            type_: Some(AccountType::Express),
            capabilities: Some(CreateAccountCapabilities {
                card_payments: Some(CreateAccountCapabilitiesCardPayments {
                    requested: Some(true),
                }),
                transfers: Some(CreateAccountCapabilitiesTransfers { requested: Some(true) }),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let link = AccountLink::create(
        &client,
        CreateAccountLink {
            account: account.id.clone(),
            type_: AccountLinkType::AccountOnboarding,
            collect: None,
            expand: &[],
            refresh_url: Some("https://test.com/refresh"),
            return_url: Some("https://test.com/return"),
            collection_options: None,
        },
    )
    .await
    .unwrap();

    println!("created a stripe connect link at {}", link.url);
}
