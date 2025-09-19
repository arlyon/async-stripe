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
//! stripe trigger checkout.session.completed
//! ```

use axum::{
    Error, Router,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use stripe_webhook::{Event, EventObject, Webhook};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/stripe_webhooks", post(handle_webhook));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4242").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

struct StripeEvent(Event);

impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload =
            String::from_request(req, state).await.map_err(IntoResponse::into_response)?;

        Ok(Self(
            Webhook::construct_event(&payload, signature.to_str().unwrap(), "whsec_xxxxx")
                .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

#[axum::debug_handler]
async fn handle_webhook(StripeEvent(event): StripeEvent) {
    match event.data.object {
        EventObject::CheckoutSessionCompleted(session) => {
            println!("Received checkout session completed webhook with id: {:?}", session.id);
        }
        EventObject::AccountUpdated(account) => {
            println!("Received account updated webhook for account: {:?}", account.id);
        }
        _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
    }
}
