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
use stripe_connect::account::{
    CapabilitiesParam, CapabilityParam, CreateAccount, CreateAccountType,
};
use stripe_connect::account_link::{CreateAccountLink, CreateAccountLinkType};

pub async fn run_connect_example(client: &stripe::Client) -> Result<(), StripeError> {
    let account = CreateAccount::new()
        .type_(CreateAccountType::Express)
        .capabilities(CapabilitiesParam {
            card_payments: Some(CapabilityParam { requested: Some(true) }),
            transfers: Some(CapabilityParam { requested: Some(true) }),
            ..Default::default()
        })
        .send(client)
        .await?;

    let link = CreateAccountLink::new(account.id, CreateAccountLinkType::AccountOnboarding)
        .refresh_url("https://test.com/refresh")
        .return_url("https://test.com/return")
        .send(client)
        .await?;

    println!("created a stripe connect link at {}", link.url);
    Ok(())
}
