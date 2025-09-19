//! Web Hooks
//! =========
//!
//! Reference: <https://stripe.com/docs/webhooks/test>
//!
//! This example shows how to manage web hooks.
//! To trigger it, you can use the stripe cli.
//!
//! TLDR;
//! ```
//! stripe listen --forward-to localhost:4242/stripe_webhooks
//! Provide webhook secret to construct_event
//! stripe trigger checkout.session.completed
//! stripe trigger account.updated
//! ```

use std::borrow::Borrow;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, post, web};
use stripe_connect::Account;
use stripe_webhook::{EventObject, Webhook, WebhookError};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(webhook_handler)).bind(("127.0.0.1", 4242))?.run().await
}

#[post("stripe_webhooks")]
pub async fn webhook_handler(req: HttpRequest, payload: web::Bytes) -> HttpResponse {
    handle_webhook(req, payload).unwrap();
    HttpResponse::Ok().finish()
}

pub fn handle_webhook(req: HttpRequest, payload: web::Bytes) -> Result<(), WebhookError> {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, "whsec_xxxxx") {
        match event.data.object {
            EventObject::AccountUpdated(account) => {
                handle_account_updated(account)?;
            }
            EventObject::CheckoutSessionCompleted(session) => {
                handle_checkout_session(session)?;
            }
            _ => {
                println!("Unknown event encountered in webhook: {:?}", event.type_);
            }
        }
    } else {
        println!("Failed to construct webhook event, ensure your webhook secret is correct.");
    }

    Ok(())
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}

fn handle_account_updated(account: Account) -> Result<(), WebhookError> {
    println!("Received account updated webhook for account: {:?}", account.id);
    Ok(())
}

fn handle_checkout_session(session: stripe_checkout::CheckoutSession) -> Result<(), WebhookError> {
    println!("Received checkout session completed webhook with id: {:?}", session.id);
    Ok(())
}
