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

        let signature = signature.to_str().map_err(|_| StatusCode::BAD_REQUEST.into_response())?;
        let payload =
            String::from_request(req, state).await.map_err(IntoResponse::into_response)?;

        Ok(Self(
            Webhook::construct_event(&payload, signature, "whsec_xxxxx")
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

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    const VALID_PAYLOAD: &str = r#"{"id":"evt_test","object":"event","created":1492774577,"livemode":false,"pending_webhooks":1,"data":{"object":{"object":"bank_account","country":"us","currency":"usd","id":"ba_test","last4":"6789","status":"verified"}},"type":"account.external_account.created"}"#;

    async fn status(payload: Vec<u8>, signature: Option<HeaderValue>) -> StatusCode {
        let mut request = Request::builder().method("POST").uri("/stripe_webhooks");
        if let Some(signature) = signature {
            request = request.header("stripe-signature", signature);
        }
        let request = request.body(Body::from(payload)).unwrap();
        match StripeEvent::from_request(request, &()).await {
            Ok(StripeEvent(event)) => {
                assert_eq!(event.id.as_str(), "evt_test");
                StatusCode::OK
            }
            Err(response) => response.status(),
        }
    }

    fn signature(payload: &str) -> HeaderValue {
        Webhook::generate_test_header(payload, "whsec_xxxxx", None).parse().unwrap()
    }

    #[tokio::test]
    async fn accepts_valid_event() {
        assert_eq!(
            status(VALID_PAYLOAD.as_bytes().to_vec(), Some(signature(VALID_PAYLOAD))).await,
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn rejects_invalid_signature() {
        assert_eq!(
            status(VALID_PAYLOAD.as_bytes().to_vec(), Some(HeaderValue::from_static("invalid")))
                .await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn rejects_missing_signature() {
        assert_eq!(status(VALID_PAYLOAD.as_bytes().to_vec(), None).await, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn rejects_non_ascii_signature() {
        assert_eq!(
            status(
                VALID_PAYLOAD.as_bytes().to_vec(),
                Some(HeaderValue::from_bytes(&[0xff]).unwrap())
            )
            .await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn rejects_non_utf8_payload() {
        assert_eq!(
            status(vec![0xff], Some(HeaderValue::from_static("invalid"))).await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn rejects_correctly_signed_invalid_event() {
        assert_eq!(status(b"{}".to_vec(), Some(signature("{}"))).await, StatusCode::BAD_REQUEST);
    }
}
