//! Web Hooks
//! =========
//!
//! Reference: https://stripe.com/docs/webhooks/test
//!
//! This example shows how to manage web hooks.
//! To trigger it, you can use the stripe cli.
//!
//! TLDR;
//! ```
//! stripe listen --forward-to localhost:8000/stripe_webhooks
//! Provide webhook secret to construct_event
//! stripe trigger checkout.session.completed
//! ```

#[macro_use]
extern crate rocket;
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use stripe::{CheckoutSession, EventObject, EventType, Webhook};

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![stripe_webhooks])
}

#[post("/stripe_webhooks", data = "<payload>")]
pub async fn stripe_webhooks(stripe_signature: StripeSignature<'_>, payload: Payload) -> Status {
    if let Ok(event) = Webhook::construct_event(
        &payload.contents,
        stripe_signature.signature,
        "webhook_secret_key",
    ) {
        match event.event_type {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    match checkout_session_completed(session) {
                        Ok(_) => Status::Accepted,
                        Err(_) => Status::BadRequest,
                    }
                } else {
                    Status::BadRequest
                }
            }
            _ => Status::Accepted,
        }
    } else {
        Status::BadRequest
    }
}

fn checkout_session_completed<'a>(session: CheckoutSession) -> Result<(), &'a str> {
    println!("Checkout Session Completed");
    println!("{:?}", session.id);
    Ok(())
}

pub struct Payload {
    pub contents: String,
}

#[derive(Debug)]
pub enum Error {
    TooLarge,
    NoColon,
    InvalidAge,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Payload {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;
        use Error::*;

        let limit = req.limits().get("form").unwrap_or_else(|| 1_000_000.bytes());

        let contents = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };
        Success(Payload { contents })
    }
}

pub struct StripeSignature<'a> {
    pub signature: &'a str,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StripeSignature<'r> {
    type Error = &'r str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Stripe-Signature") {
            None => Outcome::Failure((Status::BadRequest, "No signature provided")),
            Some(signature) => Outcome::Success(StripeSignature { signature }),
        }
    }
}
