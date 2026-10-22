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

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    error::{ErrorBadRequest, ErrorInternalServerError},
    post, web,
};
use stripe_connect::Account;
use stripe_webhook::{EventObject, Webhook, WebhookError};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(webhook_handler)).bind(("127.0.0.1", 4242))?.run().await
}

#[post("stripe_webhooks")]
pub async fn webhook_handler(
    req: HttpRequest,
    payload: web::Bytes,
) -> actix_web::Result<HttpResponse> {
    handle_webhook(req, payload)?;
    Ok(HttpResponse::Ok().finish())
}

pub fn handle_webhook(req: HttpRequest, payload: web::Bytes) -> actix_web::Result<()> {
    let payload_str = std::str::from_utf8(&payload).map_err(ErrorBadRequest)?;
    let stripe_signature = get_header_value(&req, "Stripe-Signature")
        .ok_or_else(|| ErrorBadRequest("missing or invalid Stripe-Signature header"))?;
    let event = Webhook::construct_event(payload_str, stripe_signature, "whsec_xxxxx")
        .map_err(ErrorBadRequest)?;

    match event.data.object {
        EventObject::AccountUpdated(account) => {
            handle_account_updated(*account).map_err(ErrorInternalServerError)?;
        }
        EventObject::CheckoutSessionCompleted(session) => {
            handle_checkout_session(*session).map_err(ErrorInternalServerError)?;
        }
        _ => {
            println!("Unknown event encountered in webhook: {:?}", event.type_);
        }
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

#[cfg(test)]
mod tests {
    use actix_web::{
        http::{StatusCode, header::HeaderValue},
        test,
    };

    use super::*;

    const VALID_PAYLOAD: &str = r#"{"id":"evt_test","object":"event","created":1492774577,"livemode":false,"pending_webhooks":1,"data":{"object":{"object":"bank_account","country":"us","currency":"usd","id":"ba_test","last4":"6789","status":"verified"}},"type":"account.external_account.created"}"#;

    async fn status(payload: Vec<u8>, signature: Option<HeaderValue>) -> StatusCode {
        let app = test::init_service(App::new().service(webhook_handler)).await;
        let mut request = test::TestRequest::post().uri("/stripe_webhooks").set_payload(payload);
        if let Some(signature) = signature {
            request = request.insert_header(("stripe-signature", signature));
        }
        test::call_service(&app, request.to_request()).await.status()
    }

    fn signature(payload: &str) -> HeaderValue {
        Webhook::generate_test_header(payload, "whsec_xxxxx", None).parse().unwrap()
    }

    #[actix_web::test]
    async fn accepts_valid_event() {
        assert_eq!(
            status(VALID_PAYLOAD.as_bytes().to_vec(), Some(signature(VALID_PAYLOAD))).await,
            StatusCode::OK
        );
    }

    #[actix_web::test]
    async fn rejects_invalid_signature() {
        assert_eq!(
            status(VALID_PAYLOAD.as_bytes().to_vec(), Some(HeaderValue::from_static("invalid")))
                .await,
            StatusCode::BAD_REQUEST
        );
    }

    #[actix_web::test]
    async fn rejects_missing_signature() {
        assert_eq!(status(VALID_PAYLOAD.as_bytes().to_vec(), None).await, StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
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

    #[actix_web::test]
    async fn rejects_non_utf8_payload() {
        assert_eq!(
            status(vec![0xff], Some(HeaderValue::from_static("invalid"))).await,
            StatusCode::BAD_REQUEST
        );
    }

    #[actix_web::test]
    async fn rejects_correctly_signed_invalid_event() {
        assert_eq!(status(b"{}".to_vec(), Some(signature("{}"))).await, StatusCode::BAD_REQUEST);
    }
}
