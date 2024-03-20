//! Stripe Connect
//! ==============
//!
//! Reference: <https://stripe.com/docs/api/account_links>
//!
//! This example shows how to manage a Stripe connect account,
//! initiating an account link which can be used to onboard a
//! new user to your application.
//!
//! Note: To get started, you'll need to make sure you have signed up to
//! use stripe connect and configure branding settings with an icon and a
//! brand color. See more: <https://dashboard.stripe.com/connect/accounts/overview>

use stripe::StripeError;
use stripe_connect::account::{CapabilitiesParam, CapabilityParam, CreateAccount};
use stripe_connect::account_link::{CreateAccountLink, CreateAccountLinkType};
use stripe_connect::AccountType;

pub async fn run_connect_example(client: &stripe::Client) -> Result<(), StripeError> {
    let account = CreateAccount {
        type_: Some(AccountType::Express),
        capabilities: Some(CapabilitiesParam {
            card_payments: Some(CapabilityParam { requested: Some(true) }),
            transfers: Some(CapabilityParam { requested: Some(true) }),
            ..Default::default()
        }),
        ..Default::default()
    }
    .send(client)
    .await?;

    let link = CreateAccountLink {
        account: &account.id,
        type_: CreateAccountLinkType::AccountOnboarding,
        refresh_url: Some("https://test.com/refresh"),
        return_url: Some("https://test.com/return"),
        expand: None,
        collect: None,
        collection_options: None,
    }
    .send(client)
    .await?;

    println!("created a stripe connect link at {}", link.url);
    Ok(())
}
